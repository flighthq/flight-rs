// @generated from upstream/packages/geometry/src/frustum.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_plane;
use flighthq_entity::create_entity;
use flighthq_types::{
    AabbLike, BoundingSphereLike, Frustum, FrustumLike, Matrix4Like, PlaneLike, Vector3Like,
};

// Source: upstream/packages/geometry/src/frustum.ts:18 (sha256:0c8df1bb31e8186f69c63a35e90c9cc80368e8b85661ddaffeea6917300f98f8)
pub fn create_frustum() -> Frustum {
    return create_entity(Some(Frustum {
        __flight_identity: std::sync::Arc::new(()),
        bottom: create_plane(None, None, None, None),
        far: create_plane(None, None, None, None),
        left: create_plane(None, None, None, None),
        near: create_plane(None, None, None, None),
        right: create_plane(None, None, None, None),
        top: create_plane(None, None, None, None),
    }));
}

// Source: upstream/packages/geometry/src/frustum.ts:42 (sha256:33fc5962453387aea599632f46dbe747a643ddfedf9a5f27d5964bd9dcb50c40)
pub fn get_frustum_corners(
    out: &mut Vec<Vector3Like>,
    inverse_view_projection: &Matrix4Like,
) -> () {
    let ndc_corners: Vec<Vec<f64>> = vec![
        vec![(-1.0_f64), (-1.0_f64), (-1.0_f64)],
        vec![1.0_f64, (-1.0_f64), (-1.0_f64)],
        vec![1.0_f64, 1.0_f64, (-1.0_f64)],
        vec![(-1.0_f64), 1.0_f64, (-1.0_f64)],
        vec![(-1.0_f64), (-1.0_f64), 1.0_f64],
        vec![1.0_f64, (-1.0_f64), 1.0_f64],
        vec![1.0_f64, 1.0_f64, 1.0_f64],
        vec![(-1.0_f64), 1.0_f64, 1.0_f64],
    ];
    let len = (out.len() as f64).min((ndc_corners.len() as f64));
    {
        let mut i = 0.0_f64;
        while (i < len) {
            let __destructure0 = ndc_corners[i as usize].clone();
            let nx = __destructure0[0.0_f64 as usize].clone();
            let ny = __destructure0[1.0_f64 as usize].clone();
            let nz = __destructure0[2.0_f64 as usize].clone();
            let x = (((((inverse_view_projection.m[0.0_f64 as usize] as f64) * nx)
                + ((inverse_view_projection.m[4.0_f64 as usize] as f64) * ny))
                + ((inverse_view_projection.m[8.0_f64 as usize] as f64) * nz))
                + (inverse_view_projection.m[12.0_f64 as usize] as f64));
            let y = (((((inverse_view_projection.m[1.0_f64 as usize] as f64) * nx)
                + ((inverse_view_projection.m[5.0_f64 as usize] as f64) * ny))
                + ((inverse_view_projection.m[9.0_f64 as usize] as f64) * nz))
                + (inverse_view_projection.m[13.0_f64 as usize] as f64));
            let z = (((((inverse_view_projection.m[2.0_f64 as usize] as f64) * nx)
                + ((inverse_view_projection.m[6.0_f64 as usize] as f64) * ny))
                + ((inverse_view_projection.m[10.0_f64 as usize] as f64) * nz))
                + (inverse_view_projection.m[14.0_f64 as usize] as f64));
            let w = (((((inverse_view_projection.m[3.0_f64 as usize] as f64) * nx)
                + ((inverse_view_projection.m[7.0_f64 as usize] as f64) * ny))
                + ((inverse_view_projection.m[11.0_f64 as usize] as f64) * nz))
                + (inverse_view_projection.m[15.0_f64 as usize] as f64));
            let inv_w = if (w != 0.0_f64) {
                (1.0_f64 / w)
            } else {
                1.0_f64
            };
            let mut corner = out[i as usize].clone();
            corner.x = (x * inv_w);
            corner.y = (y * inv_w);
            corner.z = (z * inv_w);
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/geometry/src/frustum.ts:74 (sha256:ec07b7e6fe2aa96684e0cf438b8c475d66b0d510a8ef3f7d33a9e9ab5d970b10)
pub fn is_frustum_containing_point(frustum: &FrustumLike, point: &Vector3Like) -> bool {
    return ((((((__plane_signed_distance(&frustum.left, point) >= 0.0_f64)
        && (__plane_signed_distance(&frustum.right, point) >= 0.0_f64))
        && (__plane_signed_distance(&frustum.bottom, point) >= 0.0_f64))
        && (__plane_signed_distance(&frustum.top, point) >= 0.0_f64))
        && (__plane_signed_distance(&frustum.near, point) >= 0.0_f64))
        && (__plane_signed_distance(&frustum.far, point) >= 0.0_f64));
}

// Source: upstream/packages/geometry/src/frustum.ts:91 (sha256:34739c072bec664cd64231f94d69f27876e6aee2c8e898c8980d02f176272958)
pub fn is_frustum_intersecting_aabb(frustum: &FrustumLike, aabb: &AabbLike) -> bool {
    return (((((__plane_intersects_aabb(&frustum.left, aabb)
        && __plane_intersects_aabb(&frustum.right, aabb))
        && __plane_intersects_aabb(&frustum.bottom, aabb))
        && __plane_intersects_aabb(&frustum.top, aabb))
        && __plane_intersects_aabb(&frustum.near, aabb))
        && __plane_intersects_aabb(&frustum.far, aabb));
}

// Source: upstream/packages/geometry/src/frustum.ts:109 (sha256:37e3a47ead4503ee5b56c8e7115d29c8190d9dda40ea335196d468f0e10ec9d1)
pub fn is_frustum_intersecting_sphere(frustum: &FrustumLike, sphere: &BoundingSphereLike) -> bool {
    if (sphere.radius < 0.0_f64) {
        return false;
    }
    let r = sphere.radius;
    return ((((((__plane_signed_distance(&frustum.left, &sphere.center) >= (-r))
        && (__plane_signed_distance(&frustum.right, &sphere.center) >= (-r)))
        && (__plane_signed_distance(&frustum.bottom, &sphere.center) >= (-r)))
        && (__plane_signed_distance(&frustum.top, &sphere.center) >= (-r)))
        && (__plane_signed_distance(&frustum.near, &sphere.center) >= (-r)))
        && (__plane_signed_distance(&frustum.far, &sphere.center) >= (-r)));
}

// Source: upstream/packages/geometry/src/frustum.ts:132 (sha256:12022dda18811bc664c3270aa393982f7ebf4b4cedf0d83f12da271ce472f0e7)
pub fn set_frustum_from_matrix4(out: &mut FrustumLike, view_projection: &Matrix4Like) -> () {
    let r00 = (view_projection.m[0.0_f64 as usize] as f64);
    let r01 = (view_projection.m[4.0_f64 as usize] as f64);
    let r02 = (view_projection.m[8.0_f64 as usize] as f64);
    let r03 = (view_projection.m[12.0_f64 as usize] as f64);
    let r10 = (view_projection.m[1.0_f64 as usize] as f64);
    let r11 = (view_projection.m[5.0_f64 as usize] as f64);
    let r12 = (view_projection.m[9.0_f64 as usize] as f64);
    let r13 = (view_projection.m[13.0_f64 as usize] as f64);
    let r20 = (view_projection.m[2.0_f64 as usize] as f64);
    let r21 = (view_projection.m[6.0_f64 as usize] as f64);
    let r22 = (view_projection.m[10.0_f64 as usize] as f64);
    let r23 = (view_projection.m[14.0_f64 as usize] as f64);
    let r30 = (view_projection.m[3.0_f64 as usize] as f64);
    let r31 = (view_projection.m[7.0_f64 as usize] as f64);
    let r32 = (view_projection.m[11.0_f64 as usize] as f64);
    let r33 = (view_projection.m[15.0_f64 as usize] as f64);
    __set_plane(
        &mut out.left,
        (r30 + r00),
        (r31 + r01),
        (r32 + r02),
        (r33 + r03),
    );
    __set_plane(
        &mut out.right,
        (r30 - r00),
        (r31 - r01),
        (r32 - r02),
        (r33 - r03),
    );
    __set_plane(
        &mut out.bottom,
        (r30 + r10),
        (r31 + r11),
        (r32 + r12),
        (r33 + r13),
    );
    __set_plane(
        &mut out.top,
        (r30 - r10),
        (r31 - r11),
        (r32 - r12),
        (r33 - r13),
    );
    __set_plane(
        &mut out.near,
        (r30 + r20),
        (r31 + r21),
        (r32 + r22),
        (r33 + r23),
    );
    __set_plane(
        &mut out.far,
        (r30 - r20),
        (r31 - r21),
        (r32 - r22),
        (r33 - r23),
    );
}

// Source: upstream/packages/geometry/src/frustum.ts:162 (sha256:ef03575556c4decc84d17785e2278df853cd02a8d00e53b1ecc96286a498666c)
fn __plane_intersects_aabb(plane: &PlaneLike, aabb: &AabbLike) -> bool {
    let px = if (plane.a >= 0.0_f64) {
        aabb.max.x
    } else {
        aabb.min.x
    };
    let py = if (plane.b >= 0.0_f64) {
        aabb.max.y
    } else {
        aabb.min.y
    };
    let pz = if (plane.c >= 0.0_f64) {
        aabb.max.z
    } else {
        aabb.min.z
    };
    return (((((plane.a * px) + (plane.b * py)) + (plane.c * pz)) + plane.d) >= 0.0_f64);
}

// Source: upstream/packages/geometry/src/frustum.ts:169 (sha256:c5d16650ccdc58df242ee4e50604f25f55c47c481a2faff14c9c1d96cef7f0ce)
fn __plane_signed_distance(plane: &PlaneLike, point: &Vector3Like) -> f64 {
    return ((((plane.a * point.x) + (plane.b * point.y)) + (plane.c * point.z)) + plane.d);
}

// Source: upstream/packages/geometry/src/frustum.ts:173 (sha256:2f4df7cdc6f240cba6d68326d6a62e7bdf13e635d3eab4ee1ea75dc03704bdda)
fn __set_plane(out: &mut PlaneLike, a: f64, b: f64, c: f64, d: f64) -> () {
    let l = (((a * a) + (b * b)) + (c * c)).sqrt();
    if (l != 0.0_f64) {
        let inv = (1.0_f64 / l);
        out.a = (a * inv);
        out.b = (b * inv);
        out.c = (c * inv);
        out.d = (d * inv);
    } else {
        out.a = a;
        out.b = b;
        out.c = c;
        out.d = d;
    }
}
