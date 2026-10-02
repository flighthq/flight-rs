//! Hand-written Rust mirrors of the `@flighthq/bitmap` kernels where the generated lowering is measurably
//! slower than the TypeScript it replaces.
//!
//! This crate exists for one reason, and it is a measurement rather than a preference. `npm run bench:barrier`
//! reports the shipped generated bitmap wasm at roughly half the speed of upstream's TypeScript at every size
//! and on every function tried, and a second measurement shows the wasm boundary is not where the time goes:
//! copying the pixel buffer in and back out is 6% of a 1024² call, so even a zero-copy facade would still lose.
//! `agents/wasm-barrier.md` records both tables and the diagnosis — `f64` loop counters, every array index
//! computed in floating point and cast per access, and closed string unions lowered to `String` so an edge-mode
//! check becomes a string comparison inside the innermost loop.
//!
//! What this crate is NOT is a second implementation. Each kernel here is a structural port of upstream's own
//! function, in upstream's order of operations, held to upstream's own test suite through the conformance lane
//! in `packages/bitmap-wasm/vitest.config.upstream.ts`. The only deliberate differences are representational:
//! integer induction variables and indices, and an enum where upstream has a string union. Those are exactly the
//! two lowering improvements `agents/wasm-barrier.md` asks the generator for; doing them by hand here is the
//! interim, and the precedent is `crates/flighthq-compression-core`.
//!
//! Floating-point arithmetic is deliberately NOT changed. Accumulators stay `f64` and accumulate in upstream's
//! order, because float addition is not associative and the conformance suite compares exact bytes.

#![forbid(unsafe_code)]

mod alpha;
mod convolution;
mod morphological;
mod pixelate;

pub use alpha::{multiply_bitmap_alpha, set_bitmap_alpha};
pub use convolution::{ConvolveError, convolve_bitmap};
pub use morphological::{Morphology, apply_morphological};
pub use pixelate::pixelate_bitmap;

/// How a kernel samples outside the bitmap.
///
/// Upstream models this as the closed string union `'clamp' | 'transparent' | 'wrap' | 'mirror'`. An enum is the
/// representation that union deserves: the generated lowering makes it an owned `String` and compares it up to
/// three times per kernel tap — seventy-five times per pixel for a 5×5 — where this costs nothing.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EdgeMode {
    /// Repeat the nearest edge pixel. Upstream's default, and also its fallback for an unrecognised value.
    #[default]
    Clamp,
    /// Treat out-of-bounds samples as transparent black by skipping the tap entirely.
    Transparent,
    /// Tile the bitmap toroidally.
    Wrap,
    /// Reflect the image at boundaries.
    Mirror,
}

impl EdgeMode {
    /// Maps upstream's string union, falling back to `Clamp` for anything unrecognised.
    ///
    /// The fallback is not leniency for its own sake: upstream's `if/else if/else` chain ends in the clamp
    /// branch, so an unknown string already clamps there. Returning an error here would make this port stricter
    /// than the thing it mirrors.
    pub fn from_upstream(value: &str) -> Self {
        match value {
            "transparent" => Self::Transparent,
            "wrap" => Self::Wrap,
            "mirror" => Self::Mirror,
            _ => Self::Clamp,
        }
    }
}

/// A rectangular view into a bitmap's pixel buffer, in integer pixel coordinates.
///
/// Upstream's `BitmapRegion` carries these as `number`, which is `f64`. The caller is responsible for
/// establishing that they are integral before building one of these — see `convolve_bitmap`'s contract — so that
/// this crate can index in integers without changing what any arithmetic means.
#[derive(Clone, Copy, Debug)]
pub struct RegionView<'a> {
    /// The whole bitmap's pixels, RGBA8, `bitmap_width * bitmap_height * 4` bytes.
    pub data: &'a [u8],
    pub bitmap_width: i32,
    pub bitmap_height: i32,
    /// The region's origin within the bitmap.
    pub x: i32,
    pub y: i32,
    /// The region's extent.
    pub width: i32,
    pub height: i32,
}

/// Rounds the way JavaScript's `Math.round` does, then clamps to a byte.
///
/// `Math.round` is defined as rounding half toward +∞, which is `floor(x + 0.5)` — NOT Rust's `f64::round`,
/// which rounds half away from zero. The two disagree only on exact negative halves, and every such value
/// clamps to 0 here anyway, so this could have used either; it uses the JavaScript definition because matching
/// upstream exactly is cheaper to reason about than arguing that a difference is unobservable.
pub(crate) fn clamp_byte(value: f64) -> u8 {
    let rounded = (value + 0.5).floor();
    if rounded <= 0.0 {
        0
    } else if rounded >= 255.0 {
        255
    } else {
        rounded as u8
    }
}

/// Upstream's implicit divisor: the sum of the first `length` weights, or 1 when they sum to zero.
pub(crate) fn convolution_divisor(matrix: &[f64], length: usize) -> f64 {
    // Summed in upstream's order over the first `length` weights; float addition is not associative, so the
    // iterator form has to preserve the sequence rather than merely the set.
    let mut sum = 0.0;
    for weight in matrix.iter().take(length) {
        sum += weight;
    }
    if sum == 0.0 { 1.0 } else { sum }
}

/// Reflects `v` into `[0, size)`, the period being `2 * size`.
pub(crate) fn resolve_mirror(v: i32, size: i32) -> i32 {
    let period = 2 * size;
    let wrapped = ((v % period) + period) % period;
    if wrapped < size {
        wrapped
    } else {
        period - 1 - wrapped
    }
}

/// A rectangular view into a bitmap's pixel buffer that a kernel writes through.
///
/// The mutable twin of [`RegionView`], with the same integral-coordinate contract: the caller establishes that
/// the `f64` coordinates upstream carries are whole numbers before narrowing them here.
#[derive(Debug)]
pub struct RegionViewMut<'a> {
    /// The whole bitmap's pixels, RGBA8, `bitmap_width * bitmap_height * 4` bytes.
    pub data: &'a mut [u8],
    pub bitmap_width: i32,
    pub bitmap_height: i32,
    /// The region's origin within the bitmap.
    pub x: i32,
    pub y: i32,
    /// The region's extent.
    pub width: i32,
    pub height: i32,
}
