// @generated from upstream/packages/geometry/src/capsule.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{BoundingSphereLike, Capsule, CapsuleLike, Ray3DLike, Vector3Like};

// Source: upstream/packages/geometry/src/capsule.ts:8 (sha256:2870eeaa46719202636291712981cb1c04ba3d53bf149b4d115f4613ca9e2472)
pub fn create_capsule(
    start_x: f64,
    start_y: f64,
    start_z: f64,
    end_x: f64,
    end_y: f64,
    end_z: f64,
    radius: f64,
) -> Capsule {
    return create_entity(Some(Capsule {
        __flight_identity: std::sync::Arc::new(()),
        end_x: end_x,
        end_y: end_y,
        end_z: end_z,
        radius: radius,
        start_x: start_x,
        start_y: start_y,
        start_z: start_z,
    }));
}

// Source: upstream/packages/geometry/src/capsule.ts:28 (sha256:58c421afdd67352a6cb882d34cfd8e42e650a64e0bba2a67123600234d812a3f)
pub fn get_closest_point_on_capsule(
    out: &mut Vector3Like,
    capsule: &CapsuleLike,
    point: &Vector3Like,
) -> () {
    let ax = capsule.start_x;
    let ay = capsule.start_y;
    let az = capsule.start_z;
    let bx = capsule.end_x;
    let by = capsule.end_y;
    let bz = capsule.end_z;
    let px = point.x;
    let py = point.y;
    let pz = point.z;
    let r = capsule.radius;
    let abx = (bx - ax);
    let aby = (by - ay);
    let abz = (bz - az);
    let ab_len2 = (((abx * abx) + (aby * aby)) + (abz * abz));
    let mut closest_x: f64;
    let mut closest_y: f64;
    let mut closest_z: f64;
    if (ab_len2 < 1e-20_f64) {
        closest_x = ax;
        closest_y = ay;
        closest_z = az;
    } else {
        let t = ((((((px - ax) * abx) + ((py - ay) * aby)) + ((pz - az) * abz)) / ab_len2)
            .max(0.0_f64))
        .min(1.0_f64);
        closest_x = (ax + (t * abx));
        closest_y = (ay + (t * aby));
        closest_z = (az + (t * abz));
    }
    let dx = (px - closest_x);
    let dy = (py - closest_y);
    let dz = (pz - closest_z);
    let dist = (((dx * dx) + (dy * dy)) + (dz * dz)).sqrt();
    if (dist < 1e-10_f64) {
        out.x = (closest_x + r);
        out.y = closest_y;
        out.z = closest_z;
    } else {
        let inv = (r / dist);
        out.x = (closest_x + (dx * inv));
        out.y = (closest_y + (dy * inv));
        out.z = (closest_z + (dz * inv));
    }
}

// Source: upstream/packages/geometry/src/capsule.ts:85 (sha256:cfa41ef84a4fdf0d758be78eaba69662ea038573b2f06419c588d78b9296828b)
pub fn intersect_ray3_d_capsule(ray: &Ray3DLike, capsule: &CapsuleLike) -> f64 {
    let ox = ray.origin.x;
    let oy = ray.origin.y;
    let oz = ray.origin.z;
    let dx = ray.direction.x;
    let dy = ray.direction.y;
    let dz = ray.direction.z;
    let ax = capsule.start_x;
    let ay = capsule.start_y;
    let az = capsule.start_z;
    let bx = capsule.end_x;
    let by = capsule.end_y;
    let bz = capsule.end_z;
    let r = capsule.radius;
    let abx = (bx - ax);
    let aby = (by - ay);
    let abz = (bz - az);
    let ab_len2 = (((abx * abx) + (aby * aby)) + (abz * abz));
    let mut sphere_hit: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64, f64, f64) -> f64 + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new(
        move |cx: f64, cy: f64, cz: f64| -> f64 {
            let mx = (ox - cx);
            let my = (oy - cy);
            let mz = (oz - cz);
            let len_d2 = (((dx * dx) + (dy * dy)) + (dz * dz));
            if (len_d2 == 0.0_f64) {
                return (-1.0_f64);
            }
            let b = (((mx * dx) + (my * dy)) + (mz * dz));
            let c = ((((mx * mx) + (my * my)) + (mz * mz)) - (r * r));
            let disc = ((b * b) - (len_d2 * c));
            if (disc < 0.0_f64) {
                return (-1.0_f64);
            }
            let sqrt_d = (disc).sqrt();
            let t1 = (((-b) - sqrt_d) / len_d2);
            if (t1 >= 0.0_f64) {
                return t1;
            }
            let t2 = (((-b) + sqrt_d) / len_d2);
            return if (t2 >= 0.0_f64) { 0.0_f64 } else { (-1.0_f64) };
        },
    )
        as Box<dyn FnMut(f64, f64, f64) -> f64 + Send + 'static>));
    if (ab_len2 < 1e-20_f64) {
        return ((sphere_hit).clone()).lock().unwrap()(ax, ay, az);
    }
    let mut t_best = (-1.0_f64);
    let inv_ab2 = (1.0_f64 / ab_len2);
    let aox = (ox - ax);
    let aoy = (oy - ay);
    let aoz = (oz - az);
    let dab = (((dx * abx) + (dy * aby)) + (dz * abz));
    let aoab = (((aox * abx) + (aoy * aby)) + (aoz * abz));
    let dpx = (dx - ((dab * inv_ab2) * abx));
    let dpy = (dy - ((dab * inv_ab2) * aby));
    let dpz = (dz - ((dab * inv_ab2) * abz));
    let apx = (aox - ((aoab * inv_ab2) * abx));
    let apy = (aoy - ((aoab * inv_ab2) * aby));
    let apz = (aoz - ((aoab * inv_ab2) * abz));
    let qa = (((dpx * dpx) + (dpy * dpy)) + (dpz * dpz));
    let qb = (((apx * dpx) + (apy * dpy)) + (apz * dpz));
    let qc = ((((apx * apx) + (apy * apy)) + (apz * apz)) - (r * r));
    if (qa > 1e-20_f64) {
        let disc = ((qb * qb) - (qa * qc));
        if (disc >= 0.0_f64) {
            let sqrt_d = (disc).sqrt();
            let t1 = (((-qb) - sqrt_d) / qa);
            let s1 = ((aoab + (t1 * dab)) * inv_ab2);
            if (((t1 >= 0.0_f64) && (s1 >= 0.0_f64)) && (s1 <= 1.0_f64)) {
                t_best = t1;
            } else {
                if (t1 < 0.0_f64) {
                    let t2 = (((-qb) + sqrt_d) / qa);
                    if (t2 >= 0.0_f64) {
                        let s0 = (aoab * inv_ab2);
                        if ((s0 >= 0.0_f64) && (s0 <= 1.0_f64)) {
                            return 0.0_f64;
                        }
                    }
                }
            }
        }
    }
    let t_a = ((sphere_hit).clone()).lock().unwrap()(ax, ay, az);
    if ((t_a >= 0.0_f64) && ((t_best < 0.0_f64) || (t_a < t_best))) {
        t_best = t_a;
    }
    let t_b = ((sphere_hit).clone()).lock().unwrap()(bx, by, bz);
    if ((t_b >= 0.0_f64) && ((t_best < 0.0_f64) || (t_b < t_best))) {
        t_best = t_b;
    }
    return t_best;
}

// Source: upstream/packages/geometry/src/capsule.ts:177 (sha256:21afe3a2c7f68e65b86a18cb5a6ad6798467795587a5c956f2737aae533760de)
pub fn is_capsule_intersecting_capsule(a: &CapsuleLike, b: &CapsuleLike) -> bool {
    if ((a.radius < 0.0_f64) || (b.radius < 0.0_f64)) {
        return false;
    }
    let dist = segment_to_segment_distance_sq(
        a.start_x, a.start_y, a.start_z, a.end_x, a.end_y, a.end_z, b.start_x, b.start_y,
        b.start_z, b.end_x, b.end_y, b.end_z,
    );
    let sum_r = (a.radius + b.radius);
    return (dist <= (sum_r * sum_r));
}

// Source: upstream/packages/geometry/src/capsule.ts:201 (sha256:91dd82175d4bb57b0bff839f0cc90adb669da2c946659c33f832f3f437fac390)
pub fn is_capsule_intersecting_sphere(capsule: &CapsuleLike, sphere: &BoundingSphereLike) -> bool {
    if ((capsule.radius < 0.0_f64) || (sphere.radius < 0.0_f64)) {
        return false;
    }
    let dist2 = point_to_segment_distance_sq(
        sphere.center.x,
        sphere.center.y,
        sphere.center.z,
        capsule.start_x,
        capsule.start_y,
        capsule.start_z,
        capsule.end_x,
        capsule.end_y,
        capsule.end_z,
    );
    let sum_r = (capsule.radius + sphere.radius);
    return (dist2 <= (sum_r * sum_r));
}

// Source: upstream/packages/geometry/src/capsule.ts:224 (sha256:83f3a0ed4d591e762547da8f8d8b79526503ac9e60d2329d5610a1791c1ca178)
pub fn set_capsule(
    out: &mut CapsuleLike,
    start_x: f64,
    start_y: f64,
    start_z: f64,
    end_x: f64,
    end_y: f64,
    end_z: f64,
    radius: f64,
) -> () {
    out.start_x = start_x;
    out.start_y = start_y;
    out.start_z = start_z;
    out.end_x = end_x;
    out.end_y = end_y;
    out.end_z = end_z;
    out.radius = radius;
}

// Source: upstream/packages/geometry/src/capsule.ts:244 (sha256:fb12acc176a00b7bdab104181788748c12ed431894799269a412f546a0c87aa6)
fn point_to_segment_distance_sq(
    px: f64,
    py: f64,
    pz: f64,
    ax: f64,
    ay: f64,
    az: f64,
    bx: f64,
    by: f64,
    bz: f64,
) -> f64 {
    let abx = (bx - ax);
    let aby = (by - ay);
    let abz = (bz - az);
    let apx = (px - ax);
    let apy = (py - ay);
    let apz = (pz - az);
    let len2 = (((abx * abx) + (aby * aby)) + (abz * abz));
    let mut t = if (len2 > 0.0_f64) {
        ((((apx * abx) + (apy * aby)) + (apz * abz)) / len2)
    } else {
        0.0_f64
    };
    t = ((t).max(0.0_f64)).min(1.0_f64);
    let cx = ((ax + (t * abx)) - px);
    let cy = ((ay + (t * aby)) - py);
    let cz = ((az + (t * abz)) - pz);
    return (((cx * cx) + (cy * cy)) + (cz * cz));
}

// Source: upstream/packages/geometry/src/capsule.ts:271 (sha256:69fd095b4fc471d8bae3a166a738be2643c71bd62529571ce2e5f9b2f9536ded)
fn segment_to_segment_distance_sq(
    ax: f64,
    ay: f64,
    az: f64,
    bx: f64,
    by: f64,
    bz: f64,
    cx: f64,
    cy: f64,
    cz: f64,
    dx: f64,
    dy: f64,
    dz: f64,
) -> f64 {
    let d1x = (bx - ax);
    let d1y = (by - ay);
    let d1z = (bz - az);
    let d2x = (dx - cx);
    let d2y = (dy - cy);
    let d2z = (dz - cz);
    let rx = (ax - cx);
    let ry = (ay - cy);
    let rz = (az - cz);
    let a = (((d1x * d1x) + (d1y * d1y)) + (d1z * d1z));
    let e = (((d2x * d2x) + (d2y * d2y)) + (d2z * d2z));
    let f = (((d2x * rx) + (d2y * ry)) + (d2z * rz));
    let mut s: f64;
    let mut t: f64;
    if ((a < 1e-20_f64) && (e < 1e-20_f64)) {
        s = 0.0_f64;
        t = 0.0_f64;
    } else {
        if (a < 1e-20_f64) {
            s = 0.0_f64;
            t = ((f / e).max(0.0_f64)).min(1.0_f64);
        } else {
            let c = (((d1x * rx) + (d1y * ry)) + (d1z * rz));
            if (e < 1e-20_f64) {
                t = 0.0_f64;
                s = (((-c) / a).max(0.0_f64)).min(1.0_f64);
            } else {
                let b = (((d1x * d2x) + (d1y * d2y)) + (d1z * d2z));
                let denom = ((a * e) - (b * b));
                if (denom > 1e-20_f64) {
                    s = ((((b * f) - (c * e)) / denom).max(0.0_f64)).min(1.0_f64);
                } else {
                    s = 0.0_f64;
                }
                t = (((b * s) + f) / e);
                if (t < 0.0_f64) {
                    t = 0.0_f64;
                    s = (((-c) / a).max(0.0_f64)).min(1.0_f64);
                } else {
                    if (t > 1.0_f64) {
                        t = 1.0_f64;
                        s = (((b - c) / a).max(0.0_f64)).min(1.0_f64);
                    }
                }
            }
        }
    }
    let qx = ((ax + (s * d1x)) - (cx + (t * d2x)));
    let qy = ((ay + (s * d1y)) - (cy + (t * d2y)));
    let qz = ((az + (s * d1z)) - (cz + (t * d2z)));
    return (((qx * qx) + (qy * qy)) + (qz * qz));
}
