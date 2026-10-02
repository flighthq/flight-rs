// @generated from upstream/packages/geometry/src/capsule.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    AabbLike, BoundingSphereLike, Capsule, CapsuleLike, EntityConstruction, Ray3DLike, Vector3Like,
};

// Source: upstream/packages/geometry/src/capsule.ts:16 (sha256:ba714a14e7ddf81c5ac4aba3a9a11f863bd77c2f2a2e6d8153901ab383dcd0f9)
pub fn create_capsule(
    start_x: f64,
    start_y: f64,
    start_z: f64,
    end_x: f64,
    end_y: f64,
    end_z: f64,
    radius: f64,
) -> Capsule {
    let mut out = allocate_entity();
    initialize_capsule(
        (out).clone(),
        start_x,
        start_y,
        start_z,
        end_x,
        end_y,
        end_z,
        radius,
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/geometry/src/capsule.ts:38 (sha256:24b43918a2970d13271a24d6466d19dbabe9a72efdfd74fbd30d8475fd13a3fa)
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
        let __destructure0 = axis_perpendicular(abx, aby, abz, ab_len2);
        let perp_x = __destructure0[0.0_f64 as usize].clone();
        let perp_y = __destructure0[1.0_f64 as usize].clone();
        let perp_z = __destructure0[2.0_f64 as usize].clone();
        out.x = (closest_x + (r * perp_x));
        out.y = (closest_y + (r * perp_y));
        out.z = (closest_z + (r * perp_z));
    } else {
        let inv = (r / dist);
        out.x = (closest_x + (dx * inv));
        out.y = (closest_y + (dy * inv));
        out.z = (closest_z + (dz * inv));
    }
}

// Source: upstream/packages/geometry/src/capsule.ts:92 (sha256:f7326cb03915e2a831bbb11cc87e81557068d6f3d60bcc59cd39bd7cceda4e32)
pub fn initialize_capsule(
    out: EntityConstruction<Capsule>,
    start_x: f64,
    start_y: f64,
    start_z: f64,
    end_x: f64,
    end_y: f64,
    end_z: f64,
    radius: f64,
) -> () {
    crate::host_set("host.startX", start_x);
    crate::host_set("host.startY", start_y);
    crate::host_set("host.startZ", start_z);
    crate::host_set("host.endX", end_x);
    crate::host_set("host.endY", end_y);
    crate::host_set("host.endZ", end_z);
    crate::host_set("host.radius", radius);
}

// Source: upstream/packages/geometry/src/capsule.ts:119 (sha256:8c75f5da7df6f1a16c28ece1b917dd521208aa0f8d7bb187d4311cc2cedb8517)
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
    if (r < 0.0_f64) {
        return (-1.0_f64);
    }
    let direction_length_sq = (((dx * dx) + (dy * dy)) + (dz * dz));
    if (direction_length_sq == 0.0_f64) {
        return (-1.0_f64);
    }
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
            let b = (((mx * dx) + (my * dy)) + (mz * dz));
            let c = ((((mx * mx) + (my * my)) + (mz * mz)) - (r * r));
            let disc = ((b * b) - (direction_length_sq * c));
            if (disc < 0.0_f64) {
                return (-1.0_f64);
            }
            let sqrt_d = (disc).sqrt();
            let t1 = (((-b) - sqrt_d) / direction_length_sq);
            if (t1 >= 0.0_f64) {
                return t1;
            }
            let t2 = (((-b) + sqrt_d) / direction_length_sq);
            return if (t2 >= 0.0_f64) { 0.0_f64 } else { (-1.0_f64) };
        },
    )
        as Box<dyn FnMut(f64, f64, f64) -> f64 + Send + 'static>));
    if (ab_len2 < 1e-20_f64) {
        return {
            let __flight_callback = (sphere_hit).clone();
            let __flight_result = __flight_callback.lock().unwrap()(ax, ay, az);
            __flight_result
        };
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
    if ((qa).sqrt() > (1e-10_f64 * (direction_length_sq).sqrt())) {
        let disc = ((qb * qb) - (qa * qc));
        if (disc >= 0.0_f64) {
            let sqrt_d = (disc).sqrt();
            let t1 = (((-qb) - sqrt_d) / qa);
            let s1 = ((aoab + (t1 * dab)) * inv_ab2);
            if ((t1 >= 0.0_f64) && (s1 >= 0.0_f64)) && (s1 <= 1.0_f64) {
                t_best = t1;
            } else {
                if (t1 < 0.0_f64) {
                    let t2 = (((-qb) + sqrt_d) / qa);
                    if (t2 >= 0.0_f64) {
                        let s0 = (aoab * inv_ab2);
                        if (s0 >= 0.0_f64) && (s0 <= 1.0_f64) {
                            return 0.0_f64;
                        }
                    }
                }
            }
        }
    }
    let t_a = {
        let __flight_callback = (sphere_hit).clone();
        let __flight_result = __flight_callback.lock().unwrap()(ax, ay, az);
        __flight_result
    };
    if (t_a >= 0.0_f64) && ((t_best < 0.0_f64) || (t_a < t_best)) {
        t_best = t_a;
    }
    let t_b = {
        let __flight_callback = (sphere_hit).clone();
        let __flight_result = __flight_callback.lock().unwrap()(bx, by, bz);
        __flight_result
    };
    if (t_b >= 0.0_f64) && ((t_best < 0.0_f64) || (t_b < t_best)) {
        t_best = t_b;
    }
    return t_best;
}

// Source: upstream/packages/geometry/src/capsule.ts:213 (sha256:216218f8ef2362df1fe9ed25538e55d8b26bca97df62c8db608abc6751a5c036)
pub fn is_capsule_intersecting_aabb(capsule: &CapsuleLike, aabb: &AabbLike) -> bool {
    if (capsule.radius < 0.0_f64) {
        return false;
    }
    if ((aabb.min.x > aabb.max.x) || (aabb.min.y > aabb.max.y)) || (aabb.min.z > aabb.max.z) {
        return false;
    }
    let dist2 = segment_to_aabb_distance_sq(
        capsule.start_x,
        capsule.start_y,
        capsule.start_z,
        capsule.end_x,
        capsule.end_y,
        capsule.end_z,
        aabb.min.x,
        aabb.min.y,
        aabb.min.z,
        aabb.max.x,
        aabb.max.y,
        aabb.max.z,
    );
    return (dist2 <= (capsule.radius * capsule.radius));
}

// Source: upstream/packages/geometry/src/capsule.ts:237 (sha256:21afe3a2c7f68e65b86a18cb5a6ad6798467795587a5c956f2737aae533760de)
pub fn is_capsule_intersecting_capsule(a: &CapsuleLike, b: &CapsuleLike) -> bool {
    if (a.radius < 0.0_f64) || (b.radius < 0.0_f64) {
        return false;
    }
    let dist = segment_to_segment_distance_sq(
        a.start_x, a.start_y, a.start_z, a.end_x, a.end_y, a.end_z, b.start_x, b.start_y,
        b.start_z, b.end_x, b.end_y, b.end_z,
    );
    let sum_r = (a.radius + b.radius);
    return (dist <= (sum_r * sum_r));
}

// Source: upstream/packages/geometry/src/capsule.ts:261 (sha256:91dd82175d4bb57b0bff839f0cc90adb669da2c946659c33f832f3f437fac390)
pub fn is_capsule_intersecting_sphere(capsule: &CapsuleLike, sphere: &BoundingSphereLike) -> bool {
    if (capsule.radius < 0.0_f64) || (sphere.radius < 0.0_f64) {
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

// Source: upstream/packages/geometry/src/capsule.ts:284 (sha256:83f3a0ed4d591e762547da8f8d8b79526503ac9e60d2329d5610a1791c1ca178)
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

// Source: upstream/packages/geometry/src/capsule.ts:306 (sha256:75ada33010d6f3d43787614a91e71ce5ef22c5a2e8f5dfe0e91ed4297b69a9ac)
fn axis_perpendicular(abx: f64, aby: f64, abz: f64, ab_len2: f64) -> Vec<f64> {
    if (ab_len2 < 1e-20_f64) {
        return vec![1.0_f64, 0.0_f64, 0.0_f64];
    }
    let abs_x = (abx).abs();
    let abs_y = (aby).abs();
    let abs_z = (abz).abs();
    let least_aligned_x = if (abs_x <= abs_y) && (abs_x <= abs_z) {
        1.0_f64
    } else {
        0.0_f64
    };
    let least_aligned_y = if (least_aligned_x == 0.0_f64) && (abs_y <= abs_z) {
        1.0_f64
    } else {
        0.0_f64
    };
    let least_aligned_z = if (least_aligned_x == 0.0_f64) && (least_aligned_y == 0.0_f64) {
        1.0_f64
    } else {
        0.0_f64
    };
    let px = ((aby * least_aligned_z) - (abz * least_aligned_y));
    let py = ((abz * least_aligned_x) - (abx * least_aligned_z));
    let pz = ((abx * least_aligned_y) - (aby * least_aligned_x));
    let length = (((px * px) + (py * py)) + (pz * pz)).sqrt();
    return vec![(px / length), (py / length), (pz / length)];
}

// Source: upstream/packages/geometry/src/capsule.ts:324 (sha256:fb12acc176a00b7bdab104181788748c12ed431894799269a412f546a0c87aa6)
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

// Source: upstream/packages/geometry/src/capsule.ts:351 (sha256:69fd095b4fc471d8bae3a166a738be2643c71bd62529571ce2e5f9b2f9536ded)
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
    if (a < 1e-20_f64) && (e < 1e-20_f64) {
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

// Source: upstream/packages/geometry/src/capsule.ts:421 (sha256:0962ad4d3c19ad8f4ed138ffc22907958e5a4cb9d4a4b05de39facf08a332a3a)
fn segment_to_aabb_distance_sq(
    ax: f64,
    ay: f64,
    az: f64,
    bx: f64,
    by: f64,
    bz: f64,
    min_x: f64,
    min_y: f64,
    min_z: f64,
    max_x: f64,
    max_y: f64,
    max_z: f64,
) -> f64 {
    let dx = (bx - ax);
    let dy = (by - ay);
    let dz = (bz - az);
    let len2 = (((dx * dx) + (dy * dy)) + (dz * dz));
    if (len2 < 1e-20_f64) {
        let ex = ((min_x - ax).max(0.0_f64)).max((ax - max_x));
        let ey = ((min_y - ay).max(0.0_f64)).max((ay - max_y));
        let ez = ((min_z - az).max(0.0_f64)).max((az - max_z));
        return (((ex * ex) + (ey * ey)) + (ez * ez));
    }
    let mut best_dist2 = f64::INFINITY;
    let mut count = 2.0_f64;
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = 0.0_f64;
        if __flight_index == (*_SEG_CANDIDATES.lock().unwrap()).len() {
            (*_SEG_CANDIDATES.lock().unwrap()).push(__flight_value);
        } else {
            (*_SEG_CANDIDATES.lock().unwrap())[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = 1.0_f64;
        if __flight_index == (*_SEG_CANDIDATES.lock().unwrap()).len() {
            (*_SEG_CANDIDATES.lock().unwrap()).push(__flight_value);
        } else {
            (*_SEG_CANDIDATES.lock().unwrap())[__flight_index] = __flight_value;
        }
    };
    if ((dx).abs() > 1e-20_f64) {
        {
            let __flight_index = ({
                count += 1.0;
                count
            }) as usize;
            let __flight_value = (((min_x - ax) / dx).max(0.0_f64)).min(1.0_f64);
            if __flight_index == (*_SEG_CANDIDATES.lock().unwrap()).len() {
                (*_SEG_CANDIDATES.lock().unwrap()).push(__flight_value);
            } else {
                (*_SEG_CANDIDATES.lock().unwrap())[__flight_index] = __flight_value;
            }
        };
        {
            let __flight_index = ({
                count += 1.0;
                count
            }) as usize;
            let __flight_value = (((max_x - ax) / dx).max(0.0_f64)).min(1.0_f64);
            if __flight_index == (*_SEG_CANDIDATES.lock().unwrap()).len() {
                (*_SEG_CANDIDATES.lock().unwrap()).push(__flight_value);
            } else {
                (*_SEG_CANDIDATES.lock().unwrap())[__flight_index] = __flight_value;
            }
        };
    }
    if ((dy).abs() > 1e-20_f64) {
        {
            let __flight_index = ({
                count += 1.0;
                count
            }) as usize;
            let __flight_value = (((min_y - ay) / dy).max(0.0_f64)).min(1.0_f64);
            if __flight_index == (*_SEG_CANDIDATES.lock().unwrap()).len() {
                (*_SEG_CANDIDATES.lock().unwrap()).push(__flight_value);
            } else {
                (*_SEG_CANDIDATES.lock().unwrap())[__flight_index] = __flight_value;
            }
        };
        {
            let __flight_index = ({
                count += 1.0;
                count
            }) as usize;
            let __flight_value = (((max_y - ay) / dy).max(0.0_f64)).min(1.0_f64);
            if __flight_index == (*_SEG_CANDIDATES.lock().unwrap()).len() {
                (*_SEG_CANDIDATES.lock().unwrap()).push(__flight_value);
            } else {
                (*_SEG_CANDIDATES.lock().unwrap())[__flight_index] = __flight_value;
            }
        };
    }
    if ((dz).abs() > 1e-20_f64) {
        {
            let __flight_index = ({
                count += 1.0;
                count
            }) as usize;
            let __flight_value = (((min_z - az) / dz).max(0.0_f64)).min(1.0_f64);
            if __flight_index == (*_SEG_CANDIDATES.lock().unwrap()).len() {
                (*_SEG_CANDIDATES.lock().unwrap()).push(__flight_value);
            } else {
                (*_SEG_CANDIDATES.lock().unwrap())[__flight_index] = __flight_value;
            }
        };
        {
            let __flight_index = ({
                count += 1.0;
                count
            }) as usize;
            let __flight_value = (((max_z - az) / dz).max(0.0_f64)).min(1.0_f64);
            if __flight_index == (*_SEG_CANDIDATES.lock().unwrap()).len() {
                (*_SEG_CANDIDATES.lock().unwrap()).push(__flight_value);
            } else {
                (*_SEG_CANDIDATES.lock().unwrap())[__flight_index] = __flight_value;
            }
        };
    }
    {
        let mut i = 0.0_f64;
        while (i < count) {
            let t = (*_SEG_CANDIDATES.lock().unwrap())[i as usize].clone();
            let px = (ax + (t * dx));
            let py = (ay + (t * dy));
            let pz = (az + (t * dz));
            let ex = ((min_x - px).max(0.0_f64)).max((px - max_x));
            let ey = ((min_y - py).max(0.0_f64)).max((py - max_y));
            let ez = ((min_z - pz).max(0.0_f64)).max((pz - max_z));
            let d2 = (((ex * ex) + (ey * ey)) + (ez * ez));
            if (d2 < best_dist2) {
                best_dist2 = d2;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let cx = ((min_x + max_x) * 0.5_f64);
    let cy = ((min_y + max_y) * 0.5_f64);
    let cz = ((min_z + max_z) * 0.5_f64);
    let t_center = ((((((cx - ax) * dx) + ((cy - ay) * dy)) + ((cz - az) * dz)) / len2)
        .max(0.0_f64))
    .min(1.0_f64);
    {
        let px = (ax + (t_center * dx));
        let py = (ay + (t_center * dy));
        let pz = (az + (t_center * dz));
        let ex = ((min_x - px).max(0.0_f64)).max((px - max_x));
        let ey = ((min_y - py).max(0.0_f64)).max((py - max_y));
        let ez = ((min_z - pz).max(0.0_f64)).max((pz - max_z));
        let d2 = (((ex * ex) + (ey * ey)) + (ez * ez));
        if (d2 < best_dist2) {
            best_dist2 = d2;
        }
    }
    return best_dist2;
}

// Source: upstream/packages/geometry/src/capsule.ts:498 (sha256:137e6adde58a1cf690aac82be68092240dcf9036ec77b0d74e3ffe93eacce3e8)
static _SEG_CANDIDATES: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![
            0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
        ])
    });
