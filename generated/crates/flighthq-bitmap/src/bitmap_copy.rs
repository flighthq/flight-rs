// @generated from upstream/packages/bitmap/src/bitmapCopy.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::invalidate_bitmap;
use flighthq_types::BitmapRegion;

// Source: upstream/packages/bitmap/src/bitmapCopy.ts:48 (sha256:4120a143f608cdf51cc02c0fc691e089232b8f76a2e090920112c8ae5c706519)
pub fn copy_bitmap_pixels(
    dest: &mut BitmapRegion,
    source: &BitmapRegion,
    composite: Option<bool>,
) -> () {
    let composite = composite.unwrap_or(false);
    let w = (dest.width).min(source.width);
    let h = (dest.height).min(source.height);
    {
        let mut py = 0.0_f64;
        while (py < h) {
            let sy = (source.y + py);
            let dy = (dest.y + py);
            if (((sy < 0.0_f64) || (sy >= source.bitmap.height)) || (dy < 0.0_f64))
                || (dy >= dest.bitmap.height)
            {
                {
                    py += 1.0;
                    py
                };
                continue;
            }
            {
                let mut px = 0.0_f64;
                while (px < w) {
                    let sx = (source.x + px);
                    let dx = (dest.x + px);
                    if (((sx < 0.0_f64) || (sx >= source.bitmap.width)) || (dx < 0.0_f64))
                        || (dx >= dest.bitmap.width)
                    {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let si = (((sy * source.bitmap.width) + sx) * 4.0_f64);
                    let di = (((dy * dest.bitmap.width) + dx) * 4.0_f64);
                    if composite {
                        let src_a =
                            ((source.bitmap.data[(si + 3.0_f64) as usize] as f64) / 255.0_f64);
                        let dst_a =
                            ((dest.bitmap.data[(di + 3.0_f64) as usize] as f64) / 255.0_f64);
                        let out_a = (src_a + (dst_a * (1.0_f64 - src_a)));
                        if (out_a > 0.0_f64) {
                            dest.bitmap.data[di as usize] =
                                (((((source.bitmap.data[si as usize] as f64) * src_a)
                                    + (((dest.bitmap.data[di as usize] as f64) * dst_a)
                                        * (1.0_f64 - src_a)))
                                    / out_a)
                                    .round()) as u8;
                            dest.bitmap.data[(di + 1.0_f64) as usize] =
                                (((((source.bitmap.data[(si + 1.0_f64) as usize] as f64) * src_a)
                                    + (((dest.bitmap.data[(di + 1.0_f64) as usize] as f64)
                                        * dst_a)
                                        * (1.0_f64 - src_a)))
                                    / out_a)
                                    .round()) as u8;
                            dest.bitmap.data[(di + 2.0_f64) as usize] =
                                (((((source.bitmap.data[(si + 2.0_f64) as usize] as f64) * src_a)
                                    + (((dest.bitmap.data[(di + 2.0_f64) as usize] as f64)
                                        * dst_a)
                                        * (1.0_f64 - src_a)))
                                    / out_a)
                                    .round()) as u8;
                            dest.bitmap.data[(di + 3.0_f64) as usize] =
                                ((out_a * 255.0_f64).round()) as u8;
                        }
                    } else {
                        dest.bitmap.data[di as usize] =
                            (source.bitmap.data[si as usize] as f64) as u8;
                        dest.bitmap.data[(di + 1.0_f64) as usize] =
                            (source.bitmap.data[(si + 1.0_f64) as usize] as f64) as u8;
                        dest.bitmap.data[(di + 2.0_f64) as usize] =
                            (source.bitmap.data[(si + 2.0_f64) as usize] as f64) as u8;
                        dest.bitmap.data[(di + 3.0_f64) as usize] =
                            (source.bitmap.data[(si + 3.0_f64) as usize] as f64) as u8;
                    }
                    {
                        px += 1.0;
                        px
                    };
                }
            }
            {
                py += 1.0;
                py
            };
        }
    }
    invalidate_bitmap(&mut dest.bitmap);
}
