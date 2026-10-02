//! Upstream's alpha-channel kernels from `bitmapAlpha.ts`, mirrored with integer loop counters and indices.
//!
//! These are the bandwidth-bound end of the package: one byte read or written per pixel, and almost no
//! arithmetic to win. They are mirrored anyway because the generated versions measured 0.31x-0.68x of the
//! TypeScript they replace, and the cost there is not the work — it is computing `(y * width + x) * 4 + 3` in
//! floating point and casting per access. Whether removing that is enough to overtake a JIT that specialises the
//! same loop on `Uint8ClampedArray` is an empirical question, and `npm run bench:barrier` is what answers it.

use crate::{RegionViewMut, clamp_byte};

/// Writes a constant alpha to every pixel in the region, clamping it to a byte.
///
/// A structural port of upstream's `setBitmapAlpha`, including the per-row and per-column bounds checks that let
/// a region hang off the edge of its bitmap. Invalidation is the caller's: the facade calls upstream's
/// `invalidateBitmap` on the JavaScript side, where the version counter lives.
pub fn set_bitmap_alpha(out: &mut RegionViewMut<'_>, alpha: f64) {
    let a = clamp_byte(alpha);
    let bitmap_width = out.bitmap_width;
    for py in 0..out.height {
        let y = out.y + py;
        if y < 0 || y >= out.bitmap_height {
            continue;
        }
        for px in 0..out.width {
            let x = out.x + px;
            if x < 0 || x >= bitmap_width {
                continue;
            }
            let index = ((y as i64 * bitmap_width as i64 + x as i64) * 4 + 3) as usize;
            out.data[index] = a;
        }
    }
}

/// Scales every alpha value in the region by `factor`, which is clamped to `[0, 1]`.
///
/// `Math.round` is `floor(x + 0.5)`, and `data[i] * f` is never negative here because both operands are, so the
/// rounding agrees with upstream without needing the negative-half reasoning [`clamp_byte`] documents.
///
/// The scale is applied through a 256-entry table rather than per pixel. That is not an approximation: the
/// function being computed is `clamp_byte(v * f)` over a `u8`, so there are only 256 possible results and the
/// table holds all of them, each from the same `f64` arithmetic upstream performs. Measured, it is the difference
/// between losing and winning — per pixel this kernel was converting `u8` to `f64`, multiplying, rounding and
/// converting back for one byte of output, which is most of its cost at 0.68x; the table leaves a byte load, a
/// table index and a byte store.
pub fn multiply_bitmap_alpha(out: &mut RegionViewMut<'_>, factor: f64) {
    let f = factor.clamp(0.0, 1.0);
    let scaled: [u8; 256] = std::array::from_fn(|value| clamp_byte(value as f64 * f));
    let bitmap_width = out.bitmap_width;
    for py in 0..out.height {
        let y = out.y + py;
        if y < 0 || y >= out.bitmap_height {
            continue;
        }
        for px in 0..out.width {
            let x = out.x + px;
            if x < 0 || x >= bitmap_width {
                continue;
            }
            let index = ((y as i64 * bitmap_width as i64 + x as i64) * 4 + 3) as usize;
            out.data[index] = scaled[out.data[index] as usize];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn region(data: &mut Vec<u8>, width: i32, height: i32) -> RegionViewMut<'_> {
        RegionViewMut {
            data,
            bitmap_width: width,
            bitmap_height: height,
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    #[test]
    fn sets_every_alpha_and_leaves_rgb_alone() {
        let mut data = vec![10, 20, 30, 255, 40, 50, 60, 255];
        set_bitmap_alpha(&mut region(&mut data, 2, 1), 128.0);
        assert_eq!(data, vec![10, 20, 30, 128, 40, 50, 60, 128]);
    }

    #[test]
    fn rounds_a_fractional_alpha_rather_than_truncating_it() {
        // The difference between `Math.round` and a truncating cast shows up only here: both agree on every
        // integer and on out-of-range values, and disagree on 128.7. A mutation replacing the clamp with
        // `alpha as u8` passed every other test in this crate.
        let mut data = vec![0, 0, 0, 0];
        set_bitmap_alpha(&mut region(&mut data, 1, 1), 128.7);
        assert_eq!(data[3], 129);
        set_bitmap_alpha(&mut region(&mut data, 1, 1), 128.4);
        assert_eq!(data[3], 128);
        set_bitmap_alpha(&mut region(&mut data, 1, 1), 128.5);
        assert_eq!(data[3], 129);
    }

    #[test]
    fn clamps_the_alpha_it_is_given() {
        let mut data = vec![0, 0, 0, 0, 0, 0, 0, 0];
        set_bitmap_alpha(&mut region(&mut data, 2, 1), 999.0);
        assert_eq!(data[3], 255);
        set_bitmap_alpha(&mut region(&mut data, 2, 1), -5.0);
        assert_eq!(data[3], 0);
    }

    #[test]
    fn scales_alpha_and_rounds_half_up() {
        // 255 * 0.5 is 127.5, which `Math.round` takes to 128 rather than 127.
        let mut data = vec![0, 0, 0, 255];
        multiply_bitmap_alpha(&mut region(&mut data, 1, 1), 0.5);
        assert_eq!(data[3], 128);
    }

    #[test]
    fn the_scale_table_agrees_with_per_pixel_arithmetic_for_every_byte_and_factor() {
        // The table is only sound if it holds exactly what the per-pixel form would have produced, so this
        // compares all 256 entries against that arithmetic across a sweep of factors.
        for step in 0..=64 {
            let f = f64::from(step) / 64.0;
            for value in 0u8..=255 {
                let mut data = vec![0, 0, 0, value];
                multiply_bitmap_alpha(&mut region(&mut data, 1, 1), f);
                assert_eq!(
                    data[3],
                    clamp_byte(f64::from(value) * f),
                    "value {value} at factor {f}"
                );
            }
        }
    }

    #[test]
    fn clamps_the_factor_to_the_unit_interval() {
        let mut data = vec![0, 0, 0, 200];
        multiply_bitmap_alpha(&mut region(&mut data, 1, 1), 4.0);
        assert_eq!(data[3], 200);
        multiply_bitmap_alpha(&mut region(&mut data, 1, 1), -1.0);
        assert_eq!(data[3], 0);
    }

    #[test]
    fn skips_pixels_outside_the_bitmap() {
        // A 2x2 region anchored at (1,1) of a 2x2 bitmap covers one real pixel; the rest is off the edge.
        let mut data = vec![0u8; 2 * 2 * 4];
        let mut view = RegionViewMut {
            data: &mut data,
            bitmap_width: 2,
            bitmap_height: 2,
            x: 1,
            y: 1,
            width: 2,
            height: 2,
        };
        set_bitmap_alpha(&mut view, 77.0);
        assert_eq!(data, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 77]);
    }
}
