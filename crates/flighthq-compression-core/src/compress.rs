// RFC 1951 (raw DEFLATE) and RFC 1950 (zlib) encoding.
//
// Source of record: `upstream/packages/compression/src/compress.ts` at upstream `85d85a3b1`, with the
// shared code tables from `deflateFormat.ts`. Like `lzma`, this mirrors a file that is NOT in the pinned
// submodule — the encoder landed upstream after the current pin.
//
// Upstream states that its output is a pure function of its input: one fixed-Huffman block, greedy LZ77
// through a hash chain, no timestamps and no heuristics that vary. That makes BYTE IDENTITY the oracle
// here, which is stronger than the round-trip oracle a decoder gets: the mirror does not merely have to
// produce something that decodes back, it has to produce the SAME BYTES. Every search bound is therefore
// part of the contract — the 32-candidate chain limit, the 32768-byte window, the insertion of positions
// interior to a match, and the exact hash function. Changing any of them still round-trips and is still
// wrong.
//
// Upstream's shape that is not carried over: a bit writer as a class with a growable `Uint8Array`, and
// `Int32Array` fill(-1) sentinels for the chain. The emitted bits are identical.

use crate::deflate::{DISTANCE_BASE, DISTANCE_EXTRA, LENGTH_BASE, LENGTH_EXTRA, compute_adler32};

const END_OF_BLOCK: u32 = 256;
const FIRST_LENGTH_SYMBOL: u32 = 257;
const FIXED_HUFFMAN_BLOCK: u32 = 1;
const HASH_BITS: u32 = 15;
const HASH_MASK: usize = (1 << HASH_BITS) - 1;
const HASH_SIZE: usize = 1 << HASH_BITS;
const MAX_CHAIN: usize = 32;
const MAX_DISTANCE: usize = 32768;
const MAX_MATCH: usize = 258;
const MIN_MATCH: usize = 3;
const STORED_BLOCK: u32 = 0;
const STORED_BLOCK_MAX: usize = 65535;
const STORED_BLOCK_OVERHEAD: usize = 5;
const ZLIB_CMF: u8 = 0x78;
const ZLIB_FLG: u8 = 0x01;
const ZLIB_HEADER_BYTES: usize = 2;
const ZLIB_TRAILER_BYTES: usize = 4;

/// Encodes to a bare RFC 1951 stream, the form `decompress_deflate` reads as `Framing::Raw`.
///
/// Falls back to stored blocks when the Huffman encoding would not be smaller, so compressing never
/// meaningfully grows a payload — which is what incompressible input would otherwise do.
pub fn compress_deflate(bytes: &[u8]) -> Vec<u8> {
    let huffman = encode_fixed_huffman_block(bytes);
    if huffman.len() <= stored_length(bytes.len()) {
        huffman
    } else {
        encode_stored_blocks(bytes)
    }
}

/// Encodes to RFC 1950: a two-byte header, the same stream, and a big-endian Adler-32 of the
/// UNCOMPRESSED bytes. The form `decompress_deflate` reads as `Framing::Rfc1950`.
pub fn compress_deflate_zlib(bytes: &[u8]) -> Vec<u8> {
    let deflated = compress_deflate(bytes);
    let mut out = Vec::with_capacity(ZLIB_HEADER_BYTES + deflated.len() + ZLIB_TRAILER_BYTES);
    out.push(ZLIB_CMF);
    out.push(ZLIB_FLG);
    out.extend_from_slice(&deflated);
    out.extend_from_slice(&compute_adler32(bytes).to_be_bytes());
    out
}

/// Bit order is the format's rather than one convention throughout: block headers and extra bits pack
/// least-significant-bit first, while a Huffman code packs most-significant-bit first. Confusing the two
/// yields a stream that still decodes for short inputs and fails on long ones.
struct BitWriter {
    bytes: Vec<u8>,
    bit_buffer: u8,
    bit_count: u32,
}

impl BitWriter {
    fn new() -> Self {
        Self {
            bytes: Vec::with_capacity(1024),
            bit_buffer: 0,
            bit_count: 0,
        }
    }

    fn write_bit(&mut self, bit: u32) {
        self.bit_buffer |= (bit as u8) << self.bit_count;
        self.bit_count += 1;
        if self.bit_count == 8 {
            self.bytes.push(self.bit_buffer);
            self.bit_buffer = 0;
            self.bit_count = 0;
        }
    }

    /// Least-significant bit first: block headers and extra bits.
    fn write_bits(&mut self, value: u32, count: u32) {
        for index in 0..count {
            self.write_bit((value >> index) & 1);
        }
    }

    /// Most-significant bit first: Huffman codes.
    fn write_code(&mut self, code: u32, count: u32) {
        for index in (0..count).rev() {
            self.write_bit((code >> index) & 1);
        }
    }

    fn align_to_byte(&mut self) {
        if self.bit_count != 0 {
            self.bytes.push(self.bit_buffer);
            self.bit_buffer = 0;
            self.bit_count = 0;
        }
    }

    fn write_byte(&mut self, value: u8) {
        self.bytes.push(value);
    }

    fn finish(mut self) -> Vec<u8> {
        self.align_to_byte();
        self.bytes
    }
}

/// One fixed-Huffman block over the whole input, with greedy LZ77 matching through a hash chain: `head`
/// holds the most recent position for a three-byte prefix and `previous` links each position to the one
/// before it, so the search walks candidates newest-first and is cut off by depth rather than by window.
fn encode_fixed_huffman_block(input: &[u8]) -> Vec<u8> {
    let mut writer = BitWriter::new();
    writer.write_bits(1, 1);
    writer.write_bits(FIXED_HUFFMAN_BLOCK, 2);

    let mut head = vec![-1i32; HASH_SIZE];
    let mut previous = vec![-1i32; input.len()];

    let mut position = 0usize;
    while position < input.len() {
        let mut match_length = 0usize;
        let mut match_distance = 0usize;

        if position + MIN_MATCH <= input.len() {
            let key = hash_at(input, position);
            let mut candidate = head[key];
            let mut attempts = 0usize;
            while candidate >= 0 && attempts < MAX_CHAIN {
                let candidate_position = candidate as usize;
                let distance = position - candidate_position;
                if distance > MAX_DISTANCE {
                    break;
                }
                let length = match_run_length(input, candidate_position, position);
                if length > match_length {
                    match_length = length;
                    match_distance = distance;
                    if length == MAX_MATCH {
                        break;
                    }
                }
                candidate = previous[candidate_position];
                attempts += 1;
            }
            previous[position] = head[key];
            head[key] = position as i32;
        }

        if match_length >= MIN_MATCH {
            write_length_symbol(&mut writer, match_length);
            write_distance_symbol(&mut writer, match_distance);
            // Positions interior to the match still enter the chain, so a later repeat that begins
            // mid-match is findable. A ratio choice rather than a correctness one — the stream decodes
            // either way — but it changes the bytes, so a mirror has to make the same choice.
            for offset in 1..match_length {
                let inner = position + offset;
                if inner + MIN_MATCH <= input.len() {
                    let key = hash_at(input, inner);
                    previous[inner] = head[key];
                    head[key] = inner as i32;
                }
            }
            position += match_length;
            continue;
        }

        write_literal_symbol(&mut writer, u32::from(input[position]));
        position += 1;
    }

    write_literal_symbol(&mut writer, END_OF_BLOCK);
    writer.finish()
}

/// Type-00 blocks: a byte-aligned length and its one's complement, then the bytes verbatim. One block
/// carries at most 65535 bytes, so a long input needs several, and an empty input still needs one.
fn encode_stored_blocks(input: &[u8]) -> Vec<u8> {
    let mut writer = BitWriter::new();
    let mut offset = 0usize;
    loop {
        let size = STORED_BLOCK_MAX.min(input.len() - offset);
        let final_block = u32::from(offset + size >= input.len());
        writer.write_bits(final_block, 1);
        writer.write_bits(STORED_BLOCK, 2);
        writer.align_to_byte();
        let length = size as u32;
        writer.write_byte((length & 0xff) as u8);
        writer.write_byte(((length >> 8) & 0xff) as u8);
        writer.write_byte((!length & 0xff) as u8);
        writer.write_byte(((!length >> 8) & 0xff) as u8);
        writer
            .bytes
            .extend_from_slice(&input[offset..offset + size]);
        offset += size;
        if offset >= input.len() {
            break;
        }
    }
    writer.finish()
}

fn stored_length(input_length: usize) -> usize {
    let blocks = input_length.div_ceil(STORED_BLOCK_MAX).max(1);
    blocks * STORED_BLOCK_OVERHEAD + input_length
}

/// The fixed literal/length alphabet changes code width three times across its range. The boundaries are
/// the format's, and getting one wrong yields a stream that decodes correctly only below it.
fn write_literal_symbol(writer: &mut BitWriter, symbol: u32) {
    match symbol {
        0..=143 => writer.write_code(0x30 + symbol, 8),
        144..=255 => writer.write_code(0x190 + symbol - 144, 9),
        256..=279 => writer.write_code(symbol - 256, 7),
        _ => writer.write_code(0xc0 + symbol - 280, 8),
    }
}

fn write_length_symbol(writer: &mut BitWriter, length: usize) {
    let mut index = LENGTH_BASE.len() - 1;
    while index > 0 && LENGTH_BASE[index] as usize > length {
        index -= 1;
    }
    write_literal_symbol(writer, FIRST_LENGTH_SYMBOL + index as u32);
    writer.write_bits(length as u32 - LENGTH_BASE[index], LENGTH_EXTRA[index]);
}

fn write_distance_symbol(writer: &mut BitWriter, distance: usize) {
    let mut index = DISTANCE_BASE.len() - 1;
    while index > 0 && DISTANCE_BASE[index] as usize > distance {
        index -= 1;
    }
    writer.write_code(index as u32, 5);
    writer.write_bits(
        distance as u32 - DISTANCE_BASE[index],
        DISTANCE_EXTRA[index],
    );
}

/// How far the bytes at `candidate` and `position` agree, capped at the format's maximum match. The ranges
/// may overlap, which is how a run is encoded, so this compares forward rather than slicing.
fn match_run_length(input: &[u8], candidate: usize, position: usize) -> usize {
    let limit = (input.len() - position).min(MAX_MATCH);
    let mut length = 0usize;
    while length < limit && input[candidate + length] == input[position + length] {
        length += 1;
    }
    length
}

fn hash_at(input: &[u8], position: usize) -> usize {
    ((usize::from(input[position]) << 10)
        ^ (usize::from(input[position + 1]) << 5)
        ^ usize::from(input[position + 2]))
        & HASH_MASK
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Framing, decompress_deflate};

    // Upstream's encoder is documented as a pure function of its input, so the oracle is BYTE IDENTITY
    // rather than a round-trip. Checking only that output decodes back would pass for any correct
    // encoder, including one that makes different search choices and emits different bytes.
    //
    // The corpus below is built identically on both sides — the LCG is transcribed rather than shared —
    // and each case carries the length and Adler-32 of upstream's output for it, which pins the bytes in
    // eight characters instead of embedding kilobytes of fixtures. Produced by running upstream's
    // `compressDeflate`/`compressDeflateZlib` at `85d85a3b1` over this corpus.
    //
    // The cases are chosen for the branches, not for variety: `empty` and `one-byte` for the degenerate
    // paths, `all-byte-values` for every literal code width, `incompressible-4096` for the stored-block
    // fallback, `incompressible-70000` for MULTIPLE stored blocks, `single-byte-run-600` for runs past
    // the 258-byte maximum match, and `cycle-251-x40000` for distances deep into the window.
    struct Case {
        name: &'static str,
        input: Vec<u8>,
        raw_length: usize,
        raw_adler: u32,
        zlib_length: usize,
        zlib_adler: u32,
    }

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

    fn corpus() -> Vec<Case> {
        let case = |name, input, raw_length, raw_adler, zlib_length, zlib_adler| Case {
            name,
            input,
            raw_length,
            raw_adler,
            zlib_length,
            zlib_adler,
        };
        vec![
            case("empty", Vec::new(), 2, 0x0008_0004, 8, 0x03e2_007e),
            case("one-byte", b"a".to_vec(), 3, 0x00ec_0050, 9, 0x07f6_018d),
            case(
                "short-literal",
                b"flighthq scene-formats".to_vec(),
                24,
                0x8b90_09d2,
                30,
                0xc451_0bba,
            ),
            case(
                "all-byte-values",
                cycle(256, 256),
                261,
                0xb01d_8180,
                267,
                0x3b93_849c,
            ),
            case(
                "cycle-256-x1024",
                cycle(256, 1024),
                279,
                0x61a1_9e2b,
                285,
                0x6927_a15f,
            ),
            case(
                "abcABC123-x600",
                "abcABC123".repeat(600).into_bytes(),
                51,
                0xdecb_1270,
                57,
                0x4948_15ab,
            ),
            case(
                "single-byte-run-600",
                vec![b'A'; 600],
                8,
                0x09c3_01be,
                14,
                0x1c99_041a,
            ),
            case(
                "incompressible-4096",
                lcg(1, 4096),
                4101,
                0x6932_fb3c,
                4107,
                0xf07b_fd8a,
            ),
            case(
                "incompressible-70000",
                lcg(7, 70000),
                70010,
                0x403f_625a,
                70016,
                0x1e47_6560,
            ),
            case(
                "cycle-251-x40000",
                cycle(251, 40000),
                634,
                0x211a_7c61,
                640,
                0x4302_7d9f,
            ),
            case(
                "words-2304",
                words(2304),
                928,
                0xb4ca_5d68,
                934,
                0xe864_5f7f,
            ),
            case(
                "words-65536",
                words(65536),
                20157,
                0xcfab_ba6d,
                20163,
                0xf8d7_bc4c,
            ),
        ]
    }

    #[test]
    fn emits_byte_identical_output_to_upstream() {
        for case in corpus() {
            let raw = compress_deflate(&case.input);
            assert_eq!(raw.len(), case.raw_length, "{}: raw length", case.name);
            assert_eq!(
                compute_adler32(&raw),
                case.raw_adler,
                "{}: raw bytes",
                case.name
            );

            let zlib = compress_deflate_zlib(&case.input);
            assert_eq!(zlib.len(), case.zlib_length, "{}: zlib length", case.name);
            assert_eq!(
                compute_adler32(&zlib),
                case.zlib_adler,
                "{}: zlib bytes",
                case.name
            );
        }
    }

    #[test]
    fn round_trips_every_case_through_the_mirror_decoder() {
        // Byte identity above says the encoder agrees with upstream. This says the two halves of this
        // crate agree with each other, which byte identity alone would not catch if both were wrong in
        // the same way — and it is the property a consumer actually depends on.
        for case in corpus() {
            assert_eq!(
                decompress_deflate(
                    &compress_deflate(&case.input),
                    case.input.len(),
                    Framing::Raw
                )
                .as_deref(),
                Some(case.input.as_slice()),
                "{}: raw round-trip",
                case.name
            );
            assert_eq!(
                decompress_deflate(
                    &compress_deflate_zlib(&case.input),
                    case.input.len(),
                    Framing::Rfc1950
                )
                .as_deref(),
                Some(case.input.as_slice()),
                "{}: zlib round-trip",
                case.name
            );
        }
    }

    #[test]
    fn round_trips_without_a_declared_length() {
        // A container that declares no length passes 0, and the decoder then relies on its own cap. The
        // encoder's output must be readable either way.
        for case in corpus() {
            assert_eq!(
                decompress_deflate(&compress_deflate(&case.input), 0, Framing::Raw).as_deref(),
                Some(case.input.as_slice()),
                "{}: undeclared round-trip",
                case.name
            );
        }
    }

    #[test]
    fn never_meaningfully_grows_an_incompressible_payload() {
        // The stored-block fallback exists for exactly this: a Huffman block over random bytes is larger
        // than the bytes. Five bytes of overhead per 65535-byte block is the whole budget.
        for count in [1usize, 4096, 70000] {
            let input = lcg(3, count);
            let compressed = compress_deflate(&input);
            let blocks = count.div_ceil(STORED_BLOCK_MAX).max(1);
            assert!(
                compressed.len() <= count + blocks * STORED_BLOCK_OVERHEAD,
                "{count} bytes grew to {}",
                compressed.len()
            );
        }
    }

    #[test]
    fn emits_no_zlib_wrapper_on_the_raw_form_and_one_on_the_zlib_form() {
        let input = b"flighthq scene-formats";
        let raw = compress_deflate(input);
        // The raw stream is not readable as zlib, and the zlib stream is not readable as raw. Upstream
        // asserts both directions because a wrapper silently accepted either way is the bug that makes a
        // container's framing field pointless.
        assert_eq!(decompress_deflate(&raw, 0, Framing::Rfc1950), None);

        let zlib = compress_deflate_zlib(input);
        assert_eq!(zlib[0], ZLIB_CMF);
        assert_eq!(zlib[1], ZLIB_FLG);
        assert_eq!(
            &zlib[ZLIB_HEADER_BYTES..zlib.len() - ZLIB_TRAILER_BYTES],
            raw.as_slice()
        );
        assert_eq!(
            &zlib[zlib.len() - ZLIB_TRAILER_BYTES..],
            &compute_adler32(input).to_be_bytes()
        );
        assert_ne!(
            decompress_deflate(&zlib, 0, Framing::Raw).as_deref(),
            Some(&input[..])
        );
    }

    #[test]
    fn a_corrupted_payload_byte_fails_the_trailer() {
        let mut zlib = compress_deflate_zlib(b"flighthq scene-formats");
        zlib[ZLIB_HEADER_BYTES + 2] ^= 0x01;
        assert_eq!(decompress_deflate(&zlib, 0, Framing::Rfc1950), None);
    }

    #[test]
    fn is_deterministic_and_leaves_its_input_alone() {
        let input = words(4096);
        let before = input.clone();
        assert_eq!(compress_deflate(&input), compress_deflate(&input));
        assert_eq!(compress_deflate_zlib(&input), compress_deflate_zlib(&input));
        assert_eq!(input, before);
    }
}
