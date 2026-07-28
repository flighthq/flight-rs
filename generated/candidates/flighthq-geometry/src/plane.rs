// @generated from upstream/packages/geometry/src/plane.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{Plane, PlaneLike, Vector3Like};

// Source: upstream/packages/geometry/src/plane.ts:4 (sha256:24d25e5a1733ca70586714b4e538c91f09558398fc783784a7455f48a660a4f4)
pub fn clone_plane(source: &PlaneLike) -> Plane {
    return create_plane(
        Some(source.a),
        Some(source.b),
        Some(source.c),
        Some(source.d),
    );
}

// Source: upstream/packages/geometry/src/plane.ts:13 (sha256:8cb1c6ad2e2da2c91e2fa2c9f1887d526c8ca1ab5e4d470e53e503ce3cfc0cfa)
pub fn copy_plane(out: &mut PlaneLike, source: &PlaneLike) -> () {
    out.a = source.a;
    out.b = source.b;
    out.c = source.c;
    out.d = source.d;
}

// Source: upstream/packages/geometry/src/plane.ts:25 (sha256:f41ca5385ead7607aabec5faf6dab37c630e3d0455d1555d0aa89e6bfe7b3e18)
pub fn create_plane(a: Option<f64>, b: Option<f64>, c: Option<f64>, d: Option<f64>) -> Plane {
    return create_entity(Some(Plane {
        __flight_identity: std::sync::Arc::new(()),
        a: (a).unwrap_or(0.0_f64),
        b: (b).unwrap_or(0.0_f64),
        c: (c).unwrap_or(0.0_f64),
        d: (d).unwrap_or(0.0_f64),
    }));
}

// Source: upstream/packages/geometry/src/plane.ts:37 (sha256:1ef9ab4026a796f7c7f498841067b8acc12ca11130b00208325dc98f8cc74851)
pub fn get_closest_point_on_plane(
    out: &mut Vector3Like,
    plane: &PlaneLike,
    point: &Vector3Like,
) -> () {
    let px = point.x;
    let py = point.y;
    let pz = point.z;
    let dist = ((((plane.a * px) + (plane.b * py)) + (plane.c * pz)) + plane.d);
    out.x = (px - (dist * plane.a));
    out.y = (py - (dist * plane.b));
    out.z = (pz - (dist * plane.c));
}

// Source: upstream/packages/geometry/src/plane.ts:55 (sha256:dab2ec8f019d85a898cd79bf57591625d6b995097bae87bf8a68eab6060a38cc)
pub fn get_plane_coplanar_point(out: &mut Vector3Like, plane: &PlaneLike) -> () {
    out.x = ((-plane.a) * plane.d);
    out.y = ((-plane.b) * plane.d);
    out.z = ((-plane.c) * plane.d);
}

// Source: upstream/packages/geometry/src/plane.ts:65 (sha256:351833e9fc00d6eaf84edb43c203fc1ac4e39fadd5769388717afba21f16526e)
pub fn get_plane_signed_distance_to_point(plane: &PlaneLike, point: &Vector3Like) -> f64 {
    return ((((plane.a * point.x) + (plane.b * point.y)) + (plane.c * point.z)) + plane.d);
}

// Source: upstream/packages/geometry/src/plane.ts:75 (sha256:436857e9c11409f6da725c7e2cbf5c71823a6b9d5c36ea3fec32fcf97848933e)
pub fn normalize_plane(out: &mut PlaneLike, source: &PlaneLike) -> () {
    let a = source.a;
    let b = source.b;
    let c = source.c;
    let d = source.d;
    let len = (((a * a) + (b * b)) + (c * c)).sqrt();
    if (len == 0.0_f64) {
        out.a = a;
        out.b = b;
        out.c = c;
        out.d = d;
        return;
    }
    let inv = (1.0_f64 / len);
    out.a = (a * inv);
    out.b = (b * inv);
    out.c = (c * inv);
    out.d = (d * inv);
}

// Source: upstream/packages/geometry/src/plane.ts:101 (sha256:51876000e5144b859333676c774f9cdee9cdbf7e4058b7b872d95ff133faab94)
pub fn project_vector3_onto_plane(
    out: &mut Vector3Like,
    point: &Vector3Like,
    plane: &PlaneLike,
) -> () {
    let px = point.x;
    let py = point.y;
    let pz = point.z;
    let dist = ((((plane.a * px) + (plane.b * py)) + (plane.c * pz)) + plane.d);
    out.x = (px - (dist * plane.a));
    out.y = (py - (dist * plane.b));
    out.z = (pz - (dist * plane.c));
}

// Source: upstream/packages/geometry/src/plane.ts:118 (sha256:ad1af65cd19c30779e688f57b876fc43b1f0b1ae8a2331da5946494ec41d5ee7)
pub fn set_plane(out: &mut PlaneLike, a: f64, b: f64, c: f64, d: f64) -> () {
    out.a = a;
    out.b = b;
    out.c = c;
    out.d = d;
}

// Source: upstream/packages/geometry/src/plane.ts:128 (sha256:3fccacd46f7ee8eb78e9bda00510b3e78ec63960df52f1a382a4afa387119b71)
pub fn set_plane_from_normal_and_point(
    out: &mut PlaneLike,
    normal: &Vector3Like,
    point: &Vector3Like,
) -> () {
    out.a = normal.x;
    out.b = normal.y;
    out.c = normal.z;
    out.d = (-(((normal.x * point.x) + (normal.y * point.y)) + (normal.z * point.z)));
}

// Source: upstream/packages/geometry/src/plane.ts:148 (sha256:97ad79f2b4a7a5aa91004d0038e47d642ecd78a8bbf7b0512169599a49d443c0)
pub fn set_plane_from_points(
    out: &mut PlaneLike,
    a: &Vector3Like,
    b: &Vector3Like,
    c: &Vector3Like,
) -> () {
    let e1x = (b.x - a.x);
    let e1y = (b.y - a.y);
    let e1z = (b.z - a.z);
    let e2x = (c.x - a.x);
    let e2y = (c.y - a.y);
    let e2z = (c.z - a.z);
    let nx = ((e1y * e2z) - (e1z * e2y));
    let ny = ((e1z * e2x) - (e1x * e2z));
    let nz = ((e1x * e2y) - (e1y * e2x));
    let len = (((nx * nx) + (ny * ny)) + (nz * nz)).sqrt();
    if (len == 0.0_f64) {
        out.a = nx;
        out.b = ny;
        out.c = nz;
        out.d = 0.0_f64;
        return;
    }
    let inv = (1.0_f64 / len);
    out.a = (nx * inv);
    out.b = (ny * inv);
    out.c = (nz * inv);
    out.d = (-(((out.a * a.x) + (out.b * a.y)) + (out.c * a.z)));
}
