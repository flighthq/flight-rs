// @generated from upstream/packages/surface/src/surfaceAlpha.ts; do not edit.
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

// Source: upstream/packages/surface/src/surfaceAlpha.ts:12 (sha256:d89135018077bd9071ff926e6849d202a68b62c75b49ef2813e7e64a92acef31)
pub fn copy_surface_alpha(dest: &mut SurfaceRegion, source: &SurfaceRegion) -> () {
    let w = (dest.width).min(source.width);
    let h = (dest.height).min(source.height);
    {
        let mut py = 0.0_f64;
        while (py < h) {
            let sy = (source.y + py);
            let dy = (dest.y + py);
            if (((sy < 0.0_f64) || (sy >= source.surface.height)) || (dy < 0.0_f64))
                || (dy >= dest.surface.height)
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
                    if (((sx < 0.0_f64) || (sx >= source.surface.width)) || (dx < 0.0_f64))
                        || (dx >= dest.surface.width)
                    {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let si = (((sy * source.surface.width) + sx) * 4.0_f64);
                    let di = (((dy * dest.surface.width) + dx) * 4.0_f64);
                    let alpha = (source.surface.data[(si + 3.0_f64) as usize] as f64);
                    dest.surface.data[(di + 3.0_f64) as usize] = ((alpha).clone()) as u8;
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

// Source: upstream/packages/surface/src/surfaceAlpha.ts:42 (sha256:515e0a6eceeb3c99029794245b36c952250a7795d3d68e80f1be893d6776a268)
pub fn multiply_surface_alpha(out: &mut SurfaceRegion, factor: f64) -> () {
    let f = (0.0_f64).max((1.0_f64).min(factor));
    let surface_width = out.surface.width;
    {
        let mut py = 0.0_f64;
        while (py < out.height) {
            let y = (out.y + py);
            if (y < 0.0_f64) || (y >= out.surface.height) {
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
                    if (x < 0.0_f64) || (x >= surface_width) {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let i = ((((y * surface_width) + x) * 4.0_f64) + 3.0_f64);
                    out.surface.data[i as usize] =
                        (((out.surface.data[i as usize] as f64) * f).round()) as u8;
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

// Source: upstream/packages/surface/src/surfaceAlpha.ts:66 (sha256:b962bda9a91a273a5ea9f007a1ed45666a22f3e76dda088f7e8a8ac3459f382a)
pub fn set_surface_alpha(out: &mut SurfaceRegion, alpha: f64) -> () {
    let a = (0.0_f64).max((255.0_f64).min((alpha).round()));
    let surface_width = out.surface.width;
    {
        let mut py = 0.0_f64;
        while (py < out.height) {
            let y = (out.y + py);
            if (y < 0.0_f64) || (y >= out.surface.height) {
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
                    if (x < 0.0_f64) || (x >= surface_width) {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    out.surface.data[((((y * surface_width) + x) * 4.0_f64) + 3.0_f64) as usize] =
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
    {
        out.surface.version = (__flight_js_to_u32((out.surface.version + 1.0_f64))
            >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
    };
}
