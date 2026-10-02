//! Hand-written Rust mirrors of the `@flighthq/bitmap` kernels, where the generated lowering is slower than the
//! TypeScript it replaces.
//!
//! # Why this crate exists
//!
//! `packages/bitmap-wasm` wraps generated Rust, and the generated kernels measured **0.24x-0.68x** of upstream's
//! TypeScript — slower, at every size and on every function tried. Two measurements located the cause before any
//! of this was written:
//!
//! 1. **It is not the wasm boundary.** Copying the pixel buffer in and back out is 6% of a 1024² call, so even a
//!    zero-copy facade would still have lost at 0.71x. That is what licenses not rewriting the marshalling.
//! 2. **It is the arithmetic.** The generated code runs loop counters in `f64`, computes every array index in
//!    floating point and casts per access, and compares closed string unions as owned `String`s inside the
//!    innermost loop — up to three comparisons per kernel tap.
//!
//! Each kernel here removes exactly those two things and changes nothing else. `agents/wasm-barrier.md` carries
//! both tables.
//!
//! # This is an interim, not the destination
//!
//! **The permanent fix is integer induction variables and closed-union enums in the generator.** That would fix
//! every generated package at once and keep each kernel tracking upstream automatically, which is what the whole
//! repository is for. This crate exists because that lowering work has no delivery date and the wasm packages are
//! separately maintained so they can ship sooner — a deliberate trade, recorded here so nobody later reads these
//! files as the intended end state and ports the remaining twenty-seven by hand out of consistency.
//!
//! When the generator grows integer lowering, the right move is to delete a module from this crate and re-point
//! its facade binding back at the generated kernel, one at a time, with `npm run bench:barrier` confirming each
//! swap does not regress.
//!
//! # The cost this accepts, and what contains it
//!
//! A mirrored kernel stops following upstream on its own. Four things keep that honest:
//!
//! - **Structural ports only.** Each function follows upstream's own control flow and order of operations. The
//!   only deliberate differences are representational: integer counters and indices, an enum where upstream has a
//!   string union, and — in `multiply_bitmap_alpha` — a 256-entry table holding exactly what the per-pixel
//!   arithmetic would have produced.
//! - **Floating-point arithmetic is never changed.** Accumulators stay `f64` and accumulate in upstream's
//!   sequence, because float addition is not associative and the conformance suite compares exact bytes.
//! - **Upstream's own suite is the oracle.** All 372 of its bitmap tests run unmodified against this crate
//!   through `packages/bitmap-wasm/vitest.config.upstream.ts`.
//! - **Differential fixtures whose expected bytes come from upstream's TypeScript**, in `tests/`, reaching the
//!   input space upstream's hand-written cases do not: random RGBA, regions hanging off the bitmap edge, partial
//!   windows, fractional parameters. Mutation testing earned these — swapping two colour channels passed all of
//!   upstream's hand-written cases, and replacing a rounding clamp with a truncating cast passed every test in
//!   this crate until the fixture started generating fractional alpha.
//!
//! # Measured result
//!
//! Every mirrored kernel now beats the TypeScript it replaces, at 256² and above:
//!
//! | Kernel                    | generated   | mirrored          |
//! | ------------------------- | ----------: | ----------------: |
//! | `multiplyBitmapAlpha`     | 0.31x-0.33x | **3.13x - 3.18x** |
//! | `setBitmapAlpha`          | 0.63x-0.68x | **1.95x - 2.17x** |
//! | `pixelateBitmap`          | 0.44x-0.54x | **1.69x - 1.74x** |
//! | `convolveBitmap` 5x5      | 0.44x-0.50x | **1.52x - 1.54x** |
//! | `dilateBitmap` / `erode`  | 0.52x-0.69x | **1.33x - 1.44x** |
//!
//! **At 16² the mirrors still lose** (0.42x for `setBitmapAlpha`, 0.69x for `multiplyBitmapAlpha`), and no amount
//! of kernel work fixes that: a 1 KB operation is dominated by the cost of crossing the boundary at all. That is
//! the real boundary of this approach, and it is an argument about call size rather than about kernels.
//!
//! # What is not mirrored
//!
//! Twenty-seven of the facade's thirty-three wasm-backed exports still use the generated kernels, and are
//! therefore still slower than upstream. They were left because the six here cover the families with real
//! arithmetic per pixel, and because each mirror has to earn its own differential fixture before it is believed.
//! `agents/wasm-barrier.md` ranks what remains and records which functions should never cross the barrier at all.

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
