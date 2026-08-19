// @generated from upstream/packages/bitmap/src/bitmapTone.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::invalidate_bitmap;
use flighthq_types::BitmapRegion;

// Source: upstream/packages/bitmap/src/bitmapTone.ts:17 (sha256:9a95dee64e765fdcfb5db4fbc6de5e674c64ca1a0e7ba2eef13942d0253d2f0a)
pub fn apply_bitmap_curve(
    out: &mut BitmapRegion,
    source: &BitmapRegion,
    red_lut: Option<Vec<u8>>,
    green_lut: Option<Vec<u8>>,
    blue_lut: Option<Vec<u8>>,
    alpha_lut: Option<Vec<u8>>,
) -> () {
    let w = (out.width).min(source.width);
    let h = (out.height).min(source.height);
    {
        let mut py = 0.0_f64;
        while (py < h) {
            let sy = (source.y + py);
            let oy = (out.y + py);
            if (((sy < 0.0_f64) || (sy >= source.bitmap.height)) || (oy < 0.0_f64))
                || (oy >= out.bitmap.height)
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
                    let ox = (out.x + px);
                    if (((sx < 0.0_f64) || (sx >= source.bitmap.width)) || (ox < 0.0_f64))
                        || (ox >= out.bitmap.width)
                    {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let si = (((sy * source.bitmap.width) + sx) * 4.0_f64);
                    let oi = (((oy * out.bitmap.width) + ox) * 4.0_f64);
                    let r = (source.bitmap.data[si as usize] as f64);
                    let g = (source.bitmap.data[(si + 1.0_f64) as usize] as f64);
                    let b = (source.bitmap.data[(si + 2.0_f64) as usize] as f64);
                    let a = (source.bitmap.data[(si + 3.0_f64) as usize] as f64);
                    out.bitmap.data[oi as usize] = if (red_lut).is_some() {
                        (red_lut.as_ref().unwrap()[(r).clone() as usize] as f64) as u8
                    } else {
                        ((r).clone()) as u8
                    };
                    out.bitmap.data[(oi + 1.0_f64) as usize] = if (green_lut).is_some() {
                        (green_lut.as_ref().unwrap()[(g).clone() as usize] as f64) as u8
                    } else {
                        ((g).clone()) as u8
                    };
                    out.bitmap.data[(oi + 2.0_f64) as usize] = if (blue_lut).is_some() {
                        (blue_lut.as_ref().unwrap()[(b).clone() as usize] as f64) as u8
                    } else {
                        ((b).clone()) as u8
                    };
                    out.bitmap.data[(oi + 3.0_f64) as usize] = if (alpha_lut).is_some() {
                        (alpha_lut.as_ref().unwrap()[(a).clone() as usize] as f64) as u8
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
    invalidate_bitmap(&mut out.bitmap);
}

// Source: upstream/packages/bitmap/src/bitmapTone.ts:65 (sha256:b9350085b7be06da974df3e9d8f232a4d406d906adfc210ebc73f1e7f9f14994)
pub fn apply_bitmap_levels(
    out: &mut BitmapRegion,
    source: &BitmapRegion,
    black_point: Option<f64>,
    white_point: Option<f64>,
    gamma: Option<f64>,
) -> () {
    let black_point = black_point.unwrap_or(0.0_f64);
    let white_point = white_point.unwrap_or(255.0_f64);
    let gamma = gamma.unwrap_or(1.0_f64);
    let bp = (0.0_f64).max((254.0_f64).min(black_point));
    let wp = (bp + 1.0_f64).max((255.0_f64).min(white_point));
    let span = (wp - bp);
    let inv_gamma = if (gamma > 0.0_f64) {
        (1.0_f64 / gamma)
    } else {
        1.0_f64
    };
    let mut lut: Vec<u8> = vec![0_u8; (256.0_f64) as usize];
    {
        let mut i = 0.0_f64;
        while (i < 256.0_f64) {
            let normalized = (0.0_f64).max((1.0_f64).min(((i - bp) / span)));
            lut[i as usize] = (((normalized).powf(inv_gamma) * 255.0_f64).round()) as u8;
            {
                i += 1.0;
                i
            };
        }
    }
    apply_bitmap_curve(
        out,
        source,
        Some((lut).clone()),
        Some((lut).clone()),
        Some((lut).clone()),
        None,
    );
}
