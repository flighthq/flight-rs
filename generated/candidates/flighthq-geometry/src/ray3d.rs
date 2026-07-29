// @generated from upstream/packages/geometry/src/ray3d.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_vector3;
use flighthq_entity::create_entity;
use flighthq_types::{AabbLike, BoundingSphereLike, PlaneLike, Ray3D, Ray3DLike, Vector3Like};

// Source: upstream/packages/geometry/src/ray3d.ts:8 (sha256:641551e2486de3359bf502a1d45955989bfb14eb3da05e80062ce482f10e198a)
pub fn create_ray3_d(
    origin_x: Option<f64>,
    origin_y: Option<f64>,
    origin_z: Option<f64>,
    direction_x: Option<f64>,
    direction_y: Option<f64>,
    direction_z: Option<f64>,
) -> Ray3D {
    return create_entity(Some(Ray3D {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        direction: create_vector3(
            Some((direction_x).unwrap_or(0.0_f64)),
            Some((direction_y).unwrap_or(0.0_f64)),
            Some((direction_z).unwrap_or(1.0_f64)),
        ),
        origin: create_vector3(
            Some((origin_x).unwrap_or(0.0_f64)),
            Some((origin_y).unwrap_or(0.0_f64)),
            Some((origin_z).unwrap_or(0.0_f64)),
        ),
    }));
}

// Source: upstream/packages/geometry/src/ray3d.ts:31 (sha256:3547f4bfabf6b0e7aaee6a03f658b6b93c1ae0ffc276f70d114d968c06332199)
pub fn get_closest_point_between_ray3_ds(
    out_a: &mut Vector3Like,
    out_b: &mut Vector3Like,
    a: &Ray3DLike,
    b: &Ray3DLike,
) -> () {
    let aox = a.origin.x;
    let aoy = a.origin.y;
    let aoz = a.origin.z;
    let adx = a.direction.x;
    let ady = a.direction.y;
    let adz = a.direction.z;
    let box_ = b.origin.x;
    let boy = b.origin.y;
    let boz = b.origin.z;
    let bdx = b.direction.x;
    let bdy = b.direction.y;
    let bdz = b.direction.z;
    let aa = (((adx * adx) + (ady * ady)) + (adz * adz));
    let bb = (((bdx * bdx) + (bdy * bdy)) + (bdz * bdz));
    let ab = (((adx * bdx) + (ady * bdy)) + (adz * bdz));
    let rx = (aox - box_);
    let ry = (aoy - boy);
    let rz = (aoz - boz);
    let ar = (((adx * rx) + (ady * ry)) + (adz * rz));
    let br = (((bdx * rx) + (bdy * ry)) + (bdz * rz));
    let denom = ((aa * bb) - (ab * ab));
    let mut ta: f64;
    let mut tb: f64;
    if (denom != 0.0_f64) {
        ta = (((ab * br) - (bb * ar)) / denom);
    } else {
        ta = 0.0_f64;
    }
    if (ta < 0.0_f64) {
        ta = 0.0_f64;
    }
    tb = if (bb != 0.0_f64) {
        (((ab * ta) + br) / bb)
    } else {
        0.0_f64
    };
    if (tb < 0.0_f64) {
        tb = 0.0_f64;
        ta = if (aa != 0.0_f64) {
            ((-ar) / aa)
        } else {
            0.0_f64
        };
        if (ta < 0.0_f64) {
            ta = 0.0_f64;
        }
    }
    out_a.x = (aox + (adx * ta));
    out_a.y = (aoy + (ady * ta));
    out_a.z = (aoz + (adz * ta));
    out_b.x = (box_ + (bdx * tb));
    out_b.y = (boy + (bdy * tb));
    out_b.z = (boz + (bdz * tb));
}

// Source: upstream/packages/geometry/src/ray3d.ts:96 (sha256:13ad1ac90b7e347345c5114a3d34b2dca30d97871cdc2177923fd54f09e81cac)
pub fn get_closest_point_on_ray3_d(
    out: &mut Vector3Like,
    ray: &Ray3DLike,
    point: &Vector3Like,
) -> () {
    let ox = ray.origin.x;
    let oy = ray.origin.y;
    let oz = ray.origin.z;
    let dx = ray.direction.x;
    let dy = ray.direction.y;
    let dz = ray.direction.z;
    let px = point.x;
    let py = point.y;
    let pz = point.z;
    let len_sq = (((dx * dx) + (dy * dy)) + (dz * dz));
    let mut t = if (len_sq != 0.0_f64) {
        (((((px - ox) * dx) + ((py - oy) * dy)) + ((pz - oz) * dz)) / len_sq)
    } else {
        0.0_f64
    };
    if (t < 0.0_f64) {
        t = 0.0_f64;
    }
    out.x = (ox + (dx * t));
    out.y = (oy + (dy * t));
    out.z = (oz + (dz * t));
}

// Source: upstream/packages/geometry/src/ray3d.ts:122 (sha256:e524f85d205b34210b0699dde72b1d84d42283ad2bcca5b80ee5d4d1c6a13f19)
pub fn get_ray3_d_point_at(out: &mut Vector3Like, ray: &Ray3DLike, t: f64) -> () {
    let ox = ray.origin.x;
    let oy = ray.origin.y;
    let oz = ray.origin.z;
    let dx = ray.direction.x;
    let dy = ray.direction.y;
    let dz = ray.direction.z;
    out.x = (ox + (dx * t));
    out.y = (oy + (dy * t));
    out.z = (oz + (dz * t));
}

// Source: upstream/packages/geometry/src/ray3d.ts:142 (sha256:07968273024ccd712968bfc15e53495e94aa2aab603dce69ff1698dded75aedc)
pub fn intersect_ray3_d_aabb(ray: &Ray3DLike, aabb: &AabbLike) -> f64 {
    let ox = ray.origin.x;
    let oy = ray.origin.y;
    let oz = ray.origin.z;
    let dx = ray.direction.x;
    let dy = ray.direction.y;
    let dz = ray.direction.z;
    let mut t_min = 0.0_f64;
    let mut t_max = f64::INFINITY;
    if (dx != 0.0_f64) {
        let inv_dx = (1.0_f64 / dx);
        let mut t1 = ((aabb.min.x - ox) * inv_dx);
        let mut t2 = ((aabb.max.x - ox) * inv_dx);
        if (t1 > t2) {
            let tmp = t1;
            t1 = t2;
            t2 = tmp;
        }
        t_min = (t_min).max(t1);
        t_max = (t_max).min(t2);
        if (t_min > t_max) {
            return (-1.0_f64);
        }
    } else {
        if (ox < aabb.min.x) || (ox > aabb.max.x) {
            return (-1.0_f64);
        }
    }
    if (dy != 0.0_f64) {
        let inv_dy = (1.0_f64 / dy);
        let mut t1 = ((aabb.min.y - oy) * inv_dy);
        let mut t2 = ((aabb.max.y - oy) * inv_dy);
        if (t1 > t2) {
            let tmp = t1;
            t1 = t2;
            t2 = tmp;
        }
        t_min = (t_min).max(t1);
        t_max = (t_max).min(t2);
        if (t_min > t_max) {
            return (-1.0_f64);
        }
    } else {
        if (oy < aabb.min.y) || (oy > aabb.max.y) {
            return (-1.0_f64);
        }
    }
    if (dz != 0.0_f64) {
        let inv_dz = (1.0_f64 / dz);
        let mut t1 = ((aabb.min.z - oz) * inv_dz);
        let mut t2 = ((aabb.max.z - oz) * inv_dz);
        if (t1 > t2) {
            let tmp = t1;
            t1 = t2;
            t2 = tmp;
        }
        t_min = (t_min).max(t1);
        t_max = (t_max).min(t2);
        if (t_min > t_max) {
            return (-1.0_f64);
        }
    } else {
        if (oz < aabb.min.z) || (oz > aabb.max.z) {
            return (-1.0_f64);
        }
    }
    return t_min;
}

// Source: upstream/packages/geometry/src/ray3d.ts:216 (sha256:b6cb75603564946f9a9cdfa95462e903eb200e30c2194d1829b8ef0093b79b42)
pub fn intersect_ray3_d_plane(ray: &Ray3DLike, plane: &PlaneLike) -> f64 {
    let denom =
        (((plane.a * ray.direction.x) + (plane.b * ray.direction.y)) + (plane.c * ray.direction.z));
    if ((denom).abs() < 1e-10_f64) {
        return (-1.0_f64);
    }
    let t = ((-((((plane.a * ray.origin.x) + (plane.b * ray.origin.y))
        + (plane.c * ray.origin.z))
        + plane.d))
        / denom);
    return if (t >= 0.0_f64) { t } else { (-1.0_f64) };
}

// Source: upstream/packages/geometry/src/ray3d.ts:232 (sha256:8e554db0f73a4d7b0a53189709d9ea65a11533872e9cf8db511600529eee268c)
pub fn intersect_ray3_d_sphere(ray: &Ray3DLike, sphere: &BoundingSphereLike) -> f64 {
    if (sphere.radius < 0.0_f64) {
        return (-1.0_f64);
    }
    let ox = (ray.origin.x - sphere.center.x);
    let oy = (ray.origin.y - sphere.center.y);
    let oz = (ray.origin.z - sphere.center.z);
    let dx = ray.direction.x;
    let dy = ray.direction.y;
    let dz = ray.direction.z;
    let a = (((dx * dx) + (dy * dy)) + (dz * dz));
    if (a == 0.0_f64) {
        return (-1.0_f64);
    }
    let b = (((ox * dx) + (oy * dy)) + (oz * dz));
    let c = ((((ox * ox) + (oy * oy)) + (oz * oz)) - (sphere.radius * sphere.radius));
    let disc = ((b * b) - (a * c));
    if (disc < 0.0_f64) {
        return (-1.0_f64);
    }
    let sqrt_disc = (disc).sqrt();
    let t = (((-b) - sqrt_disc) / a);
    if (t >= 0.0_f64) {
        return t;
    }
    let t2 = (((-b) + sqrt_disc) / a);
    return if (t2 >= 0.0_f64) { 0.0_f64 } else { (-1.0_f64) };
}

// Source: upstream/packages/geometry/src/ray3d.ts:265 (sha256:1a5ecdb74fb21217f2ff1a1b3bcc2e4bd080d9c135a32efd1f877054f91d2a06)
pub fn intersect_ray3_d_triangle(
    ray: &Ray3DLike,
    a: &Vector3Like,
    b: &Vector3Like,
    c: &Vector3Like,
) -> f64 {
    let e1x = (b.x - a.x);
    let e1y = (b.y - a.y);
    let e1z = (b.z - a.z);
    let e2x = (c.x - a.x);
    let e2y = (c.y - a.y);
    let e2z = (c.z - a.z);
    let dx = ray.direction.x;
    let dy = ray.direction.y;
    let dz = ray.direction.z;
    let hx = ((dy * e2z) - (dz * e2y));
    let hy = ((dz * e2x) - (dx * e2z));
    let hz = ((dx * e2y) - (dy * e2x));
    let det = (((e1x * hx) + (e1y * hy)) + (e1z * hz));
    if ((det).abs() < 1e-10_f64) {
        return (-1.0_f64);
    }
    let inv_det = (1.0_f64 / det);
    let sx = (ray.origin.x - a.x);
    let sy = (ray.origin.y - a.y);
    let sz = (ray.origin.z - a.z);
    let u = ((((sx * hx) + (sy * hy)) + (sz * hz)) * inv_det);
    if (u < 0.0_f64) || (u > 1.0_f64) {
        return (-1.0_f64);
    }
    let qx = ((sy * e1z) - (sz * e1y));
    let qy = ((sz * e1x) - (sx * e1z));
    let qz = ((sx * e1y) - (sy * e1x));
    let v = ((((dx * qx) + (dy * qy)) + (dz * qz)) * inv_det);
    if (v < 0.0_f64) || ((u + v) > 1.0_f64) {
        return (-1.0_f64);
    }
    let t = ((((e2x * qx) + (e2y * qy)) + (e2z * qz)) * inv_det);
    return if (t >= 0.0_f64) { t } else { (-1.0_f64) };
}

// Source: upstream/packages/geometry/src/ray3d.ts:319 (sha256:726bdf41eec8ab7f900b72f0d5fca38ef597d7a9695f55c6b4654c7f8d2a3c07)
pub fn set_ray3_d(out: &mut Ray3DLike, origin: &Vector3Like, direction: &Vector3Like) -> () {
    let ox = origin.x;
    let oy = origin.y;
    let oz = origin.z;
    let dx = direction.x;
    let dy = direction.y;
    let dz = direction.z;
    out.origin.x = ox;
    out.origin.y = oy;
    out.origin.z = oz;
    out.direction.x = dx;
    out.direction.y = dy;
    out.direction.z = dz;
}
