// @generated from upstream/packages/collision/src/testCollision.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    clear_collision_manifold, test_aabb_aabb_collision, test_aabb_obb_collision,
    test_aabb_polygon_collision, test_circle_aabb_collision, test_circle_circle_collision,
    test_circle_obb_collision, test_circle_polygon_collision, test_obb_obb_collision,
    test_obb_polygon_collision, test_polygon_polygon_collision,
};
use flighthq_types::{CollisionManifold, CollisionShape, CollisionShapeKind};

// Source: upstream/packages/collision/src/testCollision.ts:24 (sha256:4294b8fc2c253569a6d4bfe35061f4a78bd931cde1b4dbd622b11fa14851d5e5)
pub fn test_collision(a: &CollisionShape, b: &CollisionShape, out: &mut CollisionManifold) -> bool {
    let rank_a = shape_kind_rank((a.kind).clone());
    let rank_b = shape_kind_rank((b.kind).clone());
    if (rank_a < 0.0_f64) || (rank_b < 0.0_f64) {
        clear_collision_manifold(out);
        return false;
    }
    let swapped = (rank_a > rank_b);
    let lo = if swapped { (*b).clone() } else { (*a).clone() };
    let hi = if swapped { (*a).clone() } else { (*b).clone() };
    let mut overlapping = false;
    {
        let __switch_value = (lo.kind).clone();
        let __flight_case = if __switch_value == "circle" {
            0_usize
        } else if __switch_value == "aabb" {
            1_usize
        } else if __switch_value == "obb" {
            2_usize
        } else if __switch_value == "polygon" {
            3_usize
        } else {
            4_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                {
                    let __switch_value = (hi.kind).clone();
                    let __flight_case = if __switch_value == "circle" {
                        0_usize
                    } else if __switch_value == "aabb" {
                        1_usize
                    } else if __switch_value == "obb" {
                        2_usize
                    } else if __switch_value == "polygon" {
                        3_usize
                    } else {
                        4_usize
                    };
                    '__flight_switch: {
                        if __flight_case <= 0_usize {
                            overlapping = test_circle_circle_collision(&lo, &hi, out);
                            break '__flight_switch;
                        }
                        if __flight_case <= 1_usize {
                            overlapping = test_circle_aabb_collision(&lo, &hi, out);
                            break '__flight_switch;
                        }
                        if __flight_case <= 2_usize {
                            overlapping = test_circle_obb_collision(&lo, &hi, out);
                            break '__flight_switch;
                        }
                        if __flight_case <= 3_usize {
                            overlapping = test_circle_polygon_collision(&lo, &hi, out);
                            break '__flight_switch;
                        }
                    }
                }
                break '__flight_switch;
            }
            if __flight_case <= 1_usize {
                {
                    let __switch_value = (hi.kind).clone();
                    let __flight_case = if __switch_value == "aabb" {
                        0_usize
                    } else if __switch_value == "obb" {
                        1_usize
                    } else if __switch_value == "polygon" {
                        2_usize
                    } else {
                        3_usize
                    };
                    '__flight_switch: {
                        if __flight_case <= 0_usize {
                            overlapping = test_aabb_aabb_collision(&lo, &hi, out);
                            break '__flight_switch;
                        }
                        if __flight_case <= 1_usize {
                            overlapping = test_aabb_obb_collision(&lo, &hi, out);
                            break '__flight_switch;
                        }
                        if __flight_case <= 2_usize {
                            overlapping = test_aabb_polygon_collision(&lo, &hi, out);
                            break '__flight_switch;
                        }
                    }
                }
                break '__flight_switch;
            }
            if __flight_case <= 2_usize {
                {
                    let __switch_value = (hi.kind).clone();
                    let __flight_case = if __switch_value == "obb" {
                        0_usize
                    } else if __switch_value == "polygon" {
                        1_usize
                    } else {
                        2_usize
                    };
                    '__flight_switch: {
                        if __flight_case <= 0_usize {
                            overlapping = test_obb_obb_collision(&lo, &hi, out);
                            break '__flight_switch;
                        }
                        if __flight_case <= 1_usize {
                            overlapping = test_obb_polygon_collision(&lo, &hi, out);
                            break '__flight_switch;
                        }
                    }
                }
                break '__flight_switch;
            }
            if __flight_case <= 3_usize {
                if ((hi.kind).clone() == "polygon") {
                    {
                        overlapping = test_polygon_polygon_collision(&lo, &hi, out);
                    }
                }
                break '__flight_switch;
            }
        }
    }
    if (overlapping) && (swapped) {
        out.normal_x = (-out.normal_x);
        out.normal_y = (-out.normal_y);
    }
    return overlapping;
}

// Source: upstream/packages/collision/src/testCollision.ts:98 (sha256:f45072651a6b1f4e4609214d9536dd588f12a6046661b6dd3448a63f376b578f)
fn shape_kind_rank(kind: CollisionShapeKind) -> f64 {
    {
        let __switch_value = kind;
        let __flight_case = if __switch_value == "circle" {
            0_usize
        } else if __switch_value == "aabb" {
            1_usize
        } else if __switch_value == "obb" {
            2_usize
        } else if __switch_value == "polygon" {
            3_usize
        } else {
            4_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                return 0.0_f64;
            }
            if __flight_case <= 1_usize {
                return 1.0_f64;
            }
            if __flight_case <= 2_usize {
                return 2.0_f64;
            }
            if __flight_case <= 3_usize {
                return 3.0_f64;
            }
            if __flight_case <= 4_usize {
                return (-1.0_f64);
            }
            unreachable!("exhaustive TypeScript switch completed without returning");
        }
    }
}
