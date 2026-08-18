// @generated from upstream/packages/bitmap/src/bitmapChannel.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::invalidate_bitmap;
use flighthq_types::BitmapRegion;

// Source: upstream/packages/bitmap/src/bitmapChannel.ts:18 (sha256:047f722d8abba9cadcd8dff8df5ce2486323dfe8e5d0b05703b836dc903a682a)
pub fn merge_bitmap_channels(
    out: &mut BitmapRegion,
    r: &BitmapRegion,
    g: &BitmapRegion,
    b: &BitmapRegion,
    a: &BitmapRegion,
) -> () {
    let w = ((((out.width).min(r.width)).min(g.width)).min(b.width)).min(a.width);
    let h = ((((out.height).min(r.height)).min(g.height)).min(b.height)).min(a.height);
    {
        let mut py = 0.0_f64;
        while (py < h) {
            let oy = (out.y + py);
            let ry = (r.y + py);
            let gy = (g.y + py);
            let by = (b.y + py);
            let ay = (a.y + py);
            if (((((((((oy < 0.0_f64) || (oy >= out.bitmap.height)) || (ry < 0.0_f64))
                || (ry >= r.bitmap.height))
                || (gy < 0.0_f64))
                || (gy >= g.bitmap.height))
                || (by < 0.0_f64))
                || (by >= b.bitmap.height))
                || (ay < 0.0_f64))
                || (ay >= a.bitmap.height)
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
                    let ox = (out.x + px);
                    let rx = (r.x + px);
                    let gx = (g.x + px);
                    let bx = (b.x + px);
                    let ax = (a.x + px);
                    if (((((((((ox < 0.0_f64) || (ox >= out.bitmap.width)) || (rx < 0.0_f64))
                        || (rx >= r.bitmap.width))
                        || (gx < 0.0_f64))
                        || (gx >= g.bitmap.width))
                        || (bx < 0.0_f64))
                        || (bx >= b.bitmap.width))
                        || (ax < 0.0_f64))
                        || (ax >= a.bitmap.width)
                    {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let di = (((oy * out.bitmap.width) + ox) * 4.0_f64);
                    out.bitmap.data[di as usize] = (r.bitmap.data
                        [(((ry * r.bitmap.width) + rx) * 4.0_f64) as usize]
                        as f64) as u8;
                    out.bitmap.data[(di + 1.0_f64) as usize] = (g.bitmap.data
                        [((((gy * g.bitmap.width) + gx) * 4.0_f64) + 1.0_f64) as usize]
                        as f64)
                        as u8;
                    out.bitmap.data[(di + 2.0_f64) as usize] = (b.bitmap.data
                        [((((by * b.bitmap.width) + bx) * 4.0_f64) + 2.0_f64) as usize]
                        as f64)
                        as u8;
                    out.bitmap.data[(di + 3.0_f64) as usize] = (a.bitmap.data
                        [((((ay * a.bitmap.width) + ax) * 4.0_f64) + 3.0_f64) as usize]
                        as f64)
                        as u8;
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
