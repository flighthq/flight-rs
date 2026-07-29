// @generated from upstream/packages/collision/src/pointContainment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::CollisionShape;

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

// Source: upstream/packages/collision/src/pointContainment.ts:3 (sha256:fb0f61a65a44ffb9d5adc1078fdedd619ec9fb49227fe74de5b381d73c8416ce)
const EPS: f64 = 1e-9_f64;

// Source: upstream/packages/collision/src/pointContainment.ts:9 (sha256:75ad3c80686c72bd3e0b1f116ae4507f991696c494665d75ebb1c62d37989500)
pub fn get_collision_shape_contains_point(shape: &CollisionShape, x: f64, y: f64) -> bool {
    {
        let __switch_value = (shape.kind).clone();
        let __flight_case = if __switch_value == "circle" {
            0_usize
        } else if __switch_value == "aabb" {
            1_usize
        } else if __switch_value == "obb" {
            2_usize
        } else if __switch_value == "polygon" {
            3_usize
        } else if __switch_value == "segment" {
            4_usize
        } else if __switch_value == "point" {
            5_usize
        } else {
            6_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                {
                    let dx = (x - shape.x);
                    let dy = (y - shape.y);
                    return (((dx * dx) + (dy * dy)) <= (shape.radius * shape.radius));
                }
            }
            if __flight_case <= 1_usize {
                return (((x >= shape.min_x) && (x <= shape.max_x)) && (y >= shape.min_y))
                    && (y <= shape.max_y);
            }
            if __flight_case <= 2_usize {
                {
                    let cos = (shape.rotation).cos();
                    let sin = (shape.rotation).sin();
                    let dx = (x - shape.x);
                    let dy = (y - shape.y);
                    let local_x = ((dx * cos) + (dy * sin));
                    let local_y = (((-dx) * sin) + (dy * cos));
                    return ((local_x).abs() <= shape.half_w) && ((local_y).abs() <= shape.half_h);
                }
            }
            if __flight_case <= 3_usize {
                return is_point_in_convex_polygon(
                    x,
                    y,
                    &shape.points,
                    (__flight_js_to_i32(shape.points.length) >> (__flight_js_to_u32(1.0_f64) & 31))
                        as f64,
                );
            }
            if __flight_case <= 4_usize {
                {
                    let dx = (shape.x1 - shape.x0);
                    let dy = (shape.y1 - shape.y0);
                    let length_squared = ((dx * dx) + (dy * dy));
                    let mut t = 0.0_f64;
                    if (length_squared > EPS) {
                        {
                            t = ((((x - shape.x0) * dx) + ((y - shape.y0) * dy)) / length_squared);
                            t = if (t < 0.0_f64) {
                                0.0_f64
                            } else {
                                if (t > 1.0_f64) { 1.0_f64 } else { t }
                            };
                        }
                    }
                    let closest_x = (shape.x0 + (t * dx));
                    let closest_y = (shape.y0 + (t * dy));
                    let ddx = (x - closest_x);
                    let ddy = (y - closest_y);
                    return (((ddx * ddx) + (ddy * ddy)) <= EPS);
                }
            }
            if __flight_case <= 5_usize {
                {
                    let dx = (x - shape.x);
                    let dy = (y - shape.y);
                    return (((dx * dx) + (dy * dy)) <= EPS);
                }
            }
            if __flight_case <= 6_usize {
                return false;
            }
            unreachable!("exhaustive TypeScript switch completed without returning");
        }
    }
}

// Source: upstream/packages/collision/src/pointContainment.ts:56 (sha256:e6a6d858daf8c9c544e3d0d08b23820fa5f3b139e2af87feda4f16f294872c2a)
fn is_point_in_convex_polygon(x: f64, y: f64, px: &Vec<f64>, pn: f64) -> bool {
    let mut positive = false;
    let mut negative = false;
    {
        let mut i = 0.0_f64;
        while (i < pn) {
            let j = ((i + 1.0_f64) % pn);
            let x0 = px[__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                as f64 as usize]
                .clone();
            let y0 = px[(__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                as f64
                + 1.0_f64) as usize]
                .clone();
            let x1 = px[__flight_js_to_i32(j).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                as f64 as usize]
                .clone();
            let y1 = px[(__flight_js_to_i32(j).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                as f64
                + 1.0_f64) as usize]
                .clone();
            let cross = (((x1 - x0) * (y - y0)) - ((y1 - y0) * (x - x0)));
            if (cross > EPS) {
                positive = true;
            } else {
                if (cross < (-EPS)) {
                    negative = true;
                }
            }
            if (positive) && (negative) {
                return false;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return true;
}
