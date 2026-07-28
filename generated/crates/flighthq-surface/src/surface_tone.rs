// @generated from upstream/packages/surface/src/surfaceTone.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::SurfaceRegion;

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

// Source: upstream/packages/surface/src/surfaceTone.ts:16 (sha256:c9fd7fa5739df108066ab482e39b09a389277dc1ed3cbb6098916fc45f3d86c2)
pub fn apply_surface_curve(
    out: &mut SurfaceRegion,
    source: &SurfaceRegion,
    red_lut: Option<Vec<u8>>,
    green_lut: Option<Vec<u8>>,
    blue_lut: Option<Vec<u8>>,
    alpha_lut: Option<Option<Vec<u8>>>,
) -> () {
    let alpha_lut = alpha_lut.unwrap_or(None);
    let w = (out.width).min(source.width);
    let h = (out.height).min(source.height);
    let od = &mut out.surface.data;
    let sd = &source.surface.data;
    {
        let mut py = 0.0_f64;
        while (py < h) {
            let sy = (source.y + py);
            let oy = (out.y + py);
            if ((((sy < 0.0_f64) || (sy >= source.surface.height)) || (oy < 0.0_f64))
                || (oy >= out.surface.height))
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
                    if ((((sx < 0.0_f64) || (sx >= source.surface.width)) || (ox < 0.0_f64))
                        || (ox >= out.surface.width))
                    {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let si = (((sy * source.surface.width) + sx) * 4.0_f64);
                    let oi = (((oy * out.surface.width) + ox) * 4.0_f64);
                    let r = (sd[si as usize] as f64);
                    let g = (sd[(si + 1.0_f64) as usize] as f64);
                    let b = (sd[(si + 2.0_f64) as usize] as f64);
                    let a = (sd[(si + 3.0_f64) as usize] as f64);
                    od[oi as usize] = if (red_lut).is_some() {
                        (red_lut.as_ref().unwrap()[r as usize].clone()) as u8
                    } else {
                        (r) as u8
                    };
                    od[(oi + 1.0_f64) as usize] = if (green_lut).is_some() {
                        (green_lut.as_ref().unwrap()[g as usize].clone()) as u8
                    } else {
                        (g) as u8
                    };
                    od[(oi + 2.0_f64) as usize] = if (blue_lut).is_some() {
                        (blue_lut.as_ref().unwrap()[b as usize].clone()) as u8
                    } else {
                        (b) as u8
                    };
                    od[(oi + 3.0_f64) as usize] = if (alpha_lut).is_some() {
                        (alpha_lut.as_ref().unwrap()[a as usize].clone()) as u8
                    } else {
                        (a) as u8
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
    {
        out.surface.version = (__flight_js_to_u32((out.surface.version + 1.0_f64))
            >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
    };
}

// Source: upstream/packages/surface/src/surfaceTone.ts:64 (sha256:806eefd00e9b21f02ab729771249cc2e6cc7d42b1cf3f1951185d7bfbf8c190e)
pub fn apply_surface_levels(
    out: &mut SurfaceRegion,
    source: &SurfaceRegion,
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
    let mut lut = vec![0_u8; (256.0_f64) as usize];
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
    apply_surface_curve(
        out,
        source,
        Some((lut).clone()),
        Some((lut).clone()),
        Some((lut).clone()),
        None,
    );
}
