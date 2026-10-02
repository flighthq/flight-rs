//! Upstream's `convolveBitmap`, mirrored structurally with integer indices and an `EdgeMode` enum.

use crate::{EdgeMode, RegionView, clamp_byte, convolution_divisor, resolve_mirror};

/// Errors this kernel refuses on, mirroring the two `throw`s in upstream's `convolveBitmap`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConvolveError {
    NonPositiveDimensions,
    MatrixTooShort,
}

impl ConvolveError {
    /// Upstream's own message, so a caller that surfaces this across the wasm boundary raises what upstream does.
    pub fn message(self) -> &'static str {
        match self {
            Self::NonPositiveDimensions => "Convolution filter matrix dimensions must be positive",
            Self::MatrixTooShort => "Convolution filter matrix does not match its dimensions",
        }
    }
}

/// Applies a convolution kernel to `source`, writing `source.width * source.height * 4` bytes into `out`.
///
/// A structural port of upstream's `convolveBitmap`. `out` must not alias `source.data`, which upstream also
/// documents — convolution reads neighbouring pixels, so an overlapping destination is undefined there and here.
///
/// The caller must have established that the region's coordinates and extent are integral; `RegionView` cannot
/// check it, because by then the `f64` has already been narrowed.
#[allow(clippy::too_many_arguments)]
pub fn convolve_bitmap(
    out: &mut [u8],
    source: &RegionView<'_>,
    matrix: &[f64],
    matrix_x: i32,
    matrix_y: i32,
    divisor: Option<f64>,
    bias: f64,
    edge: EdgeMode,
    preserve_alpha: bool,
) -> Result<(), ConvolveError> {
    if matrix_x <= 0 || matrix_y <= 0 {
        return Err(ConvolveError::NonPositiveDimensions);
    }
    let taps = (matrix_x as i64) * (matrix_y as i64);
    if (matrix.len() as i64) < taps {
        return Err(ConvolveError::MatrixTooShort);
    }

    // `?? divisor` then `=== 0 ? 1`, in upstream's order: an explicit zero is a passthrough rather than a
    // division by zero, and that is a behaviour its suite pins directly.
    let raw_divisor = divisor.unwrap_or_else(|| convolution_divisor(matrix, taps as usize));
    let divisor = if raw_divisor == 0.0 { 1.0 } else { raw_divisor };
    let offset_x = matrix_x / 2;
    let offset_y = matrix_y / 2;
    let bitmap_width = source.bitmap_width;
    let bitmap_height = source.bitmap_height;
    let data = source.data;

    for py in 0..source.height {
        for px in 0..source.width {
            let mut r = 0.0;
            let mut g = 0.0;
            let mut b = 0.0;
            let mut a = 0.0;
            for ky in 0..matrix_y {
                let raw_sample_y = source.y + py + ky - offset_y;
                let weight_row_start = ky * matrix_x;
                for kx in 0..matrix_x {
                    let raw_sample_x = source.x + px + kx - offset_x;
                    let weight = matrix[(weight_row_start + kx) as usize];
                    let sample_x;
                    let sample_y;

                    if raw_sample_y < 0
                        || raw_sample_y >= bitmap_height
                        || raw_sample_x < 0
                        || raw_sample_x >= bitmap_width
                    {
                        match edge {
                            EdgeMode::Transparent => continue,
                            EdgeMode::Wrap => {
                                sample_x =
                                    ((raw_sample_x % bitmap_width) + bitmap_width) % bitmap_width;
                                sample_y = ((raw_sample_y % bitmap_height) + bitmap_height)
                                    % bitmap_height;
                            }
                            EdgeMode::Mirror => {
                                sample_x = resolve_mirror(raw_sample_x, bitmap_width);
                                sample_y = resolve_mirror(raw_sample_y, bitmap_height);
                            }
                            EdgeMode::Clamp => {
                                sample_x = raw_sample_x.clamp(0, bitmap_width - 1);
                                sample_y = raw_sample_y.clamp(0, bitmap_height - 1);
                            }
                        }
                    } else {
                        sample_x = raw_sample_x;
                        sample_y = raw_sample_y;
                    }

                    let i =
                        ((sample_y as i64 * bitmap_width as i64 + sample_x as i64) * 4) as usize;
                    // Accumulated in upstream's order, because float addition is not associative and the
                    // conformance suite compares exact bytes.
                    r += data[i] as f64 * weight;
                    g += data[i + 1] as f64 * weight;
                    b += data[i + 2] as f64 * weight;
                    a += data[i + 3] as f64 * weight;
                }
            }
            let di = ((py as i64 * source.width as i64 + px as i64) * 4) as usize;
            out[di] = clamp_byte(r / divisor + bias);
            out[di + 1] = clamp_byte(g / divisor + bias);
            out[di + 2] = clamp_byte(b / divisor + bias);
            if preserve_alpha {
                let cy = (source.y + py).clamp(0, bitmap_height - 1);
                let cx = (source.x + px).clamp(0, bitmap_width - 1);
                out[di + 3] =
                    data[((cy as i64 * bitmap_width as i64 + cx as i64) * 4 + 3) as usize];
            } else {
                out[di + 3] = clamp_byte(a / divisor + bias);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EdgeMode, RegionView};

    // Every case below is upstream's own, from `upstream/packages/bitmap/src/bitmapConvolution.test.ts`, with
    // its fixtures and expectations carried over rather than invented. The suite itself also runs against this
    // kernel unmodified through the conformance lane; these exist so `cargo test` alone still covers the four
    // edge modes, and so a change here fails without needing the wasm artifact rebuilt.

    /// Upstream's `createBitmap(width, height, color)`: RGBA8, `color` as 0xRRGGBBAA, default transparent black.
    fn bitmap(width: i32, height: i32, color: u32) -> Vec<u8> {
        let mut data = vec![0u8; (width * height * 4) as usize];
        for pixel in data.chunks_exact_mut(4) {
            pixel[0] = (color >> 24) as u8;
            pixel[1] = (color >> 16) as u8;
            pixel[2] = (color >> 8) as u8;
            pixel[3] = color as u8;
        }
        data
    }

    fn view<'a>(data: &'a [u8], width: i32, height: i32) -> RegionView<'a> {
        RegionView {
            data,
            bitmap_width: width,
            bitmap_height: height,
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    /// `[10, 20, 30]` in the red channel of a 3×1 bitmap, which several upstream cases share.
    fn ramp() -> Vec<u8> {
        let mut data = bitmap(3, 1, 0);
        data[0] = 10;
        data[4] = 20;
        data[8] = 30;
        data
    }

    #[test]
    fn applies_a_convolution_matrix_to_the_source_region() {
        let data = ramp();
        let source = RegionView {
            data: &data,
            bitmap_width: 3,
            bitmap_height: 1,
            x: 1,
            y: 0,
            width: 1,
            height: 1,
        };
        let mut out = vec![0u8; 4];
        convolve_bitmap(
            &mut out,
            &source,
            &[1.0, 1.0, 1.0],
            3,
            1,
            None,
            0.0,
            EdgeMode::Clamp,
            false,
        )
        .unwrap();
        assert_eq!(out[0], 20);
    }

    #[test]
    fn clamp_edge_mode_repeats_the_nearest_edge_pixel() {
        let data = bitmap(1, 1, 0x606060ff);
        let mut out = vec![0u8; 4];
        convolve_bitmap(
            &mut out,
            &view(&data, 1, 1),
            &[1.0, 1.0, 1.0],
            3,
            1,
            Some(3.0),
            0.0,
            EdgeMode::Clamp,
            false,
        )
        .unwrap();
        assert_eq!(out[0], 0x60);
    }

    #[test]
    fn mirror_edge_mode_reflects_at_boundaries() {
        // 5-wide kernel picking two positions left: mirror(-2, 3) is x=1 (value 20), where clamp would give 10.
        let data = ramp();
        let mut out = vec![0u8; 3 * 4];
        convolve_bitmap(
            &mut out,
            &view(&data, 3, 1),
            &[1.0, 0.0, 0.0, 0.0, 0.0],
            5,
            1,
            Some(1.0),
            0.0,
            EdgeMode::Mirror,
            false,
        )
        .unwrap();
        assert_eq!(out[0], 20);
    }

    #[test]
    fn transparent_edge_mode_treats_out_of_bounds_as_transparent_black() {
        let data = bitmap(1, 1, 0x000000ff);
        let mut out = vec![0u8; 4];
        convolve_bitmap(
            &mut out,
            &view(&data, 1, 1),
            &[1.0, 0.0, 0.0],
            3,
            1,
            Some(1.0),
            0.0,
            EdgeMode::Transparent,
            false,
        )
        .unwrap();
        assert_eq!(out[0], 0);
        assert_eq!(out[3], 0);
    }

    #[test]
    fn wrap_edge_mode_tiles_the_source_toroidally() {
        let data = ramp();
        let mut out = vec![0u8; 3 * 4];
        convolve_bitmap(
            &mut out,
            &view(&data, 3, 1),
            &[1.0, 0.0, 0.0],
            3,
            1,
            Some(1.0),
            0.0,
            EdgeMode::Wrap,
            false,
        )
        .unwrap();
        // The left neighbour of px=0 wraps to px=2.
        assert_eq!(out[0], 30);
    }

    #[test]
    fn preserves_source_alpha_by_default() {
        // A bias of 255 would saturate the alpha channel if it were computed rather than copied.
        let data = bitmap(1, 1, 0x00000044);
        let mut out = vec![0u8; 4];
        convolve_bitmap(
            &mut out,
            &view(&data, 1, 1),
            &[1.0],
            1,
            1,
            None,
            255.0,
            EdgeMode::Clamp,
            true,
        )
        .unwrap();
        assert_eq!(out[3], 0x44);
    }

    #[test]
    fn refuses_a_matrix_whose_dimensions_are_not_positive() {
        let data = bitmap(1, 1, 0);
        let mut out = vec![0u8; 4];
        assert_eq!(
            convolve_bitmap(
                &mut out,
                &view(&data, 1, 1),
                &[],
                0,
                1,
                None,
                0.0,
                EdgeMode::Clamp,
                true
            ),
            Err(ConvolveError::NonPositiveDimensions),
        );
    }

    #[test]
    fn refuses_a_matrix_too_short_for_its_declared_dimensions() {
        let data = bitmap(1, 1, 0);
        let mut out = vec![0u8; 4];
        assert_eq!(
            convolve_bitmap(
                &mut out,
                &view(&data, 1, 1),
                &[1.0],
                3,
                3,
                None,
                0.0,
                EdgeMode::Clamp,
                true
            ),
            Err(ConvolveError::MatrixTooShort),
        );
    }

    #[test]
    fn treats_an_explicit_divisor_of_zero_as_passthrough() {
        let data = bitmap(1, 1, 0x804020ff);
        let mut out = vec![0u8; 4];
        convolve_bitmap(
            &mut out,
            &view(&data, 1, 1),
            &[1.0],
            1,
            1,
            Some(0.0),
            0.0,
            EdgeMode::Clamp,
            false,
        )
        .unwrap();
        assert_eq!(out[0], 0x80);
    }

    #[test]
    fn an_unrecognised_edge_mode_clamps_as_upstream_does() {
        // Upstream's if/else chain ends in the clamp branch, so this port must not be stricter.
        assert_eq!(EdgeMode::from_upstream("nonsense"), EdgeMode::Clamp);
        assert_eq!(EdgeMode::from_upstream("clamp"), EdgeMode::Clamp);
        assert_eq!(EdgeMode::from_upstream("wrap"), EdgeMode::Wrap);
        assert_eq!(EdgeMode::from_upstream("mirror"), EdgeMode::Mirror);
        assert_eq!(
            EdgeMode::from_upstream("transparent"),
            EdgeMode::Transparent
        );
    }

    #[test]
    fn rounds_and_clamps_the_way_javascript_does() {
        // `Math.round` is floor(x + 0.5), so exact halves go toward +∞.
        assert_eq!(clamp_byte(0.5), 1);
        assert_eq!(clamp_byte(1.5), 2);
        assert_eq!(clamp_byte(2.5), 3);
        // Out of range in both directions, and negative halves, all clamp.
        assert_eq!(clamp_byte(-0.5), 0);
        assert_eq!(clamp_byte(-2.5), 0);
        assert_eq!(clamp_byte(255.4), 255);
        assert_eq!(clamp_byte(1e9), 255);
        assert_eq!(clamp_byte(254.5), 255);
    }
}
