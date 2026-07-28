// @generated from upstream/packages/geometry/src/matrix4.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{acquire_identity_matrix4, acquire_matrix4, release_matrix4};
use flighthq_entity::create_entity;
use flighthq_types::{
    Matrix3Like, Matrix4, Matrix4Like, MatrixLike, QuaternionLike, Vector3Like, Vector4Like,
};

// Source: upstream/packages/geometry/src/matrix4.ts:19 (sha256:865d06bc034f88d3caa3893055c40e0076d57d0140082efb7186018246e20b88)
pub fn append_matrix4(out: &mut Matrix4Like, source: &Matrix4Like, other: &Matrix4Like) -> () {
    multiply_matrix4(out, source, other);
}

// Source: upstream/packages/geometry/src/matrix4.ts:32 (sha256:351bc2aa101aa7d2ccd8ea73cce34058231127124beb9814cfca64a5b5eacb8e)
pub fn append_rotation_matrix4(
    out: &mut Matrix4Like,
    source: &Matrix4Like,
    radians: f64,
    axis: &Vector4Like,
    pivot_point: Option<Vector4Like>,
) -> () {
    let mut m = acquire_identity_matrix4();
    __get_axis_rotation(&mut m, axis.x, axis.y, axis.z, radians);
    if (pivot_point).is_some() {
        let mut t1 = acquire_identity_matrix4();
        let mut t2 = acquire_identity_matrix4();
        {
            let __flight_argument_1 = (t1).clone();
            append_translation_matrix4(
                &mut t1,
                &__flight_argument_1,
                (-pivot_point.as_ref().unwrap().x),
                (-pivot_point.as_ref().unwrap().y),
                (-pivot_point.as_ref().unwrap().z),
            )
        };
        {
            let __flight_argument_1 = (t2).clone();
            append_translation_matrix4(
                &mut t2,
                &__flight_argument_1,
                pivot_point.as_ref().unwrap().x,
                pivot_point.as_ref().unwrap().y,
                pivot_point.as_ref().unwrap().z,
            )
        };
        {
            let __flight_argument_2 = (m).clone();
            multiply_matrix4(&mut m, &t1, &__flight_argument_2)
        };
        {
            let __flight_argument_1 = (m).clone();
            multiply_matrix4(&mut m, &__flight_argument_1, &t2)
        };
        release_matrix4(&t1);
        release_matrix4(&t2);
    }
    append_matrix4(out, source, &m);
    release_matrix4(&m);
}

// Source: upstream/packages/geometry/src/matrix4.ts:67 (sha256:ed74a912d096c6c1bf76f39e8ddec21dd8e43c0203f65927b4b84ccb102ecb7a)
pub fn append_scale_matrix4(
    out: &mut Matrix4Like,
    source: &Matrix4Like,
    x_scale: f64,
    y_scale: f64,
    z_scale: f64,
) -> () {
    let mut m = acquire_matrix4();
    set_matrix4(
        &mut m, x_scale, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, y_scale, 0.0_f64, 0.0_f64, 0.0_f64,
        0.0_f64, z_scale, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64,
    );
    append_matrix4(out, source, &m);
    release_matrix4(&m);
}

// Source: upstream/packages/geometry/src/matrix4.ts:91 (sha256:3d79b9718e16ce47a224eacd11d13f090cd0a19ea0d628fe9314ddc399293a3d)
pub fn append_translation_matrix4(
    out: &mut Matrix4Like,
    source: &Matrix4Like,
    x: f64,
    y: f64,
    z: f64,
) -> () {
    if (out != source) {
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<f32> = ((source.m).clone())
                .iter()
                .map(|value| (*value) as f32)
                .collect();
            out.m[__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
    }
    out.m[12.0_f64 as usize] = ((source.m[12.0_f64 as usize] as f64) + x) as f32;
    out.m[13.0_f64 as usize] = ((source.m[13.0_f64 as usize] as f64) + y) as f32;
    out.m[14.0_f64 as usize] = ((source.m[14.0_f64 as usize] as f64) + z) as f32;
}

// Source: upstream/packages/geometry/src/matrix4.ts:106 (sha256:b52ced646c9da867a0b7a12d8d4d022190949cb38fd0fe5aebb6485bd61f37ba)
pub fn clone_matrix4(source: &Matrix4Like) -> Matrix4 {
    let mut m = create_matrix4(
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None,
    );
    copy_matrix4(&mut m, source);
    return (m).clone();
}

// Source: upstream/packages/geometry/src/matrix4.ts:119 (sha256:10f3e210857e0a20096d28e357f51b98b1726bb7f502fca685f3f7ef52c5397e)
pub fn compose_matrix4(
    out: &mut Matrix4Like,
    position: &Vector3Like,
    rotation: &QuaternionLike,
    scale: &Vector3Like,
) -> () {
    let x = rotation.x;
    let y = rotation.y;
    let z = rotation.z;
    let w = rotation.w;
    let x2 = (x + x);
    let y2 = (y + y);
    let z2 = (z + z);
    let xx = (x * x2);
    let xy = (x * y2);
    let xz = (x * z2);
    let yy = (y * y2);
    let yz = (y * z2);
    let zz = (z * z2);
    let wx = (w * x2);
    let wy = (w * y2);
    let wz = (w * z2);
    let sx = scale.x;
    let sy = scale.y;
    let sz = scale.z;
    out.m[0.0_f64 as usize] = ((1.0_f64 - (yy + zz)) * sx) as f32;
    out.m[1.0_f64 as usize] = ((xy + wz) * sx) as f32;
    out.m[2.0_f64 as usize] = ((xz - wy) * sx) as f32;
    out.m[3.0_f64 as usize] = (0.0_f64) as f32;
    out.m[4.0_f64 as usize] = ((xy - wz) * sy) as f32;
    out.m[5.0_f64 as usize] = ((1.0_f64 - (xx + zz)) * sy) as f32;
    out.m[6.0_f64 as usize] = ((yz + wx) * sy) as f32;
    out.m[7.0_f64 as usize] = (0.0_f64) as f32;
    out.m[8.0_f64 as usize] = ((xz + wy) * sz) as f32;
    out.m[9.0_f64 as usize] = ((yz - wx) * sz) as f32;
    out.m[10.0_f64 as usize] = ((1.0_f64 - (xx + yy)) * sz) as f32;
    out.m[11.0_f64 as usize] = (0.0_f64) as f32;
    out.m[12.0_f64 as usize] = (position.x) as f32;
    out.m[13.0_f64 as usize] = (position.y) as f32;
    out.m[14.0_f64 as usize] = (position.z) as f32;
    out.m[15.0_f64 as usize] = (1.0_f64) as f32;
}

// Source: upstream/packages/geometry/src/matrix4.ts:168 (sha256:8bc109d8f28a03f3fbef671754dde87935f12040ac774ece1328526c3ac4efcb)
pub fn copy_matrix4(out: &mut Matrix4Like, source: &Matrix4Like) -> () {
    {
        let __flight_offset = (0.0_f64) as usize;
        let __flight_values: Vec<f32> = ((source.m).clone())
            .iter()
            .map(|value| (*value) as f32)
            .collect();
        out.m[__flight_offset..__flight_offset + __flight_values.len()]
            .copy_from_slice(&__flight_values);
    };
}

// Source: upstream/packages/geometry/src/matrix4.ts:175 (sha256:2d37d23619df0e1372ab0065171ef74fa40ed1546bd36279234f7cb8b1fa154c)
pub fn copy_matrix4_column_from_vector4(
    out: &mut Matrix4Like,
    column: f64,
    source: &Vector4Like,
) -> () {
    {
        let __switch_value = column;
        let __flight_case = if __switch_value == 0.0_f64 {
            0_usize
        } else if __switch_value == 1.0_f64 {
            1_usize
        } else if __switch_value == 2.0_f64 {
            2_usize
        } else if __switch_value == 3.0_f64 {
            3_usize
        } else {
            4_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                out.m[0.0_f64 as usize] = (source.x) as f32;
                out.m[1.0_f64 as usize] = (source.y) as f32;
                out.m[2.0_f64 as usize] = (source.z) as f32;
                out.m[3.0_f64 as usize] = (source.w) as f32;
                break '__flight_switch;
            }
            if __flight_case <= 1_usize {
                out.m[4.0_f64 as usize] = (source.x) as f32;
                out.m[5.0_f64 as usize] = (source.y) as f32;
                out.m[6.0_f64 as usize] = (source.z) as f32;
                out.m[7.0_f64 as usize] = (source.w) as f32;
                break '__flight_switch;
            }
            if __flight_case <= 2_usize {
                out.m[8.0_f64 as usize] = (source.x) as f32;
                out.m[9.0_f64 as usize] = (source.y) as f32;
                out.m[10.0_f64 as usize] = (source.z) as f32;
                out.m[11.0_f64 as usize] = (source.w) as f32;
                break '__flight_switch;
            }
            if __flight_case <= 3_usize {
                out.m[12.0_f64 as usize] = (source.x) as f32;
                out.m[13.0_f64 as usize] = (source.y) as f32;
                out.m[14.0_f64 as usize] = (source.z) as f32;
                out.m[15.0_f64 as usize] = (source.w) as f32;
                break '__flight_switch;
            }
            if __flight_case <= 4_usize {
                panic!("{}", "generated Flight function threw");
            }
        }
    }
}

// Source: upstream/packages/geometry/src/matrix4.ts:214 (sha256:f4db8f512395f84068f77eac5e6b61b94824e0fc4dce64994b2d3c92bb62e1a4)
pub fn copy_matrix4_column_to_vector4(
    out: &mut Vector4Like,
    column: f64,
    source: &Matrix4Like,
) -> () {
    {
        let __switch_value = column;
        let __flight_case = if __switch_value == 0.0_f64 {
            0_usize
        } else if __switch_value == 1.0_f64 {
            1_usize
        } else if __switch_value == 2.0_f64 {
            2_usize
        } else if __switch_value == 3.0_f64 {
            3_usize
        } else {
            4_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                out.x = (source.m[0.0_f64 as usize] as f64);
                out.y = (source.m[1.0_f64 as usize] as f64);
                out.z = (source.m[2.0_f64 as usize] as f64);
                out.w = (source.m[3.0_f64 as usize] as f64);
                break '__flight_switch;
            }
            if __flight_case <= 1_usize {
                out.x = (source.m[4.0_f64 as usize] as f64);
                out.y = (source.m[5.0_f64 as usize] as f64);
                out.z = (source.m[6.0_f64 as usize] as f64);
                out.w = (source.m[7.0_f64 as usize] as f64);
                break '__flight_switch;
            }
            if __flight_case <= 2_usize {
                out.x = (source.m[8.0_f64 as usize] as f64);
                out.y = (source.m[9.0_f64 as usize] as f64);
                out.z = (source.m[10.0_f64 as usize] as f64);
                out.w = (source.m[11.0_f64 as usize] as f64);
                break '__flight_switch;
            }
            if __flight_case <= 3_usize {
                out.x = (source.m[12.0_f64 as usize] as f64);
                out.y = (source.m[13.0_f64 as usize] as f64);
                out.z = (source.m[14.0_f64 as usize] as f64);
                out.w = (source.m[15.0_f64 as usize] as f64);
                break '__flight_switch;
            }
            if __flight_case <= 4_usize {
                panic!("{}", "generated Flight function threw");
            }
        }
    }
}

// Source: upstream/packages/geometry/src/matrix4.ts:253 (sha256:79675faf59d5311654974729427760155fcfef1f52d6c9e7a7a93dcdb9662901)
pub fn copy_matrix4_row_from_vector4(out: &mut Matrix4Like, row: f64, source: &Vector4Like) -> () {
    {
        let __switch_value = row;
        let __flight_case = if __switch_value == 0.0_f64 {
            0_usize
        } else if __switch_value == 1.0_f64 {
            1_usize
        } else if __switch_value == 2.0_f64 {
            2_usize
        } else if __switch_value == 3.0_f64 {
            3_usize
        } else {
            4_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                out.m[0.0_f64 as usize] = (source.x) as f32;
                out.m[4.0_f64 as usize] = (source.y) as f32;
                out.m[8.0_f64 as usize] = (source.z) as f32;
                out.m[12.0_f64 as usize] = (source.w) as f32;
                break '__flight_switch;
            }
            if __flight_case <= 1_usize {
                out.m[1.0_f64 as usize] = (source.x) as f32;
                out.m[5.0_f64 as usize] = (source.y) as f32;
                out.m[9.0_f64 as usize] = (source.z) as f32;
                out.m[13.0_f64 as usize] = (source.w) as f32;
                break '__flight_switch;
            }
            if __flight_case <= 2_usize {
                out.m[2.0_f64 as usize] = (source.x) as f32;
                out.m[6.0_f64 as usize] = (source.y) as f32;
                out.m[10.0_f64 as usize] = (source.z) as f32;
                out.m[14.0_f64 as usize] = (source.w) as f32;
                break '__flight_switch;
            }
            if __flight_case <= 3_usize {
                out.m[3.0_f64 as usize] = (source.x) as f32;
                out.m[7.0_f64 as usize] = (source.y) as f32;
                out.m[11.0_f64 as usize] = (source.z) as f32;
                out.m[15.0_f64 as usize] = (source.w) as f32;
                break '__flight_switch;
            }
            if __flight_case <= 4_usize {
                panic!("{}", "generated Flight function threw");
            }
        }
    }
}

// Source: upstream/packages/geometry/src/matrix4.ts:292 (sha256:cf84a90a751cd46f656503ee660a45ef268562859daffaa274797a8db6c58ef0)
pub fn copy_matrix4_row_to_vector4(out: &mut Vector4Like, row: f64, source: &Matrix4Like) -> () {
    {
        let __switch_value = row;
        let __flight_case = if __switch_value == 0.0_f64 {
            0_usize
        } else if __switch_value == 1.0_f64 {
            1_usize
        } else if __switch_value == 2.0_f64 {
            2_usize
        } else if __switch_value == 3.0_f64 {
            3_usize
        } else {
            4_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                out.x = (source.m[0.0_f64 as usize] as f64);
                out.y = (source.m[4.0_f64 as usize] as f64);
                out.z = (source.m[8.0_f64 as usize] as f64);
                out.w = (source.m[12.0_f64 as usize] as f64);
                break '__flight_switch;
            }
            if __flight_case <= 1_usize {
                out.x = (source.m[1.0_f64 as usize] as f64);
                out.y = (source.m[5.0_f64 as usize] as f64);
                out.z = (source.m[9.0_f64 as usize] as f64);
                out.w = (source.m[13.0_f64 as usize] as f64);
                break '__flight_switch;
            }
            if __flight_case <= 2_usize {
                out.x = (source.m[2.0_f64 as usize] as f64);
                out.y = (source.m[6.0_f64 as usize] as f64);
                out.z = (source.m[10.0_f64 as usize] as f64);
                out.w = (source.m[14.0_f64 as usize] as f64);
                break '__flight_switch;
            }
            if __flight_case <= 3_usize {
                out.x = (source.m[3.0_f64 as usize] as f64);
                out.y = (source.m[7.0_f64 as usize] as f64);
                out.z = (source.m[11.0_f64 as usize] as f64);
                out.w = (source.m[15.0_f64 as usize] as f64);
                break '__flight_switch;
            }
            if __flight_case <= 4_usize {
                panic!("{}", "generated Flight function threw");
            }
        }
    }
}

// Source: upstream/packages/geometry/src/matrix4.ts:341 (sha256:9e59690d90015c3bd884db0532e3fadd8648577a87ab2365047f5cfa691437d5)
pub fn create_matrix4(
    m00: Option<f64>,
    m01: Option<f64>,
    m02: Option<f64>,
    m03: Option<f64>,
    m10: Option<f64>,
    m11: Option<f64>,
    m12: Option<f64>,
    m13: Option<f64>,
    m20: Option<f64>,
    m21: Option<f64>,
    m22: Option<f64>,
    m23: Option<f64>,
    m30: Option<f64>,
    m31: Option<f64>,
    m32: Option<f64>,
    m33: Option<f64>,
) -> Matrix4 {
    let mut m = ((*__IDENTITY).clone())
        .iter()
        .map(|value| (*value) as f32)
        .collect();
    let mut out: Matrix4 = create_entity(Some(Matrix4 {
        __flight_identity: std::sync::Arc::new(()),
        m: (m).clone(),
    }));
    if (m00).is_some() {
        out.m[0.0_f64 as usize] = (m00.as_ref().unwrap()) as f32;
    }
    if (m01).is_some() {
        out.m[1.0_f64 as usize] = (m01.as_ref().unwrap()) as f32;
    }
    if (m02).is_some() {
        out.m[2.0_f64 as usize] = (m02.as_ref().unwrap()) as f32;
    }
    if (m03).is_some() {
        out.m[3.0_f64 as usize] = (m03.as_ref().unwrap()) as f32;
    }
    if (m10).is_some() {
        out.m[4.0_f64 as usize] = (m10.as_ref().unwrap()) as f32;
    }
    if (m11).is_some() {
        out.m[5.0_f64 as usize] = (m11.as_ref().unwrap()) as f32;
    }
    if (m12).is_some() {
        out.m[6.0_f64 as usize] = (m12.as_ref().unwrap()) as f32;
    }
    if (m13).is_some() {
        out.m[7.0_f64 as usize] = (m13.as_ref().unwrap()) as f32;
    }
    if (m20).is_some() {
        out.m[8.0_f64 as usize] = (m20.as_ref().unwrap()) as f32;
    }
    if (m21).is_some() {
        out.m[9.0_f64 as usize] = (m21.as_ref().unwrap()) as f32;
    }
    if (m22).is_some() {
        out.m[10.0_f64 as usize] = (m22.as_ref().unwrap()) as f32;
    }
    if (m23).is_some() {
        out.m[11.0_f64 as usize] = (m23.as_ref().unwrap()) as f32;
    }
    if (m30).is_some() {
        out.m[12.0_f64 as usize] = (m30.as_ref().unwrap()) as f32;
    }
    if (m31).is_some() {
        out.m[13.0_f64 as usize] = (m31.as_ref().unwrap()) as f32;
    }
    if (m32).is_some() {
        out.m[14.0_f64 as usize] = (m32.as_ref().unwrap()) as f32;
    }
    if (m33).is_some() {
        out.m[15.0_f64 as usize] = (m33.as_ref().unwrap()) as f32;
    }
    return (out).clone();
}

// Source: upstream/packages/geometry/src/matrix4.ts:383 (sha256:5855d2698fc4efea6e23205dcf0ef4631b9be4bc8496d657743c94395e80d701)
pub fn create_matrix4_from2_d(
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    tx: Option<f64>,
    ty: Option<f64>,
) -> Matrix4 {
    let mut out = create_matrix4(
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None,
    );
    set_matrix4_from2_d(
        &mut out,
        a,
        b,
        c,
        d,
        Some((tx).clone().unwrap()),
        Some((ty).clone().unwrap()),
    );
    return (out).clone();
}

// Source: upstream/packages/geometry/src/matrix4.ts:392 (sha256:66163c189727e7f73a6ce04f31adc0d8c3eb2f72118895ff50341aca204b9e3e)
pub fn create_orthographic_matrix4(
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    z_near: f64,
    z_far: f64,
) -> Matrix4 {
    let mut out = create_matrix4(
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None,
    );
    set_orthographic_matrix4(&mut out, left, right, bottom, top, z_near, z_far);
    return (out).clone();
}

// Source: upstream/packages/geometry/src/matrix4.ts:408 (sha256:a2d933d0aaa115238af90cd19802db117dca17a948554c8d2eba4db2c0514875)
pub fn create_perspective_matrix4(fov: f64, aspect: f64, z_near: f64, z_far: f64) -> Matrix4 {
    let mut out = create_matrix4(
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None,
    );
    set_perspective_matrix4(&mut out, fov, aspect, z_near, z_far);
    return (out).clone();
}

// Source: upstream/packages/geometry/src/matrix4.ts:423 (sha256:ca266460444317fd9c9bb302a97444114937baa772db7de6d1968f13579a7496)
pub fn decompose_matrix4(
    out_position: &mut Vector3Like,
    out_rotation: &mut QuaternionLike,
    out_scale: &mut Vector3Like,
    m: &Matrix4Like,
) -> () {
    let m00 = (m.m[0.0_f64 as usize] as f64);
    let m01 = (m.m[1.0_f64 as usize] as f64);
    let m02 = (m.m[2.0_f64 as usize] as f64);
    let m10 = (m.m[4.0_f64 as usize] as f64);
    let m11 = (m.m[5.0_f64 as usize] as f64);
    let m12 = (m.m[6.0_f64 as usize] as f64);
    let m20 = (m.m[8.0_f64 as usize] as f64);
    let m21 = (m.m[9.0_f64 as usize] as f64);
    let m22 = (m.m[10.0_f64 as usize] as f64);
    let tx = (m.m[12.0_f64 as usize] as f64);
    let ty = (m.m[13.0_f64 as usize] as f64);
    let tz = (m.m[14.0_f64 as usize] as f64);
    let mut sx = (((m00 * m00) + (m01 * m01)) + (m02 * m02)).sqrt();
    let sy = (((m10 * m10) + (m11 * m11)) + (m12 * m12)).sqrt();
    let sz = (((m20 * m20) + (m21 * m21)) + (m22 * m22)).sqrt();
    let det = (((m00 * ((m11 * m22) - (m12 * m21))) - (m10 * ((m01 * m22) - (m02 * m21))))
        + (m20 * ((m01 * m12) - (m02 * m11))));
    if (det < 0.0_f64) {
        sx = (-sx);
    }
    out_position.x = tx;
    out_position.y = ty;
    out_position.z = tz;
    out_scale.x = sx;
    out_scale.y = sy;
    out_scale.z = sz;
    let inv_sx = if (sx != 0.0_f64) {
        (1.0_f64 / sx)
    } else {
        0.0_f64
    };
    let inv_sy = if (sy != 0.0_f64) {
        (1.0_f64 / sy)
    } else {
        0.0_f64
    };
    let inv_sz = if (sz != 0.0_f64) {
        (1.0_f64 / sz)
    } else {
        0.0_f64
    };
    let r00 = (m00 * inv_sx);
    let r01 = (m01 * inv_sx);
    let r02 = (m02 * inv_sx);
    let r10 = (m10 * inv_sy);
    let r11 = (m11 * inv_sy);
    let r12 = (m12 * inv_sy);
    let r20 = (m20 * inv_sz);
    let r21 = (m21 * inv_sz);
    let r22 = (m22 * inv_sz);
    let trace = ((r00 + r11) + r22);
    if (trace > 0.0_f64) {
        let s = (0.5_f64 / (trace + 1.0_f64).sqrt());
        out_rotation.w = (0.25_f64 / s);
        out_rotation.x = ((r12 - r21) * s);
        out_rotation.y = ((r20 - r02) * s);
        out_rotation.z = ((r01 - r10) * s);
    } else {
        if ((r00 > r11) && (r00 > r22)) {
            let s = (2.0_f64 * (((1.0_f64 + r00) - r11) - r22).sqrt());
            out_rotation.w = ((r12 - r21) / s);
            out_rotation.x = (0.25_f64 * s);
            out_rotation.y = ((r10 + r01) / s);
            out_rotation.z = ((r20 + r02) / s);
        } else {
            if (r11 > r22) {
                let s = (2.0_f64 * (((1.0_f64 + r11) - r00) - r22).sqrt());
                out_rotation.w = ((r20 - r02) / s);
                out_rotation.x = ((r10 + r01) / s);
                out_rotation.y = (0.25_f64 * s);
                out_rotation.z = ((r21 + r12) / s);
            } else {
                let s = (2.0_f64 * (((1.0_f64 + r22) - r00) - r11).sqrt());
                out_rotation.w = ((r01 - r10) / s);
                out_rotation.x = ((r20 + r02) / s);
                out_rotation.y = ((r21 + r12) / s);
                out_rotation.z = (0.25_f64 * s);
            }
        }
    }
}

// Source: upstream/packages/geometry/src/matrix4.ts:504 (sha256:c1382d0de754595f5541a2110d00ac54367c283deb5af18a593051a1a92b1db9)
pub fn equals_matrix4(a: Option<Matrix4Like>, b: Option<Matrix4Like>) -> bool {
    if (a == b) {
        return true;
    }
    if ((a).is_none() || (b).is_none()) {
        return false;
    }
    {
        let mut i = 0.0_f64;
        while (i < 16.0_f64) {
            if ((a.as_ref().unwrap().m[i as usize] as f64)
                != (b.as_ref().unwrap().m[i as usize] as f64))
            {
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

// Source: upstream/packages/geometry/src/matrix4.ts:516 (sha256:3985ea0b2d547d641f6edc07fc5f10e39f2ffd75b421c2db6d92df3b7f0588f5)
pub fn get_matrix4_determinant(source: &Matrix4Like) -> f64 {
    return (1.0_f64
        * (((((((((source.m[0.0_f64 as usize] as f64)
            * (source.m[5.0_f64 as usize] as f64))
            - ((source.m[4.0_f64 as usize] as f64) * (source.m[1.0_f64 as usize] as f64)))
            * (((source.m[10.0_f64 as usize] as f64)
                * (source.m[15.0_f64 as usize] as f64))
                - ((source.m[14.0_f64 as usize] as f64)
                    * (source.m[11.0_f64 as usize] as f64))))
            - ((((source.m[0.0_f64 as usize] as f64)
                * (source.m[9.0_f64 as usize] as f64))
                - ((source.m[8.0_f64 as usize] as f64)
                    * (source.m[1.0_f64 as usize] as f64)))
                * (((source.m[6.0_f64 as usize] as f64)
                    * (source.m[15.0_f64 as usize] as f64))
                    - ((source.m[14.0_f64 as usize] as f64)
                        * (source.m[7.0_f64 as usize] as f64)))))
            + ((((source.m[0.0_f64 as usize] as f64)
                * (source.m[13.0_f64 as usize] as f64))
                - ((source.m[12.0_f64 as usize] as f64)
                    * (source.m[1.0_f64 as usize] as f64)))
                * (((source.m[6.0_f64 as usize] as f64)
                    * (source.m[11.0_f64 as usize] as f64))
                    - ((source.m[10.0_f64 as usize] as f64)
                        * (source.m[7.0_f64 as usize] as f64)))))
            + ((((source.m[4.0_f64 as usize] as f64) * (source.m[9.0_f64 as usize] as f64))
                - ((source.m[8.0_f64 as usize] as f64)
                    * (source.m[5.0_f64 as usize] as f64)))
                * (((source.m[2.0_f64 as usize] as f64)
                    * (source.m[15.0_f64 as usize] as f64))
                    - ((source.m[14.0_f64 as usize] as f64)
                        * (source.m[3.0_f64 as usize] as f64)))))
            - ((((source.m[4.0_f64 as usize] as f64) * (source.m[13.0_f64 as usize] as f64))
                - ((source.m[12.0_f64 as usize] as f64)
                    * (source.m[5.0_f64 as usize] as f64)))
                * (((source.m[2.0_f64 as usize] as f64)
                    * (source.m[11.0_f64 as usize] as f64))
                    - ((source.m[10.0_f64 as usize] as f64)
                        * (source.m[3.0_f64 as usize] as f64)))))
            + ((((source.m[8.0_f64 as usize] as f64) * (source.m[13.0_f64 as usize] as f64))
                - ((source.m[12.0_f64 as usize] as f64) * (source.m[9.0_f64 as usize] as f64)))
                * (((source.m[2.0_f64 as usize] as f64) * (source.m[7.0_f64 as usize] as f64))
                    - ((source.m[6.0_f64 as usize] as f64)
                        * (source.m[3.0_f64 as usize] as f64))))));
}

// Source: upstream/packages/geometry/src/matrix4.ts:529 (sha256:0fe800a10090a16c74e01d43b54868d8fdb7c673f44fc682fa485183d1c22af9)
pub fn get_matrix4_element(source: &Matrix4Like, row: f64, column: f64) -> f64 {
    return (source.m[((column * 4.0_f64) + row) as usize] as f64);
}

// Source: upstream/packages/geometry/src/matrix4.ts:533 (sha256:8e73623ebcefe8467aac5769218db3adf2ac96ce095d30568d444d1e5c379cd5)
pub fn get_matrix4_position(out: &mut Vector3Like, source: &Matrix4Like) -> () {
    out.x = (source.m[12.0_f64 as usize] as f64);
    out.y = (source.m[13.0_f64 as usize] as f64);
    out.z = (source.m[14.0_f64 as usize] as f64);
}

// Source: upstream/packages/geometry/src/matrix4.ts:543 (sha256:e1b684ab54b9924b47bafc999727b40e310f521d6f5f1bd9cc3b370f646f2cf2)
pub fn interpolate_matrix4(out: &mut Matrix4Like, a: &Matrix4Like, b: &Matrix4Like, t: f64) -> () {
    {
        let mut i = 0.0_f64;
        while (i < 16.0_f64) {
            out.m[i as usize] = ((a.m[i as usize] as f64)
                + (((b.m[i as usize] as f64) - (a.m[i as usize] as f64)) * t))
                as f32;
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/geometry/src/matrix4.ts:560 (sha256:12310a9f6d3814b75af62b9a8ecef308206aff18bea83aff648660084c8ad30f)
pub fn inverse_matrix4(out: &mut Matrix4Like, source: &Matrix4Like) -> bool {
    let mut d = get_matrix4_determinant(source);
    let eps = 0.000001_f64;
    let invertable = ((d).abs() > eps);
    if (!invertable) {
        out.m.fill((f64::NAN) as f32);
        return false;
    }
    d = (1.0_f64 / d);
    let m11 = (source.m[0.0_f64 as usize] as f64);
    let m21 = (source.m[4.0_f64 as usize] as f64);
    let m31 = (source.m[8.0_f64 as usize] as f64);
    let m41 = (source.m[12.0_f64 as usize] as f64);
    let m12 = (source.m[1.0_f64 as usize] as f64);
    let m22 = (source.m[5.0_f64 as usize] as f64);
    let m32 = (source.m[9.0_f64 as usize] as f64);
    let m42 = (source.m[13.0_f64 as usize] as f64);
    let m13 = (source.m[2.0_f64 as usize] as f64);
    let m23 = (source.m[6.0_f64 as usize] as f64);
    let m33 = (source.m[10.0_f64 as usize] as f64);
    let m43 = (source.m[14.0_f64 as usize] as f64);
    let m14 = (source.m[3.0_f64 as usize] as f64);
    let m24 = (source.m[7.0_f64 as usize] as f64);
    let m34 = (source.m[11.0_f64 as usize] as f64);
    let m44 = (source.m[15.0_f64 as usize] as f64);
    out.m[0.0_f64 as usize] = (d
        * (((m22 * ((m33 * m44) - (m43 * m34))) - (m32 * ((m23 * m44) - (m43 * m24))))
            + (m42 * ((m23 * m34) - (m33 * m24))))) as f32;
    out.m[1.0_f64 as usize] = ((-d)
        * (((m12 * ((m33 * m44) - (m43 * m34))) - (m32 * ((m13 * m44) - (m43 * m14))))
            + (m42 * ((m13 * m34) - (m33 * m14))))) as f32;
    out.m[2.0_f64 as usize] = (d
        * (((m12 * ((m23 * m44) - (m43 * m24))) - (m22 * ((m13 * m44) - (m43 * m14))))
            + (m42 * ((m13 * m24) - (m23 * m14))))) as f32;
    out.m[3.0_f64 as usize] = ((-d)
        * (((m12 * ((m23 * m34) - (m33 * m24))) - (m22 * ((m13 * m34) - (m33 * m14))))
            + (m32 * ((m13 * m24) - (m23 * m14))))) as f32;
    out.m[4.0_f64 as usize] = ((-d)
        * (((m21 * ((m33 * m44) - (m43 * m34))) - (m31 * ((m23 * m44) - (m43 * m24))))
            + (m41 * ((m23 * m34) - (m33 * m24))))) as f32;
    out.m[5.0_f64 as usize] = (d
        * (((m11 * ((m33 * m44) - (m43 * m34))) - (m31 * ((m13 * m44) - (m43 * m14))))
            + (m41 * ((m13 * m34) - (m33 * m14))))) as f32;
    out.m[6.0_f64 as usize] = ((-d)
        * (((m11 * ((m23 * m44) - (m43 * m24))) - (m21 * ((m13 * m44) - (m43 * m14))))
            + (m41 * ((m13 * m24) - (m23 * m14))))) as f32;
    out.m[7.0_f64 as usize] = (d
        * (((m11 * ((m23 * m34) - (m33 * m24))) - (m21 * ((m13 * m34) - (m33 * m14))))
            + (m31 * ((m13 * m24) - (m23 * m14))))) as f32;
    out.m[8.0_f64 as usize] = (d
        * (((m21 * ((m32 * m44) - (m42 * m34))) - (m31 * ((m22 * m44) - (m42 * m24))))
            + (m41 * ((m22 * m34) - (m32 * m24))))) as f32;
    out.m[9.0_f64 as usize] = ((-d)
        * (((m11 * ((m32 * m44) - (m42 * m34))) - (m31 * ((m12 * m44) - (m42 * m14))))
            + (m41 * ((m12 * m34) - (m32 * m14))))) as f32;
    out.m[10.0_f64 as usize] = (d
        * (((m11 * ((m22 * m44) - (m42 * m24))) - (m21 * ((m12 * m44) - (m42 * m14))))
            + (m41 * ((m12 * m24) - (m22 * m14))))) as f32;
    out.m[11.0_f64 as usize] = ((-d)
        * (((m11 * ((m22 * m34) - (m32 * m24))) - (m21 * ((m12 * m34) - (m32 * m14))))
            + (m31 * ((m12 * m24) - (m22 * m14))))) as f32;
    out.m[12.0_f64 as usize] = ((-d)
        * (((m21 * ((m32 * m43) - (m42 * m33))) - (m31 * ((m22 * m43) - (m42 * m23))))
            + (m41 * ((m22 * m33) - (m32 * m23))))) as f32;
    out.m[13.0_f64 as usize] = (d
        * (((m11 * ((m32 * m43) - (m42 * m33))) - (m31 * ((m12 * m43) - (m42 * m13))))
            + (m41 * ((m12 * m33) - (m32 * m13))))) as f32;
    out.m[14.0_f64 as usize] = ((-d)
        * (((m11 * ((m22 * m43) - (m42 * m23))) - (m21 * ((m12 * m43) - (m42 * m13))))
            + (m41 * ((m12 * m23) - (m22 * m13))))) as f32;
    out.m[15.0_f64 as usize] = (d
        * (((m11 * ((m22 * m33) - (m32 * m23))) - (m21 * ((m12 * m33) - (m32 * m13))))
            + (m31 * ((m12 * m23) - (m22 * m13))))) as f32;
    return invertable;
}

// Source: upstream/packages/geometry/src/matrix4.ts:622 (sha256:3207d97bd3e73661af4da207b0af5e089a1778a2183292872d19dbf8818543b5)
pub fn is_affine_matrix4(source: &Matrix4Like) -> bool {
    return (((((source.m[3.0_f64 as usize] as f64) == 0.0_f64)
        && ((source.m[7.0_f64 as usize] as f64) == 0.0_f64))
        && ((source.m[11.0_f64 as usize] as f64) == 0.0_f64))
        && ((source.m[15.0_f64 as usize] as f64) == 1.0_f64));
}

// Source: upstream/packages/geometry/src/matrix4.ts:630 (sha256:bf6e9be9fb07969edf7ed725847e53c54509aa3656e771519aece1435e39cb43)
pub fn matrix4_transform_point(
    out: &mut Vector3Like,
    source: &Matrix4Like,
    point: &Vector3Like,
) -> () {
    let x = point.x;
    let y = point.y;
    let z = point.z;
    out.x = ((((x * (source.m[0.0_f64 as usize] as f64))
        + (y * (source.m[4.0_f64 as usize] as f64)))
        + (z * (source.m[8.0_f64 as usize] as f64)))
        + (source.m[12.0_f64 as usize] as f64));
    out.y = ((((x * (source.m[1.0_f64 as usize] as f64))
        + (y * (source.m[5.0_f64 as usize] as f64)))
        + (z * (source.m[9.0_f64 as usize] as f64)))
        + (source.m[13.0_f64 as usize] as f64));
    out.z = ((((x * (source.m[2.0_f64 as usize] as f64))
        + (y * (source.m[6.0_f64 as usize] as f64)))
        + (z * (source.m[10.0_f64 as usize] as f64)))
        + (source.m[14.0_f64 as usize] as f64));
}

// Source: upstream/packages/geometry/src/matrix4.ts:647 (sha256:d4e27fcd17ca49936843e2b9ecc3d1a786c9b1dacb31ba13178f3fb79098fdb1)
pub fn matrix4_transform_vector(
    out: &mut Vector4Like,
    source: &Matrix4Like,
    vector: &Vector4Like,
) -> () {
    let x = vector.x;
    let y = vector.y;
    let z = vector.z;
    let w = vector.w;
    out.x = ((((x * (source.m[0.0_f64 as usize] as f64))
        + (y * (source.m[4.0_f64 as usize] as f64)))
        + (z * (source.m[8.0_f64 as usize] as f64)))
        + (w * (source.m[12.0_f64 as usize] as f64)));
    out.y = ((((x * (source.m[1.0_f64 as usize] as f64))
        + (y * (source.m[5.0_f64 as usize] as f64)))
        + (z * (source.m[9.0_f64 as usize] as f64)))
        + (w * (source.m[13.0_f64 as usize] as f64)));
    out.z = ((((x * (source.m[2.0_f64 as usize] as f64))
        + (y * (source.m[6.0_f64 as usize] as f64)))
        + (z * (source.m[10.0_f64 as usize] as f64)))
        + (w * (source.m[14.0_f64 as usize] as f64)));
    out.w = ((((x * (source.m[3.0_f64 as usize] as f64))
        + (y * (source.m[7.0_f64 as usize] as f64)))
        + (z * (source.m[11.0_f64 as usize] as f64)))
        + (w * (source.m[15.0_f64 as usize] as f64)));
}

// Source: upstream/packages/geometry/src/matrix4.ts:666 (sha256:8410ea4d0a7155dfdbc1d5c82940acc7651926cd71fb1003363a2b279ad17ed2)
pub fn matrix4_transform_vectors(
    out: &mut Vec<f32>,
    source: &Matrix4Like,
    vectors: &Vec<f32>,
) -> () {
    let mut i = 0.0_f64;
    let mut x: f64;
    let mut y: f64;
    let mut z: f64;
    while ((i + 3.0_f64) <= (vectors.len() as f64)) {
        x = (vectors[i as usize] as f64);
        y = (vectors[(i + 1.0_f64) as usize] as f64);
        z = (vectors[(i + 2.0_f64) as usize] as f64);
        out[i as usize] = ((((x * (source.m[0.0_f64 as usize] as f64))
            + (y * (source.m[4.0_f64 as usize] as f64)))
            + (z * (source.m[8.0_f64 as usize] as f64)))
            + (source.m[12.0_f64 as usize] as f64)) as f32;
        out[(i + 1.0_f64) as usize] = ((((x * (source.m[1.0_f64 as usize] as f64))
            + (y * (source.m[5.0_f64 as usize] as f64)))
            + (z * (source.m[9.0_f64 as usize] as f64)))
            + (source.m[13.0_f64 as usize] as f64)) as f32;
        out[(i + 2.0_f64) as usize] = ((((x * (source.m[2.0_f64 as usize] as f64))
            + (y * (source.m[6.0_f64 as usize] as f64)))
            + (z * (source.m[10.0_f64 as usize] as f64)))
            + (source.m[14.0_f64 as usize] as f64)) as f32;
        i += 3.0_f64;
    }
}

// Source: upstream/packages/geometry/src/matrix4.ts:693 (sha256:354d63d5441985132b85c4d8aa8bb2cff90de3192f6c14a8518c46aad70c9b72)
pub fn multiply_matrix4(out: &mut Matrix4Like, a: &Matrix4Like, b: &Matrix4Like) -> () {
    let m111 = (a.m[0.0_f64 as usize] as f64);
    let m121 = (a.m[4.0_f64 as usize] as f64);
    let m131 = (a.m[8.0_f64 as usize] as f64);
    let m141 = (a.m[12.0_f64 as usize] as f64);
    let m112 = (a.m[1.0_f64 as usize] as f64);
    let m122 = (a.m[5.0_f64 as usize] as f64);
    let m132 = (a.m[9.0_f64 as usize] as f64);
    let m142 = (a.m[13.0_f64 as usize] as f64);
    let m113 = (a.m[2.0_f64 as usize] as f64);
    let m123 = (a.m[6.0_f64 as usize] as f64);
    let m133 = (a.m[10.0_f64 as usize] as f64);
    let m143 = (a.m[14.0_f64 as usize] as f64);
    let m114 = (a.m[3.0_f64 as usize] as f64);
    let m124 = (a.m[7.0_f64 as usize] as f64);
    let m134 = (a.m[11.0_f64 as usize] as f64);
    let m144 = (a.m[15.0_f64 as usize] as f64);
    let m211 = (b.m[0.0_f64 as usize] as f64);
    let m221 = (b.m[4.0_f64 as usize] as f64);
    let m231 = (b.m[8.0_f64 as usize] as f64);
    let m241 = (b.m[12.0_f64 as usize] as f64);
    let m212 = (b.m[1.0_f64 as usize] as f64);
    let m222 = (b.m[5.0_f64 as usize] as f64);
    let m232 = (b.m[9.0_f64 as usize] as f64);
    let m242 = (b.m[13.0_f64 as usize] as f64);
    let m213 = (b.m[2.0_f64 as usize] as f64);
    let m223 = (b.m[6.0_f64 as usize] as f64);
    let m233 = (b.m[10.0_f64 as usize] as f64);
    let m243 = (b.m[14.0_f64 as usize] as f64);
    let m214 = (b.m[3.0_f64 as usize] as f64);
    let m224 = (b.m[7.0_f64 as usize] as f64);
    let m234 = (b.m[11.0_f64 as usize] as f64);
    let m244 = (b.m[15.0_f64 as usize] as f64);
    out.m[0.0_f64 as usize] =
        ((((m211 * m111) + (m212 * m121)) + (m213 * m131)) + (m214 * m141)) as f32;
    out.m[1.0_f64 as usize] =
        ((((m211 * m112) + (m212 * m122)) + (m213 * m132)) + (m214 * m142)) as f32;
    out.m[2.0_f64 as usize] =
        ((((m211 * m113) + (m212 * m123)) + (m213 * m133)) + (m214 * m143)) as f32;
    out.m[3.0_f64 as usize] =
        ((((m211 * m114) + (m212 * m124)) + (m213 * m134)) + (m214 * m144)) as f32;
    out.m[4.0_f64 as usize] =
        ((((m221 * m111) + (m222 * m121)) + (m223 * m131)) + (m224 * m141)) as f32;
    out.m[5.0_f64 as usize] =
        ((((m221 * m112) + (m222 * m122)) + (m223 * m132)) + (m224 * m142)) as f32;
    out.m[6.0_f64 as usize] =
        ((((m221 * m113) + (m222 * m123)) + (m223 * m133)) + (m224 * m143)) as f32;
    out.m[7.0_f64 as usize] =
        ((((m221 * m114) + (m222 * m124)) + (m223 * m134)) + (m224 * m144)) as f32;
    out.m[8.0_f64 as usize] =
        ((((m231 * m111) + (m232 * m121)) + (m233 * m131)) + (m234 * m141)) as f32;
    out.m[9.0_f64 as usize] =
        ((((m231 * m112) + (m232 * m122)) + (m233 * m132)) + (m234 * m142)) as f32;
    out.m[10.0_f64 as usize] =
        ((((m231 * m113) + (m232 * m123)) + (m233 * m133)) + (m234 * m143)) as f32;
    out.m[11.0_f64 as usize] =
        ((((m231 * m114) + (m232 * m124)) + (m233 * m134)) + (m234 * m144)) as f32;
    out.m[12.0_f64 as usize] =
        ((((m241 * m111) + (m242 * m121)) + (m243 * m131)) + (m244 * m141)) as f32;
    out.m[13.0_f64 as usize] =
        ((((m241 * m112) + (m242 * m122)) + (m243 * m132)) + (m244 * m142)) as f32;
    out.m[14.0_f64 as usize] =
        ((((m241 * m113) + (m242 * m123)) + (m243 * m133)) + (m244 * m143)) as f32;
    out.m[15.0_f64 as usize] =
        ((((m241 * m114) + (m242 * m124)) + (m243 * m134)) + (m244 * m144)) as f32;
}

// Source: upstream/packages/geometry/src/matrix4.ts:758 (sha256:d998c5535ef128e112ffbc4f84dc5a3e105d5a8ad1965b019c694da7b47ebd55)
pub fn prepend_matrix4(out: &mut Matrix4Like, source: &Matrix4Like, other: &Matrix4Like) -> () {
    multiply_matrix4(out, other, source);
}

// Source: upstream/packages/geometry/src/matrix4.ts:771 (sha256:435eeae50a465daf754f65a9e66d729317bcd878346ff71bb568b1a3d868cc53)
pub fn prepend_rotation_matrix4(
    out: &mut Matrix4Like,
    source: &Matrix4Like,
    radians: f64,
    axis: &Vector4Like,
    pivot_point: Option<Vector4Like>,
) -> () {
    let mut m = acquire_identity_matrix4();
    __get_axis_rotation(&mut m, axis.x, axis.y, axis.z, radians);
    if (pivot_point).is_some() {
        let mut t1 = acquire_identity_matrix4();
        let mut t2 = acquire_identity_matrix4();
        {
            let __flight_argument_1 = (t1).clone();
            append_translation_matrix4(
                &mut t1,
                &__flight_argument_1,
                (-pivot_point.as_ref().unwrap().x),
                (-pivot_point.as_ref().unwrap().y),
                (-pivot_point.as_ref().unwrap().z),
            )
        };
        {
            let __flight_argument_1 = (t2).clone();
            append_translation_matrix4(
                &mut t2,
                &__flight_argument_1,
                pivot_point.as_ref().unwrap().x,
                pivot_point.as_ref().unwrap().y,
                pivot_point.as_ref().unwrap().z,
            )
        };
        {
            let __flight_argument_1 = (m).clone();
            multiply_matrix4(&mut m, &__flight_argument_1, &t1)
        };
        {
            let __flight_argument_2 = (m).clone();
            multiply_matrix4(&mut m, &t2, &__flight_argument_2)
        };
        release_matrix4(&t1);
        release_matrix4(&t2);
    }
    prepend_matrix4(out, source, &m);
    release_matrix4(&m);
}

// Source: upstream/packages/geometry/src/matrix4.ts:807 (sha256:973ed3dcdad2b556811cc87c5fe58b88688489df6865f2658a46e790ef885fbf)
pub fn prepend_scale_matrix4(
    out: &mut Matrix4Like,
    source: &Matrix4Like,
    x_scale: f64,
    y_scale: f64,
    z_scale: f64,
) -> () {
    let mut m = acquire_matrix4();
    set_matrix4(
        &mut m, x_scale, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, y_scale, 0.0_f64, 0.0_f64, 0.0_f64,
        0.0_f64, z_scale, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64,
    );
    prepend_matrix4(out, source, &m);
    release_matrix4(&m);
}

// Source: upstream/packages/geometry/src/matrix4.ts:826 (sha256:c4dde97ea5663ea53ef854b3f9bb50b8670b49e6e11d7723471217f9d7c5c843)
pub fn prepend_translation_matrix4(
    out: &mut Matrix4Like,
    source: &Matrix4Like,
    x: f64,
    y: f64,
    z: f64,
) -> () {
    let mut m = acquire_identity_matrix4();
    {
        let __flight_argument_1 = (m).clone();
        translate_matrix4(&mut m, &__flight_argument_1, x, y, z)
    };
    multiply_matrix4(out, &m, source);
    release_matrix4(&m);
}

// Source: upstream/packages/geometry/src/matrix4.ts:847 (sha256:584c475d602ed07057c04f6c3c9e9cc3b0e75ef5a14559ae76823d52d9e79c92)
pub fn rotate_matrix4(
    out: &mut Matrix4Like,
    source: &Matrix4Like,
    axis: &Vector3Like,
    radians: f64,
) -> () {
    let mut m = acquire_identity_matrix4();
    __get_axis_rotation(&mut m, axis.x, axis.y, axis.z, radians);
    multiply_matrix4(out, source, &m);
    release_matrix4(&m);
}

// Source: upstream/packages/geometry/src/matrix4.ts:864 (sha256:cca03af80f89d890f14da808286960c4d9bfc8d2cb751bbcc530d8f7a348efef)
pub fn scale_matrix4(out: &mut Matrix4Like, source: &Matrix4Like, sx: f64, sy: f64, sz: f64) -> () {
    if (out != source) {
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<f32> = ((source.m).clone())
                .iter()
                .map(|value| (*value) as f32)
                .collect();
            out.m[__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
    }
    if (sx != 1.0_f64) {
        out.m[0.0_f64 as usize] = ((source.m[0.0_f64 as usize] as f64) * sx) as f32;
        out.m[4.0_f64 as usize] = ((source.m[4.0_f64 as usize] as f64) * sx) as f32;
        out.m[8.0_f64 as usize] = ((source.m[8.0_f64 as usize] as f64) * sx) as f32;
    }
    if (sy != 1.0_f64) {
        out.m[1.0_f64 as usize] = ((source.m[1.0_f64 as usize] as f64) * sy) as f32;
        out.m[5.0_f64 as usize] = ((source.m[5.0_f64 as usize] as f64) * sy) as f32;
        out.m[9.0_f64 as usize] = ((source.m[9.0_f64 as usize] as f64) * sy) as f32;
    }
    if (sz != 1.0_f64) {
        out.m[2.0_f64 as usize] = ((source.m[2.0_f64 as usize] as f64) * sz) as f32;
        out.m[6.0_f64 as usize] = ((source.m[6.0_f64 as usize] as f64) * sz) as f32;
        out.m[10.0_f64 as usize] = ((source.m[10.0_f64 as usize] as f64) * sz) as f32;
    }
}

// Source: upstream/packages/geometry/src/matrix4.ts:895 (sha256:8e826b0ffe76d76e6eb63ddeff558777bf51dd962ef3c280661a27f8f69dccb1)
pub fn set_matrix4(
    out: &mut Matrix4Like,
    m00: f64,
    m01: f64,
    m02: f64,
    m03: f64,
    m10: f64,
    m11: f64,
    m12: f64,
    m13: f64,
    m20: f64,
    m21: f64,
    m22: f64,
    m23: f64,
    m30: f64,
    m31: f64,
    m32: f64,
    m33: f64,
) -> () {
    out.m[0.0_f64 as usize] = (m00) as f32;
    out.m[1.0_f64 as usize] = (m01) as f32;
    out.m[2.0_f64 as usize] = (m02) as f32;
    out.m[3.0_f64 as usize] = (m03) as f32;
    out.m[4.0_f64 as usize] = (m10) as f32;
    out.m[5.0_f64 as usize] = (m11) as f32;
    out.m[6.0_f64 as usize] = (m12) as f32;
    out.m[7.0_f64 as usize] = (m13) as f32;
    out.m[8.0_f64 as usize] = (m20) as f32;
    out.m[9.0_f64 as usize] = (m21) as f32;
    out.m[10.0_f64 as usize] = (m22) as f32;
    out.m[11.0_f64 as usize] = (m23) as f32;
    out.m[12.0_f64 as usize] = (m30) as f32;
    out.m[13.0_f64 as usize] = (m31) as f32;
    out.m[14.0_f64 as usize] = (m32) as f32;
    out.m[15.0_f64 as usize] = (m33) as f32;
}

// Source: upstream/packages/geometry/src/matrix4.ts:933 (sha256:05f686ac823d64efed7bbc264d3fb184140d23d901a72f5a9c32574357852802)
pub fn set_matrix4_element(out: &mut Matrix4Like, row: f64, column: f64, value: f64) -> () {
    out.m[((column * 4.0_f64) + row) as usize] = (value) as f32;
}

// Source: upstream/packages/geometry/src/matrix4.ts:940 (sha256:775e7a7c13ee13b6ec2848362448e970b8b04bc4283b02bcb0aad41ce5d10f4e)
pub fn set_matrix4_from2_d(
    out: &mut Matrix4Like,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    mut tx: Option<f64>,
    mut ty: Option<f64>,
) -> () {
    tx = Some((tx).unwrap_or(0.0_f64));
    ty = Some((ty).unwrap_or(0.0_f64));
    out.m[0.0_f64 as usize] = (a) as f32;
    out.m[1.0_f64 as usize] = (b) as f32;
    out.m[2.0_f64 as usize] = (0.0_f64) as f32;
    out.m[3.0_f64 as usize] = (0.0_f64) as f32;
    out.m[4.0_f64 as usize] = (c) as f32;
    out.m[5.0_f64 as usize] = (d) as f32;
    out.m[6.0_f64 as usize] = (0.0_f64) as f32;
    out.m[7.0_f64 as usize] = (0.0_f64) as f32;
    out.m[8.0_f64 as usize] = (0.0_f64) as f32;
    out.m[9.0_f64 as usize] = (0.0_f64) as f32;
    out.m[10.0_f64 as usize] = (1.0_f64) as f32;
    out.m[11.0_f64 as usize] = (0.0_f64) as f32;
    out.m[12.0_f64 as usize] = ((tx).clone().unwrap()) as f32;
    out.m[13.0_f64 as usize] = ((ty).clone().unwrap()) as f32;
    out.m[14.0_f64 as usize] = (0.0_f64) as f32;
    out.m[15.0_f64 as usize] = (1.0_f64) as f32;
}

// Source: upstream/packages/geometry/src/matrix4.ts:979 (sha256:a635f411090028c20faae964889108f6a9f35a4650263ed7c60f762070bb6b05)
pub fn set_matrix4_from_float32_array(out: &mut Matrix4Like, offset: f64, source: &Vec<f32>) -> () {
    {
        let __flight_offset = (0.0_f64) as usize;
        let __flight_values: Vec<f32> = (source[(offset) as usize..(offset + 16.0_f64) as usize]
            .to_vec())
        .iter()
        .map(|value| (*value) as f32)
        .collect();
        out.m[__flight_offset..__flight_offset + __flight_values.len()]
            .copy_from_slice(&__flight_values);
    };
}

// Source: upstream/packages/geometry/src/matrix4.ts:983 (sha256:2dc17460f750466baaac0e1a0db10f7dc2ed9e6ff46a8e33931b3ece831c43a1)
pub fn set_matrix4_from_matrix(out: &mut Matrix4Like, source: &mut MatrixLike) -> () {
    set_matrix4_from2_d(
        out,
        source.a,
        source.b,
        source.c,
        source.d,
        Some(source.tx),
        Some(source.ty),
    );
}

// Source: upstream/packages/geometry/src/matrix4.ts:987 (sha256:aaec5593dd92599e6f8a8c03be1c6738b60471ebf8db54e3e38890aba8299386)
pub fn set_matrix4_from_matrix3(out: &mut Matrix4Like, source: &mut Matrix3Like) -> () {
    set_matrix4_from2_d(
        out,
        (source.m[0.0_f64 as usize] as f64),
        (source.m[3.0_f64 as usize] as f64),
        (source.m[1.0_f64 as usize] as f64),
        (source.m[4.0_f64 as usize] as f64),
        Some((source.m[6.0_f64 as usize] as f64).clone()),
        Some((source.m[7.0_f64 as usize] as f64).clone()),
    );
    out.m[2.0_f64 as usize] = (source.m[2.0_f64 as usize] as f64) as f32;
    out.m[6.0_f64 as usize] = (source.m[5.0_f64 as usize] as f64) as f32;
    out.m[10.0_f64 as usize] = (source.m[8.0_f64 as usize] as f64) as f32;
}

// Source: upstream/packages/geometry/src/matrix4.ts:1002 (sha256:bd767c754fe57cdea043a2d46e36638a1e59de612a2ed097ea722fb4c3d9616c)
pub fn set_matrix4_from_quaternion(out: &mut Matrix4Like, source: &QuaternionLike) -> () {
    let x = source.x;
    let y = source.y;
    let z = source.z;
    let w = source.w;
    let x2 = (x + x);
    let y2 = (y + y);
    let z2 = (z + z);
    let xx = (x * x2);
    let xy = (x * y2);
    let xz = (x * z2);
    let yy = (y * y2);
    let yz = (y * z2);
    let zz = (z * z2);
    let wx = (w * x2);
    let wy = (w * y2);
    let wz = (w * z2);
    out.m[0.0_f64 as usize] = (1.0_f64 - (yy + zz)) as f32;
    out.m[1.0_f64 as usize] = (xy + wz) as f32;
    out.m[2.0_f64 as usize] = (xz - wy) as f32;
    out.m[3.0_f64 as usize] = (0.0_f64) as f32;
    out.m[4.0_f64 as usize] = (xy - wz) as f32;
    out.m[5.0_f64 as usize] = (1.0_f64 - (xx + zz)) as f32;
    out.m[6.0_f64 as usize] = (yz + wx) as f32;
    out.m[7.0_f64 as usize] = (0.0_f64) as f32;
    out.m[8.0_f64 as usize] = (xz + wy) as f32;
    out.m[9.0_f64 as usize] = (yz - wx) as f32;
    out.m[10.0_f64 as usize] = (1.0_f64 - (xx + yy)) as f32;
    out.m[11.0_f64 as usize] = (0.0_f64) as f32;
    out.m[12.0_f64 as usize] = (0.0_f64) as f32;
    out.m[13.0_f64 as usize] = (0.0_f64) as f32;
    out.m[14.0_f64 as usize] = (0.0_f64) as f32;
    out.m[15.0_f64 as usize] = (1.0_f64) as f32;
}

// Source: upstream/packages/geometry/src/matrix4.ts:1045 (sha256:6f5070b839610bbc697486e508d598b4f5a42315991db134599410f2ca1d5ffe)
pub fn set_matrix4_identity(out: &mut Matrix4Like) -> () {
    set_matrix4(
        out, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
        0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64,
    );
}

// Source: upstream/packages/geometry/src/matrix4.ts:1057 (sha256:eb4b9bb733ad7f661b9ecf44a052e9910126f086d05713991a4405922c9c2493)
pub fn set_matrix4_look_at(
    out: &mut Matrix4Like,
    eye: &Vector3Like,
    target: &Vector3Like,
    up: &Vector3Like,
) -> () {
    let eye_x = eye.x;
    let eye_y = eye.y;
    let eye_z = eye.z;
    let mut zx = (eye_x - target.x);
    let mut zy = (eye_y - target.y);
    let mut zz = (eye_z - target.z);
    let mut zl = (((zx * zx) + (zy * zy)) + (zz * zz)).sqrt();
    if (zl == 0.0_f64) {
        zz = 1.0_f64;
        zl = 1.0_f64;
    }
    zx /= zl;
    zy /= zl;
    zz /= zl;
    let mut xx = ((up.y * zz) - (up.z * zy));
    let mut xy = ((up.z * zx) - (up.x * zz));
    let mut xz = ((up.x * zy) - (up.y * zx));
    let mut xl = (((xx * xx) + (xy * xy)) + (xz * xz)).sqrt();
    if (xl == 0.0_f64) {
        xx = 0.0_f64;
        xy = 0.0_f64;
        xz = 0.0_f64;
    } else {
        xx /= xl;
        xy /= xl;
        xz /= xl;
    }
    let yx = ((zy * xz) - (zz * xy));
    let yy = ((zz * xx) - (zx * xz));
    let yz = ((zx * xy) - (zy * xx));
    out.m[0.0_f64 as usize] = (xx) as f32;
    out.m[1.0_f64 as usize] = (yx) as f32;
    out.m[2.0_f64 as usize] = (zx) as f32;
    out.m[3.0_f64 as usize] = (0.0_f64) as f32;
    out.m[4.0_f64 as usize] = (xy) as f32;
    out.m[5.0_f64 as usize] = (yy) as f32;
    out.m[6.0_f64 as usize] = (zy) as f32;
    out.m[7.0_f64 as usize] = (0.0_f64) as f32;
    out.m[8.0_f64 as usize] = (xz) as f32;
    out.m[9.0_f64 as usize] = (yz) as f32;
    out.m[10.0_f64 as usize] = (zz) as f32;
    out.m[11.0_f64 as usize] = (0.0_f64) as f32;
    out.m[12.0_f64 as usize] = (-(((xx * eye_x) + (xy * eye_y)) + (xz * eye_z))) as f32;
    out.m[13.0_f64 as usize] = (-(((yx * eye_x) + (yy * eye_y)) + (yz * eye_z))) as f32;
    out.m[14.0_f64 as usize] = (-(((zx * eye_x) + (zy * eye_y)) + (zz * eye_z))) as f32;
    out.m[15.0_f64 as usize] = (1.0_f64) as f32;
}

// Source: upstream/packages/geometry/src/matrix4.ts:1122 (sha256:d1fcf5d82b697a03195de6c0ace55c022e80c7fd5f4d08d54595f1a3e81097b9)
pub fn set_matrix4_position(out: &mut Matrix4Like, source: &Vector3Like) -> () {
    out.m[12.0_f64 as usize] = (source.x) as f32;
    out.m[13.0_f64 as usize] = (source.y) as f32;
    out.m[14.0_f64 as usize] = (source.z) as f32;
}

// Source: upstream/packages/geometry/src/matrix4.ts:1132 (sha256:bd3dc3cd972ba328c8e1d23e309498d6d2ed336ffc14b4f230aafbd810d46c19)
pub fn set_orthographic_matrix4(
    out: &mut Matrix4Like,
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    z_near: f64,
    z_far: f64,
) -> () {
    let sx = (1.0_f64 / (right - left));
    let sy = (1.0_f64 / (top - bottom));
    let sz = (1.0_f64 / (z_far - z_near));
    out.m[0.0_f64 as usize] = (2.0_f64 * sx) as f32;
    out.m[1.0_f64 as usize] = (0.0_f64) as f32;
    out.m[2.0_f64 as usize] = (0.0_f64) as f32;
    out.m[3.0_f64 as usize] = (0.0_f64) as f32;
    out.m[4.0_f64 as usize] = (0.0_f64) as f32;
    out.m[5.0_f64 as usize] = (2.0_f64 * sy) as f32;
    out.m[6.0_f64 as usize] = (0.0_f64) as f32;
    out.m[7.0_f64 as usize] = (0.0_f64) as f32;
    out.m[8.0_f64 as usize] = (0.0_f64) as f32;
    out.m[9.0_f64 as usize] = (0.0_f64) as f32;
    out.m[10.0_f64 as usize] = ((-2.0_f64) * sz) as f32;
    out.m[11.0_f64 as usize] = (0.0_f64) as f32;
    out.m[12.0_f64 as usize] = ((-(left + right)) * sx) as f32;
    out.m[13.0_f64 as usize] = ((-(bottom + top)) * sy) as f32;
    out.m[14.0_f64 as usize] = ((-(z_near + z_far)) * sz) as f32;
    out.m[15.0_f64 as usize] = (1.0_f64) as f32;
}

// Source: upstream/packages/geometry/src/matrix4.ts:1170 (sha256:657013889eecda4931bba766af03072c35e91158b8ecaa1646a9ca6d2411dead)
pub fn set_perspective_matrix4(
    out: &mut Matrix4Like,
    fov: f64,
    aspect: f64,
    z_near: f64,
    z_far: f64,
) -> () {
    if ((aspect > (-1e-7_f64)) && (aspect < 1e-7_f64)) {
        panic!("{}", "Aspect ratio may not be 0");
    }
    let top = (fov * z_near);
    let bottom = (-top);
    let right = (top * aspect);
    let left = (-right);
    out.m[0.0_f64 as usize] = ((2.0_f64 * z_near) / (right - left)) as f32;
    out.m[1.0_f64 as usize] = (0.0_f64) as f32;
    out.m[2.0_f64 as usize] = (0.0_f64) as f32;
    out.m[3.0_f64 as usize] = (0.0_f64) as f32;
    out.m[4.0_f64 as usize] = (0.0_f64) as f32;
    out.m[5.0_f64 as usize] = ((2.0_f64 * z_near) / (top - bottom)) as f32;
    out.m[6.0_f64 as usize] = (0.0_f64) as f32;
    out.m[7.0_f64 as usize] = (0.0_f64) as f32;
    out.m[8.0_f64 as usize] = ((right + left) / (right - left)) as f32;
    out.m[9.0_f64 as usize] = ((top + bottom) / (top - bottom)) as f32;
    out.m[10.0_f64 as usize] = ((-(z_far + z_near)) / (z_far - z_near)) as f32;
    out.m[11.0_f64 as usize] = (-1.0_f64) as f32;
    out.m[12.0_f64 as usize] = (0.0_f64) as f32;
    out.m[13.0_f64 as usize] = (0.0_f64) as f32;
    out.m[14.0_f64 as usize] = ((((-2.0_f64) * z_far) * z_near) / (z_far - z_near)) as f32;
    out.m[15.0_f64 as usize] = (0.0_f64) as f32;
}

// Source: upstream/packages/geometry/src/matrix4.ts:1216 (sha256:a897da6c0d0d7cd9f1e4adefd07784e4dad4d62baf48722653d3d499cee85f68)
pub fn translate_matrix4(
    out: &mut Matrix4Like,
    source: &Matrix4Like,
    tx: f64,
    ty: f64,
    tz: f64,
) -> () {
    if (out != source) {
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<f32> = ((source.m).clone())
                .iter()
                .map(|value| (*value) as f32)
                .collect();
            out.m[__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
    }
    out.m[12.0_f64 as usize] = (((((source.m[0.0_f64 as usize] as f64) * tx)
        + ((source.m[4.0_f64 as usize] as f64) * ty))
        + ((source.m[8.0_f64 as usize] as f64) * tz))
        + (source.m[12.0_f64 as usize] as f64)) as f32;
    out.m[13.0_f64 as usize] = (((((source.m[1.0_f64 as usize] as f64) * tx)
        + ((source.m[5.0_f64 as usize] as f64) * ty))
        + ((source.m[9.0_f64 as usize] as f64) * tz))
        + (source.m[13.0_f64 as usize] as f64)) as f32;
    out.m[14.0_f64 as usize] = (((((source.m[2.0_f64 as usize] as f64) * tx)
        + ((source.m[6.0_f64 as usize] as f64) * ty))
        + ((source.m[10.0_f64 as usize] as f64) * tz))
        + (source.m[14.0_f64 as usize] as f64)) as f32;
}

// Source: upstream/packages/geometry/src/matrix4.ts:1231 (sha256:672eeb100267f08899737e7e436b6ec65bf807abd2c95f95021294dcdf6c6cbf)
pub fn transpose_matrix4(out: &mut Matrix4Like, source: &Matrix4Like) -> () {
    if (out != source) {
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<f32> = ((source.m).clone())
                .iter()
                .map(|value| (*value) as f32)
                .collect();
            out.m[__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
    }
    __swap(out, source, 1.0_f64, 4.0_f64);
    __swap(out, source, 2.0_f64, 8.0_f64);
    __swap(out, source, 3.0_f64, 12.0_f64);
    __swap(out, source, 6.0_f64, 9.0_f64);
    __swap(out, source, 7.0_f64, 13.0_f64);
    __swap(out, source, 11.0_f64, 14.0_f64);
}

// Source: upstream/packages/geometry/src/matrix4.ts:1257 (sha256:cdf0eac9cdfe188a44bf56631ea3252ea6c3b48087232708e0eaf74c551130c4)
pub fn write_matrix4_to_float32_array(out: &mut Vec<f32>, offset: f64, source: &Matrix4Like) -> () {
    {
        let __flight_offset = (offset) as usize;
        let __flight_values: Vec<f32> = ((source.m).clone())
            .iter()
            .map(|value| (*value) as f32)
            .collect();
        out[__flight_offset..__flight_offset + __flight_values.len()]
            .copy_from_slice(&__flight_values);
    };
}

// Source: upstream/packages/geometry/src/matrix4.ts:1261 (sha256:62354f2479aa0c172003863549adbc5615308cc4a28eb4ff2998faf950d93c11)
fn __get_axis_rotation(out: &mut Matrix4Like, x: f64, y: f64, z: f64, radians: f64) -> () {
    let mut ax = x;
    let mut ay = y;
    let mut az = z;
    let rad = (-radians);
    let c = (rad).cos();
    let s = (rad).sin();
    let t = (1.0_f64 - c);
    out.m[0.0_f64 as usize] = (c + ((ax * ax) * t)) as f32;
    out.m[5.0_f64 as usize] = (c + ((ay * ay) * t)) as f32;
    out.m[10.0_f64 as usize] = (c + ((az * az) * t)) as f32;
    let mut tmp1 = ((ax * ay) * t);
    let mut tmp2 = (az * s);
    out.m[4.0_f64 as usize] = (tmp1 + tmp2) as f32;
    out.m[1.0_f64 as usize] = (tmp1 - tmp2) as f32;
    tmp1 = ((ax * az) * t);
    tmp2 = (ay * s);
    out.m[8.0_f64 as usize] = (tmp1 - tmp2) as f32;
    out.m[2.0_f64 as usize] = (tmp1 + tmp2) as f32;
    tmp1 = ((ay * az) * t);
    tmp2 = (ax * s);
    out.m[9.0_f64 as usize] = (tmp1 + tmp2) as f32;
    out.m[6.0_f64 as usize] = (tmp1 - tmp2) as f32;
}

// Source: upstream/packages/geometry/src/matrix4.ts:1292 (sha256:c6f4dc8735038d620895aaffe8175ed6e1d8f8043b59c790d83e9574b5284304)
fn __swap(out: &mut Matrix4Like, source: &Matrix4Like, a: f64, b: f64) -> () {
    let temp = (source.m[a as usize] as f64);
    out.m[a as usize] = (source.m[b as usize] as f64) as f32;
    out.m[b as usize] = ((temp).clone()) as f32;
}

// Source: upstream/packages/geometry/src/matrix4.ts:1298 (sha256:e79bc457d8148decc8e86243e73d0d44fd479e39ddf30653815e30d123ecedd5)
static __IDENTITY: std::sync::LazyLock<Vec<f32>> = std::sync::LazyLock::new(|| {
    (vec![
        1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
        1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64,
    ])
    .iter()
    .map(|value| (*value) as f32)
    .collect()
});
