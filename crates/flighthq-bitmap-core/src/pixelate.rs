//! Upstream's `pixelateBitmap` from `bitmapPixelate.ts`.
//!
//! Block averaging: integer sums over each block, then one division per block rather than per pixel. The
//! generated version accumulates in `f64` and indexes in `f64`, which is the 0.47x-0.54x it measured.

use crate::{RegionView, clamp_byte};

/// Writes the block-averaged `source` into `out`, which holds `source.width * source.height * 4` bytes.
///
/// A structural port of upstream's `pixelateBitmap`, including its two passes per block: accumulate the samples
/// that fall inside the bitmap, then fill the whole block — including any part that hung off the edge — with the
/// average of those that did. A block with no samples at all is left untouched, as upstream leaves it.
pub fn pixelate_bitmap(out: &mut [u8], source: &RegionView<'_>, block_size: f64) {
    let block = ((block_size + 0.5).floor() as i64).max(1) as i32;
    let bitmap_width = source.bitmap_width;
    let bitmap_height = source.bitmap_height;
    let data = source.data;

    let mut by = 0;
    while by < source.height {
        let y_end = (by + block).min(source.height);
        let mut bx = 0;
        while bx < source.width {
            let x_end = (bx + block).min(source.width);
            // `u32` holds the sum of any block a bitmap can actually carry: 255 per sample needs over sixteen
            // million samples to overflow, which is more pixels than the region can address.
            let mut sums = [0u32; 4];
            let mut count = 0u32;
            for py in by..y_end {
                let sy = source.y + py;
                if sy < 0 || sy >= bitmap_height {
                    continue;
                }
                for px in bx..x_end {
                    let sx = source.x + px;
                    if sx < 0 || sx >= bitmap_width {
                        continue;
                    }
                    let si = ((sy as i64 * bitmap_width as i64 + sx as i64) * 4) as usize;
                    for (channel, sum) in sums.iter_mut().enumerate() {
                        *sum += u32::from(data[si + channel]);
                    }
                    count += 1;
                }
            }
            if count == 0 {
                bx += block;
                continue;
            }
            // One division per block, in `f64` and rounded exactly as upstream's `Math.round` does.
            let divisor = f64::from(count);
            let average: [u8; 4] =
                std::array::from_fn(|channel| clamp_byte(f64::from(sums[channel]) / divisor));
            for py in by..y_end {
                for px in bx..x_end {
                    let di = ((py as i64 * source.width as i64 + px as i64) * 4) as usize;
                    out[di..di + 4].copy_from_slice(&average);
                }
            }
            bx += block;
        }
        by += block;
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
    fn averages_a_block_and_fills_it() {
        // Two pixels, one block: both become the mean, and 10 and 21 average to 15.5 which rounds up.
        let data = vec![10, 0, 0, 0, 21, 0, 0, 0];
        let mut out = vec![0u8; 2 * 4];
        pixelate_bitmap(&mut out, &view(&data, 2, 1), 2.0);
        assert_eq!(out[0], 16);
        assert_eq!(out[4], 16);
    }

    #[test]
    fn a_block_size_below_one_is_one_pixel_per_block() {
        let data = vec![10, 0, 0, 0, 200, 0, 0, 0];
        let mut out = vec![0u8; 2 * 4];
        pixelate_bitmap(&mut out, &view(&data, 2, 1), 0.0);
        assert_eq!(out[0], 10);
        assert_eq!(out[4], 200);
    }

    #[test]
    fn a_partial_block_averages_only_the_pixels_it_has() {
        // Block size 2 over a 3-wide row leaves a one-pixel block at the right edge.
        let data = vec![10, 0, 0, 0, 20, 0, 0, 0, 90, 0, 0, 0];
        let mut out = vec![0u8; 3 * 4];
        pixelate_bitmap(&mut out, &view(&data, 3, 1), 2.0);
        assert_eq!(out[0], 15);
        assert_eq!(out[4], 15);
        assert_eq!(out[8], 90);
    }

    #[test]
    fn a_block_with_no_samples_inside_the_bitmap_is_left_untouched() {
        // The region sits entirely below a 1x1 bitmap, so nothing is accumulated and `out` keeps its contents.
        let data = vec![1, 2, 3, 4];
        let mut out = vec![9u8; 4];
        let source = RegionView {
            data: &data,
            bitmap_width: 1,
            bitmap_height: 1,
            x: 0,
            y: 5,
            width: 1,
            height: 1,
        };
        pixelate_bitmap(&mut out, &source, 1.0);
        assert_eq!(out, vec![9, 9, 9, 9]);
    }
}
