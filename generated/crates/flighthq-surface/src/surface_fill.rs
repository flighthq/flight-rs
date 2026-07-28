// @generated from upstream/packages/surface/src/surfaceFill.ts; do not edit.
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

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

// Source: upstream/packages/surface/src/surfaceFill.ts:10 (sha256:bbca006e80f8dcc67d196de40b010155701786d0e9630c93eb852f487b871fe6)
pub fn fill_surface_rectangle(dest: &mut SurfaceRegion, color: f64) -> () {
    let r = (__flight_js_to_i32(
        (__flight_js_to_u32(color) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64;
    let g = (__flight_js_to_i32(
        (__flight_js_to_i32(color) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64;
    let b = (__flight_js_to_i32(
        (__flight_js_to_i32(color) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64;
    let a = (__flight_js_to_i32(color) & __flight_js_to_i32(255.0_f64)) as f64;
    {
        let mut py = 0.0_f64;
        while (py < dest.height) {
            let y = (dest.y + py);
            if (y < 0.0_f64) || (y >= dest.surface.height) {
                {
                    py += 1.0;
                    py
                };
                continue;
            }
            {
                let mut px = 0.0_f64;
                while (px < dest.width) {
                    let x = (dest.x + px);
                    if (x < 0.0_f64) || (x >= dest.surface.width) {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let i = (((y * dest.surface.width) + x) * 4.0_f64);
                    dest.surface.data[i as usize] = (r) as u8;
                    dest.surface.data[(i + 1.0_f64) as usize] = (g) as u8;
                    dest.surface.data[(i + 2.0_f64) as usize] = (b) as u8;
                    dest.surface.data[(i + 3.0_f64) as usize] = (a) as u8;
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
