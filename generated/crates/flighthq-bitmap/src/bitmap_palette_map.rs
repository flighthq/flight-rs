// @generated from upstream/packages/bitmap/src/bitmapPaletteMap.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::invalidate_bitmap;
use flighthq_types::BitmapRegion;

// Source: upstream/packages/bitmap/src/bitmapPaletteMap.ts:19 (sha256:6fc71be4bf62ab14ccf0cbb659a76e247a4dc10be3238f2c972d68bece81bca8)
pub fn apply_bitmap_palette_map(
    dest: &mut BitmapRegion,
    source: &BitmapRegion,
    red_map: &Option<Vec<f64>>,
    green_map: &Option<Vec<f64>>,
    blue_map: &Option<Vec<f64>>,
    alpha_map: &Option<Vec<f64>>,
) -> () {
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
                    let r = (source.bitmap.data[si as usize] as f64);
                    let g = (source.bitmap.data[(si + 1.0_f64) as usize] as f64);
                    let b = (source.bitmap.data[(si + 2.0_f64) as usize] as f64);
                    let a = (source.bitmap.data[(si + 3.0_f64) as usize] as f64);
                    dest.bitmap.data[di as usize] = if (red_map).is_some() {
                        (red_map.as_ref().unwrap()[(r).clone() as usize].clone()) as u8
                    } else {
                        ((r).clone()) as u8
                    };
                    dest.bitmap.data[(di + 1.0_f64) as usize] = if (green_map).is_some() {
                        (green_map.as_ref().unwrap()[(g).clone() as usize].clone()) as u8
                    } else {
                        ((g).clone()) as u8
                    };
                    dest.bitmap.data[(di + 2.0_f64) as usize] = if (blue_map).is_some() {
                        (blue_map.as_ref().unwrap()[(b).clone() as usize].clone()) as u8
                    } else {
                        ((b).clone()) as u8
                    };
                    dest.bitmap.data[(di + 3.0_f64) as usize] = if (alpha_map).is_some() {
                        (alpha_map.as_ref().unwrap()[(a).clone() as usize].clone()) as u8
                    } else {
                        ((a).clone()) as u8
                    };
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
