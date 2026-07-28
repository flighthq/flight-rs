// @generated from upstream/packages/geometry/src/obb.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{AabbLike, Matrix4Like, Obb, ObbLike, Ray3DLike, Vector3Like};

// Source: upstream/packages/geometry/src/obb.ts:8 (sha256:de07e25025b84e6305950b540b9416c3ce73c9793dd30fa2455e6dff7c59336f)
pub fn create_obb(
    center_x: f64,
    center_y: f64,
    center_z: f64,
    half_extent_x: f64,
    half_extent_y: f64,
    half_extent_z: f64,
    orientation_x: f64,
    orientation_y: f64,
    orientation_z: f64,
    orientation_w: f64,
) -> Obb {
    return create_entity(Some(Obb {
        __flight_identity: std::sync::Arc::new(()),
        center_x: center_x,
        center_y: center_y,
        center_z: center_z,
        half_extent_x: half_extent_x,
        half_extent_y: half_extent_y,
        half_extent_z: half_extent_z,
        orientation_w: orientation_w,
        orientation_x: orientation_x,
        orientation_y: orientation_y,
        orientation_z: orientation_z,
    }));
}

// Source: upstream/packages/geometry/src/obb.ts:40 (sha256:d89ff13e9afef370f8676caa086b55c2b88793f662f33e0bb2ef839684bb7954)
pub fn get_closest_point_on_obb(out: &mut Vector3Like, obb: &ObbLike, point: &Vector3Like) -> () {
    let cx = obb.center_x;
    let cy = obb.center_y;
    let cz = obb.center_z;
    let hx = obb.half_extent_x;
    let hy = obb.half_extent_y;
    let hz = obb.half_extent_z;
    let px = point.x;
    let py = point.y;
    let pz = point.z;
    let __destructure0 = obb_local_axes(obb);
    let ax0 = __destructure0[0.0_f64 as usize].clone();
    let ay0 = __destructure0[1.0_f64 as usize].clone();
    let az0 = __destructure0[2.0_f64 as usize].clone();
    let ax1 = __destructure0[3.0_f64 as usize].clone();
    let ay1 = __destructure0[4.0_f64 as usize].clone();
    let az1 = __destructure0[5.0_f64 as usize].clone();
    let ax2 = __destructure0[6.0_f64 as usize].clone();
    let ay2 = __destructure0[7.0_f64 as usize].clone();
    let az2 = __destructure0[8.0_f64 as usize].clone();
    let dx = (px - cx);
    let dy = (py - cy);
    let dz = (pz - cz);
    let d0 = ((((dx * ax0) + (dy * ay0)) + (dz * az0)).max((-hx))).min(hx);
    let d1 = ((((dx * ax1) + (dy * ay1)) + (dz * az1)).max((-hy))).min(hy);
    let d2 = ((((dx * ax2) + (dy * ay2)) + (dz * az2)).max((-hz))).min(hz);
    out.x = (((cx + (d0 * ax0)) + (d1 * ax1)) + (d2 * ax2));
    out.y = (((cy + (d0 * ay0)) + (d1 * ay1)) + (d2 * ay2));
    out.z = (((cz + (d0 * az0)) + (d1 * az1)) + (d2 * az2));
}

// Source: upstream/packages/geometry/src/obb.ts:73 (sha256:46c79c0bfbf19bafb05c76f1b2856526b2cd2cd85973b09f0f6ef46965b83256)
pub fn intersect_ray3_d_obb(ray: &Ray3DLike, obb: &ObbLike) -> f64 {
    let ox = (ray.origin.x - obb.center_x);
    let oy = (ray.origin.y - obb.center_y);
    let oz = (ray.origin.z - obb.center_z);
    let dx = ray.direction.x;
    let dy = ray.direction.y;
    let dz = ray.direction.z;
    let hx = obb.half_extent_x;
    let hy = obb.half_extent_y;
    let hz = obb.half_extent_z;
    let __destructure1 = obb_local_axes(obb);
    let ax0 = __destructure1[0.0_f64 as usize].clone();
    let ay0 = __destructure1[1.0_f64 as usize].clone();
    let az0 = __destructure1[2.0_f64 as usize].clone();
    let ax1 = __destructure1[3.0_f64 as usize].clone();
    let ay1 = __destructure1[4.0_f64 as usize].clone();
    let az1 = __destructure1[5.0_f64 as usize].clone();
    let ax2 = __destructure1[6.0_f64 as usize].clone();
    let ay2 = __destructure1[7.0_f64 as usize].clone();
    let az2 = __destructure1[8.0_f64 as usize].clone();
    let origins = vec![
        (((ox * ax0) + (oy * ay0)) + (oz * az0)),
        (((ox * ax1) + (oy * ay1)) + (oz * az1)),
        (((ox * ax2) + (oy * ay2)) + (oz * az2)),
    ];
    let dirs = vec![
        (((dx * ax0) + (dy * ay0)) + (dz * az0)),
        (((dx * ax1) + (dy * ay1)) + (dz * az1)),
        (((dx * ax2) + (dy * ay2)) + (dz * az2)),
    ];
    let half_exts = vec![hx, hy, hz];
    let mut t_min = 0.0_f64;
    let mut t_max = f64::INFINITY;
    {
        let mut i = 0.0_f64;
        while (i < 3.0_f64) {
            let o = origins[i as usize].clone();
            let d = dirs[i as usize].clone();
            let h = half_exts[i as usize].clone();
            if ((d).abs() > 1e-10_f64) {
                let inv_d = (1.0_f64 / d);
                let mut t1 = (((-h) - o) * inv_d);
                let mut t2 = ((h - o) * inv_d);
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
                if (o < (-h)) || (o > h) {
                    return (-1.0_f64);
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return t_min;
}

// Source: upstream/packages/geometry/src/obb.ts:121 (sha256:e934992ca70692aab1883a5b1246b05ac958554578b32bf4798952d512d3e230)
pub fn is_obb_intersecting_aabb(obb: &ObbLike, aabb: &AabbLike) -> bool {
    let acx = ((aabb.min.x + aabb.max.x) * 0.5_f64);
    let acy = ((aabb.min.y + aabb.max.y) * 0.5_f64);
    let acz = ((aabb.min.z + aabb.max.z) * 0.5_f64);
    let ahx = ((aabb.max.x - aabb.min.x) * 0.5_f64);
    let ahy = ((aabb.max.y - aabb.min.y) * 0.5_f64);
    let ahz = ((aabb.max.z - aabb.min.z) * 0.5_f64);
    let __destructure2 = obb_local_axes(obb);
    let ax0 = __destructure2[0.0_f64 as usize].clone();
    let ay0 = __destructure2[1.0_f64 as usize].clone();
    let az0 = __destructure2[2.0_f64 as usize].clone();
    let ax1 = __destructure2[3.0_f64 as usize].clone();
    let ay1 = __destructure2[4.0_f64 as usize].clone();
    let az1 = __destructure2[5.0_f64 as usize].clone();
    let ax2 = __destructure2[6.0_f64 as usize].clone();
    let ay2 = __destructure2[7.0_f64 as usize].clone();
    let az2 = __destructure2[8.0_f64 as usize].clone();
    let tx = (acx - obb.center_x);
    let ty = (acy - obb.center_y);
    let tz = (acz - obb.center_z);
    return (!obb_sat_separated(
        tx,
        ty,
        tz,
        ax0,
        ay0,
        az0,
        ax1,
        ay1,
        az1,
        ax2,
        ay2,
        az2,
        obb.half_extent_x,
        obb.half_extent_y,
        obb.half_extent_z,
        1.0_f64,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        1.0_f64,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        1.0_f64,
        ahx,
        ahy,
        ahz,
    ));
}

// Source: upstream/packages/geometry/src/obb.ts:170 (sha256:b8961836406d249003123b977921682477d1744aa46ff6be1864662d98020192)
pub fn is_obb_intersecting_obb(a: &ObbLike, b: &ObbLike) -> bool {
    let __destructure3 = obb_local_axes(a);
    let ax0 = __destructure3[0.0_f64 as usize].clone();
    let ay0 = __destructure3[1.0_f64 as usize].clone();
    let az0 = __destructure3[2.0_f64 as usize].clone();
    let ax1 = __destructure3[3.0_f64 as usize].clone();
    let ay1 = __destructure3[4.0_f64 as usize].clone();
    let az1 = __destructure3[5.0_f64 as usize].clone();
    let ax2 = __destructure3[6.0_f64 as usize].clone();
    let ay2 = __destructure3[7.0_f64 as usize].clone();
    let az2 = __destructure3[8.0_f64 as usize].clone();
    let __destructure4 = obb_local_axes(b);
    let bx0 = __destructure4[0.0_f64 as usize].clone();
    let by0 = __destructure4[1.0_f64 as usize].clone();
    let bz0 = __destructure4[2.0_f64 as usize].clone();
    let bx1 = __destructure4[3.0_f64 as usize].clone();
    let by1 = __destructure4[4.0_f64 as usize].clone();
    let bz1 = __destructure4[5.0_f64 as usize].clone();
    let bx2 = __destructure4[6.0_f64 as usize].clone();
    let by2 = __destructure4[7.0_f64 as usize].clone();
    let bz2 = __destructure4[8.0_f64 as usize].clone();
    let tx = (b.center_x - a.center_x);
    let ty = (b.center_y - a.center_y);
    let tz = (b.center_z - a.center_z);
    return (!obb_sat_separated(
        tx,
        ty,
        tz,
        ax0,
        ay0,
        az0,
        ax1,
        ay1,
        az1,
        ax2,
        ay2,
        az2,
        a.half_extent_x,
        a.half_extent_y,
        a.half_extent_z,
        bx0,
        by0,
        bz0,
        bx1,
        by1,
        bz1,
        bx2,
        by2,
        bz2,
        b.half_extent_x,
        b.half_extent_y,
        b.half_extent_z,
    ));
}

// Source: upstream/packages/geometry/src/obb.ts:212 (sha256:61ea95c991265b89303b10c7363faa9e5c22ca7819eeeb6f1776f6079b2ce841)
pub fn set_obb(
    out: &mut ObbLike,
    center_x: f64,
    center_y: f64,
    center_z: f64,
    half_extent_x: f64,
    half_extent_y: f64,
    half_extent_z: f64,
    orientation_x: f64,
    orientation_y: f64,
    orientation_z: f64,
    orientation_w: f64,
) -> () {
    out.center_x = center_x;
    out.center_y = center_y;
    out.center_z = center_z;
    out.half_extent_x = half_extent_x;
    out.half_extent_y = half_extent_y;
    out.half_extent_z = half_extent_z;
    out.orientation_x = orientation_x;
    out.orientation_y = orientation_y;
    out.orientation_z = orientation_z;
    out.orientation_w = orientation_w;
}

// Source: upstream/packages/geometry/src/obb.ts:244 (sha256:3dbb4edf9283b4bf0a8a668222f83f97b8ff68b7c1d078923cf2deb4643bb522)
pub fn transform_obb_by_matrix4(out: &mut ObbLike, obb: &ObbLike, m: &Matrix4Like) -> () {
    let cx = obb.center_x;
    let cy = obb.center_y;
    let cz = obb.center_z;
    let hx = obb.half_extent_x;
    let hy = obb.half_extent_y;
    let hz = obb.half_extent_z;
    let oqx = obb.orientation_x;
    let oqy = obb.orientation_y;
    let oqz = obb.orientation_z;
    let oqw = obb.orientation_w;
    let new_cx = (((((m.m[0.0_f64 as usize] as f64) * cx)
        + ((m.m[4.0_f64 as usize] as f64) * cy))
        + ((m.m[8.0_f64 as usize] as f64) * cz))
        + (m.m[12.0_f64 as usize] as f64));
    let new_cy = (((((m.m[1.0_f64 as usize] as f64) * cx)
        + ((m.m[5.0_f64 as usize] as f64) * cy))
        + ((m.m[9.0_f64 as usize] as f64) * cz))
        + (m.m[13.0_f64 as usize] as f64));
    let new_cz = (((((m.m[2.0_f64 as usize] as f64) * cx)
        + ((m.m[6.0_f64 as usize] as f64) * cy))
        + ((m.m[10.0_f64 as usize] as f64) * cz))
        + (m.m[14.0_f64 as usize] as f64));
    let sx = ((((m.m[0.0_f64 as usize] as f64) * (m.m[0.0_f64 as usize] as f64))
        + ((m.m[1.0_f64 as usize] as f64) * (m.m[1.0_f64 as usize] as f64)))
        + ((m.m[2.0_f64 as usize] as f64) * (m.m[2.0_f64 as usize] as f64)))
        .sqrt();
    let sy = ((((m.m[4.0_f64 as usize] as f64) * (m.m[4.0_f64 as usize] as f64))
        + ((m.m[5.0_f64 as usize] as f64) * (m.m[5.0_f64 as usize] as f64)))
        + ((m.m[6.0_f64 as usize] as f64) * (m.m[6.0_f64 as usize] as f64)))
        .sqrt();
    let sz = ((((m.m[8.0_f64 as usize] as f64) * (m.m[8.0_f64 as usize] as f64))
        + ((m.m[9.0_f64 as usize] as f64) * (m.m[9.0_f64 as usize] as f64)))
        + ((m.m[10.0_f64 as usize] as f64) * (m.m[10.0_f64 as usize] as f64)))
        .sqrt();
    let r00 = if (sx > 0.0_f64) {
        ((m.m[0.0_f64 as usize] as f64) / sx)
    } else {
        1.0_f64
    };
    let r10 = if (sx > 0.0_f64) {
        ((m.m[1.0_f64 as usize] as f64) / sx)
    } else {
        0.0_f64
    };
    let r20 = if (sx > 0.0_f64) {
        ((m.m[2.0_f64 as usize] as f64) / sx)
    } else {
        0.0_f64
    };
    let r01 = if (sy > 0.0_f64) {
        ((m.m[4.0_f64 as usize] as f64) / sy)
    } else {
        0.0_f64
    };
    let r11 = if (sy > 0.0_f64) {
        ((m.m[5.0_f64 as usize] as f64) / sy)
    } else {
        1.0_f64
    };
    let r21 = if (sy > 0.0_f64) {
        ((m.m[6.0_f64 as usize] as f64) / sy)
    } else {
        0.0_f64
    };
    let r02 = if (sz > 0.0_f64) {
        ((m.m[8.0_f64 as usize] as f64) / sz)
    } else {
        0.0_f64
    };
    let r12 = if (sz > 0.0_f64) {
        ((m.m[9.0_f64 as usize] as f64) / sz)
    } else {
        0.0_f64
    };
    let r22 = if (sz > 0.0_f64) {
        ((m.m[10.0_f64 as usize] as f64) / sz)
    } else {
        1.0_f64
    };
    let mut mqw: f64;
    let mut mqx: f64;
    let mut mqy: f64;
    let mut mqz: f64;
    let trace = ((r00 + r11) + r22);
    if (trace > 0.0_f64) {
        let s = (0.5_f64 / (trace + 1.0_f64).sqrt());
        mqw = (0.25_f64 / s);
        mqx = ((r12 - r21) * s);
        mqy = ((r20 - r02) * s);
        mqz = ((r01 - r10) * s);
    } else {
        if (r00 > r11) && (r00 > r22) {
            let s = (2.0_f64 * (((1.0_f64 + r00) - r11) - r22).sqrt());
            mqw = ((r12 - r21) / s);
            mqx = (0.25_f64 * s);
            mqy = ((r10 + r01) / s);
            mqz = ((r20 + r02) / s);
        } else {
            if (r11 > r22) {
                let s = (2.0_f64 * (((1.0_f64 + r11) - r00) - r22).sqrt());
                mqw = ((r20 - r02) / s);
                mqx = ((r10 + r01) / s);
                mqy = (0.25_f64 * s);
                mqz = ((r21 + r12) / s);
            } else {
                let s = (2.0_f64 * (((1.0_f64 + r22) - r00) - r11).sqrt());
                mqw = ((r01 - r10) / s);
                mqx = ((r20 + r02) / s);
                mqy = ((r21 + r12) / s);
                mqz = (0.25_f64 * s);
            }
        }
    }
    out.center_x = new_cx;
    out.center_y = new_cy;
    out.center_z = new_cz;
    out.half_extent_x = (hx * sx);
    out.half_extent_y = (hy * sy);
    out.half_extent_z = (hz * sz);
    out.orientation_x = ((((mqw * oqx) + (mqx * oqw)) + (mqy * oqz)) - (mqz * oqy));
    out.orientation_y = ((((mqw * oqy) - (mqx * oqz)) + (mqy * oqw)) + (mqz * oqx));
    out.orientation_z = ((((mqw * oqz) + (mqx * oqy)) - (mqy * oqx)) + (mqz * oqw));
    out.orientation_w = ((((mqw * oqw) - (mqx * oqx)) - (mqy * oqy)) - (mqz * oqz));
}

// Source: upstream/packages/geometry/src/obb.ts:320 (sha256:d32df1d99d4e1cf286dd0c3cf3e75867f34b207793170c3aee55164bc539cda0)
fn obb_local_axes(obb: &ObbLike) -> Vec<f64> {
    let qx = obb.orientation_x;
    let qy = obb.orientation_y;
    let qz = obb.orientation_z;
    let qw = obb.orientation_w;
    let xx = (qx * qx);
    let yy = (qy * qy);
    let zz = (qz * qz);
    let xy = (qx * qy);
    let xz = (qx * qz);
    let yz = (qy * qz);
    let wx = (qw * qx);
    let wy = (qw * qy);
    let wz = (qw * qz);
    return vec![
        (1.0_f64 - (2.0_f64 * (yy + zz))),
        (2.0_f64 * (xy + wz)),
        (2.0_f64 * (xz - wy)),
        (2.0_f64 * (xy - wz)),
        (1.0_f64 - (2.0_f64 * (xx + zz))),
        (2.0_f64 * (yz + wx)),
        (2.0_f64 * (xz + wy)),
        (2.0_f64 * (yz - wx)),
        (1.0_f64 - (2.0_f64 * (xx + yy))),
    ];
}

// Source: upstream/packages/geometry/src/obb.ts:351 (sha256:1003cbbf22fe2f3bd0faf276de2508124f2a006b3e7796bc00a10dc0116827ff)
fn obb_sat_separated(
    tx: f64,
    ty: f64,
    tz: f64,
    ax0: f64,
    ay0: f64,
    az0: f64,
    ax1: f64,
    ay1: f64,
    az1: f64,
    ax2: f64,
    ay2: f64,
    az2: f64,
    hax: f64,
    hay: f64,
    haz: f64,
    bx0: f64,
    by0: f64,
    bz0: f64,
    bx1: f64,
    by1: f64,
    bz1: f64,
    bx2: f64,
    by2: f64,
    bz2: f64,
    hbx: f64,
    hby: f64,
    hbz: f64,
) -> bool {
    let mut on_axis: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64, f64, f64) -> bool + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new(
        move |lx: f64, ly: f64, lz: f64| -> bool {
            let len_sq = (((lx * lx) + (ly * ly)) + (lz * lz));
            if (len_sq < 1e-10_f64) {
                return false;
            }
            let d = (((tx * lx) + (ty * ly)) + (tz * lz)).abs();
            let p_a = ((((((ax0 * lx) + (ay0 * ly)) + (az0 * lz)).abs() * hax)
                + ((((ax1 * lx) + (ay1 * ly)) + (az1 * lz)).abs() * hay))
                + ((((ax2 * lx) + (ay2 * ly)) + (az2 * lz)).abs() * haz));
            let p_b = ((((((bx0 * lx) + (by0 * ly)) + (bz0 * lz)).abs() * hbx)
                + ((((bx1 * lx) + (by1 * ly)) + (bz1 * lz)).abs() * hby))
                + ((((bx2 * lx) + (by2 * ly)) + (bz2 * lz)).abs() * hbz));
            return (d > (p_a + p_b));
        },
    )
        as Box<dyn FnMut(f64, f64, f64) -> bool + Send + 'static>));
    if {
        let __flight_callback = (on_axis).clone();
        let __flight_result = __flight_callback.lock().unwrap()(ax0, ay0, az0);
        __flight_result
    } {
        return true;
    }
    if {
        let __flight_callback = (on_axis).clone();
        let __flight_result = __flight_callback.lock().unwrap()(ax1, ay1, az1);
        __flight_result
    } {
        return true;
    }
    if {
        let __flight_callback = (on_axis).clone();
        let __flight_result = __flight_callback.lock().unwrap()(ax2, ay2, az2);
        __flight_result
    } {
        return true;
    }
    if {
        let __flight_callback = (on_axis).clone();
        let __flight_result = __flight_callback.lock().unwrap()(bx0, by0, bz0);
        __flight_result
    } {
        return true;
    }
    if {
        let __flight_callback = (on_axis).clone();
        let __flight_result = __flight_callback.lock().unwrap()(bx1, by1, bz1);
        __flight_result
    } {
        return true;
    }
    if {
        let __flight_callback = (on_axis).clone();
        let __flight_result = __flight_callback.lock().unwrap()(bx2, by2, bz2);
        __flight_result
    } {
        return true;
    }
    if {
        let __flight_callback = (on_axis).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            ((ay0 * bz0) - (az0 * by0)),
            ((az0 * bx0) - (ax0 * bz0)),
            ((ax0 * by0) - (ay0 * bx0)),
        );
        __flight_result
    } {
        return true;
    }
    if {
        let __flight_callback = (on_axis).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            ((ay0 * bz1) - (az0 * by1)),
            ((az0 * bx1) - (ax0 * bz1)),
            ((ax0 * by1) - (ay0 * bx1)),
        );
        __flight_result
    } {
        return true;
    }
    if {
        let __flight_callback = (on_axis).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            ((ay0 * bz2) - (az0 * by2)),
            ((az0 * bx2) - (ax0 * bz2)),
            ((ax0 * by2) - (ay0 * bx2)),
        );
        __flight_result
    } {
        return true;
    }
    if {
        let __flight_callback = (on_axis).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            ((ay1 * bz0) - (az1 * by0)),
            ((az1 * bx0) - (ax1 * bz0)),
            ((ax1 * by0) - (ay1 * bx0)),
        );
        __flight_result
    } {
        return true;
    }
    if {
        let __flight_callback = (on_axis).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            ((ay1 * bz1) - (az1 * by1)),
            ((az1 * bx1) - (ax1 * bz1)),
            ((ax1 * by1) - (ay1 * bx1)),
        );
        __flight_result
    } {
        return true;
    }
    if {
        let __flight_callback = (on_axis).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            ((ay1 * bz2) - (az1 * by2)),
            ((az1 * bx2) - (ax1 * bz2)),
            ((ax1 * by2) - (ay1 * bx2)),
        );
        __flight_result
    } {
        return true;
    }
    if {
        let __flight_callback = (on_axis).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            ((ay2 * bz0) - (az2 * by0)),
            ((az2 * bx0) - (ax2 * bz0)),
            ((ax2 * by0) - (ay2 * bx0)),
        );
        __flight_result
    } {
        return true;
    }
    if {
        let __flight_callback = (on_axis).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            ((ay2 * bz1) - (az2 * by1)),
            ((az2 * bx1) - (ax2 * bz1)),
            ((ax2 * by1) - (ay2 * bx1)),
        );
        __flight_result
    } {
        return true;
    }
    if {
        let __flight_callback = (on_axis).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            ((ay2 * bz2) - (az2 * by2)),
            ((az2 * bx2) - (ax2 * bz2)),
            ((ax2 * by2) - (ay2 * bx2)),
        );
        __flight_result
    } {
        return true;
    }
    return false;
}
