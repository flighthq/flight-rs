// @generated from upstream/packages/collision/src/segmentCollision.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    CollisionAabb, CollisionCircle, CollisionObb, CollisionPolygon, CollisionSegment,
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

// Source: upstream/packages/collision/src/segmentCollision.ts:7 (sha256:fb0f61a65a44ffb9d5adc1078fdedd619ec9fb49227fe74de5b381d73c8416ce)
const EPS: f64 = 1e-9_f64;

// Source: upstream/packages/collision/src/segmentCollision.ts:10 (sha256:539bb7e0ce8e343503996b123668b723be9a6d89cae643d6c63545b1e94b04ae)
pub fn test_segment_aabb_collision(a: &CollisionSegment, b: &CollisionAabb) -> bool {
    return is_segment_overlapping_box(a.x0, a.y0, a.x1, a.y1, b.min_x, b.min_y, b.max_x, b.max_y);
}

// Source: upstream/packages/collision/src/segmentCollision.ts:15 (sha256:946159e7e1ee79bfe940e5d8e4bbd3826f2842d7b68b21827d795677ce18225e)
pub fn test_segment_circle_collision(a: &CollisionSegment, b: &CollisionCircle) -> bool {
    let x0 = a.x0;
    let y0 = a.y0;
    let dx = (a.x1 - x0);
    let dy = (a.y1 - y0);
    let length_squared = ((dx * dx) + (dy * dy));
    let mut t = 0.0_f64;
    if (length_squared > EPS) {
        t = ((((b.x - x0) * dx) + ((b.y - y0) * dy)) / length_squared);
        t = if (t < 0.0_f64) {
            0.0_f64
        } else {
            if (t > 1.0_f64) { 1.0_f64 } else { t }
        };
    }
    let closest_x = (x0 + (t * dx));
    let closest_y = (y0 + (t * dy));
    let ddx = (b.x - closest_x);
    let ddy = (b.y - closest_y);
    return (((ddx * ddx) + (ddy * ddy)) <= (b.radius * b.radius));
}

// Source: upstream/packages/collision/src/segmentCollision.ts:35 (sha256:9f8a4e2bbb15d9ccda663e3633596486cb6b28ec66a73608b8354b3e9f7066b4)
pub fn test_segment_obb_collision(a: &CollisionSegment, b: &CollisionObb) -> bool {
    let cos = (b.rotation).cos();
    let sin = (b.rotation).sin();
    let d0x = (a.x0 - b.x);
    let d0y = (a.y0 - b.y);
    let d1x = (a.x1 - b.x);
    let d1y = (a.y1 - b.y);
    let local_x0 = ((d0x * cos) + (d0y * sin));
    let local_y0 = (((-d0x) * sin) + (d0y * cos));
    let local_x1 = ((d1x * cos) + (d1y * sin));
    let local_y1 = (((-d1x) * sin) + (d1y * cos));
    return is_segment_overlapping_box(
        local_x0,
        local_y0,
        local_x1,
        local_y1,
        (-b.half_w),
        (-b.half_h),
        b.half_w,
        b.half_h,
    );
}

// Source: upstream/packages/collision/src/segmentCollision.ts:51 (sha256:02980a65b01c0feb20603ff3f36480081603b4bb8532a4e69635bcc767eaef97)
pub fn test_segment_polygon_collision(a: &CollisionSegment, b: &CollisionPolygon) -> bool {
    let pn =
        (__flight_js_to_i32((b.points.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
    if is_point_in_convex_polygon(a.x0, a.y0, &b.points, pn) {
        return true;
    }
    if is_point_in_convex_polygon(a.x1, a.y1, &b.points, pn) {
        return true;
    }
    {
        let mut i = 0.0_f64;
        while (i < pn) {
            let j = ((i + 1.0_f64) % pn);
            if is_segments_intersecting(
                a.x0,
                a.y0,
                a.x1,
                a.y1,
                b.points[__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                    as f64 as usize]
                    .clone(),
                b.points[(__flight_js_to_i32(i).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                    as f64
                    + 1.0_f64) as usize]
                    .clone(),
                b.points[__flight_js_to_i32(j).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                    as f64 as usize]
                    .clone(),
                b.points[(__flight_js_to_i32(j).wrapping_shl((__flight_js_to_u32(1.0_f64) & 31))
                    as f64
                    + 1.0_f64) as usize]
                    .clone(),
            ) {
                return true;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return false;
}

// Source: upstream/packages/collision/src/segmentCollision.ts:77 (sha256:ba7699831d72a1f798dd29f0448f632d5bd7fd0c998d9c2bd84cc34b204a6e3f)
pub fn test_segment_segment_collision(a: &CollisionSegment, b: &CollisionSegment) -> bool {
    return is_segments_intersecting(a.x0, a.y0, a.x1, a.y1, b.x0, b.y0, b.x1, b.y1);
}

// Source: upstream/packages/collision/src/segmentCollision.ts:83 (sha256:e6a6d858daf8c9c544e3d0d08b23820fa5f3b139e2af87feda4f16f294872c2a)
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

// Source: upstream/packages/collision/src/segmentCollision.ts:102 (sha256:a8dbb252e72246411a5ecb3e9831a5c3e284be62d68d7e84fa62501bbdd3ebee)
fn is_segments_intersecting(
    ax0: f64,
    ay0: f64,
    ax1: f64,
    ay1: f64,
    bx0: f64,
    by0: f64,
    bx1: f64,
    by1: f64,
) -> bool {
    let d1x = (ax1 - ax0);
    let d1y = (ay1 - ay0);
    let d2x = (bx1 - bx0);
    let d2y = (by1 - by0);
    let denom = ((d1x * d2y) - (d1y * d2x));
    let ex = (bx0 - ax0);
    let ey = (by0 - ay0);
    if ((denom).abs() < EPS) {
        if (((ex * d1y) - (ey * d1x)).abs() > EPS) {
            return false;
        }
        let length_squared = ((d1x * d1x) + (d1y * d1y));
        if (length_squared < EPS) {
            let b_length_squared = ((d2x * d2x) + (d2y * d2y));
            if (b_length_squared < EPS) {
                return ((ex).abs() < EPS) && ((ey).abs() < EPS);
            }
            let mut tb = ((((ax0 - bx0) * d2x) + ((ay0 - by0) * d2y)) / b_length_squared);
            tb = if (tb < 0.0_f64) {
                0.0_f64
            } else {
                if (tb > 1.0_f64) { 1.0_f64 } else { tb }
            };
            let qx = (bx0 + (tb * d2x));
            let qy = (by0 + (tb * d2y));
            return ((((ax0 - qx) * (ax0 - qx)) + ((ay0 - qy) * (ay0 - qy))) < EPS);
        }
        let t0 = (((ex * d1x) + (ey * d1y)) / length_squared);
        let t1 = ((((bx1 - ax0) * d1x) + ((by1 - ay0) * d1y)) / length_squared);
        let lo = if (t0 < t1) { t0 } else { t1 };
        let hi = if (t0 < t1) { t1 } else { t0 };
        return (hi >= (-EPS)) && (lo <= (1.0_f64 + EPS));
    }
    let t = (((ex * d2y) - (ey * d2x)) / denom);
    let u = (((ex * d1y) - (ey * d1x)) / denom);
    return (((t >= (-EPS)) && (t <= (1.0_f64 + EPS))) && (u >= (-EPS))) && (u <= (1.0_f64 + EPS));
}

// Source: upstream/packages/collision/src/segmentCollision.ts:150 (sha256:f72846bfbff0dafacc583e6464f1f99f01f15abcbe791b88a2fb32481de12b0c)
fn is_segment_overlapping_box(
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) -> bool {
    let dx = (x1 - x0);
    let dy = (y1 - y0);
    (*CLIP_RANGE.lock().unwrap()).t0 = 0.0_f64;
    (*CLIP_RANGE.lock().unwrap()).t1 = 1.0_f64;
    if (!clip_segment_slab((-dx), (x0 - min_x))) {
        return false;
    }
    if (!clip_segment_slab(dx, (max_x - x0))) {
        return false;
    }
    if (!clip_segment_slab((-dy), (y0 - min_y))) {
        return false;
    }
    if (!clip_segment_slab(dy, (max_y - y0))) {
        return false;
    }
    return ((*CLIP_RANGE.lock().unwrap()).t0 <= (*CLIP_RANGE.lock().unwrap()).t1);
}

// Source: upstream/packages/collision/src/segmentCollision.ts:175 (sha256:9eba330e1aaa3b760360a6bc86b96e67be5e33671b0668f9d12b399226e81a28)
fn clip_segment_slab(p: f64, q: f64) -> bool {
    if ((p).abs() < EPS) {
        return (q >= 0.0_f64);
    }
    let r = (q / p);
    if (p < 0.0_f64) {
        if (r > (*CLIP_RANGE.lock().unwrap()).t1) {
            return false;
        }
        if (r > (*CLIP_RANGE.lock().unwrap()).t0) {
            (*CLIP_RANGE.lock().unwrap()).t0 = r;
        }
    } else {
        if (r < (*CLIP_RANGE.lock().unwrap()).t0) {
            return false;
        }
        if (r < (*CLIP_RANGE.lock().unwrap()).t1) {
            (*CLIP_RANGE.lock().unwrap()).t1 = r;
        }
    }
    return true;
}

// Source: upstream/packages/collision/src/segmentCollision.ts:190 (sha256:dd61db9d3ebba4cf5d1453c0651b456c107ae6948ea215449886d1e348d46174)
#[derive(Clone, Default)]
struct ClipRange {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub t0: f64,
    pub t1: f64,
}
impl PartialEq for ClipRange {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

static CLIP_RANGE: std::sync::LazyLock<std::sync::Mutex<ClipRange>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(ClipRange {
            __flight_identity: std::sync::Arc::new(()),
            t0: 0.0_f64,
            t1: 1.0_f64,
        })
    });
