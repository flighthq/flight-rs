// A hand-written Rust mirror of `@flighthq/compression`'s DEFLATE decoder.
//
// Hand-written rather than generated, deliberately. The decoder is a bit-serial state machine whose
// JavaScript shape — a `readBit()` method returning one bit per call, an output buffer grown by
// doubling, throws caught and turned into `null` — is the part a mechanical lowering would carry over
// faithfully and the part that makes it slow. Mirroring the BEHAVIOUR while choosing Rust's shape is
// the whole reason to write it by hand.
//
// What "mirror" means here is exact, not approximate: for every input, this must return the same bytes
// `inflateDeflate` returns, and `None` exactly where it returns `null`. That includes the refusals, the
// zlib header validations, the Adler-32 check, and the expansion cap — upstream's cap exists because
// the compression RATIO is not in the file and no per-field check can bound it, so a kilobyte of nested
// maximum-length back-references would otherwise take the process with it. The cap is load-bearing
// security behaviour, not a tuning constant, and it is mirrored at the same value.
//
// Source of record: `upstream/packages/compression/src/deflate.ts`. Its own tests are the oracle; the
// suite at the bottom of this file mirrors them so `cargo test` alone catches a divergence.

#![forbid(unsafe_code)]

/// Whether the bytes are the raw RFC 1951 stream or carry RFC 1950 zlib framing. Mirrors upstream's
/// `CompressionFraming`: the framing is independent of the algorithm, because the first raw bytes can
/// themselves form a valid-looking zlib header, so the container supplies this rather than the decoder
/// guessing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Framing {
    Raw,
    Rfc1950,
}

impl Framing {
    /// The wire encoding used across the wasm boundary. Anything else is not a framing this decoder
    /// knows, and upstream returns `null` for an unknown framing rather than assuming one.
    pub const fn from_code(code: u32) -> Option<Self> {
        match code {
            0 => Some(Self::Raw),
            1 => Some(Self::Rfc1950),
            _ => None,
        }
    }
}

/// The ceiling on a single inflate, mirroring upstream's `MAX_INFLATE_BYTES`.
pub const MAX_INFLATE_BYTES: usize = 256 * 1024 * 1024;

const INITIAL_INFLATE_BYTES: usize = 1024;
const ZLIB_HEADER_BYTES: usize = 2;
const ZLIB_TRAILER_BYTES: usize = 4;
const ADLER_MODULUS: u32 = 65521;

/// Decompresses `compressed`, or returns `None` when the stream is malformed or the framing is unknown.
///
/// `uncompressed_length` is the length the container declared, or 0 when it declared none — a ceiling
/// enforced while bytes are written, not an allocation hint. Mirrors `inflateDeflate`.
pub fn decompress_deflate(
    compressed: &[u8],
    uncompressed_length: usize,
    framing: Framing,
) -> Option<Vec<u8>> {
    let mut start = 0usize;
    let mut end = compressed.len();

    if framing == Framing::Rfc1950 {
        if compressed.len() < ZLIB_HEADER_BYTES + ZLIB_TRAILER_BYTES {
            return None;
        }
        let cmf = u32::from(compressed[0]);
        let flg = u32::from(compressed[1]);
        // Deflate method, a window no larger than 32K, a valid check value, and no preset dictionary.
        if cmf & 0x0f != 8 || cmf >> 4 > 7 || ((cmf << 8) | flg) % 31 != 0 || flg & 0x20 != 0 {
            return None;
        }
        start = ZLIB_HEADER_BYTES;
        end -= ZLIB_TRAILER_BYTES;
    }

    let output = raw_inflate(&compressed[..end], start, uncompressed_length)?;

    if framing == Framing::Rfc1950 && read_zlib_adler32(compressed, end) != compute_adler32(&output)
    {
        return None;
    }
    Some(output)
}

/// The output buffer plus an LSB-first bit reader over the DEFLATE stream. Upstream throws on overrun
/// and catches at the boundary; here every read returns `Option` and `?` carries the refusal out, which
/// is the same control flow without the unwinding.
struct Inflate<'a> {
    input: &'a [u8],
    position: usize,
    bit_buffer: u32,
    bit_count: u32,
    output: Vec<u8>,
    output_limit: usize,
}

impl<'a> Inflate<'a> {
    fn new(input: &'a [u8], position: usize, output_limit: usize) -> Self {
        Self {
            input,
            position,
            bit_buffer: 0,
            bit_count: 0,
            output: Vec::with_capacity(INITIAL_INFLATE_BYTES.min(output_limit)),
            output_limit,
        }
    }

    fn read_bit(&mut self) -> Option<u32> {
        if self.bit_count == 0 {
            let byte = *self.input.get(self.position)?;
            self.position += 1;
            self.bit_buffer = u32::from(byte);
            self.bit_count = 8;
        }
        let bit = self.bit_buffer & 1;
        self.bit_buffer >>= 1;
        self.bit_count -= 1;
        Some(bit)
    }

    fn read_bits(&mut self, count: u32, base: u32) -> Option<u32> {
        let mut value = 0u32;
        for index in 0..count {
            value |= self.read_bit()? << index;
        }
        Some(value + base)
    }

    fn write_byte(&mut self, byte: u8) -> Option<()> {
        // The one bound no per-field check can supply: the quantity that sizes this allocation is the
        // compression ratio, which is not in the file and is not bounded by its length.
        if self.output.len() >= self.output_limit {
            return None;
        }
        self.output.push(byte);
        Some(())
    }
}

fn raw_inflate(input: &[u8], start: usize, uncompressed_length: usize) -> Option<Vec<u8>> {
    let declared = if uncompressed_length > 0 {
        uncompressed_length
    } else {
        MAX_INFLATE_BYTES
    };
    let mut state = Inflate::new(input, start, declared.min(MAX_INFLATE_BYTES));

    loop {
        let final_block = state.read_bit()?;
        match state.read_bits(2, 0)? {
            0 => inflate_stored_block(&mut state)?,
            1 => inflate_huffman_block(&mut state, &fixed_literal_tree(), &fixed_distance_tree())?,
            2 => inflate_dynamic_block(&mut state)?,
            _ => return None,
        }
        if final_block != 0 {
            break;
        }
    }
    Some(state.output)
}

/// A stored block: align to the next byte, read LEN and its one's-complement NLEN, copy LEN bytes.
fn inflate_stored_block(state: &mut Inflate<'_>) -> Option<()> {
    state.bit_buffer = 0;
    state.bit_count = 0;
    let header = state.input.get(state.position..state.position + 4)?;
    let len = usize::from(header[0]) | (usize::from(header[1]) << 8);
    let nlen = usize::from(header[2]) | (usize::from(header[3]) << 8);
    state.position += 4;
    if len ^ 0xffff != nlen {
        return None;
    }
    if state.position + len > state.input.len() {
        return None;
    }
    for _ in 0..len {
        let byte = state.input[state.position];
        state.position += 1;
        state.write_byte(byte)?;
    }
    Some(())
}

/// Decodes one Huffman-coded block, emitting literals and resolving `<length, distance>` back-references
/// against the bytes already written.
fn inflate_huffman_block(
    state: &mut Inflate<'_>,
    literal_tree: &HuffmanTree,
    distance_tree: &HuffmanTree,
) -> Option<()> {
    loop {
        let symbol = decode_symbol(state, literal_tree)?;
        if symbol == 256 {
            return Some(());
        }
        if symbol < 256 {
            state.write_byte(symbol as u8)?;
            continue;
        }
        let length_index = usize::try_from(symbol - 257).ok()?;
        if length_index >= LENGTH_BASE.len() {
            return None;
        }
        let length = state.read_bits(LENGTH_EXTRA[length_index], LENGTH_BASE[length_index])?;
        let distance_symbol = usize::try_from(decode_symbol(state, distance_tree)?).ok()?;
        if distance_symbol >= DISTANCE_BASE.len() {
            return None;
        }
        let distance = usize::try_from(state.read_bits(
            DISTANCE_EXTRA[distance_symbol],
            DISTANCE_BASE[distance_symbol],
        )?)
        .ok()?;
        // A back-reference may not reach before the start of the output, and may overlap the bytes it
        // is still producing — a run is encoded exactly that way — so this copies one byte at a time
        // from a base index rather than slicing. A bulk copy would read a stale source region.
        let base = state.output.len().checked_sub(distance)?;
        for offset in 0..usize::try_from(length).ok()? {
            let byte = state.output[base + offset];
            state.write_byte(byte)?;
        }
    }
}

/// Builds the dynamic literal/length and distance trees from the block header — their code lengths are
/// themselves Huffman-coded — then decodes the block.
fn inflate_dynamic_block(state: &mut Inflate<'_>) -> Option<()> {
    let literal_count = usize::try_from(state.read_bits(5, 257)?).ok()?;
    let distance_count = usize::try_from(state.read_bits(5, 1)?).ok()?;
    let code_length_count = usize::try_from(state.read_bits(4, 4)?).ok()?;
    if literal_count > 286 {
        return None;
    }

    let mut code_length_lengths = [0u32; 19];
    for index in 0..code_length_count {
        code_length_lengths[CODE_LENGTH_ORDER[index]] = state.read_bits(3, 0)?;
    }
    let code_length_tree = build_huffman_tree(&code_length_lengths);

    // The literal and distance code lengths are one run, honouring repeat codes 16 (copy previous 3-6x),
    // 17 (zero 3-10x), and 18 (zero 11-138x).
    let mut lengths = vec![0u32; literal_count + distance_count];
    let mut index = 0usize;
    while index < lengths.len() {
        let symbol = decode_symbol(state, &code_length_tree)?;
        let (repeat, value) = match symbol {
            0..=15 => {
                lengths[index] = symbol;
                index += 1;
                continue;
            }
            16 => {
                if index == 0 {
                    return None;
                }
                (state.read_bits(2, 3)?, lengths[index - 1])
            }
            17 => (state.read_bits(3, 3)?, 0),
            // The code-length alphabet is exactly 0-18, so after the arms above this is 18.
            _ => (state.read_bits(7, 11)?, 0),
        };
        let repeat = usize::try_from(repeat).ok()?;
        if index + repeat > lengths.len() {
            return None;
        }
        for _ in 0..repeat {
            lengths[index] = value;
            index += 1;
        }
    }

    let literal_tree = build_huffman_tree(&lengths[..literal_count]);
    let distance_tree = build_huffman_tree(&lengths[literal_count..]);
    inflate_huffman_block(state, &literal_tree, &distance_tree)
}

/// A canonical Huffman decode table: `counts[len]` is how many symbols use a code of that bit length,
/// and `symbols` lists symbols ordered by (length, symbol).
struct HuffmanTree {
    counts: [u32; 16],
    symbols: Vec<u32>,
}

fn build_huffman_tree(lengths: &[u32]) -> HuffmanTree {
    let mut counts = [0u32; 16];
    for &length in lengths {
        counts[length as usize] += 1;
    }
    counts[0] = 0;

    let mut offsets = [0usize; 16];
    for length in 1..16 {
        offsets[length] = offsets[length - 1] + counts[length - 1] as usize;
    }

    let mut symbols = vec![0u32; lengths.len()];
    for (symbol, &length) in lengths.iter().enumerate() {
        if length != 0 {
            symbols[offsets[length as usize]] = symbol as u32;
            offsets[length as usize] += 1;
        }
    }
    HuffmanTree { counts, symbols }
}

/// Reads bits until they identify one canonical Huffman symbol: extend the code one bit at a time,
/// subtracting each length's code count until the code falls inside a bucket.
fn decode_symbol(state: &mut Inflate<'_>, tree: &HuffmanTree) -> Option<u32> {
    let mut code = 0i32;
    let mut first = 0i32;
    let mut index = 0usize;
    for length in 1..16 {
        code |= state.read_bit()? as i32;
        let count = tree.counts[length] as i32;
        if code - first < count {
            return tree
                .symbols
                .get(index + usize::try_from(code - first).ok()?)
                .copied();
        }
        index += usize::try_from(count).ok()?;
        first = (first + count) << 1;
        code <<= 1;
    }
    None
}

fn compute_adler32(input: &[u8]) -> u32 {
    let mut first = 1u32;
    let mut second = 0u32;
    for &byte in input {
        first += u32::from(byte);
        if first >= ADLER_MODULUS {
            first -= ADLER_MODULUS;
        }
        second += first;
        if second >= ADLER_MODULUS {
            second -= ADLER_MODULUS;
        }
    }
    (second << 16) | first
}

fn read_zlib_adler32(input: &[u8], offset: usize) -> u32 {
    let bytes = &input[offset..offset + ZLIB_TRAILER_BYTES];
    (u32::from(bytes[0]) << 24)
        | (u32::from(bytes[1]) << 16)
        | (u32::from(bytes[2]) << 8)
        | u32::from(bytes[3])
}

// RFC 1951 length codes 257-285: base copy length and the extra-bit count that follows.
const LENGTH_BASE: [u32; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LENGTH_EXTRA: [u32; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];

// RFC 1951 distance codes 0-29: base back-distance and the extra-bit count that follows.
const DISTANCE_BASE: [u32; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DISTANCE_EXTRA: [u32; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];

// The order the 19 code-length-code lengths are written in a dynamic-Huffman block header.
const CODE_LENGTH_ORDER: [usize; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

/// The RFC 1951 fixed literal/length tree: lengths 8/9/7/8 across symbols 0-287.
fn fixed_literal_tree() -> HuffmanTree {
    let mut lengths = [0u32; 288];
    for (symbol, length) in lengths.iter_mut().enumerate() {
        *length = match symbol {
            0..=143 => 8,
            144..=255 => 9,
            256..=279 => 7,
            _ => 8,
        };
    }
    build_huffman_tree(&lengths)
}

/// The RFC 1951 fixed distance tree: 30 symbols, all 5 bits.
fn fixed_distance_tree() -> HuffmanTree {
    build_huffman_tree(&[5u32; 30])
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every fixture here comes from upstream's own `deflate.test.ts`, not from a reimplementation of its
    // bit packers. The base64 streams are the ones it embeds verbatim; the bit-level malformed streams
    // were produced by RUNNING its `createFixedBackReferenceStream`, `createDynamicRepeatOverflowStream`,
    // `createValidDynamicRepeatStream`, `createReservedDynamicDistanceStream` and
    // `createRawStoredStreamStartingWithZlibHeader` helpers and recording the bytes. Sharing the fixture
    // rather than the generator is deliberate: a ported bit packer could drift from the original and
    // still agree with a ported decoder, and the two errors would cancel.
    //
    // Upstream provenance for the base64 set, generated once with node v22.22.1 — see its own comment.

    const EMPTY: &str = "eJwDAAAAAAE=";
    const HIGH_RATIO: &str = concat!(
        "eJztxbEBADAEADC0/H+yQyRLIuv9npAkSZIkSZIkSZIkSZIkSZIkSZIkSZIkSZIkSZIkSZIkSZIkSZIkSZIkSZIkSZ",
        "IkSZIkSZIkSZIkSZIkSZIkSZIkSZIkSZIkSZIkSZIkSZIkSZIkSZIkSZIkSZIkSZIk3W4BATsAKQ=="
    );
    const LITERAL: &str = "eJxLy8lMzyjJKFQoTk7NS9VNyy/KTSwpBgBjVQiv";
    const RAW_LITERAL: &str = "S8vJTM8oyShUKE5OzUvVTcsvyk0sKQYA";
    const REPETITIVE: &str = "eJztxjEBACAIALBMYgIxCdC/g0HcrlXPybtil4iIiIiIiIiIiMgfeU6Y4Pw=";
    const LOREM_COMPRESSED: &str = concat!(
        "eJztzdENQyEMQ9FVPEDVSd4SlESVJUIQSfYvQ/STb+v6PL7VwBVlEB++EUw003yh+wztqVkbTbgYnfMLHTxjqJwAyg",
        "pzQaqtE3N2CqVmohKjfc79G89FLnKRi/wX+QG1gr9k"
    );
    const STORED_L0: &str =
        "eAEBKwDU/3RoZSBxdWljayBicm93biBmb3gganVtcHMgb3ZlciB0aGUgbGF6eSBkb2dhPA/6";
    const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore. ";

    const FIXED_RESERVED_LENGTH_286: &str = "1b03";
    const FIXED_BACKREF_BEFORE_OUTPUT: &str = "0302";
    const RESERVED_DYNAMIC_DISTANCE: &str = "0dde0104000000001000000000000000000000000000000000000000000000000000000000000000800100008001";
    const DYNAMIC_REPEAT_OVERFLOW_16: &str =
        "05c005040000000020000000000000000000000000000000000000000000000000000000000000008006";
    const DYNAMIC_REPEAT_VALID_16: &str =
        "0580050400000080000000000000000000000000000000000000000000000000000000000000008026";
    const DYNAMIC_REPEAT_OVERFLOW_17: &str =
        "05c021040000000020000000000000000000000000000000000000000000000000000000000000008006";
    const DYNAMIC_REPEAT_VALID_17: &str =
        "05c0210400000000a0010000000000000000000000000000000000000000000000000000000000000002";
    const DYNAMIC_REPEAT_OVERFLOW_18: &str =
        "05c00105000000002000000000000000000000000000000000000000000000000000000000000000800600";
    const DYNAMIC_REPEAT_VALID_18: &str =
        "05c0010500000000a0010000000000000000000000000000000000000000000000000000000000002000";
    const DYNAMIC_REPEAT_NO_PREVIOUS: &str = "05c0050400000000a001";
    const RAW_STORED_LOOKS_LIKE_ZLIB: &str = concat!(
        "789c0063ff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a",
        "2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a",
        "5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a",
        "8b8c8d8e8f909192939495969798999a9b010000ffff"
    );
    const RAW_STORED_PAYLOAD_LEN: usize = 156;

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

    fn hex(input: &str) -> Vec<u8> {
        (0..input.len() / 2)
            .map(|index| u8::from_str_radix(&input[index * 2..index * 2 + 2], 16).expect("hex"))
            .collect()
    }

    fn inflate_zlib(compressed: &[u8], declared: usize) -> Option<Vec<u8>> {
        decompress_deflate(compressed, declared, Framing::Rfc1950)
    }

    fn inflate_raw(compressed: &[u8], declared: usize) -> Option<Vec<u8>> {
        decompress_deflate(compressed, declared, Framing::Raw)
    }

    #[test]
    fn round_trips_empty_input() {
        assert_eq!(inflate_zlib(&base64(EMPTY), 0), Some(Vec::new()));
    }

    #[test]
    fn keeps_inflating_a_large_but_bounded_expansion_ratio() {
        // The cap must not fire on real content. AWD bodies are float arrays with long repeating runs, so
        // a high ratio is ordinary; the cap is for a ratio that is unbounded, not merely large. This
        // fixture is 124 bytes expanding 528x to 64 KB and must round-trip byte for byte.
        let expected: Vec<u8> = (0..64 * 1024).map(|index| (index % 7) as u8).collect();
        assert_eq!(inflate_zlib(&base64(HIGH_RATIO), 0), Some(expected));
    }

    #[test]
    fn stops_expansion_at_the_container_declared_bound() {
        let compressed = base64(HIGH_RATIO);
        assert_eq!(
            inflate_zlib(&compressed, 64 * 1024).map(|out| out.len()),
            Some(64 * 1024)
        );
        assert_eq!(inflate_zlib(&compressed, 64 * 1024 - 1), None);
    }

    #[test]
    fn round_trips_a_short_literal_run_zlib_wrapped() {
        assert_eq!(
            inflate_zlib(&base64(LITERAL), 0).as_deref(),
            Some(&b"flighthq scene-formats"[..])
        );
    }

    #[test]
    fn round_trips_a_headerless_raw_stream_when_explicitly_requested() {
        assert_eq!(
            inflate_raw(&base64(RAW_LITERAL), 0).as_deref(),
            Some(&b"flighthq scene-formats"[..])
        );
    }

    #[test]
    fn does_not_mistake_a_valid_raw_stream_beginning_78_9c_for_zlib_framing() {
        // The framing is the container's fact to supply, precisely because a raw stream can open with the
        // common zlib header. Guessing here would decode this correctly and the next file wrongly.
        let stream = hex(RAW_STORED_LOOKS_LIKE_ZLIB);
        let payload: Vec<u8> = (0..RAW_STORED_PAYLOAD_LEN)
            .map(|index| index as u8)
            .collect();
        assert_eq!(inflate_raw(&stream, payload.len()), Some(payload));
        assert_eq!(inflate_zlib(&stream, RAW_STORED_PAYLOAD_LEN), None);
    }

    #[test]
    fn round_trips_repetitive_data_through_back_references_and_buffer_growth() {
        // 5400 bytes out, past the 1024-byte initial buffer, so the grow path runs.
        let expected = "abcABC123".repeat(600).into_bytes();
        assert_eq!(inflate_zlib(&base64(REPETITIVE), 0), Some(expected));
    }

    #[test]
    fn round_trips_prose_through_a_genuine_dynamic_huffman_block() {
        let expected = LOREM.repeat(12).into_bytes();
        assert_eq!(inflate_zlib(&base64(LOREM_COMPRESSED), 0), Some(expected));
    }

    #[test]
    fn round_trips_a_stored_level_zero_block() {
        let expected = b"the quick brown fox jumps over the lazy dog".to_vec();
        assert_eq!(inflate_zlib(&base64(STORED_L0), 0), Some(expected));
    }

    #[test]
    fn refuses_a_truncated_stream_rather_than_panicking() {
        assert_eq!(inflate_zlib(&base64(REPETITIVE)[..6], 0), None);
    }

    #[test]
    fn refuses_an_invalid_block_type() {
        // 0x78 0x9c is a valid zlib header; 0xff 0xff after it decodes block type 3.
        assert_eq!(inflate_zlib(&[0x78, 0x9c, 0xff, 0xff, 0, 0, 0, 0], 0), None);
    }

    #[test]
    fn refuses_an_undersized_zlib_wrapper_and_an_unknown_framing() {
        assert_eq!(inflate_zlib(&[0x78, 0x9c, 0, 0, 0], 0), None);
        // Upstream returns null for a framing it does not know; here that is unrepresentable by
        // construction, and `from_code` is the place the same refusal happens.
        assert_eq!(Framing::from_code(2), None);
        assert_eq!(Framing::from_code(0), Some(Framing::Raw));
        assert_eq!(Framing::from_code(1), Some(Framing::Rfc1950));
    }

    #[test]
    fn refuses_truncated_stored_block_headers_and_payloads() {
        assert_eq!(inflate_raw(&[0x01, 0, 0], 0), None);
        assert_eq!(inflate_raw(&[0x01, 0x02, 0, 0xfd, 0xff, 0x41], 0), None);
    }

    #[test]
    fn refuses_reserved_fixed_length_symbols_and_back_references_before_the_output() {
        assert_eq!(inflate_raw(&hex(FIXED_RESERVED_LENGTH_286), 0), None);
        assert_eq!(inflate_raw(&hex(FIXED_BACKREF_BEFORE_OUTPUT), 0), None);
    }

    #[test]
    fn refuses_a_reserved_dynamic_distance_symbol() {
        assert_eq!(inflate_raw(&hex(RESERVED_DYNAMIC_DISTANCE), 0), None);
    }

    #[test]
    fn refuses_a_missing_or_corrupt_zlib_adler32_trailer() {
        let valid = base64(LITERAL);
        assert_eq!(inflate_zlib(&valid[..valid.len() - 4], 0), None);
        let mut corrupt = valid.clone();
        let last = corrupt.len() - 1;
        corrupt[last] ^= 1;
        assert_eq!(inflate_zlib(&corrupt, 0), None);
    }

    #[test]
    fn refuses_illegal_zlib_methods_windows_check_bits_and_preset_dictionaries() {
        let mut illegal_method = base64(LITERAL);
        illegal_method[0] = 0x77;
        illegal_method[1] = 0x09; // FCHECK-valid for CM=7, so only the forbidden method rejects it.
        assert_eq!(inflate_zlib(&illegal_method, 0), None);

        let mut illegal_window = base64(LITERAL);
        illegal_window[0] = 0xf8;
        illegal_window[1] = 0x00;
        assert_eq!(inflate_zlib(&illegal_window, 0), None);

        let mut illegal_check = base64(LITERAL);
        illegal_check[1] ^= 1;
        assert_eq!(inflate_zlib(&illegal_check, 0), None);

        let mut preset_dictionary = base64(LITERAL);
        preset_dictionary[1] = 0xbb;
        assert_eq!(inflate_zlib(&preset_dictionary, 0), None);
    }

    #[test]
    fn refuses_a_dynamic_code_length_repeat_that_exceeds_the_declared_table() {
        assert_eq!(
            inflate_raw(&hex("05c0050900000000a0ffaf15"), 0),
            Some(Vec::new())
        );
        assert_eq!(inflate_raw(&hex("05c0050900000000a0ffaf0d"), 0), None);
    }

    #[test]
    fn refuses_each_dynamic_repeat_symbol_that_overruns_the_declared_table() {
        // A truncating decoder turns each of these into a valid empty block, which is exactly the bug the
        // bound exists to prevent: the overrun is hidden by the table boundary rather than reported.
        for stream in [
            DYNAMIC_REPEAT_OVERFLOW_16,
            DYNAMIC_REPEAT_OVERFLOW_17,
            DYNAMIC_REPEAT_OVERFLOW_18,
        ] {
            assert_eq!(
                inflate_raw(&hex(stream), 0),
                None,
                "overflow stream {stream} must be refused"
            );
        }
    }

    #[test]
    fn accepts_every_dynamic_repeat_form_that_ends_inside_the_declared_table() {
        for stream in [
            DYNAMIC_REPEAT_VALID_16,
            DYNAMIC_REPEAT_VALID_17,
            DYNAMIC_REPEAT_VALID_18,
        ] {
            assert_eq!(
                inflate_raw(&hex(stream), 0),
                Some(Vec::new()),
                "valid stream {stream} must decode"
            );
        }
    }

    #[test]
    fn refuses_a_previous_length_repeat_before_any_length_is_declared() {
        assert_eq!(inflate_raw(&hex(DYNAMIC_REPEAT_NO_PREVIOUS), 0), None);
    }

    #[test]
    fn refuses_forbidden_dynamic_literal_counts_without_refusing_the_maximum_valid_count() {
        assert_eq!(
            inflate_raw(&hex("edc0210900000000a0ffaf5d22"), 0),
            Some(Vec::new())
        );
        assert_eq!(inflate_raw(&hex("f5c0210900000000a0ffaf7d22"), 0), None);
        assert_eq!(inflate_raw(&hex("fdc0210900000000a0ffaf9d22"), 0), None);
    }
}
