// @generated from upstream/packages/camera/src/intersection.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_camera_screen_to_world_ray, get_camera_world_to_screen};
use flighthq_types::{BoundingSphereLike, Camera, PlaneLike, Ray3DLike, Vector3Like};

// Source: upstream/packages/camera/src/intersection.ts:18 (sha256:68f4d156a03213ed5d809975f13afd1f11bb24d2f9131758352ac0d2d021ba91)
pub fn get_camera_ray_through_bounding_sphere(
    out: &mut Ray3DLike,
    camera: &Camera,
    sphere: &BoundingSphereLike,
    aspect: f64,
) -> bool {
    if (sphere.radius < 0.0_f64) {
        return false;
    }
    if (!get_camera_world_to_screen(
        &mut (*__SCRATCH_NDC.lock().unwrap()),
        camera,
        &sphere.center,
        aspect,
    )) {
        return false;
    }
    return get_camera_screen_to_world_ray(out, camera, __scratchNdc::x, __scratchNdc::y, aspect);
}

// Source: upstream/packages/camera/src/intersection.ts:49 (sha256:d6e3c96ca65a61d8bf5758f803a98dcf25e449edf00869f32665b5f8e84c6d79)
pub fn intersect_camera_ray_with_plane(
    out: &mut Vector3Like,
    ray: &Ray3DLike,
    plane: &PlaneLike,
) -> bool {
    let dx = ray.direction.x;
    let dy = ray.direction.y;
    let dz = ray.direction.z;
    let ox = ray.origin.x;
    let oy = ray.origin.y;
    let oz = ray.origin.z;
    let a = plane.a;
    let b = plane.b;
    let c = plane.c;
    let d = plane.d;
    let denom = (((a * dx) + (b * dy)) + (c * dz));
    if (denom == 0.0_f64) {
        return false;
    }
    let t = ((-((((a * ox) + (b * oy)) + (c * oz)) + d)) / denom);
    if (t < 0.0_f64) {
        return false;
    }
    out.x = (ox + (t * dx));
    out.y = (oy + (t * dy));
    out.z = (oz + (t * dz));
    return true;
}

// Source: upstream/packages/camera/src/intersection.ts:82 (sha256:674d692ceb2cb33aaebc2ded7e5e358ee5b99384d8f9c9d7bfd9aa8bbb31494b)
struct __scratchNdc;
impl __scratchNdc {
    pub const x: f64 = 0.0_f64;
    pub const y: f64 = 0.0_f64;
    pub const z: f64 = 0.0_f64;
}
