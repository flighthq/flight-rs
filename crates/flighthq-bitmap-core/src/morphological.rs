//! Upstream's `dilateBitmap` and `erodeBitmap` from `bitmapMorphological.ts`.
//!
//! These are the clearest case in the package for mirroring: the algorithm is a per-channel min or max over a
//! square window, which is **entirely integer work on bytes** — upstream reaches for no arithmetic that needs a
//! float. The generated version nonetheless runs its window counters and every index in `f64`, which is why it
//! measured 0.52x-0.69x of the TypeScript it replaces.

use crate::RegionView;

/// Which way the window reduces: dilation keeps the brightest sample, erosion the darkest.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Morphology {
    Dilate,
    Erode,
}

/// Writes the window reduction of `source` into `out`, which holds `source.width * source.height * 4` bytes.
///
/// A structural port of upstream's `applyMorphological`, in its order: rows outer, columns inner, and the window
/// walked `ky` then `kx` with each sample's coordinates clamped into the bitmap. Clamping rather than skipping is
/// upstream's choice and it matters at the edges, where it repeats the border pixel.
pub fn apply_morphological(
    out: &mut [u8],
    source: &RegionView<'_>,
    radius: f64,
    morphology: Morphology,
) {
    // `Math.round` on a negative radius then clamping at zero, as upstream does, so a negative request is a
    // one-sample window rather than an error.
    let r = (radius + 0.5).floor().max(0.0) as i32;
    let bitmap_width = source.bitmap_width;
    let bitmap_height = source.bitmap_height;
    let data = source.data;
    let identity: u8 = match morphology {
        Morphology::Dilate => 0,
        Morphology::Erode => 255,
    };

    for py in 0..source.height {
        for px in 0..source.width {
            let mut channels = [identity; 4];
            for ky in -r..=r {
                let sy = (source.y + py + ky).clamp(0, bitmap_height - 1);
                for kx in -r..=r {
                    let sx = (source.x + px + kx).clamp(0, bitmap_width - 1);
                    let si = ((sy as i64 * bitmap_width as i64 + sx as i64) * 4) as usize;
                    for (channel, value) in channels.iter_mut().enumerate() {
                        let sample = data[si + channel];
                        let replace = match morphology {
                            Morphology::Dilate => sample > *value,
                            Morphology::Erode => sample < *value,
                        };
                        if replace {
                            *value = sample;
                        }
                    }
                }
            }
            let di = ((py as i64 * source.width as i64 + px as i64) * 4) as usize;
            out[di..di + 4].copy_from_slice(&channels);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn dilation_spreads_the_brightest_sample_across_the_window() {
        // A single bright pixel in the middle of a 3x1 row fills the row at radius 1.
        let data = vec![0, 0, 0, 0, 200, 210, 220, 230, 0, 0, 0, 0];
        let mut out = vec![0u8; 3 * 4];
        apply_morphological(&mut out, &view(&data, 3, 1), 1.0, Morphology::Dilate);
        assert_eq!(
            out,
            vec![200, 210, 220, 230, 200, 210, 220, 230, 200, 210, 220, 230]
        );
    }

    #[test]
    fn erosion_spreads_the_darkest_sample_across_the_window() {
        let data = vec![255, 255, 255, 255, 10, 20, 30, 40, 255, 255, 255, 255];
        let mut out = vec![0u8; 3 * 4];
        apply_morphological(&mut out, &view(&data, 3, 1), 1.0, Morphology::Erode);
        assert_eq!(out, vec![10, 20, 30, 40, 10, 20, 30, 40, 10, 20, 30, 40]);
    }

    #[test]
    fn a_zero_radius_window_is_the_pixel_itself() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut out = vec![0u8; 2 * 4];
        apply_morphological(&mut out, &view(&data, 2, 1), 0.0, Morphology::Dilate);
        assert_eq!(out, data);
    }

    #[test]
    fn a_negative_radius_clamps_to_a_single_sample_rather_than_failing() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut out = vec![0u8; 2 * 4];
        apply_morphological(&mut out, &view(&data, 2, 1), -3.0, Morphology::Erode);
        assert_eq!(out, data);
    }

    #[test]
    fn samples_outside_the_bitmap_repeat_the_border_rather_than_being_skipped() {
        // At radius 1 on a 2x1 bitmap, the left pixel's window is [border, self, right] — the border repeat is
        // the left pixel again, so a darker right neighbour still wins an erosion.
        let data = vec![100, 100, 100, 100, 10, 10, 10, 10];
        let mut out = vec![0u8; 2 * 4];
        apply_morphological(&mut out, &view(&data, 2, 1), 1.0, Morphology::Erode);
        assert_eq!(out, vec![10, 10, 10, 10, 10, 10, 10, 10]);
    }
}
