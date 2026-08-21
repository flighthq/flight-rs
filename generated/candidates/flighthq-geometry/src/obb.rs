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
        __flight_entity_runtime: Default::default(),
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

// Source: upstream/packages/geometry/src/obb.ts:40 (sha256:7d9eab016c0945bbf5577b3a063400c92f6b28d38490e35cbc9798172cb6fe13)
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
    let ax0 = (1.0_f64 - (2.0_f64 * (yy + zz)));
    let ay0 = (2.0_f64 * (xy + wz));
    let az0 = (2.0_f64 * (xz - wy));
    let ax1 = (2.0_f64 * (xy - wz));
    let ay1 = (1.0_f64 - (2.0_f64 * (xx + zz)));
    let az1 = (2.0_f64 * (yz + wx));
    let ax2 = (2.0_f64 * (xz + wy));
    let ay2 = (2.0_f64 * (yz - wx));
    let az2 = (1.0_f64 - (2.0_f64 * (xx + yy)));
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

// Source: upstream/packages/geometry/src/obb.ts:95 (sha256:8d6f7406490cfb55aad9334adb87c4fc58770cc0c9d7cdd8d9bb06bc281d435a)
pub fn intersect_ray3_d_obb(ray: &Ray3DLike, obb: &ObbLike) -> f64 {
    let ox = (ray.origin.x - obb.center_x);
    let oy = (ray.origin.y - obb.center_y);
    let oz = (ray.origin.z - obb.center_z);
    let dx = ray.direction.x;
    let dy = ray.direction.y;
    let dz = ray.direction.z;
    if ((((dx * dx) + (dy * dy)) + (dz * dz)) == 0.0_f64) {
        return (-1.0_f64);
    }
    let hx = obb.half_extent_x;
    let hy = obb.half_extent_y;
    let hz = obb.half_extent_z;
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
    let ax0 = (1.0_f64 - (2.0_f64 * (yy + zz)));
    let ay0 = (2.0_f64 * (xy + wz));
    let az0 = (2.0_f64 * (xz - wy));
    let ax1 = (2.0_f64 * (xy - wz));
    let ay1 = (1.0_f64 - (2.0_f64 * (xx + zz)));
    let az1 = (2.0_f64 * (yz + wx));
    let ax2 = (2.0_f64 * (xz + wy));
    let ay2 = (2.0_f64 * (yz - wx));
    let az2 = (1.0_f64 - (2.0_f64 * (xx + yy)));
    let origin0 = (((ox * ax0) + (oy * ay0)) + (oz * az0));
    let origin1 = (((ox * ax1) + (oy * ay1)) + (oz * az1));
    let origin2 = (((ox * ax2) + (oy * ay2)) + (oz * az2));
    let direction0 = (((dx * ax0) + (dy * ay0)) + (dz * az0));
    let direction1 = (((dx * ax1) + (dy * ay1)) + (dz * az1));
    let direction2 = (((dx * ax2) + (dy * ay2)) + (dz * az2));
    let mut t_min = 0.0_f64;
    let mut t_max = f64::INFINITY;
    if (direction0 != 0.0_f64) {
        let inv_d = (1.0_f64 / direction0);
        let mut t1 = (((-hx) - origin0) * inv_d);
        let mut t2 = ((hx - origin0) * inv_d);
        if (t1 > t2) {
            let swap = t1;
            t1 = t2;
            t2 = swap;
        }
        t_min = (t_min).max(t1);
        t_max = (t_max).min(t2);
        if (t_min > t_max) {
            return (-1.0_f64);
        }
    } else {
        if (origin0 < (-hx)) || (origin0 > hx) {
            return (-1.0_f64);
        }
    }
    if (direction1 != 0.0_f64) {
        let inv_d = (1.0_f64 / direction1);
        let mut t1 = (((-hy) - origin1) * inv_d);
        let mut t2 = ((hy - origin1) * inv_d);
        if (t1 > t2) {
            let swap = t1;
            t1 = t2;
            t2 = swap;
        }
        t_min = (t_min).max(t1);
        t_max = (t_max).min(t2);
        if (t_min > t_max) {
            return (-1.0_f64);
        }
    } else {
        if (origin1 < (-hy)) || (origin1 > hy) {
            return (-1.0_f64);
        }
    }
    if (direction2 != 0.0_f64) {
        let inv_d = (1.0_f64 / direction2);
        let mut t1 = (((-hz) - origin2) * inv_d);
        let mut t2 = ((hz - origin2) * inv_d);
        if (t1 > t2) {
            let swap = t1;
            t1 = t2;
            t2 = swap;
        }
        t_min = (t_min).max(t1);
        t_max = (t_max).min(t2);
        if (t_min > t_max) {
            return (-1.0_f64);
        }
    } else {
        if (origin2 < (-hz)) || (origin2 > hz) {
            return (-1.0_f64);
        }
    }
    return t_min;
}

// Source: upstream/packages/geometry/src/obb.ts:198 (sha256:b9e47cccfba5a4372514b275b9211239ffee04d944cadebad7b9ce910cdd6df4)
pub fn is_obb_intersecting_aabb(obb: &ObbLike, aabb: &AabbLike) -> bool {
    if ((aabb.min.x > aabb.max.x) || (aabb.min.y > aabb.max.y)) || (aabb.min.z > aabb.max.z) {
        return false;
    }
    let acx = ((aabb.min.x + aabb.max.x) * 0.5_f64);
    let acy = ((aabb.min.y + aabb.max.y) * 0.5_f64);
    let acz = ((aabb.min.z + aabb.max.z) * 0.5_f64);
    let ahx = ((aabb.max.x - aabb.min.x) * 0.5_f64);
    let ahy = ((aabb.max.y - aabb.min.y) * 0.5_f64);
    let ahz = ((aabb.max.z - aabb.min.z) * 0.5_f64);
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
    let ax0 = (1.0_f64 - (2.0_f64 * (yy + zz)));
    let ay0 = (2.0_f64 * (xy + wz));
    let az0 = (2.0_f64 * (xz - wy));
    let ax1 = (2.0_f64 * (xy - wz));
    let ay1 = (1.0_f64 - (2.0_f64 * (xx + zz)));
    let az1 = (2.0_f64 * (yz + wx));
    let ax2 = (2.0_f64 * (xz + wy));
    let ay2 = (2.0_f64 * (yz - wx));
    let az2 = (1.0_f64 - (2.0_f64 * (xx + yy)));
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

// Source: upstream/packages/geometry/src/obb.ts:270 (sha256:4fa21c2b84288204df6e762d0ddce8aa10bbdeb6ab71549dc3ab1f3e33818b29)
pub fn is_obb_intersecting_obb(a: &ObbLike, b: &ObbLike) -> bool {
    let aqx = a.orientation_x;
    let aqy = a.orientation_y;
    let aqz = a.orientation_z;
    let aqw = a.orientation_w;
    let axx = (aqx * aqx);
    let ayy = (aqy * aqy);
    let azz = (aqz * aqz);
    let axy = (aqx * aqy);
    let axz = (aqx * aqz);
    let ayz = (aqy * aqz);
    let awx = (aqw * aqx);
    let awy = (aqw * aqy);
    let awz = (aqw * aqz);
    let ax0 = (1.0_f64 - (2.0_f64 * (ayy + azz)));
    let ay0 = (2.0_f64 * (axy + awz));
    let az0 = (2.0_f64 * (axz - awy));
    let ax1 = (2.0_f64 * (axy - awz));
    let ay1 = (1.0_f64 - (2.0_f64 * (axx + azz)));
    let az1 = (2.0_f64 * (ayz + awx));
    let ax2 = (2.0_f64 * (axz + awy));
    let ay2 = (2.0_f64 * (ayz - awx));
    let az2 = (1.0_f64 - (2.0_f64 * (axx + ayy)));
    let bqx = b.orientation_x;
    let bqy = b.orientation_y;
    let bqz = b.orientation_z;
    let bqw = b.orientation_w;
    let bxx = (bqx * bqx);
    let byy = (bqy * bqy);
    let bzz = (bqz * bqz);
    let bxy = (bqx * bqy);
    let bxz = (bqx * bqz);
    let byz = (bqy * bqz);
    let bwx = (bqw * bqx);
    let bwy = (bqw * bqy);
    let bwz = (bqw * bqz);
    let bx0 = (1.0_f64 - (2.0_f64 * (byy + bzz)));
    let by0 = (2.0_f64 * (bxy + bwz));
    let bz0 = (2.0_f64 * (bxz - bwy));
    let bx1 = (2.0_f64 * (bxy - bwz));
    let by1 = (1.0_f64 - (2.0_f64 * (bxx + bzz)));
    let bz1 = (2.0_f64 * (byz + bwx));
    let bx2 = (2.0_f64 * (bxz + bwy));
    let by2 = (2.0_f64 * (byz - bwx));
    let bz2 = (1.0_f64 - (2.0_f64 * (bxx + byy)));
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

// Source: upstream/packages/geometry/src/obb.ts:355 (sha256:61ea95c991265b89303b10c7363faa9e5c22ca7819eeeb6f1776f6079b2ce841)
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

// Source: upstream/packages/geometry/src/obb.ts:387 (sha256:d7864a7c8899ff4ab64b082710a669cff4c4e94daaba268959a3b20aadada350)
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
        mqx = ((r21 - r12) * s);
        mqy = ((r02 - r20) * s);
        mqz = ((r10 - r01) * s);
    } else {
        if (r00 > r11) && (r00 > r22) {
            let s = (2.0_f64 * (((1.0_f64 + r00) - r11) - r22).sqrt());
            mqw = ((r21 - r12) / s);
            mqx = (0.25_f64 * s);
            mqy = ((r10 + r01) / s);
            mqz = ((r20 + r02) / s);
        } else {
            if (r11 > r22) {
                let s = (2.0_f64 * (((1.0_f64 + r11) - r00) - r22).sqrt());
                mqw = ((r02 - r20) / s);
                mqx = ((r10 + r01) / s);
                mqy = (0.25_f64 * s);
                mqz = ((r21 + r12) / s);
            } else {
                let s = (2.0_f64 * (((1.0_f64 + r22) - r00) - r11).sqrt());
                mqw = ((r10 - r01) / s);
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

// Source: upstream/packages/geometry/src/obb.ts:465 (sha256:d0e7f55b1d34e4af252b2f3c41bae7ea2a94d029dc2bf6b80f58fa0877c21fcb)
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
    {
        let mut axis = 0.0_f64;
        while (axis < 15.0_f64) {
            let mut lx: f64;
            let mut ly: f64;
            let mut lz: f64;
            {
                let __switch_value = axis;
                let __flight_case = if __switch_value == 0.0_f64 {
                    0_usize
                } else if __switch_value == 1.0_f64 {
                    1_usize
                } else if __switch_value == 2.0_f64 {
                    2_usize
                } else if __switch_value == 3.0_f64 {
                    3_usize
                } else if __switch_value == 4.0_f64 {
                    4_usize
                } else if __switch_value == 5.0_f64 {
                    5_usize
                } else if __switch_value == 6.0_f64 {
                    6_usize
                } else if __switch_value == 7.0_f64 {
                    7_usize
                } else if __switch_value == 8.0_f64 {
                    8_usize
                } else if __switch_value == 9.0_f64 {
                    9_usize
                } else if __switch_value == 10.0_f64 {
                    10_usize
                } else if __switch_value == 11.0_f64 {
                    11_usize
                } else if __switch_value == 12.0_f64 {
                    12_usize
                } else if __switch_value == 13.0_f64 {
                    13_usize
                } else {
                    14_usize
                };
                '__flight_switch: {
                    if __flight_case <= 0_usize {
                        lx = ax0;
                        ly = ay0;
                        lz = az0;
                        break '__flight_switch;
                    }
                    if __flight_case <= 1_usize {
                        lx = ax1;
                        ly = ay1;
                        lz = az1;
                        break '__flight_switch;
                    }
                    if __flight_case <= 2_usize {
                        lx = ax2;
                        ly = ay2;
                        lz = az2;
                        break '__flight_switch;
                    }
                    if __flight_case <= 3_usize {
                        lx = bx0;
                        ly = by0;
                        lz = bz0;
                        break '__flight_switch;
                    }
                    if __flight_case <= 4_usize {
                        lx = bx1;
                        ly = by1;
                        lz = bz1;
                        break '__flight_switch;
                    }
                    if __flight_case <= 5_usize {
                        lx = bx2;
                        ly = by2;
                        lz = bz2;
                        break '__flight_switch;
                    }
                    if __flight_case <= 6_usize {
                        lx = ((ay0 * bz0) - (az0 * by0));
                        ly = ((az0 * bx0) - (ax0 * bz0));
                        lz = ((ax0 * by0) - (ay0 * bx0));
                        break '__flight_switch;
                    }
                    if __flight_case <= 7_usize {
                        lx = ((ay0 * bz1) - (az0 * by1));
                        ly = ((az0 * bx1) - (ax0 * bz1));
                        lz = ((ax0 * by1) - (ay0 * bx1));
                        break '__flight_switch;
                    }
                    if __flight_case <= 8_usize {
                        lx = ((ay0 * bz2) - (az0 * by2));
                        ly = ((az0 * bx2) - (ax0 * bz2));
                        lz = ((ax0 * by2) - (ay0 * bx2));
                        break '__flight_switch;
                    }
                    if __flight_case <= 9_usize {
                        lx = ((ay1 * bz0) - (az1 * by0));
                        ly = ((az1 * bx0) - (ax1 * bz0));
                        lz = ((ax1 * by0) - (ay1 * bx0));
                        break '__flight_switch;
                    }
                    if __flight_case <= 10_usize {
                        lx = ((ay1 * bz1) - (az1 * by1));
                        ly = ((az1 * bx1) - (ax1 * bz1));
                        lz = ((ax1 * by1) - (ay1 * bx1));
                        break '__flight_switch;
                    }
                    if __flight_case <= 11_usize {
                        lx = ((ay1 * bz2) - (az1 * by2));
                        ly = ((az1 * bx2) - (ax1 * bz2));
                        lz = ((ax1 * by2) - (ay1 * bx2));
                        break '__flight_switch;
                    }
                    if __flight_case <= 12_usize {
                        lx = ((ay2 * bz0) - (az2 * by0));
                        ly = ((az2 * bx0) - (ax2 * bz0));
                        lz = ((ax2 * by0) - (ay2 * bx0));
                        break '__flight_switch;
                    }
                    if __flight_case <= 13_usize {
                        lx = ((ay2 * bz1) - (az2 * by1));
                        ly = ((az2 * bx1) - (ax2 * bz1));
                        lz = ((ax2 * by1) - (ay2 * bx1));
                        break '__flight_switch;
                    }
                    if __flight_case <= 14_usize {
                        lx = ((ay2 * bz2) - (az2 * by2));
                        ly = ((az2 * bx2) - (ax2 * bz2));
                        lz = ((ax2 * by2) - (ay2 * bx2));
                        break '__flight_switch;
                    }
                }
            }
            let len_sq = (((lx * lx) + (ly * ly)) + (lz * lz));
            if (len_sq < 1e-10_f64) {
                {
                    axis += 1.0;
                    axis
                };
                continue;
            }
            let d = (((tx * lx) + (ty * ly)) + (tz * lz)).abs();
            let p_a = ((((((ax0 * lx) + (ay0 * ly)) + (az0 * lz)).abs() * hax)
                + ((((ax1 * lx) + (ay1 * ly)) + (az1 * lz)).abs() * hay))
                + ((((ax2 * lx) + (ay2 * ly)) + (az2 * lz)).abs() * haz));
            let p_b = ((((((bx0 * lx) + (by0 * ly)) + (bz0 * lz)).abs() * hbx)
                + ((((bx1 * lx) + (by1 * ly)) + (bz1 * lz)).abs() * hby))
                + ((((bx2 * lx) + (by2 * ly)) + (bz2 * lz)).abs() * hbz));
            if (d > (p_a + p_b)) {
                return true;
            }
            {
                axis += 1.0;
                axis
            };
        }
    }
    return false;
}
