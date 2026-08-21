// @generated from upstream/packages/geometry/src/matrix3.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{acquire_matrix3, release_matrix3};
use flighthq_entity::create_entity;
use flighthq_types::{Matrix3, Matrix3Like, Matrix4Like, MatrixLike, Vector3Like};

// Source: upstream/packages/geometry/src/matrix3.ts:6 (sha256:a9b9d47db5c559e5ec5f20bfa50929539a09552866311bce152180885cf1c46b)
pub fn clone_matrix3(source: &Matrix3Like) -> Matrix3 {
    let mut m = create_matrix3(None, None, None, None, None, None, None, None, None);
    copy_matrix3(&mut m, source);
    return m;
}

// Source: upstream/packages/geometry/src/matrix3.ts:12 (sha256:ddc657472e41ddc590678e47a18dc441dcad8cb7dfc4fc5fc99530915bd83c2f)
pub fn copy_matrix3(out: &mut Matrix3Like, source: &Matrix3Like) -> () {
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

// Source: upstream/packages/geometry/src/matrix3.ts:16 (sha256:243a5925e72d116375f94c057cee3658b99e7c8204c07a9b5d99271017d489da)
pub fn copy_matrix3_column_from_vector3(
    out: &mut Matrix3Like,
    column: f64,
    source: &Vector3Like,
) -> () {
    let x = source.x;
    let y = source.y;
    let z = source.z;
    {
        let __switch_value = column;
        let __flight_case = if __switch_value == 0.0_f64 {
            0_usize
        } else if __switch_value == 1.0_f64 {
            1_usize
        } else if __switch_value == 2.0_f64 {
            2_usize
        } else {
            3_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {}
            if __flight_case <= 1_usize {}
            if __flight_case <= 2_usize {
                {
                    let base = (column * 3.0_f64);
                    out.m[base as usize] = (x) as f32;
                    out.m[(base + 1.0_f64) as usize] = (y) as f32;
                    out.m[(base + 2.0_f64) as usize] = (z) as f32;
                    return;
                }
            }
            if __flight_case <= 3_usize {
                panic!("{}", "generated Flight function threw");
            }
        }
    }
}

// Source: upstream/packages/geometry/src/matrix3.ts:36 (sha256:97c19ca12d1a733e5d31fe4c431c9afd300d6fb129fbe6862da1ebf227129d1f)
pub fn copy_matrix3_column_to_vector3(
    out: &mut Vector3Like,
    column: f64,
    source: &Matrix3Like,
) -> () {
    {
        let __switch_value = column;
        let __flight_case = if __switch_value == 0.0_f64 {
            0_usize
        } else if __switch_value == 1.0_f64 {
            1_usize
        } else if __switch_value == 2.0_f64 {
            2_usize
        } else {
            3_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {}
            if __flight_case <= 1_usize {}
            if __flight_case <= 2_usize {
                {
                    let base = (column * 3.0_f64);
                    let x = (source.m[base as usize] as f64);
                    let y = (source.m[(base + 1.0_f64) as usize] as f64);
                    let z = (source.m[(base + 2.0_f64) as usize] as f64);
                    out.x = x;
                    out.y = y;
                    out.z = z;
                    return;
                }
            }
            if __flight_case <= 3_usize {
                panic!("{}", "generated Flight function threw");
            }
        }
    }
}

// Source: upstream/packages/geometry/src/matrix3.ts:55 (sha256:5a6d23248cf82076acd89c41dfb85d4610f8dca81a9b6171979ea34a5eeaf0f8)
pub fn copy_matrix3_row_from_vector3(out: &mut Matrix3Like, row: f64, source: &Vector3Like) -> () {
    let x = source.x;
    let y = source.y;
    let z = source.z;
    {
        let __switch_value = row;
        let __flight_case = if __switch_value == 0.0_f64 {
            0_usize
        } else if __switch_value == 1.0_f64 {
            1_usize
        } else if __switch_value == 2.0_f64 {
            2_usize
        } else {
            3_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {}
            if __flight_case <= 1_usize {}
            if __flight_case <= 2_usize {
                out.m[row as usize] = (x) as f32;
                out.m[(row + 3.0_f64) as usize] = (y) as f32;
                out.m[(row + 6.0_f64) as usize] = (z) as f32;
                return;
            }
            if __flight_case <= 3_usize {
                panic!("{}", "generated Flight function threw");
            }
        }
    }
}

// Source: upstream/packages/geometry/src/matrix3.ts:73 (sha256:40faf4894cddce6a2e85957ed6b96eb2dbac0425eea08fd5354fbadcc33c3884)
pub fn copy_matrix3_row_to_vector3(out: &mut Vector3Like, row: f64, source: &Matrix3Like) -> () {
    {
        let __switch_value = row;
        let __flight_case = if __switch_value == 0.0_f64 {
            0_usize
        } else if __switch_value == 1.0_f64 {
            1_usize
        } else if __switch_value == 2.0_f64 {
            2_usize
        } else {
            3_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {}
            if __flight_case <= 1_usize {}
            if __flight_case <= 2_usize {
                {
                    let x = (source.m[row as usize] as f64);
                    let y = (source.m[(row + 3.0_f64) as usize] as f64);
                    let z = (source.m[(row + 6.0_f64) as usize] as f64);
                    out.x = x;
                    out.y = y;
                    out.z = z;
                    return;
                }
            }
            if __flight_case <= 3_usize {
                panic!("{}", "generated Flight function threw");
            }
        }
    }
}

// Source: upstream/packages/geometry/src/matrix3.ts:102 (sha256:67cadeb51627aa7235960c745d8f8be904b302994a2e3d3cd93a8df2d9495847)
pub fn create_matrix3(
    m00: Option<f64>,
    m01: Option<f64>,
    m02: Option<f64>,
    m10: Option<f64>,
    m11: Option<f64>,
    m12: Option<f64>,
    m20: Option<f64>,
    m21: Option<f64>,
    m22: Option<f64>,
) -> Matrix3 {
    let mut m: Vec<f32> = ((*__IDENTITY).clone())
        .iter()
        .map(|value| (*value) as f32)
        .collect();
    let mut out: Matrix3 = create_entity(Some(Matrix3 {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        m: (m).clone(),
    }));
    if (m00).is_some() {
        out.m[0.0_f64 as usize] = (*(m00.as_ref().unwrap())) as f32;
    }
    if (m01).is_some() {
        out.m[3.0_f64 as usize] = (*(m01.as_ref().unwrap())) as f32;
    }
    if (m02).is_some() {
        out.m[6.0_f64 as usize] = (*(m02.as_ref().unwrap())) as f32;
    }
    if (m10).is_some() {
        out.m[1.0_f64 as usize] = (*(m10.as_ref().unwrap())) as f32;
    }
    if (m11).is_some() {
        out.m[4.0_f64 as usize] = (*(m11.as_ref().unwrap())) as f32;
    }
    if (m12).is_some() {
        out.m[7.0_f64 as usize] = (*(m12.as_ref().unwrap())) as f32;
    }
    if (m20).is_some() {
        out.m[2.0_f64 as usize] = (*(m20.as_ref().unwrap())) as f32;
    }
    if (m21).is_some() {
        out.m[5.0_f64 as usize] = (*(m21.as_ref().unwrap())) as f32;
    }
    if (m22).is_some() {
        out.m[8.0_f64 as usize] = (*(m22.as_ref().unwrap())) as f32;
    }
    return out;
}

// Source: upstream/packages/geometry/src/matrix3.ts:127 (sha256:28d06eb0f236afd0b1d115966d3c5ee3353cccf1320d74b3cfd8ca61fd88bcf8)
pub fn equals_matrix3(a: &Option<Matrix3Like>, b: &Option<Matrix3Like>) -> bool {
    if (a == b) {
        return true;
    }
    if ((a).is_none()) || ((b).is_none()) {
        return false;
    }
    {
        let mut i = 0.0_f64;
        while (i < 9.0_f64) {
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

// Source: upstream/packages/geometry/src/matrix3.ts:143 (sha256:bfb1a1c4db3aa79ab04b1376003c0cbeca288a14752d382d66e9a2d86b13c0cc)
pub fn get_matrix3_determinant(source: &Matrix3Like) -> f64 {
    return ((((((((source.m[0.0_f64 as usize] as f64) * (source.m[4.0_f64 as usize] as f64))
        * (source.m[8.0_f64 as usize] as f64))
        + (((source.m[1.0_f64 as usize] as f64) * (source.m[5.0_f64 as usize] as f64))
            * (source.m[6.0_f64 as usize] as f64)))
        + (((source.m[2.0_f64 as usize] as f64) * (source.m[3.0_f64 as usize] as f64))
            * (source.m[7.0_f64 as usize] as f64)))
        - (((source.m[2.0_f64 as usize] as f64) * (source.m[4.0_f64 as usize] as f64))
            * (source.m[6.0_f64 as usize] as f64)))
        - (((source.m[1.0_f64 as usize] as f64) * (source.m[3.0_f64 as usize] as f64))
            * (source.m[8.0_f64 as usize] as f64)))
        - (((source.m[0.0_f64 as usize] as f64) * (source.m[5.0_f64 as usize] as f64))
            * (source.m[7.0_f64 as usize] as f64)));
}

// Source: upstream/packages/geometry/src/matrix3.ts:155 (sha256:7e630fea2cc19245522b35280dfa3a1a81e87874f5cce1cd99faf997e8e560c1)
pub fn get_matrix3_element(source: &Matrix3Like, row: f64, column: f64) -> f64 {
    return (source.m[((column * 3.0_f64) + row) as usize] as f64);
}

// Source: upstream/packages/geometry/src/matrix3.ts:164 (sha256:a7c96654144d4bf289a078e7a026d8836968ceda776f03cf7f5084deff9efd5b)
pub fn inverse_matrix3(out: &mut Matrix3Like, source: &Matrix3Like) -> bool {
    let a00 = (source.m[0.0_f64 as usize] as f64);
    let a10 = (source.m[1.0_f64 as usize] as f64);
    let a20 = (source.m[2.0_f64 as usize] as f64);
    let a01 = (source.m[3.0_f64 as usize] as f64);
    let a11 = (source.m[4.0_f64 as usize] as f64);
    let a21 = (source.m[5.0_f64 as usize] as f64);
    let a02 = (source.m[6.0_f64 as usize] as f64);
    let a12 = (source.m[7.0_f64 as usize] as f64);
    let a22 = (source.m[8.0_f64 as usize] as f64);
    if is_affine_matrix3(source) {
        let det = ((a00 * a11) - (a01 * a10));
        if (det == 0.0_f64) {
            {
                let __flight_value = (f64::NAN) as f32;
                let __flight_collection = &mut out.m;
                __flight_collection.fill(__flight_value);
                __flight_collection.clone()
            };
            return false;
        }
        let inv_det = (1.0_f64 / det);
        let i00 = (a11 * inv_det);
        let i01 = ((-a01) * inv_det);
        let i10 = ((-a10) * inv_det);
        let i11 = (a00 * inv_det);
        out.m[0.0_f64 as usize] = (i00) as f32;
        out.m[1.0_f64 as usize] = (i10) as f32;
        out.m[2.0_f64 as usize] = (0.0_f64) as f32;
        out.m[3.0_f64 as usize] = (i01) as f32;
        out.m[4.0_f64 as usize] = (i11) as f32;
        out.m[5.0_f64 as usize] = (0.0_f64) as f32;
        out.m[6.0_f64 as usize] = (-((i00 * a02) + (i01 * a12))) as f32;
        out.m[7.0_f64 as usize] = (-((i10 * a02) + (i11 * a12))) as f32;
        out.m[8.0_f64 as usize] = (1.0_f64) as f32;
        return true;
    }
    let det = (((a00 * ((a11 * a22) - (a12 * a21))) - (a01 * ((a10 * a22) - (a12 * a20))))
        + (a02 * ((a10 * a21) - (a11 * a20))));
    if (det == 0.0_f64) {
        {
            let __flight_value = (f64::NAN) as f32;
            let __flight_collection = &mut out.m;
            __flight_collection.fill(__flight_value);
            __flight_collection.clone()
        };
        return false;
    }
    let inv = (1.0_f64 / det);
    out.m[0.0_f64 as usize] = (((a11 * a22) - (a12 * a21)) * inv) as f32;
    out.m[1.0_f64 as usize] = (((a12 * a20) - (a10 * a22)) * inv) as f32;
    out.m[2.0_f64 as usize] = (((a10 * a21) - (a11 * a20)) * inv) as f32;
    out.m[3.0_f64 as usize] = (((a02 * a21) - (a01 * a22)) * inv) as f32;
    out.m[4.0_f64 as usize] = (((a00 * a22) - (a02 * a20)) * inv) as f32;
    out.m[5.0_f64 as usize] = (((a01 * a20) - (a00 * a21)) * inv) as f32;
    out.m[6.0_f64 as usize] = (((a01 * a12) - (a02 * a11)) * inv) as f32;
    out.m[7.0_f64 as usize] = (((a02 * a10) - (a00 * a12)) * inv) as f32;
    out.m[8.0_f64 as usize] = (((a00 * a11) - (a01 * a10)) * inv) as f32;
    return true;
}

// Source: upstream/packages/geometry/src/matrix3.ts:228 (sha256:1c634e8dc737b307b81e8e57f60d4f803790b3908742a8724dffa256dcad0c6d)
pub fn is_affine_matrix3(source: &Matrix3Like) -> bool {
    return (((source.m[2.0_f64 as usize] as f64) == 0.0_f64)
        && ((source.m[5.0_f64 as usize] as f64) == 0.0_f64))
        && ((source.m[8.0_f64 as usize] as f64) == 1.0_f64);
}

// Source: upstream/packages/geometry/src/matrix3.ts:236 (sha256:8766615cd1ec47c2ba6c4e32b4f87b9469fe8727cc30fd8c8cb2ce90d38b222a)
pub fn multiply_matrix3(out: &mut Matrix3Like, a: &Matrix3Like, b: &Matrix3Like) -> () {
    let a00 = (a.m[0.0_f64 as usize] as f64);
    let a10 = (a.m[1.0_f64 as usize] as f64);
    let a20 = (a.m[2.0_f64 as usize] as f64);
    let a01 = (a.m[3.0_f64 as usize] as f64);
    let a11 = (a.m[4.0_f64 as usize] as f64);
    let a21 = (a.m[5.0_f64 as usize] as f64);
    let a02 = (a.m[6.0_f64 as usize] as f64);
    let a12 = (a.m[7.0_f64 as usize] as f64);
    let a22 = (a.m[8.0_f64 as usize] as f64);
    let b00 = (b.m[0.0_f64 as usize] as f64);
    let b10 = (b.m[1.0_f64 as usize] as f64);
    let b20 = (b.m[2.0_f64 as usize] as f64);
    let b01 = (b.m[3.0_f64 as usize] as f64);
    let b11 = (b.m[4.0_f64 as usize] as f64);
    let b21 = (b.m[5.0_f64 as usize] as f64);
    let b02 = (b.m[6.0_f64 as usize] as f64);
    let b12 = (b.m[7.0_f64 as usize] as f64);
    let b22 = (b.m[8.0_f64 as usize] as f64);
    if (is_affine_matrix3(a)) && (is_affine_matrix3(b)) {
        out.m[0.0_f64 as usize] = ((a00 * b00) + (a01 * b10)) as f32;
        out.m[1.0_f64 as usize] = ((a10 * b00) + (a11 * b10)) as f32;
        out.m[2.0_f64 as usize] = (0.0_f64) as f32;
        out.m[3.0_f64 as usize] = ((a00 * b01) + (a01 * b11)) as f32;
        out.m[4.0_f64 as usize] = ((a10 * b01) + (a11 * b11)) as f32;
        out.m[5.0_f64 as usize] = (0.0_f64) as f32;
        out.m[6.0_f64 as usize] = (((a00 * b02) + (a01 * b12)) + a02) as f32;
        out.m[7.0_f64 as usize] = (((a10 * b02) + (a11 * b12)) + a12) as f32;
        out.m[8.0_f64 as usize] = (1.0_f64) as f32;
        return;
    }
    out.m[0.0_f64 as usize] = (((a00 * b00) + (a01 * b10)) + (a02 * b20)) as f32;
    out.m[1.0_f64 as usize] = (((a10 * b00) + (a11 * b10)) + (a12 * b20)) as f32;
    out.m[2.0_f64 as usize] = (((a20 * b00) + (a21 * b10)) + (a22 * b20)) as f32;
    out.m[3.0_f64 as usize] = (((a00 * b01) + (a01 * b11)) + (a02 * b21)) as f32;
    out.m[4.0_f64 as usize] = (((a10 * b01) + (a11 * b11)) + (a12 * b21)) as f32;
    out.m[5.0_f64 as usize] = (((a20 * b01) + (a21 * b11)) + (a22 * b21)) as f32;
    out.m[6.0_f64 as usize] = (((a00 * b02) + (a01 * b12)) + (a02 * b22)) as f32;
    out.m[7.0_f64 as usize] = (((a10 * b02) + (a11 * b12)) + (a12 * b22)) as f32;
    out.m[8.0_f64 as usize] = (((a20 * b02) + (a21 * b12)) + (a22 * b22)) as f32;
}

// Source: upstream/packages/geometry/src/matrix3.ts:293 (sha256:fd182a41cefaf9f46dc81fd5a7375e99de181124c5e68407b6d55f94f7341d7c)
pub fn rotate_matrix3(out: &mut Matrix3Like, source: &Matrix3Like, theta: f64) -> () {
    let c = (theta).cos();
    let s = (theta).sin();
    let a0 = (source.m[0.0_f64 as usize] as f64);
    let a1 = (source.m[1.0_f64 as usize] as f64);
    let a2 = (source.m[2.0_f64 as usize] as f64);
    let a3 = (source.m[3.0_f64 as usize] as f64);
    let a4 = (source.m[4.0_f64 as usize] as f64);
    let a5 = (source.m[5.0_f64 as usize] as f64);
    let a6 = (source.m[6.0_f64 as usize] as f64);
    let a7 = (source.m[7.0_f64 as usize] as f64);
    let a8 = (source.m[8.0_f64 as usize] as f64);
    out.m[0.0_f64 as usize] = ((a0 * c) + (a3 * s)) as f32;
    out.m[1.0_f64 as usize] = ((a1 * c) + (a4 * s)) as f32;
    out.m[2.0_f64 as usize] = ((a2 * c) + (a5 * s)) as f32;
    out.m[3.0_f64 as usize] = ((a0 * (-s)) + (a3 * c)) as f32;
    out.m[4.0_f64 as usize] = ((a1 * (-s)) + (a4 * c)) as f32;
    out.m[5.0_f64 as usize] = ((a2 * (-s)) + (a5 * c)) as f32;
    out.m[6.0_f64 as usize] = ((a6).clone()) as f32;
    out.m[7.0_f64 as usize] = ((a7).clone()) as f32;
    out.m[8.0_f64 as usize] = ((a8).clone()) as f32;
}

// Source: upstream/packages/geometry/src/matrix3.ts:329 (sha256:c022ee8005ea84dbab3007198b6956ba57ccd5aec4b77fe98cd9544e936dc2b1)
pub fn scale_matrix3(out: &mut Matrix3Like, source: &Matrix3Like, sx: f64, sy: f64) -> () {
    let a0 = (source.m[0.0_f64 as usize] as f64);
    let a1 = (source.m[1.0_f64 as usize] as f64);
    let a2 = (source.m[2.0_f64 as usize] as f64);
    let a3 = (source.m[3.0_f64 as usize] as f64);
    let a4 = (source.m[4.0_f64 as usize] as f64);
    let a5 = (source.m[5.0_f64 as usize] as f64);
    let a6 = (source.m[6.0_f64 as usize] as f64);
    let a7 = (source.m[7.0_f64 as usize] as f64);
    let a8 = (source.m[8.0_f64 as usize] as f64);
    out.m[0.0_f64 as usize] = (a0 * sx) as f32;
    out.m[1.0_f64 as usize] = (a1 * sx) as f32;
    out.m[2.0_f64 as usize] = (a2 * sx) as f32;
    out.m[3.0_f64 as usize] = (a3 * sy) as f32;
    out.m[4.0_f64 as usize] = (a4 * sy) as f32;
    out.m[5.0_f64 as usize] = (a5 * sy) as f32;
    out.m[6.0_f64 as usize] = ((a6).clone()) as f32;
    out.m[7.0_f64 as usize] = ((a7).clone()) as f32;
    out.m[8.0_f64 as usize] = ((a8).clone()) as f32;
}

// Source: upstream/packages/geometry/src/matrix3.ts:356 (sha256:f4cd9d3acb91c4f62043fab29c41940b147e28fee7ab2d718af01dd0f36eaa53)
pub fn set_matrix3(
    out: &mut Matrix3Like,
    m00: f64,
    m01: f64,
    m02: f64,
    m10: f64,
    m11: f64,
    m12: f64,
    m20: f64,
    m21: f64,
    m22: f64,
) -> () {
    out.m[0.0_f64 as usize] = (m00) as f32;
    out.m[3.0_f64 as usize] = (m01) as f32;
    out.m[6.0_f64 as usize] = (m02) as f32;
    out.m[1.0_f64 as usize] = (m10) as f32;
    out.m[4.0_f64 as usize] = (m11) as f32;
    out.m[7.0_f64 as usize] = (m12) as f32;
    out.m[2.0_f64 as usize] = (m20) as f32;
    out.m[5.0_f64 as usize] = (m21) as f32;
    out.m[8.0_f64 as usize] = (m22) as f32;
}

// Source: upstream/packages/geometry/src/matrix3.ts:381 (sha256:533571d8ba4ff118fa2ac28bcc7c3fcd7bb44fba52382e93005825a465933e40)
pub fn set_matrix3_element(out: &mut Matrix3Like, row: f64, column: f64, value: f64) -> () {
    out.m[((column * 3.0_f64) + row) as usize] = (value) as f32;
}

// Source: upstream/packages/geometry/src/matrix3.ts:388 (sha256:2d5e4f6382f799a2fabbf8af4d773b58d01b521d5a677256ebc34cdb67afbd6a)
pub fn set_matrix3_from_float32_array(out: &mut Matrix3Like, offset: f64, source: &Vec<f32>) -> () {
    {
        let mut i = 0.0_f64;
        while (i < 9.0_f64) {
            out.m[i as usize] = (source[(offset + i) as usize] as f64) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/geometry/src/matrix3.ts:395 (sha256:9b82aecd1f1f6d6cb0d9463d43a86cff029ff6fff993f5b2cf76dcdd8240056d)
pub fn set_matrix3_from_matrix(out: &mut Matrix3Like, source: &MatrixLike) -> () {
    out.m[0.0_f64 as usize] = (source.a) as f32;
    out.m[1.0_f64 as usize] = (source.c) as f32;
    out.m[2.0_f64 as usize] = (0.0_f64) as f32;
    out.m[3.0_f64 as usize] = (source.b) as f32;
    out.m[4.0_f64 as usize] = (source.d) as f32;
    out.m[5.0_f64 as usize] = (0.0_f64) as f32;
    out.m[6.0_f64 as usize] = (source.tx) as f32;
    out.m[7.0_f64 as usize] = (source.ty) as f32;
    out.m[8.0_f64 as usize] = (1.0_f64) as f32;
}

// Source: upstream/packages/geometry/src/matrix3.ts:409 (sha256:efc4971587e76091d103141ac15d8009a8b6baa9c7127a02b346881a3df186a6)
pub fn set_matrix3_from_matrix4(out: &mut Matrix3Like, source: &Matrix4Like) -> () {
    out.m[0.0_f64 as usize] = (source.m[0.0_f64 as usize] as f64) as f32;
    out.m[1.0_f64 as usize] = (source.m[1.0_f64 as usize] as f64) as f32;
    out.m[2.0_f64 as usize] = (source.m[2.0_f64 as usize] as f64) as f32;
    out.m[3.0_f64 as usize] = (source.m[4.0_f64 as usize] as f64) as f32;
    out.m[4.0_f64 as usize] = (source.m[5.0_f64 as usize] as f64) as f32;
    out.m[5.0_f64 as usize] = (source.m[6.0_f64 as usize] as f64) as f32;
    out.m[6.0_f64 as usize] = (source.m[8.0_f64 as usize] as f64) as f32;
    out.m[7.0_f64 as usize] = (source.m[9.0_f64 as usize] as f64) as f32;
    out.m[8.0_f64 as usize] = (source.m[10.0_f64 as usize] as f64) as f32;
}

// Source: upstream/packages/geometry/src/matrix3.ts:426 (sha256:03d3f721edbca689ad5f5b720de8bc08336f2799c59ca5be5a2ee64f0cfb4581)
pub fn set_matrix3_identity(out: &mut Matrix3Like) -> () {
    {
        let __flight_offset = (0.0_f64) as usize;
        let __flight_values: Vec<f32> = ((*__IDENTITY).clone())
            .iter()
            .map(|value| (*value) as f32)
            .collect();
        out.m[__flight_offset..__flight_offset + __flight_values.len()]
            .copy_from_slice(&__flight_values);
    };
}

// Source: upstream/packages/geometry/src/matrix3.ts:438 (sha256:5bec6f1e9f3009e83c5e43c532e35f32aa108baeb50f83addf6a5af08dedc3cd)
pub fn set_matrix3_normal_from_matrix4(out: &mut Matrix3Like, source: &Matrix4Like) -> () {
    let mut scratch = acquire_matrix3();
    set_matrix3_from_matrix4(&mut scratch, source);
    {
        let __flight_argument_1 = (scratch).clone();
        let __flight_result = inverse_matrix3(&mut scratch, &__flight_argument_1);
        __flight_result
    };
    transpose_matrix3(out, &{
        let __flight_source = &(scratch);
        Matrix3Like {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            m: (__flight_source.m).clone(),
        }
    });
    release_matrix3(&mut scratch);
}

// Source: upstream/packages/geometry/src/matrix3.ts:451 (sha256:35fd87b18893c116ff16c07b2dc09c58d8d4b42b8994e09e9379b00de852637a)
pub fn translate_matrix3(out: &mut Matrix3Like, source: &Matrix3Like, tx: f64, ty: f64) -> () {
    let a0 = (source.m[0.0_f64 as usize] as f64);
    let a1 = (source.m[1.0_f64 as usize] as f64);
    let a2 = (source.m[2.0_f64 as usize] as f64);
    let a3 = (source.m[3.0_f64 as usize] as f64);
    let a4 = (source.m[4.0_f64 as usize] as f64);
    let a5 = (source.m[5.0_f64 as usize] as f64);
    let a6 = (source.m[6.0_f64 as usize] as f64);
    let a7 = (source.m[7.0_f64 as usize] as f64);
    let a8 = (source.m[8.0_f64 as usize] as f64);
    out.m[0.0_f64 as usize] = ((a0).clone()) as f32;
    out.m[1.0_f64 as usize] = ((a1).clone()) as f32;
    out.m[2.0_f64 as usize] = ((a2).clone()) as f32;
    out.m[3.0_f64 as usize] = ((a3).clone()) as f32;
    out.m[4.0_f64 as usize] = ((a4).clone()) as f32;
    out.m[5.0_f64 as usize] = ((a5).clone()) as f32;
    out.m[6.0_f64 as usize] = (((a0 * tx) + (a3 * ty)) + a6) as f32;
    out.m[7.0_f64 as usize] = (((a1 * tx) + (a4 * ty)) + a7) as f32;
    out.m[8.0_f64 as usize] = (((a2 * tx) + (a5 * ty)) + a8) as f32;
}

// Source: upstream/packages/geometry/src/matrix3.ts:484 (sha256:1ad2265c1306be86bf478eda58c24ebac9ef71402167ef9ddd86a036cc4a694f)
pub fn transpose_matrix3(out: &mut Matrix3Like, source: &Matrix3Like) -> () {
    let m1 = (source.m[1.0_f64 as usize] as f64);
    let m2 = (source.m[2.0_f64 as usize] as f64);
    let m3 = (source.m[3.0_f64 as usize] as f64);
    let m5 = (source.m[5.0_f64 as usize] as f64);
    let m6 = (source.m[6.0_f64 as usize] as f64);
    let m7 = (source.m[7.0_f64 as usize] as f64);
    out.m[0.0_f64 as usize] = (source.m[0.0_f64 as usize] as f64) as f32;
    out.m[1.0_f64 as usize] = ((m3).clone()) as f32;
    out.m[2.0_f64 as usize] = ((m6).clone()) as f32;
    out.m[3.0_f64 as usize] = ((m1).clone()) as f32;
    out.m[4.0_f64 as usize] = (source.m[4.0_f64 as usize] as f64) as f32;
    out.m[5.0_f64 as usize] = ((m7).clone()) as f32;
    out.m[6.0_f64 as usize] = ((m2).clone()) as f32;
    out.m[7.0_f64 as usize] = ((m5).clone()) as f32;
    out.m[8.0_f64 as usize] = (source.m[8.0_f64 as usize] as f64) as f32;
}

// Source: upstream/packages/geometry/src/matrix3.ts:508 (sha256:3b16b676a660ec564b34dbc72624bd919a09ab253a4caa4ad70409441869c05b)
pub fn write_matrix3_to_float32_array(out: &mut Vec<f32>, offset: f64, source: &Matrix3Like) -> () {
    {
        let mut i = 0.0_f64;
        while (i < 9.0_f64) {
            out[(offset + i) as usize] = (source.m[i as usize] as f64) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/geometry/src/matrix3.ts:515 (sha256:28a686d8cd9af5b21a1476f4bcc015b555853277f1fdb34d898e74589122da3a)
static __IDENTITY: std::sync::LazyLock<Vec<f32>> = std::sync::LazyLock::new(|| {
    (vec![
        1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64,
    ])
    .iter()
    .map(|value| (*value) as f32)
    .collect()
});
