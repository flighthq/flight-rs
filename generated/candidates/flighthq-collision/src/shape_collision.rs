// @generated from upstream/packages/collision/src/shapeCollision.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::clear_collision_manifold;
use flighthq_geometry::{create_vector2, normalize_vector2};
use flighthq_types::{
    CollisionAabb, CollisionCircle, CollisionManifold, CollisionObb, CollisionPolygon, Vector2,
};

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

// Source: upstream/packages/collision/src/shapeCollision.ts:25 (sha256:fb0f61a65a44ffb9d5adc1078fdedd619ec9fb49227fe74de5b381d73c8416ce)
const EPS: f64 = 1e-9_f64;

// Source: upstream/packages/collision/src/shapeCollision.ts:29 (sha256:b546935fc128c50a318044f92bde98ae652fa52e13cfdfd74811e3cada1f94f1)
pub fn test_aabb_aabb_collision(
    a: &CollisionAabb,
    b: &CollisionAabb,
    out: &mut CollisionManifold,
) -> bool {
    let a_min_x = a.min_x;
    let a_min_y = a.min_y;
    let a_max_x = a.max_x;
    let a_max_y = a.max_y;
    let b_min_x = b.min_x;
    let b_min_y = b.min_y;
    let b_max_x = b.max_x;
    let b_max_y = b.max_y;
    let pen_left_x = (a_max_x - b_min_x);
    let pen_right_x = (b_max_x - a_min_x);
    let overlap_x = if (pen_left_x < pen_right_x) {
        pen_left_x
    } else {
        pen_right_x
    };
    let pen_down_y = (a_max_y - b_min_y);
    let pen_up_y = (b_max_y - a_min_y);
    let overlap_y = if (pen_down_y < pen_up_y) {
        pen_down_y
    } else {
        pen_up_y
    };
    if (overlap_x <= 0.0_f64) || (overlap_y <= 0.0_f64) {
        clear_collision_manifold(out);
        return false;
    }
    if (overlap_x < overlap_y) {
        out.normal_x = if (pen_left_x < pen_right_x) {
            (-1.0_f64)
        } else {
            1.0_f64
        };
        out.normal_y = 0.0_f64;
        out.depth = overlap_x;
    } else {
        out.normal_x = 0.0_f64;
        out.normal_y = if (pen_down_y < pen_up_y) {
            (-1.0_f64)
        } else {
            1.0_f64
        };
        out.depth = overlap_y;
    }
    out.overlapping = true;
    return true;
}

// Source: upstream/packages/collision/src/shapeCollision.ts:70 (sha256:cb3bd4c885a9b3cc21b0a286e6e0a137485f3761fb3a30fae23e03365ff0e1af)
pub fn test_aabb_obb_collision(
    a: &CollisionAabb,
    b: &CollisionObb,
    out: &mut CollisionManifold,
) -> bool {
    write_aabb_vertices(a, &mut (*SCRATCH_A.lock().unwrap()));
    write_obb_vertices(b, &mut (*SCRATCH_B.lock().unwrap()));
    return sat_convex_overlap(
        &(*SCRATCH_A.lock().unwrap()),
        4.0_f64,
        &(*SCRATCH_B.lock().unwrap()),
        4.0_f64,
        out,
    );
}

// Source: upstream/packages/collision/src/shapeCollision.ts:81 (sha256:f954ec165682354efabe8fd55981d9df7f37827eb7591a2a8e0d49c1ca90f5cc)
pub fn test_aabb_polygon_collision(
    a: &CollisionAabb,
    b: &CollisionPolygon,
    out: &mut CollisionManifold,
) -> bool {
    write_aabb_vertices(a, &mut (*SCRATCH_A.lock().unwrap()));
    return sat_convex_overlap(
        &(*SCRATCH_A.lock().unwrap()),
        4.0_f64,
        &b.points,
        (__flight_js_to_i32((b.points.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64,
        out,
    );
}

// Source: upstream/packages/collision/src/shapeCollision.ts:93 (sha256:a73dea5b0ef54bc4e8c964a808ec5e7fc2e7fe119774c943191530a0cf0c771e)
pub fn test_circle_aabb_collision(
    a: &CollisionCircle,
    b: &CollisionAabb,
    out: &mut CollisionManifold,
) -> bool {
    return circle_aabb_overlap(a.x, a.y, a.radius, b.min_x, b.min_y, b.max_x, b.max_y, out);
}

// Source: upstream/packages/collision/src/shapeCollision.ts:103 (sha256:8034446e12e6f47755211a00f4d7838274c24c144eefa9f6da329b436f50a88e)
pub fn test_circle_circle_collision(
    a: &CollisionCircle,
    b: &CollisionCircle,
    out: &mut CollisionManifold,
) -> bool {
    let ax = a.x;
    let ay = a.y;
    let bx = b.x;
    let by = b.y;
    let radius_sum = (a.radius + b.radius);
    let dx = (ax - bx);
    let dy = (ay - by);
    let dist_squared = ((dx * dx) + (dy * dy));
    if (dist_squared >= (radius_sum * radius_sum)) {
        clear_collision_manifold(out);
        return false;
    }
    let dist = (dist_squared).sqrt();
    if (dist > EPS) {
        let inv = (1.0_f64 / dist);
        out.normal_x = (dx * inv);
        out.normal_y = (dy * inv);
        out.depth = (radius_sum - dist);
    } else {
        out.normal_x = 1.0_f64;
        out.normal_y = 0.0_f64;
        out.depth = radius_sum;
    }
    out.overlapping = true;
    return true;
}

// Source: upstream/packages/collision/src/shapeCollision.ts:139 (sha256:32657dce63f03c42ff970ec3e83383c3d594a646547eea067acf62f47908932a)
pub fn test_circle_obb_collision(
    a: &CollisionCircle,
    b: &CollisionObb,
    out: &mut CollisionManifold,
) -> bool {
    let cx = a.x;
    let cy = a.y;
    let radius = a.radius;
    let half_w = b.half_w;
    let half_h = b.half_h;
    let cos = (b.rotation).cos();
    let sin = (b.rotation).sin();
    let dx = (cx - b.x);
    let dy = (cy - b.y);
    let local_x = ((dx * cos) + (dy * sin));
    let local_y = (((-dx) * sin) + (dy * cos));
    if (!circle_aabb_overlap(
        local_x,
        local_y,
        radius,
        (-half_w),
        (-half_h),
        half_w,
        half_h,
        out,
    )) {
        return false;
    }
    let local_normal_x = out.normal_x;
    let local_normal_y = out.normal_y;
    out.normal_x = ((local_normal_x * cos) - (local_normal_y * sin));
    out.normal_y = ((local_normal_x * sin) + (local_normal_y * cos));
    return true;
}

// Source: upstream/packages/collision/src/shapeCollision.ts:169 (sha256:a50de772d042d623aa969712d70c624f5f350ec12c21c5947e96ebe195cdbadf)
pub fn test_circle_polygon_collision(
    a: &CollisionCircle,
    b: &CollisionPolygon,
    out: &mut CollisionManifold,
) -> bool {
    return sat_circle_convex_overlap(
        a.x,
        a.y,
        a.radius,
        &b.points,
        (__flight_js_to_i32((b.points.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64,
        out,
    );
}

// Source: upstream/packages/collision/src/shapeCollision.ts:179 (sha256:cea9601da072de1f9328491c0928d97dc502b60e086cc206936d1f90f2c521ae)
pub fn test_obb_obb_collision(
    a: &CollisionObb,
    b: &CollisionObb,
    out: &mut CollisionManifold,
) -> bool {
    write_obb_vertices(a, &mut (*SCRATCH_A.lock().unwrap()));
    write_obb_vertices(b, &mut (*SCRATCH_B.lock().unwrap()));
    return sat_convex_overlap(
        &(*SCRATCH_A.lock().unwrap()),
        4.0_f64,
        &(*SCRATCH_B.lock().unwrap()),
        4.0_f64,
        out,
    );
}

// Source: upstream/packages/collision/src/shapeCollision.ts:190 (sha256:63ef9eab43eb5510689ed657345e4fe55d992154b4e25bde51f07491e3f5b9e3)
pub fn test_obb_polygon_collision(
    a: &CollisionObb,
    b: &CollisionPolygon,
    out: &mut CollisionManifold,
) -> bool {
    write_obb_vertices(a, &mut (*SCRATCH_A.lock().unwrap()));
    return sat_convex_overlap(
        &(*SCRATCH_A.lock().unwrap()),
        4.0_f64,
        &b.points,
        (__flight_js_to_i32((b.points.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64,
        out,
    );
}

// Source: upstream/packages/collision/src/shapeCollision.ts:201 (sha256:e881d87a25b0b44bb2325c9547e09e4d2b9bba229cd3b40ecbbba0fffa772012)
pub fn test_polygon_polygon_collision(
    a: &CollisionPolygon,
    b: &CollisionPolygon,
    out: &mut CollisionManifold,
) -> bool {
    return sat_convex_overlap(
        &a.points,
        (__flight_js_to_i32((a.points.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64,
        &b.points,
        (__flight_js_to_i32((b.points.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64,
        out,
    );
}

// Source: upstream/packages/collision/src/shapeCollision.ts:213 (sha256:a270acffa39a6ab362fe7ca8f8baa2c714355023575628eeefc8e4b031be77c2)
fn circle_aabb_overlap(
    cx: f64,
    cy: f64,
    radius: f64,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
    out: &mut CollisionManifold,
) -> bool {
    let closest_x = if (cx < min_x) {
        min_x
    } else {
        if (cx > max_x) { max_x } else { cx }
    };
    let closest_y = if (cy < min_y) {
        min_y
    } else {
        if (cy > max_y) { max_y } else { cy }
    };
    let dx = (cx - closest_x);
    let dy = (cy - closest_y);
    let dist_squared = ((dx * dx) + (dy * dy));
    if (dist_squared > (EPS * EPS)) {
        let dist = (dist_squared).sqrt();
        if (dist >= radius) {
            clear_collision_manifold(out);
            return false;
        }
        let inv = (1.0_f64 / dist);
        out.normal_x = (dx * inv);
        out.normal_y = (dy * inv);
        out.depth = (radius - dist);
        out.overlapping = true;
        return true;
    }
    let mut left = (cx - min_x);
    let right = (max_x - cx);
    let bottom = (cy - min_y);
    let top = (max_y - cy);
    let mut min = left;
    let mut normal_x = (-1.0_f64);
    let mut normal_y = 0.0_f64;
    if (right < min) {
        min = right;
        normal_x = 1.0_f64;
        normal_y = 0.0_f64;
    }
    if (bottom < min) {
        min = bottom;
        normal_x = 0.0_f64;
        normal_y = (-1.0_f64);
    }
    if (top < min) {
        min = top;
        normal_x = 0.0_f64;
        normal_y = 1.0_f64;
    }
    out.normal_x = normal_x;
    out.normal_y = normal_y;
    out.depth = (min + radius);
    out.overlapping = true;
    return true;
}

// Source: upstream/packages/collision/src/shapeCollision.ts:277 (sha256:9869ffe09e2f32406c997a9f209179e1dbeb1af0c2153f32587d6708d4028192)
fn sat_circle_convex_overlap(
    cx: f64,
    cy: f64,
    radius: f64,
    px: &Vec<f64>,
    pn: f64,
    out: &mut CollisionManifold,
) -> bool {
    let mut min_overlap = f64::INFINITY;
    let mut normal_x = 0.0_f64;
    let mut normal_y = 0.0_f64;
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
            (*SCRATCH_AXIS.lock().unwrap()).x = (y1 - y0);
            (*SCRATCH_AXIS.lock().unwrap()).y = (-(x1 - x0));
            let len = {
                let __flight_argument_1 = (*SCRATCH_AXIS.lock().unwrap()).clone();
                normalize_vector2(&mut (*SCRATCH_AXIS.lock().unwrap()), &__flight_argument_1)
            };
            if (len < EPS) {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let axis_x = (*SCRATCH_AXIS.lock().unwrap()).x;
            let axis_y = (*SCRATCH_AXIS.lock().unwrap()).y;
            let overlap = circle_polygon_axis_overlap(axis_x, axis_y, cx, cy, radius, px, pn);
            if (overlap <= 0.0_f64) {
                clear_collision_manifold(out);
                return false;
            }
            if (overlap < min_overlap) {
                min_overlap = overlap;
                normal_x = axis_x;
                normal_y = axis_y;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let mut nearest_x = 0.0_f64;
    let mut nearest_y = 0.0_f64;
    let mut nearest_dist_squared = f64::INFINITY;
    {
        let mut i = 0.0_f64;
        while (i < pn) {
            let vx = px[__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                as f64 as usize]
                .clone();
            let vy = px[(__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                as f64
                + 1.0_f64) as usize]
                .clone();
            let ddx = (cx - vx);
            let ddy = (cy - vy);
            let d2 = ((ddx * ddx) + (ddy * ddy));
            if (d2 < nearest_dist_squared) {
                nearest_dist_squared = d2;
                nearest_x = vx;
                nearest_y = vy;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    (*SCRATCH_AXIS.lock().unwrap()).x = (cx - nearest_x);
    (*SCRATCH_AXIS.lock().unwrap()).y = (cy - nearest_y);
    let vertex_axis_len = {
        let __flight_argument_1 = (*SCRATCH_AXIS.lock().unwrap()).clone();
        normalize_vector2(&mut (*SCRATCH_AXIS.lock().unwrap()), &__flight_argument_1)
    };
    if (vertex_axis_len > EPS) {
        let axis_x = (*SCRATCH_AXIS.lock().unwrap()).x;
        let axis_y = (*SCRATCH_AXIS.lock().unwrap()).y;
        let overlap = circle_polygon_axis_overlap(axis_x, axis_y, cx, cy, radius, px, pn);
        if (overlap <= 0.0_f64) {
            clear_collision_manifold(out);
            return false;
        }
        if (overlap < min_overlap) {
            min_overlap = overlap;
            normal_x = axis_x;
            normal_y = axis_y;
        }
    }
    if (min_overlap == f64::INFINITY) {
        clear_collision_manifold(out);
        return false;
    }
    let mut centroid_x = 0.0_f64;
    let mut centroid_y = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < pn) {
            centroid_x += px[__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                as f64 as usize]
                .clone();
            centroid_y +=
                px[(__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31)) as f64
                    + 1.0_f64) as usize]
                    .clone();
            {
                i += 1.0;
                i
            };
        }
    }
    centroid_x /= pn;
    centroid_y /= pn;
    if (((normal_x * (cx - centroid_x)) + (normal_y * (cy - centroid_y))) < 0.0_f64) {
        normal_x = (-normal_x);
        normal_y = (-normal_y);
    }
    out.normal_x = normal_x;
    out.normal_y = normal_y;
    out.depth = min_overlap;
    out.overlapping = true;
    return true;
}

// Source: upstream/packages/collision/src/shapeCollision.ts:374 (sha256:b3410add973220e3242f11bb9f514c30d22c75836e84f1d89b7bc00bdaa29f3e)
fn circle_polygon_axis_overlap(
    axis_x: f64,
    axis_y: f64,
    cx: f64,
    cy: f64,
    radius: f64,
    px: &Vec<f64>,
    pn: f64,
) -> f64 {
    let mut min_p = f64::INFINITY;
    let mut max_p = (-f64::INFINITY);
    {
        let mut i = 0.0_f64;
        while (i < pn) {
            let d = ((px[__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                as f64 as usize]
                .clone()
                * axis_x)
                + (px[(__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                    as f64
                    + 1.0_f64) as usize]
                    .clone()
                    * axis_y));
            if (d < min_p) {
                min_p = d;
            }
            if (d > max_p) {
                max_p = d;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let c = ((cx * axis_x) + (cy * axis_y));
    let c_min = (c - radius);
    let c_max = (c + radius);
    let pen_low = (max_p - c_min);
    let pen_high = (c_max - min_p);
    return if (pen_low < pen_high) {
        pen_low
    } else {
        pen_high
    };
}

// Source: upstream/packages/collision/src/shapeCollision.ts:401 (sha256:4c0a85875885bd88d1adf42c050d9a3057da8ee9638acf6be2fbd04718da386b)
fn sat_convex_overlap(
    ax: &Vec<f64>,
    an: f64,
    bx: &Vec<f64>,
    bn: f64,
    out: &mut CollisionManifold,
) -> bool {
    (*MIN_OVERLAP_AXIS.lock().unwrap()).overlap = f64::INFINITY;
    (*MIN_OVERLAP_AXIS.lock().unwrap()).x = 0.0_f64;
    (*MIN_OVERLAP_AXIS.lock().unwrap()).y = 0.0_f64;
    if (!accumulate_polygon_axes(ax, an, ax, an, bx, bn, out)) {
        return false;
    }
    if (!accumulate_polygon_axes(bx, bn, ax, an, bx, bn, out)) {
        return false;
    }
    if ((*MIN_OVERLAP_AXIS.lock().unwrap()).overlap == f64::INFINITY) {
        clear_collision_manifold(out);
        return false;
    }
    let mut a_centroid_x = 0.0_f64;
    let mut a_centroid_y = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < an) {
            a_centroid_x += ax[__flight_js_to_i32(i)
                .wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                as f64 as usize]
                .clone();
            a_centroid_y +=
                ax[(__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31)) as f64
                    + 1.0_f64) as usize]
                    .clone();
            {
                i += 1.0;
                i
            };
        }
    }
    a_centroid_x /= an;
    a_centroid_y /= an;
    let mut b_centroid_x = 0.0_f64;
    let mut b_centroid_y = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < bn) {
            b_centroid_x += bx[__flight_js_to_i32(i)
                .wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                as f64 as usize]
                .clone();
            b_centroid_y +=
                bx[(__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31)) as f64
                    + 1.0_f64) as usize]
                    .clone();
            {
                i += 1.0;
                i
            };
        }
    }
    b_centroid_x /= bn;
    b_centroid_y /= bn;
    let mut normal_x = (*MIN_OVERLAP_AXIS.lock().unwrap()).x;
    let mut normal_y = (*MIN_OVERLAP_AXIS.lock().unwrap()).y;
    if (((normal_x * (a_centroid_x - b_centroid_x)) + (normal_y * (a_centroid_y - b_centroid_y)))
        < 0.0_f64)
    {
        normal_x = (-normal_x);
        normal_y = (-normal_y);
    }
    out.normal_x = normal_x;
    out.normal_y = normal_y;
    out.depth = (*MIN_OVERLAP_AXIS.lock().unwrap()).overlap;
    out.overlapping = true;
    return true;
}

// Source: upstream/packages/collision/src/shapeCollision.ts:451 (sha256:dc4da3e6b68b4dd8915789dd294085d062b5e563b3c1f5e8ab5a39a8e48b4905)
fn accumulate_polygon_axes(
    sx: &Vec<f64>,
    sn: f64,
    ax: &Vec<f64>,
    an: f64,
    bx: &Vec<f64>,
    bn: f64,
    out: &mut CollisionManifold,
) -> bool {
    {
        let mut i = 0.0_f64;
        while (i < sn) {
            let j = ((i + 1.0_f64) % sn);
            let x0 = sx[__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                as f64 as usize]
                .clone();
            let y0 = sx[(__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                as f64
                + 1.0_f64) as usize]
                .clone();
            let x1 = sx[__flight_js_to_i32(j).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                as f64 as usize]
                .clone();
            let y1 = sx[(__flight_js_to_i32(j).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                as f64
                + 1.0_f64) as usize]
                .clone();
            (*SCRATCH_AXIS.lock().unwrap()).x = (y1 - y0);
            (*SCRATCH_AXIS.lock().unwrap()).y = (-(x1 - x0));
            let len = {
                let __flight_argument_1 = (*SCRATCH_AXIS.lock().unwrap()).clone();
                normalize_vector2(&mut (*SCRATCH_AXIS.lock().unwrap()), &__flight_argument_1)
            };
            if (len < EPS) {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let axis_x = (*SCRATCH_AXIS.lock().unwrap()).x;
            let axis_y = (*SCRATCH_AXIS.lock().unwrap()).y;
            let overlap = polygon_axis_overlap(axis_x, axis_y, ax, an, bx, bn);
            if (overlap <= 0.0_f64) {
                clear_collision_manifold(out);
                return false;
            }
            if (overlap < (*MIN_OVERLAP_AXIS.lock().unwrap()).overlap) {
                (*MIN_OVERLAP_AXIS.lock().unwrap()).overlap = overlap;
                (*MIN_OVERLAP_AXIS.lock().unwrap()).x = axis_x;
                (*MIN_OVERLAP_AXIS.lock().unwrap()).y = axis_y;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return true;
}

// Source: upstream/packages/collision/src/shapeCollision.ts:488 (sha256:b1f2556dfb69390219a655884627518ad38c0a956e33837d83d7ff2948413b79)
fn polygon_axis_overlap(
    axis_x: f64,
    axis_y: f64,
    ax: &Vec<f64>,
    an: f64,
    bx: &Vec<f64>,
    bn: f64,
) -> f64 {
    let mut min_a = f64::INFINITY;
    let mut max_a = (-f64::INFINITY);
    {
        let mut i = 0.0_f64;
        while (i < an) {
            let d = ((ax[__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                as f64 as usize]
                .clone()
                * axis_x)
                + (ax[(__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                    as f64
                    + 1.0_f64) as usize]
                    .clone()
                    * axis_y));
            if (d < min_a) {
                min_a = d;
            }
            if (d > max_a) {
                max_a = d;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let mut min_b = f64::INFINITY;
    let mut max_b = (-f64::INFINITY);
    {
        let mut i = 0.0_f64;
        while (i < bn) {
            let d = ((bx[__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                as f64 as usize]
                .clone()
                * axis_x)
                + (bx[(__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                    as f64
                    + 1.0_f64) as usize]
                    .clone()
                    * axis_y));
            if (d < min_b) {
                min_b = d;
            }
            if (d > max_b) {
                max_b = d;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let pen_low = (max_a - min_b);
    let pen_high = (max_b - min_a);
    return if (pen_low < pen_high) {
        pen_low
    } else {
        pen_high
    };
}

// Source: upstream/packages/collision/src/shapeCollision.ts:517 (sha256:0bb2de478510a6c552b5f7d181f418457ccfaf2e9781a064c9770f52ab43f59b)
fn write_aabb_vertices(aabb: &CollisionAabb, out: &mut Vec<f64>) -> () {
    let min_x = aabb.min_x;
    let min_y = aabb.min_y;
    let max_x = aabb.max_x;
    let max_y = aabb.max_y;
    out[0.0_f64 as usize] = min_x;
    out[1.0_f64 as usize] = min_y;
    out[2.0_f64 as usize] = max_x;
    out[3.0_f64 as usize] = min_y;
    out[4.0_f64 as usize] = max_x;
    out[5.0_f64 as usize] = max_y;
    out[6.0_f64 as usize] = min_x;
    out[7.0_f64 as usize] = max_y;
}

// Source: upstream/packages/collision/src/shapeCollision.ts:533 (sha256:274b74a497ddc05526ee2a228089ead09f8d95c2ccdc5e2653c9b69da1538f7e)
fn write_obb_vertices(obb: &CollisionObb, out: &mut Vec<f64>) -> () {
    let cx = obb.x;
    let cy = obb.y;
    let half_w = obb.half_w;
    let half_h = obb.half_h;
    let cos = (obb.rotation).cos();
    let sin = (obb.rotation).sin();
    let wx = (cos * half_w);
    let wy = (sin * half_w);
    let hx = ((-sin) * half_h);
    let hy = (cos * half_h);
    out[0.0_f64 as usize] = ((cx - wx) - hx);
    out[1.0_f64 as usize] = ((cy - wy) - hy);
    out[2.0_f64 as usize] = ((cx + wx) - hx);
    out[3.0_f64 as usize] = ((cy + wy) - hy);
    out[4.0_f64 as usize] = ((cx + wx) + hx);
    out[5.0_f64 as usize] = ((cy + wy) + hy);
    out[6.0_f64 as usize] = ((cx - wx) + hx);
    out[7.0_f64 as usize] = ((cy - wy) + hy);
}

// Source: upstream/packages/collision/src/shapeCollision.ts:554 (sha256:1c652eed12d1c3c894459f6f22172f728f426121c56b1e7f29e6e7a62df6c792)
static SCRATCH_A: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64; (8.0_f64) as usize]));

// Source: upstream/packages/collision/src/shapeCollision.ts:555 (sha256:83efc3e7a9e94361f2e5bf8e519480bd4aad4b001451684929db60cf6380f710)
static SCRATCH_B: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64; (8.0_f64) as usize]));

// Source: upstream/packages/collision/src/shapeCollision.ts:556 (sha256:836ad851898a4c861e9af66587ae5a5bd0dcc75661a89a1393efb2e9ff8b219c)
static SCRATCH_AXIS: std::sync::LazyLock<std::sync::Mutex<Vector2>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector2(None, None)));

// Source: upstream/packages/collision/src/shapeCollision.ts:557 (sha256:1fe583a0e9e72e66a8c8a36053e08fd4f89e7f1b301587ad364f7fdc4a62e4dc)
#[derive(Clone, Default)]
struct MinOverlapAxis {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub overlap: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for MinOverlapAxis {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

static MIN_OVERLAP_AXIS: std::sync::LazyLock<std::sync::Mutex<MinOverlapAxis>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(MinOverlapAxis {
            __flight_identity: std::sync::Arc::new(()),
            overlap: f64::INFINITY,
            x: 0.0_f64,
            y: 0.0_f64,
        })
    });
