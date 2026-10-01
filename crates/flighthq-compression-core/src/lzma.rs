// LZMA1 decoding, alone-format header.
//
// Source of record: `upstream/packages/compression/src/lzma.ts` as of upstream `5de055f94`, which is on
// `origin/develop` rather than `origin/main`. NOTE that this module mirrors a file that is NOT in the
// pinned submodule — LZMA landed upstream after the current pin — so the behaviour this reproduces is the
// behaviour that arrives when the pin moves. The fixtures are embedded for exactly that reason: nothing
// here reads a tree the repository does not check out.
//
// Upstream's shape that is NOT carried over: closures over mutable range-coder state, probability arrays
// reached through captured variables, and a throw-and-catch boundary standing in for a refusal. Every
// decoded symbol, every probability update, and every refusal is identical.
//
// The range coder is deliberately written with explicit `u32` wrapping where upstream relies on `>>> 0`.
// That is not defensive: `(range >> 11) * prob` reaches 4,292,870,145, inside `u32` but close enough that
// a widened intermediate would diverge from JavaScript the moment upstream's constants changed.

use crate::Framing;

/// The ceiling on a single LZMA decode, mirroring upstream's `MAX_LZMA_BYTES`.
pub const MAX_LZMA_BYTES: usize = 256 * 1024 * 1024;

const LZMA_HEADER_SIZE: usize = 13;
const LZMA_MIN_DICT_SIZE: u32 = 1 << 12;
const INITIAL_LZMA_BYTES: usize = 1024;
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

const STATE_AFTER_LITERAL: [usize; 12] = [0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 4, 5];
const STATE_AFTER_MATCH: [usize; 12] = [7, 7, 7, 7, 7, 7, 7, 10, 10, 10, 10, 10];
const STATE_AFTER_REP: [usize; 12] = [8, 8, 8, 8, 8, 8, 8, 11, 11, 11, 11, 11];
const STATE_AFTER_SHORT_REP: [usize; 12] = [9, 9, 9, 9, 9, 9, 9, 11, 11, 11, 11, 11];

/// Decompresses an LZMA1 alone-format stream, or returns `None` when it is malformed.
///
/// Only `Framing::Raw` is accepted: LZMA carries no wrapper, and upstream refuses any other framing
/// rather than ignoring the argument. `uncompressed_length` is the container-declared length, or 0 for
/// none; when the header also declares one, the two must agree.
pub fn decompress_lzma(
    compressed: &[u8],
    uncompressed_length: usize,
    framing: Framing,
) -> Option<Vec<u8>> {
    if framing != Framing::Raw {
        return None;
    }
    if compressed.len() < LZMA_HEADER_SIZE {
        return None;
    }
    decode(compressed, uncompressed_length)
}

/// Literal, match and rep probability models, sized by the `lc`/`lp`/`pb` properties in the header.
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

fn probs(count: usize) -> Vec<u16> {
    vec![PROB_INIT; count]
}

/// The binary range decoder. Upstream keeps this state in closure variables; the only change here is
/// where it lives.
struct RangeDecoder<'a> {
    input: &'a [u8],
    position: usize,
    range: u32,
    code: u32,
}

impl<'a> RangeDecoder<'a> {
    fn normalize(&mut self) {
        if self.range < TOP_VALUE {
            self.range <<= 8;
            // Reading past the end yields zero and still advances, exactly as upstream does. A stream that
            // relies on it is caught by the size and end-marker checks rather than here.
            let byte = self.input.get(self.position).copied().unwrap_or(0);
            self.code = (self.code << 8) | u32::from(byte);
            self.position += 1;
        }
    }

    fn decode_bit(&mut self, model: &mut [u16], index: usize) -> u32 {
        self.normalize();
        let bound = (self.range >> PROB_BITS).wrapping_mul(u32::from(model[index]));
        if self.code < bound {
            self.range = bound;
            model[index] += ((1 << PROB_BITS) - model[index]) >> MOVE_BITS;
            0
        } else {
            self.range -= bound;
            self.code -= bound;
            model[index] -= model[index] >> MOVE_BITS;
            1
        }
    }

    fn decode_direct_bits(&mut self, count: u32) -> u32 {
        let mut result = 0u32;
        for _ in 0..count {
            self.normalize();
            self.range >>= 1;
            result <<= 1;
            if self.code >= self.range {
                self.code -= self.range;
                result |= 1;
            }
        }
        result
    }

    fn decode_bit_tree(&mut self, model: &mut [u16], offset: usize, bits: u32) -> u32 {
        let mut m = 1u32;
        for _ in 0..bits {
            m = (m << 1) | self.decode_bit(model, offset + m as usize);
        }
        m - (1 << bits)
    }

    fn decode_bit_tree_reverse(&mut self, model: &mut [u16], offset: usize, bits: u32) -> u32 {
        let mut m = 1u32;
        let mut symbol = 0u32;
        for index in 0..bits {
            let bit = self.decode_bit(model, offset + m as usize);
            m = (m << 1) | bit;
            symbol |= bit << index;
        }
        symbol
    }

    /// The reverse bit tree used for the middle distance slots, whose model base upstream writes as
    /// `dist - distSlot - 1`.
    ///
    /// That expression is legitimately **-1** — slot 4 gives `dist == distSlot` — and JavaScript gets away
    /// with it because the tree index starts at 1, so `offset + m` lands on element 0. Taking `base` as
    /// `dist - distSlot` and indexing `base + m - 1` is the same element with no negative intermediate.
    /// Writing it as an unsigned subtraction instead refuses every stream whose first distance uses slot 4,
    /// which is most of them; upstream's own LOREM fixture caught exactly that.
    fn decode_distance_tail(&mut self, model: &mut [u16], base: usize, bits: u32) -> u32 {
        let mut m = 1u32;
        let mut symbol = 0u32;
        for index in 0..bits {
            let bit = self.decode_bit(model, base + m as usize - 1);
            m = (m << 1) | bit;
            symbol |= bit << index;
        }
        symbol
    }
}

/// The alone-format header: one properties byte, a little-endian dictionary size, and a little-endian
/// 64-bit uncompressed size where all-ones means "unknown".
struct Header {
    lc: u32,
    lp: u32,
    pb: u32,
    dict_size: u32,
    declared_size: Option<usize>,
}

fn read_u32_le(bytes: &[u8]) -> u32 {
    u32::from(bytes[0])
        | (u32::from(bytes[1]) << 8)
        | (u32::from(bytes[2]) << 16)
        | (u32::from(bytes[3]) << 24)
}

fn read_header(input: &[u8]) -> Option<Header> {
    let properties = u32::from(input[0]);
    if properties >= 225 {
        return None;
    }
    let lc = properties % 9;
    let remainder = (properties - lc) / 9;
    let lp = remainder % 5;
    let pb = (remainder - lp) / 5;

    let dict_size = read_u32_le(&input[1..5]).max(LZMA_MIN_DICT_SIZE);

    let size_low = read_u32_le(&input[5..9]);
    let size_high = read_u32_le(&input[9..13]);
    let declared_size = if size_low == 0xffff_ffff && size_high == 0xffff_ffff {
        None
    } else {
        // Upstream refuses rather than truncating: a size above 2^32 is not a stream this decoder can
        // produce, and silently decoding its low half would hand back a prefix wearing a full size.
        if size_high > 0 {
            return None;
        }
        Some(size_low as usize)
    };

    Some(Header {
        lc,
        lp,
        pb,
        dict_size,
        declared_size,
    })
}

fn decode(input: &[u8], uncompressed_length: usize) -> Option<Vec<u8>> {
    let header = read_header(input)?;

    // When both the caller and the header name a size they must agree. Trusting either silently would
    // let a container and its payload disagree about what was decoded.
    if uncompressed_length > 0
        && let Some(declared) = header.declared_size
        && uncompressed_length != declared
    {
        return None;
    }

    let expected = match (header.declared_size, uncompressed_length) {
        (Some(declared), _) => Some(declared),
        (None, 0) => None,
        (None, caller) => Some(caller),
    };
    if expected.is_some_and(|size| size > MAX_LZMA_BYTES) {
        return None;
    }

    let range_start = LZMA_HEADER_SIZE;
    if range_start + 5 > input.len() {
        return None;
    }
    if input[range_start] != 0x00 {
        return None;
    }
    let mut decoder = RangeDecoder {
        input,
        position: range_start + 5,
        range: 0xffff_ffff,
        code: (u32::from(input[range_start + 1]) << 24)
            | (u32::from(input[range_start + 2]) << 16)
            | (u32::from(input[range_start + 3]) << 8)
            | u32::from(input[range_start + 4]),
    };

    let position_states = 1usize << header.pb;
    let position_mask = position_states - 1;
    let mut models = Models {
        is_match: probs(NUM_STATES * position_states),
        is_rep: probs(NUM_STATES),
        is_rep_g0: probs(NUM_STATES),
        is_rep_g1: probs(NUM_STATES),
        is_rep_g2: probs(NUM_STATES),
        is_rep0_long: probs(NUM_STATES * position_states),
        literal: probs((1usize << (header.lc + header.lp)) * 0x300),
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
    };

    let output_limit = expected.unwrap_or(MAX_LZMA_BYTES);
    // Only a starting capacity: with no declared size there is nothing to size the buffer from but the
    // dictionary, and the cap above is what actually bounds the decode. `clamp` is sound here because the
    // bounds are constants in the right order.
    let initial = expected
        .unwrap_or_else(|| (header.dict_size as usize).clamp(INITIAL_LZMA_BYTES, MAX_LZMA_BYTES));
    let mut output: Vec<u8> = Vec::with_capacity(initial);

    let mut state = 0usize;
    let (mut rep0, mut rep1, mut rep2, mut rep3) = (0usize, 0usize, 0usize, 0usize);

    loop {
        if let Some(size) = expected
            && output.len() >= size
        {
            break;
        }
        let position_state = output.len() & position_mask;

        if decoder.decode_bit(
            &mut models.is_match,
            state * position_states + position_state,
        ) == 0
        {
            let symbol = decode_literal(&mut decoder, &mut models, &header, &output, state, rep0)?;
            push(&mut output, output_limit, symbol)?;
            state = STATE_AFTER_LITERAL[state];
            continue;
        }

        let length;
        if decoder.decode_bit(&mut models.is_rep, state) == 0 {
            length = MATCH_MIN_LEN
                + decode_match_length(&mut decoder, &mut models, position_state) as usize;
            state = STATE_AFTER_MATCH[state];

            let length_state = (length - MATCH_MIN_LEN).min(NUM_LEN_TO_POS_STATES - 1);
            let distance = decode_distance(&mut decoder, &mut models, length_state)?;

            // The end marker is a distance of all ones. Upstream checks it before touching the rep slots,
            // so a stream that ends here leaves them as they were.
            if distance == 0xffff_ffff {
                if expected.is_some_and(|size| output.len() != size) {
                    return None;
                }
                break;
            }

            rep3 = rep2;
            rep2 = rep1;
            rep1 = rep0;
            rep0 = distance as usize;
        } else if decoder.decode_bit(&mut models.is_rep_g0, state) == 0 {
            if decoder.decode_bit(
                &mut models.is_rep0_long,
                state * position_states + position_state,
            ) == 0
            {
                let byte = back_reference(&output, rep0)?;
                state = STATE_AFTER_SHORT_REP[state];
                push(&mut output, output_limit, byte)?;
                continue;
            }
            length = MATCH_MIN_LEN
                + decode_rep_length(&mut decoder, &mut models, position_state) as usize;
            state = STATE_AFTER_REP[state];
        } else {
            // Upstream fixed this in `5de055f94`, and the mirror follows: `rep2` shifts ONLY when the
            // distance came from `rep2` or `rep3`, matching the LZMA reference decoder. Before that fix
            // the shift ran for all three sub-cases, so after the first `rep1` match `rep2` held a
            // duplicate of `rep1` and the next `rep2`/`rep3` match resolved a wrong distance — silently
            // corrupting output when that distance happened to stay in range, and returning `null` when
            // it did not. See `agents/compression-mirror.md` for what that cost and how it was found.
            let distance = if decoder.decode_bit(&mut models.is_rep_g1, state) == 0 {
                rep1
            } else {
                let picked = if decoder.decode_bit(&mut models.is_rep_g2, state) == 0 {
                    rep2
                } else {
                    let third = rep3;
                    rep3 = rep2;
                    third
                };
                rep2 = rep1;
                picked
            };
            rep1 = rep0;
            rep0 = distance;
            length = MATCH_MIN_LEN
                + decode_rep_length(&mut decoder, &mut models, position_state) as usize;
            state = STATE_AFTER_REP[state];
        }

        // Checked once before the copy, as upstream does, and then the copy proceeds byte at a time
        // because a match may overlap the bytes it is still producing.
        if rep0 >= output.len() {
            return None;
        }
        if output.len() + length > output_limit {
            return None;
        }
        for _ in 0..length {
            let byte = output[output.len() - rep0 - 1];
            output.push(byte);
        }
    }

    if expected.is_some_and(|size| output.len() != size) {
        return None;
    }
    Some(output)
}

fn push(output: &mut Vec<u8>, limit: usize, byte: u8) -> Option<()> {
    if output.len() >= limit {
        return None;
    }
    output.push(byte);
    Some(())
}

fn back_reference(output: &[u8], rep0: usize) -> Option<u8> {
    if rep0 >= output.len() {
        return None;
    }
    Some(output[output.len() - rep0 - 1])
}

/// One literal, decoded against the previous byte and — once a match has occurred — against the byte at
/// the current rep0 distance, which is what lets LZMA spend fewer bits on a near-repeat.
fn decode_literal(
    decoder: &mut RangeDecoder<'_>,
    models: &mut Models,
    header: &Header,
    output: &[u8],
    state: usize,
    rep0: usize,
) -> Option<u8> {
    let previous = output.last().copied().unwrap_or(0);
    let literal_state = ((output.len() & ((1usize << header.lp) - 1)) << header.lc)
        + (usize::from(previous) >> (8 - header.lc));
    let offset = literal_state * 0x300;

    let mut symbol = 1u32;
    if state >= 7 {
        let mut match_byte = u32::from(back_reference(output, rep0)?);
        for _ in 0..8 {
            let match_bit = (match_byte >> 7) & 1;
            match_byte = (match_byte << 1) & 0xff;
            let bit = decoder.decode_bit(
                &mut models.literal,
                offset + ((1 + match_bit as usize) << 8) + symbol as usize,
            );
            symbol = (symbol << 1) | bit;
            if match_bit != bit {
                while symbol < 0x100 {
                    symbol = (symbol << 1)
                        | decoder.decode_bit(&mut models.literal, offset + symbol as usize);
                }
                break;
            }
        }
    } else {
        while symbol < 0x100 {
            symbol =
                (symbol << 1) | decoder.decode_bit(&mut models.literal, offset + symbol as usize);
        }
    }
    Some((symbol & 0xff) as u8)
}

fn decode_match_length(
    decoder: &mut RangeDecoder<'_>,
    models: &mut Models,
    position_state: usize,
) -> u32 {
    if decoder.decode_bit(&mut models.match_len_choice, 0) == 0 {
        return decoder.decode_bit_tree(&mut models.match_len_low, position_state << 3, 3);
    }
    if decoder.decode_bit(&mut models.match_len_choice, 1) == 0 {
        return 8 + decoder.decode_bit_tree(&mut models.match_len_mid, position_state << 3, 3);
    }
    16 + decoder.decode_bit_tree(&mut models.match_len_high, 0, 8)
}

fn decode_rep_length(
    decoder: &mut RangeDecoder<'_>,
    models: &mut Models,
    position_state: usize,
) -> u32 {
    if decoder.decode_bit(&mut models.rep_len_choice, 0) == 0 {
        return decoder.decode_bit_tree(&mut models.rep_len_low, position_state << 3, 3);
    }
    if decoder.decode_bit(&mut models.rep_len_choice, 1) == 0 {
        return 8 + decoder.decode_bit_tree(&mut models.rep_len_mid, position_state << 3, 3);
    }
    16 + decoder.decode_bit_tree(&mut models.rep_len_high, 0, 8)
}

/// A match distance: a six-bit slot, then either nothing, a reverse bit tree, or direct bits plus the
/// four-bit aligned tail.
fn decode_distance(
    decoder: &mut RangeDecoder<'_>,
    models: &mut Models,
    length_state: usize,
) -> Option<u32> {
    let slot = decoder.decode_bit_tree(&mut models.dist_slot, length_state * 64, 6);
    if slot < 4 {
        return Some(slot);
    }
    let direct_bits = (slot >> 1) - 1;
    let mut distance = (2 | (slot & 1)) << direct_bits;
    if slot < END_POS_MODEL_INDEX {
        let base = usize::try_from(distance - slot).ok()?;
        distance += decoder.decode_distance_tail(&mut models.pos, base, direct_bits);
    } else {
        distance += decoder.decode_direct_bits(direct_bits - NUM_ALIGN_BITS) << NUM_ALIGN_BITS;
        distance += decoder.decode_bit_tree_reverse(&mut models.align, 0, NUM_ALIGN_BITS);
    }
    Some(distance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Framing;

    // Every fixture is upstream's, verbatim from `lzma.test.ts`'s `FIXTURES`, extracted mechanically
    // rather than transcribed. Upstream's provenance note: generated once with Python 3.12,
    // `lzma.compress(data, format=lzma.FORMAT_ALONE)`, default properties lc=3/lp=0/pb=2 unless a fixture
    // says otherwise. So the oracle is a third-party encoder, not either implementation of the decoder —
    // which is the strongest form this can take: both upstream and this mirror are measured against bytes
    // neither of them produced.
    //
    // Upstream's two `sdkHostDecompressLzma` cases are not mirrored: they assert that the Host capability
    // slot carries the portable decoder, which is a TypeScript wiring fact with no counterpart in a crate.

    const EMPTY: &str = "XQAAgAAAAAAAAAAAAACD//v//8AAAAA=";
    const LITERAL: &str = "XQAAgAAWAAAAAAAAAAAzGwlhGvxuQ0djOEOOegx+SF/9GpGR4mriTv//OtQAAA==";
    const REPETITIVE: &str =
        "XQAAgAAYFQAAAAAAAAAwmIiVWA1cL8CWPd4EwdfDUzb4YxrC89ZwXRCrpoeyUdvTx5o45hzWZ//+5qgA";
    const LOREM_COMPRESSED: &str = concat!(
        "XQAAgAC8BAAAAAAAAAAmG8pGZ1ryd7h9hthB2wU1zYOlfBKlBduQvS8U03FylqiKfYRWcY1qIpirnj3DVe/MpcPdW4",
        "6/A4EhQNYmkQJFT5KheLuKAK+QKiaSAiPlXLMt4+hcLPsyIhobwQyDdnLg9O1dkZ1vqP+nSEAA"
    );
    const HIGH_RATIO: &str = concat!(
        "XQAAgAAAAAEAAAAAAAAAAFJQCoT5shSEwNQfjcv62XEVMXFIzslqZUB7WJdNx+nyYL98NdWkQKtgY2m70luPToPtjk",
        "2J35Y9+g1XU+rioRtXt20WiAZxVK393ZkH6XV7zoP//30wAAA="
    );
    const BINARY: &str = concat!(
        "XQAAgAAABAAAAAAAAAAAAFJQCoT5m7KAIalp1ifgPgZaXwSNU9QEujlXBQnBVSTenbhxWTFgoZ/5b0lz8sjqjLoaiy",
        "lpIYD+M4Nmr0Zt7J6JiguD8DwOiY4/7V/nnpDZHP8y9LLgOVGy0hQVtMVxutsG43man7s4wbAArJMLqgYZAxIIFVub",
        "yEjwMi7+LaCHyPCk4NJR641nVpKyTYTF8YYx32piW8J5Ldn3PHO6dHQH2DypViIkoWb4WoRfMGfS9ktJLn8g69v4EA",
        "6UeHfHP2vvtM2V4m/2RG4GzwuCGsvbevBXjZj/kMA+5sESQXXuAyiW6xP7pyjMryzes4P///iKnQA="
    );
    const NON_DEFAULT_PROPS: &str = concat!(
        "EgAAgACEAwAAAAAAAAAqGtnUdETvKUbQpX7kuGSBCVmJ/pkOyXrALgn2sW0227s2SUzMgfoYYCWHaBoAHVQGrWUzhT",
        "DeGD//2rVAAA=="
    );
    const EOS_TERMINATED: &str = "XQAAgAD//////////wAzGwlhGvxuQ0djOEOOegx+SF/9GpGR4mriTv//OtQAAA==";

    const PYTHON_STREAM_ONCE_REFUSED: &str = concat!(
        "XQAAgAD//////////wA5nUqMoNRnJcTrlP6QkZsMhtJNup2SIuSN7sJUbw5dfJqXqGvYZ4qZ31IXpGiHlX0KKZ5r5T",
        "2fgQH6Ekj1ZgU5iGeapmlTh0Q/m1ORW1mD9NrZnEEE1eG3ldkrHXCHbo7fLagJbgdgBaW0hjUOquuYEay3IO2b/R5r",
        "Ztk82m3vd204sXs4le3WwuJ4w1/FloZvmQSQDr6UcIsf/hsH0ykno7fQEM9eaYFZ1/Ng3s0dOnWscBoaDsa4qCbfuY",
        "JEafH9zN+z8UVAug5be6TX+J6uFGHc54lZoRAGjC+lY3vkcyJb7DDlvLTkELLtzJSfGoIFw2SS8ZQute94wafq3/dE",
        "yaI8QzIzB3UskFOnhco6hpX4Xpe43H6q7SJ8NhTVc40v/YnRAH2KMngWV9Yyt9Kcxe8koC5P4VfQB1RHtw0AKW9XmF",
        "FkgYTcLUX55l4XENVz7nqoX05xYeWqCGWzw+Ndkm2nGW5+s2HBaqDjpYYjUnabU+o03dvIv1w0nvX2bCnbmzSZwaIH",
        "dG7Kwrm8e+SDo3zAhFHsMgTmVTROVeO7lFN4cYnd4prIShYt4w4Ecu90SNJSgQm8W+nVG3V6ts8Kf6ej0GkfQ5MB1B",
        "rlzgNwNOjImnU4t+Gnd8/xA3mJqWOiyrxEpLyr+cn6mDapJzKC6qDog/HC2O3+DVUSEwTUMEzDIbhxsVfKOd+CfOX/",
        "nAPWTiWQYLxEyFpbAg4rlLcu1hKBlxgBY5t3H6L5vR6jUQ2irLn5H0xE9FRBybjQCovPRYoFHsbMVui9qYWDVbD3NY",
        "baJldxKwLxSXJ9dD6bEgkseJbT2ZqOJtg8OHRVYIcF1lCuY3AiDZznJM4iQmHljWtbsrAt6MOFAWhPcyioRY+U2qjb",
        "Rsd+8jil1qGW6NRrNbQ0ajtnX/96PIwA"
    );

    const ROUND_TRIP_ONCE_CORRUPTED: &str = "XQAQAABZAAAAAAAAAAAymQiQuVaJ+gBKeJd02ETQbYMGhCZGg38s2wATcTR4DE5Lpl68dK3FE+2lakfReCGaTWLigaDclAAA";

    // Upstream's own rep-shuffle regressions, shared verbatim from `lzma.test.ts` at `a62784923`.
    const REP_SHUFFLE_CORRUPTION_STREAM: &str = concat!(
        "XQAQAABYAAAAAAAAAAABAONpl4/4ZVy4Rhy9I1ecQB4qjQSwkD7GlIZGbwxgbka7aCCbU0OngOD6Z9oSV58Dkq",
        "TY45dVqGmAHVVO7+Q1iTY="
    );
    const REP_SHUFFLE_CORRUPTION_PLAIN: &str = concat!(
        "AgMCBAMFBAEFAgIGAAIEAQYGBQQGBAMDAgICBQABAwAAAQMBAwQBBQEDAQEBAwAAAQMCAQEAAQAGAwAFAAADBA",
        "QEBQMDAQABAwUCBgIEBgAEBQABBAYDAg=="
    );
    const REP_SHUFFLE_NULL_STREAM: &str =
        "XQAQAAA/AAAAAAAAAAAAglNJ8CRbC1Bvfw4Bn3KRgimvIEUMESap889L1RxPvXH3baBJ5QGxV6SlI8Mwg8/3Hg==";
    const REP_SHUFFLE_NULL_PLAIN: &str =
        "AQgBAAcAAwIBAAUIAQADAAkGCQYJAAMAAQABBgEGCQAFAAEIAQgJCAcGBwAFCAcCAwQHCAMCBwgHAgMCBwIJ";

    const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore. ";

    fn base64(input: &str) -> Vec<u8> {
        const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = Vec::new();
        let mut accumulator = 0u32;
        let mut bits = 0u32;
        for byte in input.bytes().filter(|byte| *byte != b'=') {
            let value = ALPHABET
                .iter()
                .position(|candidate| *candidate == byte)
                .expect("base64 alphabet");
            accumulator = (accumulator << 6) | value as u32;
            bits += 6;
            if bits >= 8 {
                bits -= 8;
                out.push(((accumulator >> bits) & 0xff) as u8);
            }
        }
        out
    }

    fn lzma(compressed: &[u8], declared: usize) -> Option<Vec<u8>> {
        decompress_lzma(compressed, declared, Framing::Raw)
    }

    #[test]
    fn round_trips_empty_input() {
        assert_eq!(lzma(&base64(EMPTY), 0), Some(Vec::new()));
    }

    #[test]
    fn round_trips_a_short_literal_run() {
        assert_eq!(
            lzma(&base64(LITERAL), 0).as_deref(),
            Some(&b"flighthq scene-formats"[..])
        );
    }

    #[test]
    fn round_trips_repetitive_data_through_back_references_and_buffer_growth() {
        assert_eq!(
            lzma(&base64(REPETITIVE), 0),
            Some("abcABC123".repeat(600).into_bytes())
        );
    }

    #[test]
    fn round_trips_prose_through_genuine_matches() {
        assert_eq!(
            lzma(&base64(LOREM_COMPRESSED), 0),
            Some(LOREM.repeat(12).into_bytes())
        );
    }

    #[test]
    fn keeps_decompressing_a_large_but_bounded_expansion_ratio() {
        let expected: Vec<u8> = (0..64 * 1024).map(|index| (index % 7) as u8).collect();
        assert_eq!(lzma(&base64(HIGH_RATIO), 0), Some(expected));
    }

    #[test]
    fn round_trips_binary_data_covering_every_byte_value() {
        let expected: Vec<u8> = (0..1024).map(|index| (index % 256) as u8).collect();
        assert_eq!(lzma(&base64(BINARY), 0), Some(expected));
    }

    #[test]
    fn round_trips_with_non_default_properties() {
        // lc=0, lp=2, pb=0 — properties byte 18. The literal context and position-state models are sized
        // from those three numbers, so a decoder that hard-codes the common lc=3/lp=0/pb=2 decodes the
        // default fixtures above and nothing else.
        let expected = "The quick brown fox jumps over the lazy dog. "
            .repeat(20)
            .into_bytes();
        assert_eq!(lzma(&base64(NON_DEFAULT_PROPS), 0), Some(expected));
    }

    #[test]
    fn decompresses_an_eos_terminated_stream_when_the_header_declares_unknown_size() {
        assert_eq!(
            lzma(&base64(EOS_TERMINATED), 0).as_deref(),
            Some(&b"flighthq scene-formats"[..])
        );
    }

    #[test]
    fn stops_expansion_at_the_container_declared_bound() {
        let compressed = base64(LITERAL);
        let text = &b"flighthq scene-formats"[..];
        assert_eq!(lzma(&compressed, text.len()).as_deref(), Some(text));
        assert_eq!(lzma(&compressed, text.len() - 1), None);
    }

    #[test]
    fn uses_the_caller_length_when_the_header_declares_unknown_size() {
        let text = &b"flighthq scene-formats"[..];
        assert_eq!(
            lzma(&base64(EOS_TERMINATED), text.len()).as_deref(),
            Some(text)
        );
    }

    #[test]
    fn refuses_a_truncated_stream_rather_than_panicking() {
        assert_eq!(lzma(&base64(LITERAL)[..15], 0), None);
    }

    #[test]
    fn refuses_a_corrupt_stream() {
        let mut corrupted = base64(LITERAL);
        corrupted[20] ^= 0xff;
        assert_eq!(lzma(&corrupted, 0), None);
    }

    #[test]
    fn refuses_a_header_that_is_too_short() {
        assert_eq!(lzma(&[0u8; 12], 0), None);
    }

    #[test]
    fn refuses_an_invalid_properties_byte() {
        let mut bad = base64(LITERAL);
        bad[0] = 225;
        assert_eq!(lzma(&bad, 0), None);
    }

    #[test]
    fn refuses_unsupported_framing() {
        // LZMA carries no wrapper, so zlib framing is not a stricter request — it is a different format.
        assert_eq!(decompress_lzma(&base64(LITERAL), 0, Framing::Rfc1950), None);
        assert_eq!(Framing::from_code(2), None);
    }

    #[test]
    fn decodes_a_rep1_match_without_clobbering_rep2() {
        // Upstream's own regression fixtures for the rep-shuffle fix, shared verbatim from
        // `lzma.test.ts` as added in `a62784923`. The first decoded to WRONG BYTES before the fix and the
        // second returned null, which is why there are two: the same defect had both faces depending on
        // whether the corrupted distance stayed in range.
        let compressed = base64(REP_SHUFFLE_CORRUPTION_STREAM);
        let expected = base64(REP_SHUFFLE_CORRUPTION_PLAIN);
        assert_eq!(lzma(&compressed, expected.len()), Some(expected));

        let compressed = base64(REP_SHUFFLE_NULL_STREAM);
        let expected = base64(REP_SHUFFLE_NULL_PLAIN);
        assert_eq!(lzma(&compressed, expected.len()), Some(expected));
    }

    #[test]
    fn decodes_the_streams_the_rep_shuffle_defect_used_to_break() {
        // The two reproducers this crate found independently, kept as positive cases now that upstream is
        // fixed. They are worth keeping rather than deleting with the defect: they were produced by two
        // different encoders — Python's `lzma` and upstream's own `compressLzma` — so between them they
        // guard the fix against a regression that only one encoder's output would expose.

        // Python `lzma`, FORMAT_ALONE, 2304 bytes of low-entropy text. Returned null before the fix.
        // Pinned by length and Adler-32 rather than by embedding the plaintext.
        let decoded =
            lzma(&base64(PYTHON_STREAM_ONCE_REFUSED), 0).expect("decodes since 5de055f94");
        assert_eq!(decoded.len(), 2304);
        assert_eq!(crate::deflate::compute_adler32(&decoded), 0xaf4c_1390);

        // Upstream `compressLzma`'s own output for these 89 bytes. Decoded to 89 WRONG bytes before the
        // fix, diverging at index 87 with no error raised — the sharper half of the same defect.
        const PLAIN: &[u8] = b"edcdafebebcfcbebabcfededcdabcdadcfabcfcfadefebafefefcdefcdadcbcdcfabcdabedebadabcbedcfafe";
        assert_eq!(
            lzma(&base64(ROUND_TRIP_ONCE_CORRUPTED), 0).as_deref(),
            Some(PLAIN)
        );
    }

    #[test]
    fn refuses_a_nonzero_range_coder_start_byte() {
        let mut bad = base64(LITERAL);
        bad[13] = 0x01;
        assert_eq!(lzma(&bad, 0), None);
    }
}
