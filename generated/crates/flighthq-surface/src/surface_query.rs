// @generated from upstream/packages/surface/src/surfaceQuery.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{RectangleLike, SurfaceRegion};

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

// Source: upstream/packages/surface/src/surfaceQuery.ts:13 (sha256:679d9a411d486997cf56d04ac44067395789bf36f2e1f90519ed2ab8e5d7d69e)
pub fn get_surface_color_bounds_rectangle(
    source: &SurfaceRegion,
    mask: f64,
    color: f64,
    find_color: Option<bool>,
) -> Option<RectangleLike> {
    let find_color = find_color.unwrap_or(true);
    let surface_width = source.surface.width;
    let masked_color = (__flight_js_to_i32(
        (__flight_js_to_u32(color) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(
        (__flight_js_to_u32(mask) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64,
    )) as f64;
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = (-1.0_f64);
    let mut max_y = (-1.0_f64);
    {
        let mut py = 0.0_f64;
        while (py < source.height) {
            let y = (source.y + py);
            if (y < 0.0_f64) || (y >= source.surface.height) {
                {
                    py += 1.0;
                    py
                };
                continue;
            }
            {
                let mut px = 0.0_f64;
                while (px < source.width) {
                    let x = (source.x + px);
                    if (x < 0.0_f64) || (x >= surface_width) {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let i = (((y * surface_width) + x) * 4.0_f64);
                    let pixel = (__flight_js_to_i32(
                        (__flight_js_to_u32(
                            (__flight_js_to_i32(
                                (__flight_js_to_i32(
                                    (__flight_js_to_i32(
                                        __flight_js_to_i32((source.surface.data[i as usize] as f64))
                                            .wrapping_shl((__flight_js_to_u32(24.0_f64) & 31))
                                            as f64,
                                    ) | __flight_js_to_i32(
                                        __flight_js_to_i32(
                                            (source.surface.data[(i + 1.0_f64) as usize] as f64),
                                        )
                                        .wrapping_shl((__flight_js_to_u32(16.0_f64) & 31))
                                            as f64,
                                    )) as f64,
                                ) | __flight_js_to_i32(
                                    __flight_js_to_i32(
                                        (source.surface.data[(i + 2.0_f64) as usize] as f64),
                                    )
                                    .wrapping_shl((__flight_js_to_u32(8.0_f64) & 31))
                                        as f64,
                                )) as f64,
                            ) | __flight_js_to_i32(
                                (source.surface.data[(i + 3.0_f64) as usize] as f64),
                            )) as f64,
                        ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64,
                    ) & __flight_js_to_i32(
                        (__flight_js_to_u32(mask) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64,
                    )) as f64;
                    let matches = (pixel == masked_color);
                    if (matches == find_color) {
                        if (x < min_x) {
                            min_x = x;
                        }
                        if (x > max_x) {
                            max_x = x;
                        }
                        if (y < min_y) {
                            min_y = y;
                        }
                        if (y > max_y) {
                            max_y = y;
                        }
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
    if (max_x == (-1.0_f64)) {
        return None;
    }
    return Some(RectangleLike {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        x: min_x,
        y: min_y,
        width: ((max_x - min_x) + 1.0_f64),
        height: ((max_y - min_y) + 1.0_f64),
    });
}
