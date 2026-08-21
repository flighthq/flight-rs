// @generated from upstream/packages/geometry/src/quaternion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{EulerOrder, Matrix4Like, Quaternion, QuaternionLike, Vector3Like};

// Source: upstream/packages/geometry/src/quaternion.ts:4 (sha256:d3e869870aa92cc03b39d5d18610594862eb043d8345f5a2ce03a06f28c542ab)
pub fn clone_quaternion(source: &QuaternionLike) -> Quaternion {
    return create_quaternion(
        Some(source.x),
        Some(source.y),
        Some(source.z),
        Some(source.w),
    );
}

// Source: upstream/packages/geometry/src/quaternion.ts:15 (sha256:5eefe28f7eaf46a354fe0d7617155c1c05299484671f2a7afcb41eb4cd90ef3c)
pub fn conjugate_quaternion(out: &mut QuaternionLike, source: &QuaternionLike) -> () {
    out.x = (-source.x);
    out.y = (-source.y);
    out.z = (-source.z);
    out.w = source.w;
}

// Source: upstream/packages/geometry/src/quaternion.ts:27 (sha256:e9018db391c9ab2998401d4c88a69fff7dd328d47e683a909b2feb5493e8f97c)
pub fn copy_quaternion(out: &mut QuaternionLike, source: &QuaternionLike) -> () {
    out.x = source.x;
    out.y = source.y;
    out.z = source.z;
    out.w = source.w;
}

// Source: upstream/packages/geometry/src/quaternion.ts:40 (sha256:f863786f474125298a9d24468ba9f54e6bf22260ec7accd2626bccf97e43ddb3)
pub fn create_quaternion(
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
    w: Option<f64>,
) -> Quaternion {
    return create_entity(Some(Quaternion {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        x: (x).clone().unwrap_or(0.0_f64),
        y: (y).clone().unwrap_or(0.0_f64),
        z: (z).clone().unwrap_or(0.0_f64),
        w: (w).clone().unwrap_or(1.0_f64),
    }));
}

// Source: upstream/packages/geometry/src/quaternion.ts:44 (sha256:b98a3df826eadbf196dda80a21801b3a78d38f69401610e009fe29fde4cfbbe9)
pub fn equals_quaternion(a: &Option<QuaternionLike>, b: &Option<QuaternionLike>) -> bool {
    if (a == b) {
        return true;
    }
    if ((a).is_none()) || ((b).is_none()) {
        return false;
    }
    return (((a.as_ref().unwrap().x == b.as_ref().unwrap().x)
        && (a.as_ref().unwrap().y == b.as_ref().unwrap().y))
        && (a.as_ref().unwrap().z == b.as_ref().unwrap().z))
        && (a.as_ref().unwrap().w == b.as_ref().unwrap().w);
}

// Source: upstream/packages/geometry/src/quaternion.ts:57 (sha256:41310dda90cd0338123af0975edeffa0004c9593dd0310e6647ed2ad662d5aa6)
pub fn get_quaternion_angle_between(a: &QuaternionLike, b: &QuaternionLike) -> f64 {
    let dot = (get_quaternion_dot(a, b)).abs();
    return (2.0_f64 * ((1.0_f64).min(dot)).acos());
}

// Source: upstream/packages/geometry/src/quaternion.ts:66 (sha256:6a01cc4dd67cd5aaf4301dfd4ca95150b7c598d0df8becc09a5eb2c3b853408a)
pub fn get_quaternion_dot(a: &QuaternionLike, b: &QuaternionLike) -> f64 {
    return ((((a.x * b.x) + (a.y * b.y)) + (a.z * b.z)) + (a.w * b.w));
}

// Source: upstream/packages/geometry/src/quaternion.ts:78 (sha256:75e4fce0532b1e7a2ac45f93fbf7aab6f78c339481dd3dfed3b54845708dd8be)
pub fn get_quaternion_euler(
    out: &mut Vector3Like,
    source: &QuaternionLike,
    order: Option<EulerOrder>,
) -> () {
    let order = order.unwrap_or("XYZ".to_owned());
    let x = source.x;
    let y = source.y;
    let z = source.z;
    let w = source.w;
    let xx = (x * x);
    let yy = (y * y);
    let zz = (z * z);
    let xy = (x * y);
    let xz = (x * z);
    let yz = (y * z);
    let wx = (w * x);
    let wy = (w * y);
    let wz = (w * z);
    let m00 = (1.0_f64 - (2.0_f64 * (yy + zz)));
    let m01 = (2.0_f64 * (xy - wz));
    let m02 = (2.0_f64 * (xz + wy));
    let m10 = (2.0_f64 * (xy + wz));
    let m11 = (1.0_f64 - (2.0_f64 * (xx + zz)));
    let m12 = (2.0_f64 * (yz - wx));
    let m20 = (2.0_f64 * (xz - wy));
    let m21 = (2.0_f64 * (yz + wx));
    let m22 = (1.0_f64 - (2.0_f64 * (xx + yy)));
    {
        let __switch_value = order;
        let __flight_case = if __switch_value == "XYZ" {
            0_usize
        } else if __switch_value == "XZY" {
            1_usize
        } else if __switch_value == "YXZ" {
            2_usize
        } else if __switch_value == "YZX" {
            3_usize
        } else if __switch_value == "ZXY" {
            4_usize
        } else if __switch_value == "ZYX" {
            5_usize
        } else {
            6_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                {
                    out.y = ((1.0_f64).min((-1.0_f64).max(m02))).asin();
                    if ((m02).abs() < 0.9999999_f64) {
                        {
                            out.x = (-m12).atan2(m22);
                            out.z = (-m01).atan2(m00);
                        }
                    } else {
                        {
                            out.x = (m21).atan2(m11);
                            out.z = 0.0_f64;
                        }
                    }
                    break '__flight_switch;
                }
            }
            if __flight_case <= 1_usize {
                {
                    out.z = ((1.0_f64).min((-1.0_f64).max((-m01)))).asin();
                    if ((m01).abs() < 0.9999999_f64) {
                        {
                            out.x = (m21).atan2(m11);
                            out.y = (m02).atan2(m00);
                        }
                    } else {
                        {
                            out.x = (-m12).atan2(m22);
                            out.y = 0.0_f64;
                        }
                    }
                    break '__flight_switch;
                }
            }
            if __flight_case <= 2_usize {
                {
                    out.x = ((1.0_f64).min((-1.0_f64).max((-m12)))).asin();
                    if ((m12).abs() < 0.9999999_f64) {
                        {
                            out.y = (m02).atan2(m22);
                            out.z = (m10).atan2(m11);
                        }
                    } else {
                        {
                            out.y = (-m20).atan2(m00);
                            out.z = 0.0_f64;
                        }
                    }
                    break '__flight_switch;
                }
            }
            if __flight_case <= 3_usize {
                {
                    out.z = ((1.0_f64).min((-1.0_f64).max(m10))).asin();
                    if ((m10).abs() < 0.9999999_f64) {
                        {
                            out.x = (-m12).atan2(m11);
                            out.y = (-m20).atan2(m00);
                        }
                    } else {
                        {
                            out.x = 0.0_f64;
                            out.y = (m02).atan2(m22);
                        }
                    }
                    break '__flight_switch;
                }
            }
            if __flight_case <= 4_usize {
                {
                    out.x = ((1.0_f64).min((-1.0_f64).max(m21))).asin();
                    if ((m21).abs() < 0.9999999_f64) {
                        {
                            out.y = (-m20).atan2(m22);
                            out.z = (-m01).atan2(m11);
                        }
                    } else {
                        {
                            out.y = 0.0_f64;
                            out.z = (m10).atan2(m00);
                        }
                    }
                    break '__flight_switch;
                }
            }
            if __flight_case <= 5_usize {
                {
                    out.y = ((1.0_f64).min((-1.0_f64).max((-m20)))).asin();
                    if ((m20).abs() < 0.9999999_f64) {
                        {
                            out.x = (m21).atan2(m22);
                            out.z = (m10).atan2(m00);
                        }
                    } else {
                        {
                            out.x = 0.0_f64;
                            out.z = (-m01).atan2(m11);
                        }
                    }
                    break '__flight_switch;
                }
            }
        }
    }
}

// Source: upstream/packages/geometry/src/quaternion.ts:188 (sha256:91acadf36afdcd72ed709ebdd465e29828e50a62204061a38f34260d55b432e0)
pub fn inverse_quaternion(out: &mut QuaternionLike, source: &QuaternionLike) -> () {
    let x = source.x;
    let y = source.y;
    let z = source.z;
    let w = source.w;
    let len_sq = ((((x * x) + (y * y)) + (z * z)) + (w * w));
    if (len_sq == 0.0_f64) {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        out.z = 0.0_f64;
        out.w = 1.0_f64;
        return;
    }
    let inv = (1.0_f64 / len_sq);
    out.x = ((-x) * inv);
    out.y = ((-y) * inv);
    out.z = ((-z) * inv);
    out.w = (w * inv);
}

// Source: upstream/packages/geometry/src/quaternion.ts:214 (sha256:4a9a64808819d22b33d0c2ef23da48654e9c9d021b38cc225f549a2c49277023)
pub fn multiply_quaternion(out: &mut QuaternionLike, a: &QuaternionLike, b: &QuaternionLike) -> () {
    let ax = a.x;
    let ay = a.y;
    let az = a.z;
    let aw = a.w;
    let bx = b.x;
    let by = b.y;
    let bz = b.z;
    let bw = b.w;
    out.x = ((((aw * bx) + (ax * bw)) + (ay * bz)) - (az * by));
    out.y = ((((aw * by) - (ax * bz)) + (ay * bw)) + (az * bx));
    out.z = ((((aw * bz) + (ax * by)) - (ay * bx)) + (az * bw));
    out.w = ((((aw * bw) - (ax * bx)) - (ay * by)) - (az * bz));
}

// Source: upstream/packages/geometry/src/quaternion.ts:240 (sha256:4980eb547d012fbad501eb83ca1e6ec0ea148d9e785297fb8f7c2a5b5cbf4747)
pub fn normalize_quaternion(out: &mut QuaternionLike, source: &QuaternionLike) -> f64 {
    let x = source.x;
    let y = source.y;
    let z = source.z;
    let w = source.w;
    let l = ((((x * x) + (y * y)) + (z * z)) + (w * w)).sqrt();
    if (l != 0.0_f64) {
        let inv = (1.0_f64 / l);
        out.x = (x * inv);
        out.y = (y * inv);
        out.z = (z * inv);
        out.w = (w * inv);
    } else {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        out.z = 0.0_f64;
        out.w = 1.0_f64;
    }
    return l;
}

// Source: upstream/packages/geometry/src/quaternion.ts:269 (sha256:779dcd90f7508bb3e9fa11a5442ac739a648143ad9a3a7455aafdc8e5e56e758)
pub fn rotate_vector3_by_quaternion(
    out: &mut Vector3Like,
    vector: &Vector3Like,
    q: &QuaternionLike,
) -> () {
    let qx = q.x;
    let qy = q.y;
    let qz = q.z;
    let qw = q.w;
    let vx = vector.x;
    let vy = vector.y;
    let vz = vector.z;
    let tx = (2.0_f64 * ((qy * vz) - (qz * vy)));
    let ty = (2.0_f64 * ((qz * vx) - (qx * vz)));
    let tz = (2.0_f64 * ((qx * vy) - (qy * vx)));
    out.x = ((vx + (qw * tx)) + ((qy * tz) - (qz * ty)));
    out.y = ((vy + (qw * ty)) + ((qz * tx) - (qx * tz)));
    out.z = ((vz + (qw * tz)) + ((qx * ty) - (qy * tx)));
}

// Source: upstream/packages/geometry/src/quaternion.ts:297 (sha256:724e1e2124af215fb9562797196edbb91e14863ea5b2f44299c8f406d9a2768d)
pub fn set_quaternion(out: &mut QuaternionLike, x: f64, y: f64, z: f64, w: f64) -> () {
    out.x = x;
    out.y = y;
    out.z = z;
    out.w = w;
}

// Source: upstream/packages/geometry/src/quaternion.ts:310 (sha256:fb289adb432e73f7d2bad263447f99899c3f8e3f7a564f556388ea1d138b4212)
pub fn set_quaternion_from_axis_angle(
    out: &mut QuaternionLike,
    axis: &Vector3Like,
    angle: f64,
) -> () {
    let half = (angle * 0.5_f64);
    let s = (half).sin();
    out.x = (axis.x * s);
    out.y = (axis.y * s);
    out.z = (axis.z * s);
    out.w = (half).cos();
}

// Source: upstream/packages/geometry/src/quaternion.ts:326 (sha256:5116680d0b2e47a7725abcffea7259594abb7f63f02ba75423bd09fe565b6001)
pub fn set_quaternion_from_euler(
    out: &mut QuaternionLike,
    x: f64,
    y: f64,
    z: f64,
    order: Option<EulerOrder>,
) -> () {
    let order = order.unwrap_or("XYZ".to_owned());
    let c1 = (x / 2.0_f64).cos();
    let s1 = (x / 2.0_f64).sin();
    let c2 = (y / 2.0_f64).cos();
    let s2 = (y / 2.0_f64).sin();
    let c3 = (z / 2.0_f64).cos();
    let s3 = (z / 2.0_f64).sin();
    {
        let __switch_value = order;
        let __flight_case = if __switch_value == "XYZ" {
            0_usize
        } else if __switch_value == "XZY" {
            1_usize
        } else if __switch_value == "YXZ" {
            2_usize
        } else if __switch_value == "YZX" {
            3_usize
        } else if __switch_value == "ZXY" {
            4_usize
        } else if __switch_value == "ZYX" {
            5_usize
        } else {
            6_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                out.x = (((s1 * c2) * c3) + ((c1 * s2) * s3));
                out.y = (((c1 * s2) * c3) - ((s1 * c2) * s3));
                out.z = (((c1 * c2) * s3) + ((s1 * s2) * c3));
                out.w = (((c1 * c2) * c3) - ((s1 * s2) * s3));
                break '__flight_switch;
            }
            if __flight_case <= 1_usize {
                out.x = (((s1 * c2) * c3) - ((c1 * s2) * s3));
                out.y = (((c1 * s2) * c3) - ((s1 * c2) * s3));
                out.z = (((c1 * c2) * s3) + ((s1 * s2) * c3));
                out.w = (((c1 * c2) * c3) + ((s1 * s2) * s3));
                break '__flight_switch;
            }
            if __flight_case <= 2_usize {
                out.x = (((s1 * c2) * c3) + ((c1 * s2) * s3));
                out.y = (((c1 * s2) * c3) - ((s1 * c2) * s3));
                out.z = (((c1 * c2) * s3) - ((s1 * s2) * c3));
                out.w = (((c1 * c2) * c3) + ((s1 * s2) * s3));
                break '__flight_switch;
            }
            if __flight_case <= 3_usize {
                out.x = (((s1 * c2) * c3) + ((c1 * s2) * s3));
                out.y = (((c1 * s2) * c3) + ((s1 * c2) * s3));
                out.z = (((c1 * c2) * s3) - ((s1 * s2) * c3));
                out.w = (((c1 * c2) * c3) - ((s1 * s2) * s3));
                break '__flight_switch;
            }
            if __flight_case <= 4_usize {
                out.x = (((s1 * c2) * c3) - ((c1 * s2) * s3));
                out.y = (((c1 * s2) * c3) + ((s1 * c2) * s3));
                out.z = (((c1 * c2) * s3) + ((s1 * s2) * c3));
                out.w = (((c1 * c2) * c3) - ((s1 * s2) * s3));
                break '__flight_switch;
            }
            if __flight_case <= 5_usize {
                out.x = (((s1 * c2) * c3) - ((c1 * s2) * s3));
                out.y = (((c1 * s2) * c3) + ((s1 * c2) * s3));
                out.z = (((c1 * c2) * s3) - ((s1 * s2) * c3));
                out.w = (((c1 * c2) * c3) + ((s1 * s2) * s3));
                break '__flight_switch;
            }
        }
    }
}

// Source: upstream/packages/geometry/src/quaternion.ts:384 (sha256:f70e765892b4db3b92cb527a1dcf137180ee16f24f7f9865c4fc57d76de53d06)
pub fn set_quaternion_from_matrix4(out: &mut QuaternionLike, source: &Matrix4Like) -> () {
    let m00 = (source.m[0.0_f64 as usize] as f64);
    let m10 = (source.m[4.0_f64 as usize] as f64);
    let m20 = (source.m[8.0_f64 as usize] as f64);
    let m01 = (source.m[1.0_f64 as usize] as f64);
    let m11 = (source.m[5.0_f64 as usize] as f64);
    let m21 = (source.m[9.0_f64 as usize] as f64);
    let m02 = (source.m[2.0_f64 as usize] as f64);
    let m12 = (source.m[6.0_f64 as usize] as f64);
    let m22 = (source.m[10.0_f64 as usize] as f64);
    let trace = ((m00 + m11) + m22);
    if (trace > 0.0_f64) {
        let s = (0.5_f64 / (trace + 1.0_f64).sqrt());
        out.w = (0.25_f64 / s);
        out.x = ((m12 - m21) * s);
        out.y = ((m20 - m02) * s);
        out.z = ((m01 - m10) * s);
    } else {
        if (m00 > m11) && (m00 > m22) {
            let s = (2.0_f64 * (((1.0_f64 + m00) - m11) - m22).sqrt());
            out.w = ((m12 - m21) / s);
            out.x = (0.25_f64 * s);
            out.y = ((m10 + m01) / s);
            out.z = ((m20 + m02) / s);
        } else {
            if (m11 > m22) {
                let s = (2.0_f64 * (((1.0_f64 + m11) - m00) - m22).sqrt());
                out.w = ((m20 - m02) / s);
                out.x = ((m10 + m01) / s);
                out.y = (0.25_f64 * s);
                out.z = ((m21 + m12) / s);
            } else {
                let s = (2.0_f64 * (((1.0_f64 + m22) - m00) - m11).sqrt());
                out.w = ((m01 - m10) / s);
                out.x = ((m20 + m02) / s);
                out.y = ((m21 + m12) / s);
                out.z = (0.25_f64 * s);
            }
        }
    }
}

// Source: upstream/packages/geometry/src/quaternion.ts:434 (sha256:ed0a59109c2d673a6dde40ae653052864e33c4e1c08a0de0051613d7a282514d)
pub fn set_quaternion_from_unit_vectors(
    out: &mut QuaternionLike,
    from: &Vector3Like,
    to: &Vector3Like,
) -> () {
    let fx = from.x;
    let fy = from.y;
    let fz = from.z;
    let tx = to.x;
    let ty = to.y;
    let tz = to.z;
    let dot = (((fx * tx) + (fy * ty)) + (fz * tz));
    if (dot > 0.999999_f64) {
        set_quaternion_identity(out);
        return;
    }
    if (dot < (-0.999999_f64)) {
        let mut ax = 1.0_f64;
        let mut ay = 0.0_f64;
        let mut az = 0.0_f64;
        if ((fx).abs() > 0.9_f64) {
            ax = 0.0_f64;
            ay = 1.0_f64;
            az = 0.0_f64;
        }
        let mut px = ((fy * az) - (fz * ay));
        let mut py = ((fz * ax) - (fx * az));
        let mut pz = ((fx * ay) - (fy * ax));
        let p_len = (((px * px) + (py * py)) + (pz * pz)).sqrt();
        px /= p_len;
        py /= p_len;
        pz /= p_len;
        out.x = px;
        out.y = py;
        out.z = pz;
        out.w = 0.0_f64;
        return;
    }
    let cx = ((fy * tz) - (fz * ty));
    let cy = ((fz * tx) - (fx * tz));
    let cz = ((fx * ty) - (fy * tx));
    out.x = cx;
    out.y = cy;
    out.z = cz;
    out.w = (1.0_f64 + dot);
    let inv = (1.0_f64
        / ((((out.x * out.x) + (out.y * out.y)) + (out.z * out.z)) + (out.w * out.w)).sqrt());
    out.x *= inv;
    out.y *= inv;
    out.z *= inv;
    out.w *= inv;
}

// Source: upstream/packages/geometry/src/quaternion.ts:499 (sha256:b96649d5293e8a71fc2bfdf6fb7c6c50b492e877990b251425bc17990c7d5a25)
pub fn set_quaternion_identity(out: &mut QuaternionLike) -> () {
    out.x = 0.0_f64;
    out.y = 0.0_f64;
    out.z = 0.0_f64;
    out.w = 1.0_f64;
}

// Source: upstream/packages/geometry/src/quaternion.ts:519 (sha256:d08a2b5729a716402ff064e0f0eed619a7d28bbeb573ca6fc4a61e82cec8c494)
pub fn set_quaternion_look_rotation(
    out: &mut QuaternionLike,
    forward: &Vector3Like,
    up: &Vector3Like,
) -> () {
    let fx = forward.x;
    let fy = forward.y;
    let fz = forward.z;
    let ux = up.x;
    let uy = up.y;
    let uz = up.z;
    if ((fx == 0.0_f64) && (fy == 0.0_f64)) && (fz == 0.0_f64) {
        set_quaternion_identity(out);
        return;
    }
    let mut rx = ((uy * fz) - (uz * fy));
    let mut ry = ((uz * fx) - (ux * fz));
    let mut rz = ((ux * fy) - (uy * fx));
    let mut r_len = (((rx * rx) + (ry * ry)) + (rz * rz)).sqrt();
    if (r_len == 0.0_f64) {
        if ((fz).abs() < 0.9_f64) {
            rx = (-fy);
            ry = fx;
            rz = 0.0_f64;
        } else {
            rx = 0.0_f64;
            ry = (-fz);
            rz = fy;
        }
        r_len = (((rx * rx) + (ry * ry)) + (rz * rz)).sqrt();
    }
    let r_inv = (1.0_f64 / r_len);
    rx *= r_inv;
    ry *= r_inv;
    rz *= r_inv;
    let cux = ((fy * rz) - (fz * ry));
    let cuy = ((fz * rx) - (fx * rz));
    let cuz = ((fx * ry) - (fy * rx));
    let m00 = rx;
    let m10 = cux;
    let m20 = fx;
    let m01 = ry;
    let m11 = cuy;
    let m21 = fy;
    let m02 = rz;
    let m12 = cuz;
    let m22 = fz;
    let trace = ((m00 + m11) + m22);
    if (trace > 0.0_f64) {
        let s = (0.5_f64 / (trace + 1.0_f64).sqrt());
        out.w = (0.25_f64 / s);
        out.x = ((m12 - m21) * s);
        out.y = ((m20 - m02) * s);
        out.z = ((m01 - m10) * s);
    } else {
        if (m00 > m11) && (m00 > m22) {
            let s = (2.0_f64 * (((1.0_f64 + m00) - m11) - m22).sqrt());
            out.w = ((m12 - m21) / s);
            out.x = (0.25_f64 * s);
            out.y = ((m10 + m01) / s);
            out.z = ((m20 + m02) / s);
        } else {
            if (m11 > m22) {
                let s = (2.0_f64 * (((1.0_f64 + m11) - m00) - m22).sqrt());
                out.w = ((m20 - m02) / s);
                out.x = ((m10 + m01) / s);
                out.y = (0.25_f64 * s);
                out.z = ((m21 + m12) / s);
            } else {
                let s = (2.0_f64 * (((1.0_f64 + m22) - m00) - m11).sqrt());
                out.w = ((m01 - m10) / s);
                out.x = ((m20 + m02) / s);
                out.y = ((m21 + m12) / s);
                out.z = (0.25_f64 * s);
            }
        }
    }
}

// Source: upstream/packages/geometry/src/quaternion.ts:612 (sha256:fa0c91e6eb70912f99ccdb7321c503a9889853bc1722e6045bef8f95c9ce918f)
pub fn slerp_quaternion(
    out: &mut QuaternionLike,
    a: &QuaternionLike,
    b: &QuaternionLike,
    t: f64,
) -> () {
    let ax = a.x;
    let ay = a.y;
    let az = a.z;
    let aw = a.w;
    let mut bx = b.x;
    let mut by = b.y;
    let mut bz = b.z;
    let mut bw = b.w;
    let mut cos_half_theta = ((((ax * bx) + (ay * by)) + (az * bz)) + (aw * bw));
    if (cos_half_theta < 0.0_f64) {
        cos_half_theta = (-cos_half_theta);
        bx = (-bx);
        by = (-by);
        bz = (-bz);
        bw = (-bw);
    }
    let mut scale_a: f64;
    let mut scale_b: f64;
    if (cos_half_theta < 0.999999_f64) {
        let half_theta = (cos_half_theta).acos();
        let sin_half_theta = (half_theta).sin();
        scale_a = (((1.0_f64 - t) * half_theta).sin() / sin_half_theta);
        scale_b = ((t * half_theta).sin() / sin_half_theta);
    } else {
        scale_a = (1.0_f64 - t);
        scale_b = t;
    }
    out.x = ((ax * scale_a) + (bx * scale_b));
    out.y = ((ay * scale_a) + (by * scale_b));
    out.z = ((az * scale_a) + (bz * scale_b));
    out.w = ((aw * scale_a) + (bw * scale_b));
}
