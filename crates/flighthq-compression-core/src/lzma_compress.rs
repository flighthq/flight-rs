// LZMA1 encoding, alone-format header.
//
// Source of record: `upstream/packages/compression/src/lzmaCompress.ts` at upstream `5de055f94`, which is on
// `origin/develop`. Like `lzma` and `compress`, this mirrors a file that is NOT in the pinned submodule.
//
// Upstream's encoder is deterministic, so the oracle here is BYTE IDENTITY rather than round-tripping, and
// every search bound is therefore part of the contract: the 32-candidate chain, the 1<<25 window, the fixed
// lc=3/lp=0/pb=2 properties, the rep-slot preference order, and the exact hash. Changing any of them still
// round-trips and is still wrong.
//
// Two places where JavaScript's semantics are load-bearing and are reproduced rather than tidied:
//
//   * `rcLow` is a JS Number that exceeds 32 bits — `low += bound` is how the carry is carried — while
//     `(rcLow << 8) >>> 0` truncates to 32 bits first. So `low` is a `u64` whose high bit is the carry and
//     whose shift is deliberately taken on the low 32 bits only.
//   * `hashAt` reads `input[pos + 2]` under a guard of `pos + 2 <= len`, so the last position reads one past
//     the end. In JavaScript that yields `undefined`, which `^` coerces to 0; here it is an explicit
//     `unwrap_or(0)`. Dropping it changes the hash for the final position and therefore the output bytes.

const LZMA_HEADER_SIZE: usize = 13;
const LZMA_MIN_DICT_SIZE: u32 = 1 << 12;
const MAX_DICT_SIZE: u32 = 1 << 25;
const PROB_INIT: u16 = 1024;
const PROB_BITS: u32 = 11;
const MOVE_BITS: u32 = 5;
const TOP_VALUE: u32 = 1 << 24;
const NUM_STATES: usize = 12;
const NUM_LEN_TO_POS_STATES: usize = 4;
const NUM_ALIGN_BITS: u32 = 4;
const END_POS_MODEL_INDEX: u32 = 14;
const NUM_FULL_DISTANCES: usize = 1 << (END_POS_MODEL_INDEX >> 1);
const MATCH_MIN_LEN: usize = 2;
const MATCH_MAX_LEN: usize = MATCH_MIN_LEN + 271;
const HASH_BITS: u32 = 15;
const HASH_MASK: usize = (1 << HASH_BITS) - 1;
const HASH_SIZE: usize = 1 << HASH_BITS;
const MAX_CHAIN: usize = 32;
const MAX_DISTANCE: usize = 1 << 25;

const LC: u32 = 3;
const LP: u32 = 0;
const PB: u32 = 2;

const STATE_AFTER_LITERAL: [usize; 12] = [0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 4, 5];
const STATE_AFTER_MATCH: [usize; 12] = [7, 7, 7, 7, 7, 7, 7, 10, 10, 10, 10, 10];
const STATE_AFTER_REP: [usize; 12] = [8, 8, 8, 8, 8, 8, 8, 11, 11, 11, 11, 11];
const STATE_AFTER_SHORT_REP: [usize; 12] = [9, 9, 9, 9, 9, 9, 9, 11, 11, 11, 11, 11];

/// Encodes `bytes` as an LZMA1 alone-format stream, byte-identically to upstream's `compressLzma`.
pub fn compress_lzma(bytes: &[u8]) -> Vec<u8> {
    Encoder::new(bytes).run()
}

/// What `findMatch` chose: a literal, a normal match, a one-byte short rep, or one of the four rep slots.
enum Choice {
    Literal,
    Match { length: usize, distance: usize },
    ShortRep,
    Rep { slot: usize, length: usize },
}

/// Probability models, sized by the fixed lc/lp/pb this encoder emits.
struct Models {
    is_match: Vec<u16>,
    is_rep: Vec<u16>,
    is_rep_g0: Vec<u16>,
    is_rep_g1: Vec<u16>,
    is_rep_g2: Vec<u16>,
    is_rep0_long: Vec<u16>,
    literal: Vec<u16>,
    match_len_choice: Vec<u16>,
    match_len_low: Vec<u16>,
    match_len_mid: Vec<u16>,
    match_len_high: Vec<u16>,
    rep_len_choice: Vec<u16>,
    rep_len_low: Vec<u16>,
    rep_len_mid: Vec<u16>,
    rep_len_high: Vec<u16>,
    dist_slot: Vec<u16>,
    pos: Vec<u16>,
    align: Vec<u16>,
}

struct Encoder<'a> {
    input: &'a [u8],
    out: Vec<u8>,
    low: u64,
    range: u32,
    cache: u8,
    cache_size: u64,
    models: Models,
    head: Vec<i32>,
    previous: Vec<i32>,
    position_states: usize,
}

fn probs(count: usize) -> Vec<u16> {
    vec![PROB_INIT; count]
}

fn nearest_power_of_two(n: usize) -> u32 {
    if n <= LZMA_MIN_DICT_SIZE as usize {
        return LZMA_MIN_DICT_SIZE;
    }
    let mut p = LZMA_MIN_DICT_SIZE;
    while (p as usize) < n && p < MAX_DICT_SIZE {
        p <<= 1;
    }
    p
}

impl<'a> Encoder<'a> {
    fn new(input: &'a [u8]) -> Self {
        let position_states = 1usize << PB;
        Self {
            input,
            out: Vec::with_capacity(input.len() + LZMA_HEADER_SIZE + 64),
            low: 0,
            range: 0xffff_ffff,
            cache: 0,
            cache_size: 1,
            models: Models {
                is_match: probs(NUM_STATES * position_states),
                is_rep: probs(NUM_STATES),
                is_rep_g0: probs(NUM_STATES),
                is_rep_g1: probs(NUM_STATES),
                is_rep_g2: probs(NUM_STATES),
                is_rep0_long: probs(NUM_STATES * position_states),
                literal: probs((1usize << (LC + LP)) * 0x300),
                match_len_choice: probs(2),
                match_len_low: probs(position_states << 3),
                match_len_mid: probs(position_states << 3),
                match_len_high: probs(256),
                rep_len_choice: probs(2),
                rep_len_low: probs(position_states << 3),
                rep_len_mid: probs(position_states << 3),
                rep_len_high: probs(256),
                dist_slot: probs(NUM_LEN_TO_POS_STATES * 64),
                pos: probs(NUM_FULL_DISTANCES - END_POS_MODEL_INDEX as usize),
                align: probs(1usize << NUM_ALIGN_BITS),
            },
            head: vec![-1i32; HASH_SIZE],
            previous: vec![-1i32; input.len().max(1)],
            position_states,
        }
    }

    fn write_header(&mut self) {
        let dictionary = nearest_power_of_two(self.input.len());
        self.out.push((PB * 45 + LP * 9 + LC) as u8);
        self.out.extend_from_slice(&dictionary.to_le_bytes());
        self.out
            .extend_from_slice(&(self.input.len() as u32).to_le_bytes());
        self.out.extend_from_slice(&[0, 0, 0, 0]);
    }

    /// Flushes one byte of the range coder's pending output, carrying into the cached byte.
    fn shift_low(&mut self) {
        let low_high = (self.low >> 32) as u8;
        if low_high != 0 || self.low < 0xff00_0000 {
            let mut temp = self.cache;
            loop {
                self.out.push(temp.wrapping_add(low_high));
                temp = 0xff;
                self.cache_size -= 1;
                if self.cache_size == 0 {
                    break;
                }
            }
            self.cache = ((self.low as u32) >> 24) as u8;
        }
        self.cache_size += 1;
        self.low = u64::from((self.low as u32) << 8);
    }

    fn encode_bit(&mut self, model: &mut [u16], index: usize, bit: u32) {
        let bound = (self.range >> PROB_BITS).wrapping_mul(u32::from(model[index]));
        if bit == 0 {
            self.range = bound;
            model[index] += ((1 << PROB_BITS) - model[index]) >> MOVE_BITS;
        } else {
            self.low += u64::from(bound);
            self.range -= bound;
            model[index] -= model[index] >> MOVE_BITS;
        }
        if self.range < TOP_VALUE {
            self.range <<= 8;
            self.shift_low();
        }
    }

    fn encode_direct_bits(&mut self, value: u32, count: u32) {
        for index in (0..count).rev() {
            self.range >>= 1;
            if (value >> index) & 1 == 1 {
                self.low += u64::from(self.range);
            }
            if self.range < TOP_VALUE {
                self.range <<= 8;
                self.shift_low();
            }
        }
    }

    fn run(mut self) -> Vec<u8> {
        self.write_header();

        let mut state = 0usize;
        let mut reps = [0usize; 4];
        let mut position = 0usize;

        while position < self.input.len() {
            let position_state = position & (self.position_states - 1);
            let choice = self.find_match(position, &reps);
            match choice {
                Choice::Literal => {
                    self.encode_is_match(state, position_state, 0);
                    let byte = self.input[position];
                    self.encode_literal(position, byte, state, reps[0]);
                    state = STATE_AFTER_LITERAL[state];
                    position += 1;
                }
                Choice::Match { length, distance } => {
                    self.encode_is_match(state, position_state, 1);
                    self.encode_is_rep(state, 0);
                    self.encode_match_length(position_state, length - MATCH_MIN_LEN);
                    state = STATE_AFTER_MATCH[state];
                    let length_state = (length - MATCH_MIN_LEN).min(NUM_LEN_TO_POS_STATES - 1);
                    self.encode_distance(distance, length_state);
                    reps = [distance, reps[0], reps[1], reps[2]];
                    self.insert_interior(position, length);
                    position += length;
                }
                Choice::ShortRep => {
                    self.encode_is_match(state, position_state, 1);
                    self.encode_is_rep(state, 1);
                    let index = state;
                    self.encode_bit_in(Model::IsRepG0, index, 0);
                    let slot = state * self.position_states + position_state;
                    self.encode_bit_in(Model::IsRep0Long, slot, 0);
                    state = STATE_AFTER_SHORT_REP[state];
                    position += 1;
                }
                Choice::Rep { slot, length } => {
                    self.encode_is_match(state, position_state, 1);
                    self.encode_is_rep(state, 1);
                    if slot == 0 {
                        self.encode_bit_in(Model::IsRepG0, state, 0);
                        let long = state * self.position_states + position_state;
                        self.encode_bit_in(Model::IsRep0Long, long, 1);
                    } else {
                        self.encode_bit_in(Model::IsRepG0, state, 1);
                        if slot == 1 {
                            self.encode_bit_in(Model::IsRepG1, state, 0);
                        } else {
                            self.encode_bit_in(Model::IsRepG1, state, 1);
                            self.encode_bit_in(Model::IsRepG2, state, u32::from(slot != 2));
                        }
                        // The encoder's rep shuffle, which upstream had right all along: a slot only
                        // displaces the slots above it.
                        let chosen = reps[slot];
                        for index in (1..=slot).rev() {
                            reps[index] = reps[index - 1];
                        }
                        reps[0] = chosen;
                    }
                    self.encode_rep_length(position_state, length - MATCH_MIN_LEN);
                    state = STATE_AFTER_REP[state];
                    self.insert_interior(position, length);
                    position += length;
                }
            }
        }

        for _ in 0..5 {
            self.shift_low();
        }
        self.out
    }

    fn encode_is_match(&mut self, state: usize, position_state: usize, bit: u32) {
        let index = state * self.position_states + position_state;
        let mut model = std::mem::take(&mut self.models.is_match);
        self.encode_bit(&mut model, index, bit);
        self.models.is_match = model;
    }

    fn encode_is_rep(&mut self, state: usize, bit: u32) {
        let mut model = std::mem::take(&mut self.models.is_rep);
        self.encode_bit(&mut model, state, bit);
        self.models.is_rep = model;
    }

    fn encode_bit_in(&mut self, which: Model, index: usize, bit: u32) {
        let mut model = which.take(&mut self.models);
        self.encode_bit(&mut model, index, bit);
        which.put(&mut self.models, model);
    }

    fn encode_tree(&mut self, which: Model, offset: usize, bits: u32, value: u32) {
        let mut model = which.take(&mut self.models);
        let mut m = 1u32;
        for index in (0..bits).rev() {
            let bit = (value >> index) & 1;
            self.encode_bit(&mut model, offset + m as usize, bit);
            m = (m << 1) | bit;
        }
        which.put(&mut self.models, model);
    }

    /// The reverse tree, whose model base upstream writes as `base - distSlot - 1` and which is legitimately
    /// -1 for slot 4. Indexing `base + m - 1` from `base - distSlot` is the same element unsigned.
    fn encode_tree_reverse(&mut self, which: Model, base: usize, bits: u32, value: u32) {
        let mut model = which.take(&mut self.models);
        let mut m = 1u32;
        for index in 0..bits {
            let bit = (value >> index) & 1;
            self.encode_bit(&mut model, base + m as usize - 1, bit);
            m = (m << 1) | bit;
        }
        which.put(&mut self.models, model);
    }

    fn encode_match_length(&mut self, position_state: usize, length: usize) {
        self.encode_length(
            position_state,
            length,
            Model::MatchLenChoice,
            Model::MatchLenLow,
            Model::MatchLenMid,
            Model::MatchLenHigh,
        );
    }

    fn encode_rep_length(&mut self, position_state: usize, length: usize) {
        self.encode_length(
            position_state,
            length,
            Model::RepLenChoice,
            Model::RepLenLow,
            Model::RepLenMid,
            Model::RepLenHigh,
        );
    }

    fn encode_length(
        &mut self,
        position_state: usize,
        length: usize,
        choice: Model,
        low: Model,
        mid: Model,
        high: Model,
    ) {
        if length < 8 {
            self.encode_bit_in(choice, 0, 0);
            self.encode_tree(low, position_state << 3, 3, length as u32);
        } else if length < 16 {
            self.encode_bit_in(choice, 0, 1);
            self.encode_bit_in(choice, 1, 0);
            self.encode_tree(mid, position_state << 3, 3, (length - 8) as u32);
        } else {
            self.encode_bit_in(choice, 0, 1);
            self.encode_bit_in(choice, 1, 1);
            self.encode_tree(high, 0, 8, (length - 16) as u32);
        }
    }

    fn encode_literal(&mut self, position: usize, byte: u8, state: usize, rep0: usize) {
        let previous = if position > 0 {
            self.input[position - 1]
        } else {
            0
        };
        let literal_state =
            ((position & ((1usize << LP) - 1)) << LC) + (usize::from(previous) >> (8 - LC));
        let offset = literal_state * 0x300;
        let mut model = std::mem::take(&mut self.models.literal);

        if state >= 7 {
            let mut match_byte = u32::from(self.input[position - rep0 - 1]);
            let mut symbol = 1u32;
            for bit_index in 0..8 {
                let match_bit = (match_byte >> 7) & 1;
                match_byte = (match_byte << 1) & 0xff;
                let bit = (u32::from(byte) >> (7 - bit_index)) & 1;
                self.encode_bit(
                    &mut model,
                    offset + ((1 + match_bit as usize) << 8) + symbol as usize,
                    bit,
                );
                symbol = (symbol << 1) | bit;
                if match_bit != bit {
                    for rest in (bit_index + 1)..8 {
                        let bit = (u32::from(byte) >> (7 - rest)) & 1;
                        self.encode_bit(&mut model, offset + symbol as usize, bit);
                        symbol = (symbol << 1) | bit;
                    }
                    break;
                }
            }
        } else {
            let mut symbol = 1u32;
            for bit_index in 0..8 {
                let bit = (u32::from(byte) >> (7 - bit_index)) & 1;
                self.encode_bit(&mut model, offset + symbol as usize, bit);
                symbol = (symbol << 1) | bit;
            }
        }
        self.models.literal = model;
    }

    fn distance_slot(distance: usize) -> u32 {
        if distance < 4 {
            return distance as u32;
        }
        let mut bits = 1u32;
        let mut value = distance;
        while value >= 4 {
            value >>= 1;
            bits += 1;
        }
        (bits << 1) + (((distance >> (bits - 1)) & 1) as u32)
    }

    fn encode_distance(&mut self, distance: usize, length_state: usize) {
        let slot = Self::distance_slot(distance);
        self.encode_tree(Model::DistSlot, length_state * 64, 6, slot);
        if slot < 4 {
            return;
        }
        let direct_bits = (slot >> 1) - 1;
        let base = ((2 | (slot & 1)) << direct_bits) as usize;
        let remainder = (distance - base) as u32;
        if slot < END_POS_MODEL_INDEX {
            self.encode_tree_reverse(Model::Pos, base - slot as usize, direct_bits, remainder);
        } else {
            self.encode_direct_bits(remainder >> NUM_ALIGN_BITS, direct_bits - NUM_ALIGN_BITS);
            self.encode_tree_reverse(
                Model::Align,
                1,
                NUM_ALIGN_BITS,
                remainder & ((1 << NUM_ALIGN_BITS) - 1),
            );
        }
    }

    /// Reads one past the end at the final position, exactly as upstream's `undefined` coercion does.
    fn hash_at(&self, position: usize) -> usize {
        let a = usize::from(self.input[position]);
        let b = usize::from(self.input.get(position + 1).copied().unwrap_or(0));
        let c = usize::from(self.input.get(position + 2).copied().unwrap_or(0));
        ((a << 10) ^ (b << 5) ^ c) & HASH_MASK
    }

    fn insert_hash(&mut self, position: usize) {
        if position + MATCH_MIN_LEN <= self.input.len() {
            let key = self.hash_at(position);
            self.previous[position] = self.head[key];
            self.head[key] = position as i32;
        }
    }

    fn insert_interior(&mut self, position: usize, length: usize) {
        for offset in 1..length {
            self.insert_hash(position + offset);
        }
    }

    fn run_length(&self, a: usize, b: usize, limit: usize) -> usize {
        let mut length = 0usize;
        while length < limit && self.input[a + length] == self.input[b + length] {
            length += 1;
        }
        length
    }

    fn find_match(&mut self, position: usize, reps: &[usize; 4]) -> Choice {
        let max_length = MATCH_MAX_LEN.min(self.input.len() - position);
        if max_length < MATCH_MIN_LEN {
            return Choice::Literal;
        }

        let mut best_length = 1usize;
        let mut best: Option<Choice> = None;

        for (slot, &distance) in reps.iter().enumerate() {
            if distance >= position {
                continue;
            }
            let length = self.run_length(position, position - distance - 1, max_length);
            if length >= MATCH_MIN_LEN && length > best_length {
                best_length = length;
                best = Some(Choice::Rep { slot, length });
                if length == max_length {
                    return Choice::Rep { slot, length };
                }
            }
        }

        if best.is_none()
            && reps[0] < position
            && self.input[position] == self.input[position - reps[0] - 1]
        {
            best = Some(Choice::ShortRep);
            best_length = 1;
        }
        if position + MATCH_MIN_LEN <= self.input.len() {
            let key = self.hash_at(position);
            let mut candidate = self.head[key];
            let mut attempts = 0usize;
            while candidate >= 0 && attempts < MAX_CHAIN {
                let candidate_position = candidate as usize;
                let distance = position - candidate_position - 1;
                if distance >= MAX_DISTANCE {
                    break;
                }
                let length = self.run_length(candidate_position, position, max_length);
                if length >= MATCH_MIN_LEN && length > best_length {
                    best_length = length;
                    best = Some(Choice::Match { distance, length });
                    if length == max_length {
                        break;
                    }
                }
                candidate = self.previous[candidate_position];
                attempts += 1;
            }
            self.previous[position] = self.head[key];
            self.head[key] = position as i32;
        }

        best.unwrap_or(Choice::Literal)
    }
}

/// Which probability model an operation touches. The indirection exists so the range coder can borrow the
/// encoder mutably while a model is temporarily moved out, which keeps the borrow checker satisfied without
/// changing a single emitted bit.
#[derive(Clone, Copy)]
enum Model {
    IsRepG0,
    IsRepG1,
    IsRepG2,
    IsRep0Long,
    MatchLenChoice,
    MatchLenLow,
    MatchLenMid,
    MatchLenHigh,
    RepLenChoice,
    RepLenLow,
    RepLenMid,
    RepLenHigh,
    DistSlot,
    Pos,
    Align,
}

impl Model {
    fn take(self, models: &mut Models) -> Vec<u16> {
        std::mem::take(self.slot(models))
    }

    fn put(self, models: &mut Models, value: Vec<u16>) {
        *self.slot(models) = value;
    }

    fn slot(self, models: &mut Models) -> &mut Vec<u16> {
        match self {
            Self::IsRepG0 => &mut models.is_rep_g0,
            Self::IsRepG1 => &mut models.is_rep_g1,
            Self::IsRepG2 => &mut models.is_rep_g2,
            Self::IsRep0Long => &mut models.is_rep0_long,
            Self::MatchLenChoice => &mut models.match_len_choice,
            Self::MatchLenLow => &mut models.match_len_low,
            Self::MatchLenMid => &mut models.match_len_mid,
            Self::MatchLenHigh => &mut models.match_len_high,
            Self::RepLenChoice => &mut models.rep_len_choice,
            Self::RepLenLow => &mut models.rep_len_low,
            Self::RepLenMid => &mut models.rep_len_mid,
            Self::RepLenHigh => &mut models.rep_len_high,
            Self::DistSlot => &mut models.dist_slot,
            Self::Pos => &mut models.pos,
            Self::Align => &mut models.align,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deflate::compute_adler32;
    use crate::{Framing, decompress_lzma};

    // Same oracle and same corpus as the DEFLATE encoder: upstream's output is deterministic, so BYTE
    // IDENTITY is the contract rather than round-tripping. Each case carries upstream's output length and
    // Adler-32, produced by running `compressLzma` at `5de055f94` over this corpus. The corpus generator is
    // transcribed on both sides rather than shared, so neither implementation can drift into agreement
    // through a common helper.

    fn lcg(seed: u32, count: usize) -> Vec<u8> {
        let mut state = seed;
        (0..count)
            .map(|_| {
                state = state.wrapping_mul(1103515245).wrapping_add(12345) & 0x7fff_ffff;
                ((state >> 16) & 0xff) as u8
            })
            .collect()
    }

    fn words(count: usize) -> Vec<u8> {
        const LIST: [&str; 10] = [
            "scene",
            "bitmap",
            "node",
            "transform",
            "matrix",
            "render",
            "surface",
            "vertex",
            "buffer",
            "shader",
        ];
        let mut state: u32 = 12345;
        let mut out = String::new();
        while out.len() < count {
            state = state.wrapping_mul(1103515245).wrapping_add(12345) & 0x7fff_ffff;
            out.push_str(LIST[state as usize % LIST.len()]);
            out.push_str(&(state % 97).to_string());
            out.push(' ');
        }
        out.into_bytes()[..count].to_vec()
    }

    fn cycle(period: usize, count: usize) -> Vec<u8> {
        (0..count).map(|index| (index % period) as u8).collect()
    }

    fn corpus() -> Vec<(&'static str, Vec<u8>, usize, u32)> {
        vec![
            ("empty", Vec::new(), 18, 0x079c_006e),
            ("one-byte", b"a".to_vec(), 19, 0x0df8_021a),
            (
                "short-literal",
                b"flighthq scene-formats".to_vec(),
                40,
                0x8cf0_0b9c,
            ),
            ("all-byte-values", cycle(256, 256), 248, 0x026a_7333),
            ("cycle-256-x1024", cycle(256, 1024), 255, 0x482c_775a),
            (
                "abcABC123-x600",
                "abcABC123".repeat(600).into_bytes(),
                55,
                0xc7eb_14da,
            ),
            ("single-byte-run-600", vec![b'A'; 600], 24, 0x34b4_0674),
            ("incompressible-4096", lcg(1, 4096), 4170, 0xd21c_e28b),
            ("incompressible-70000", lcg(7, 70000), 71047, 0xec0a_3d5e),
            ("cycle-251-x40000", cycle(251, 40000), 311, 0x5290_961b),
            ("words-2304", words(2304), 790, 0x031e_7849),
            ("words-65536", words(65536), 15618, 0x97fa_183e),
        ]
    }

    #[test]
    fn emits_byte_identical_output_to_upstream() {
        for (name, input, length, adler) in corpus() {
            let out = compress_lzma(&input);
            assert_eq!(out.len(), length, "{name}: output length");
            assert_eq!(compute_adler32(&out), adler, "{name}: output bytes");
        }
    }

    #[test]
    fn round_trips_every_case_through_the_mirror_decoder() {
        // Byte identity says the encoder agrees with upstream. This says the two halves of this crate agree
        // with each other, which byte identity alone would not catch if both were wrong the same way.
        for (name, input, _, _) in corpus() {
            assert_eq!(
                decompress_lzma(&compress_lzma(&input), input.len(), Framing::Raw).as_deref(),
                Some(input.as_slice()),
                "{name}: round-trip"
            );
            assert_eq!(
                decompress_lzma(&compress_lzma(&input), 0, Framing::Raw).as_deref(),
                Some(input.as_slice()),
                "{name}: round-trip with no declared length"
            );
        }
    }

    #[test]
    fn writes_the_alone_format_header_upstream_writes() {
        // lc=3, lp=0, pb=2 gives the properties byte 93, and the declared size is the input length rather
        // than the unknown marker — a decoder that trusted a wrong one here would stop in the wrong place.
        let out = compress_lzma(b"flighthq");
        assert_eq!(out[0], 93);
        assert_eq!(u32::from_le_bytes([out[5], out[6], out[7], out[8]]), 8);
        assert_eq!(&out[9..13], &[0, 0, 0, 0]);
        assert!(u32::from_le_bytes([out[1], out[2], out[3], out[4]]) >= 1 << 12);
    }

    #[test]
    fn is_deterministic_and_leaves_its_input_alone() {
        let input = words(4096);
        let before = input.clone();
        assert_eq!(compress_lzma(&input), compress_lzma(&input));
        assert_eq!(input, before);
    }
}
