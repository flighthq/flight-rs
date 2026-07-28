// @generated from upstream/packages/surface/src/surfaceCopy.ts; do not edit.
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

// Source: upstream/packages/surface/src/surfaceCopy.ts:48 (sha256:4c26105bad5520f18abe2f3f9d618560537a88cbca3f811a797d1043aeade269)
pub fn copy_surface_pixels(
    dest: &mut SurfaceRegion,
    source: &SurfaceRegion,
    composite: Option<bool>,
) -> () {
    let composite = composite.unwrap_or(false);
    let w = (dest.width).min(source.width);
    let h = (dest.height).min(source.height);
    let sd = &source.surface.data;
    let dd = &mut dest.surface.data;
    {
        let mut py = 0.0_f64;
        while (py < h) {
            let sy = (source.y + py);
            let dy = (dest.y + py);
            if ((((sy < 0.0_f64) || (sy >= source.surface.height)) || (dy < 0.0_f64))
                || (dy >= dest.surface.height))
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
                    if ((((sx < 0.0_f64) || (sx >= source.surface.width)) || (dx < 0.0_f64))
                        || (dx >= dest.surface.width))
                    {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let si = (((sy * source.surface.width) + sx) * 4.0_f64);
                    let di = (((dy * dest.surface.width) + dx) * 4.0_f64);
                    if composite {
                        let src_a = ((sd[(si + 3.0_f64) as usize] as f64) / 255.0_f64);
                        let dst_a = ((dd[(di + 3.0_f64) as usize] as f64) / 255.0_f64);
                        let out_a = (src_a + (dst_a * (1.0_f64 - src_a)));
                        if (out_a > 0.0_f64) {
                            dd[di as usize] = (((((sd[si as usize] as f64) * src_a)
                                + (((dd[di as usize] as f64) * dst_a) * (1.0_f64 - src_a)))
                                / out_a)
                                .round()) as u8;
                            dd[(di + 1.0_f64) as usize] =
                                (((((sd[(si + 1.0_f64) as usize] as f64) * src_a)
                                    + (((dd[(di + 1.0_f64) as usize] as f64) * dst_a)
                                        * (1.0_f64 - src_a)))
                                    / out_a)
                                    .round()) as u8;
                            dd[(di + 2.0_f64) as usize] =
                                (((((sd[(si + 2.0_f64) as usize] as f64) * src_a)
                                    + (((dd[(di + 2.0_f64) as usize] as f64) * dst_a)
                                        * (1.0_f64 - src_a)))
                                    / out_a)
                                    .round()) as u8;
                            dd[(di + 3.0_f64) as usize] = ((out_a * 255.0_f64).round()) as u8;
                        }
                    } else {
                        dd[di as usize] = (sd[si as usize] as f64) as u8;
                        dd[(di + 1.0_f64) as usize] = (sd[(si + 1.0_f64) as usize] as f64) as u8;
                        dd[(di + 2.0_f64) as usize] = (sd[(si + 2.0_f64) as usize] as f64) as u8;
                        dd[(di + 3.0_f64) as usize] = (sd[(si + 3.0_f64) as usize] as f64) as u8;
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
    {
        dest.surface.version = (__flight_js_to_u32((dest.surface.version + 1.0_f64))
            >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
    };
}
