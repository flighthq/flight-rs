// @generated from upstream/packages/surface/src/surfaceChannel.ts; do not edit.
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

// Source: upstream/packages/surface/src/surfaceChannel.ts:16 (sha256:475b8f72757c250f2baf4530bf8425343da9713fbe79a73b616c14805a924603)
pub fn merge_surface_channels(
    out: &mut SurfaceRegion,
    r: &SurfaceRegion,
    g: &SurfaceRegion,
    b: &SurfaceRegion,
    a: &SurfaceRegion,
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
            if ((((((((((oy < 0.0_f64) || (oy >= out.surface.height)) || (ry < 0.0_f64))
                || (ry >= r.surface.height))
                || (gy < 0.0_f64))
                || (gy >= g.surface.height))
                || (by < 0.0_f64))
                || (by >= b.surface.height))
                || (ay < 0.0_f64))
                || (ay >= a.surface.height))
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
                    if ((((((((((ox < 0.0_f64) || (ox >= out.surface.width))
                        || (rx < 0.0_f64))
                        || (rx >= r.surface.width))
                        || (gx < 0.0_f64))
                        || (gx >= g.surface.width))
                        || (bx < 0.0_f64))
                        || (bx >= b.surface.width))
                        || (ax < 0.0_f64))
                        || (ax >= a.surface.width))
                    {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let di = (((oy * out.surface.width) + ox) * 4.0_f64);
                    out.surface.data[di as usize] = (r.surface.data
                        [(((ry * r.surface.width) + rx) * 4.0_f64) as usize]
                        as f64) as u8;
                    out.surface.data[(di + 1.0_f64) as usize] = (g.surface.data
                        [((((gy * g.surface.width) + gx) * 4.0_f64) + 1.0_f64) as usize]
                        as f64)
                        as u8;
                    out.surface.data[(di + 2.0_f64) as usize] = (b.surface.data
                        [((((by * b.surface.width) + bx) * 4.0_f64) + 2.0_f64) as usize]
                        as f64)
                        as u8;
                    out.surface.data[(di + 3.0_f64) as usize] = (a.surface.data
                        [((((ay * a.surface.width) + ax) * 4.0_f64) + 3.0_f64) as usize]
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
    {
        out.surface.version = (__flight_js_to_u32((out.surface.version + 1.0_f64))
            >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
    };
}
