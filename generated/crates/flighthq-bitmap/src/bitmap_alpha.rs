// @generated from upstream/packages/bitmap/src/bitmapAlpha.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::invalidate_bitmap;
use flighthq_types::BitmapRegion;

// Source: upstream/packages/bitmap/src/bitmapAlpha.ts:13 (sha256:b322e821ee1192294903e0b2563bb786258f3f1f26af9922fb93f406d4c2dfe3)
pub fn copy_bitmap_alpha(dest: &mut BitmapRegion, source: &BitmapRegion) -> () {
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
                    let alpha = (source.bitmap.data[(si + 3.0_f64) as usize] as f64);
                    dest.bitmap.data[(di + 3.0_f64) as usize] = ((alpha).clone()) as u8;
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

// Source: upstream/packages/bitmap/src/bitmapAlpha.ts:43 (sha256:5638b2f277a887dc15d021c2f34b5286559815b2d79748bb3dc47c21eec1ca55)
pub fn multiply_bitmap_alpha(out: &mut BitmapRegion, factor: f64) -> () {
    let f = (0.0_f64).max((1.0_f64).min(factor));
    let bitmap_width = out.bitmap.width;
    {
        let mut py = 0.0_f64;
        while (py < out.height) {
            let y = (out.y + py);
            if (y < 0.0_f64) || (y >= out.bitmap.height) {
                {
                    py += 1.0;
                    py
                };
                continue;
            }
            {
                let mut px = 0.0_f64;
                while (px < out.width) {
                    let x = (out.x + px);
                    if (x < 0.0_f64) || (x >= bitmap_width) {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let i = ((((y * bitmap_width) + x) * 4.0_f64) + 3.0_f64);
                    out.bitmap.data[i as usize] =
                        (((out.bitmap.data[i as usize] as f64) * f).round()) as u8;
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
    invalidate_bitmap(&mut out.bitmap);
}

// Source: upstream/packages/bitmap/src/bitmapAlpha.ts:67 (sha256:536233d94b8ebe6c2f721ade4287d4e092968de39ccfcbb0dacd81c1a916765d)
pub fn set_bitmap_alpha(out: &mut BitmapRegion, alpha: f64) -> () {
    let a = (0.0_f64).max((255.0_f64).min((alpha).round()));
    let bitmap_width = out.bitmap.width;
    {
        let mut py = 0.0_f64;
        while (py < out.height) {
            let y = (out.y + py);
            if (y < 0.0_f64) || (y >= out.bitmap.height) {
                {
                    py += 1.0;
                    py
                };
                continue;
            }
            {
                let mut px = 0.0_f64;
                while (px < out.width) {
                    let x = (out.x + px);
                    if (x < 0.0_f64) || (x >= bitmap_width) {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    out.bitmap.data[((((y * bitmap_width) + x) * 4.0_f64) + 3.0_f64) as usize] =
                        (a) as u8;
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
    invalidate_bitmap(&mut out.bitmap);
}
