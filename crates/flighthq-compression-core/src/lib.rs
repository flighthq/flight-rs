// Hand-written Rust mirrors of the decoders in `@flighthq/compression`.
//
// Hand-written rather than generated, deliberately. Both decoders are bit-serial state machines whose
// JavaScript shape — one bit per method call, probability arrays indexed through closures, throws caught
// and turned into `null` — is exactly what a mechanical lowering would carry over faithfully, and
// exactly what makes them slow. Mirroring the BEHAVIOUR while choosing Rust's shape is the whole reason
// to write them by hand.
//
// What "mirror" obliges is exact, not approximate: for every input, a mirror returns the bytes upstream
// returns and `None` precisely where upstream returns `null`. Refusals are part of the contract, not
// error handling around it — a decoder that accepts one more malformed stream than upstream does has
// changed what the SDK treats as a valid file.
//
// Both expansion caps are mirrored at upstream's values and are load-bearing security behaviour rather
// than tuning constants. The quantity that sizes the allocation is the compression RATIO, which is not
// in the file, is not bounded by its length, and no per-field check can reach; these decoders are
// reachable from any untrusted `.swf` or `.awd`.
//
// Sources of record, and the revisions they were mirrored from, are named on each module. Upstream's own
// tests are the oracle, with fixtures SHARED from upstream rather than reimplemented — see
// `agents/compression-mirror.md` for why that distinction is load-bearing.

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

mod compress;
mod deflate;
mod lzma;

pub use compress::{compress_deflate, compress_deflate_zlib};
pub use deflate::{MAX_INFLATE_BYTES, decompress_deflate};
pub use lzma::{MAX_LZMA_BYTES, decompress_lzma};
