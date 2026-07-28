// @generated from upstream/packages/geometry/src/matrix.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{
    Matrix, Matrix3Like, Matrix4Like, MatrixLike, RectangleLike, Vector2Like, Vector3Like,
};

// Source: upstream/packages/geometry/src/matrix.ts:12 (sha256:223003a4ea40b7e755dceb8e680fb160423275d8e153f5291242decc27161cde)
pub fn clone_matrix(source: &MatrixLike) -> Matrix {
    let mut m = create_matrix(None, None, None, None, None, None);
    copy_matrix(&mut m, source);
    return (m).clone();
}

// Source: upstream/packages/geometry/src/matrix.ts:18 (sha256:e275c660484bfb81474851f530c09fda90921bca05ce8d364649f238e8742527)
pub fn copy_matrix(out: &mut MatrixLike, source: &MatrixLike) -> () {
    set_matrix(
        out, source.a, source.b, source.c, source.d, source.tx, source.ty,
    );
}

// Source: upstream/packages/geometry/src/matrix.ts:25 (sha256:6e5a58755d7e2633c8a53282e7a4b6067492efe8940e13ac3242923458f5e8e7)
pub fn copy_matrix_column_from_vector3(
    out: &mut MatrixLike,
    column: f64,
    source: &Vector3Like,
) -> () {
    if (column > 2.0_f64) {
        panic!("{}", "generated Flight function threw");
    } else {
        if (column == 0.0_f64) {
            out.a = source.x;
            out.b = source.y;
        } else {
            if (column == 1.0_f64) {
                out.c = source.x;
                out.d = source.y;
            } else {
                out.tx = source.x;
                out.ty = source.y;
            }
        }
    }
}

// Source: upstream/packages/geometry/src/matrix.ts:43 (sha256:9c500295009167275dad83afb1e032f5a7baf4404eb2795a43ec6dac01321151)
pub fn copy_matrix_column_to_vector3(
    out: &mut Vector3Like,
    column: f64,
    source: &MatrixLike,
) -> () {
    if (column > 2.0_f64) {
        panic!("{}", "generated Flight function threw");
    } else {
        if (column == 0.0_f64) {
            out.x = source.a;
            out.y = source.b;
            out.z = 0.0_f64;
        } else {
            if (column == 1.0_f64) {
                out.x = source.c;
                out.y = source.d;
                out.z = 0.0_f64;
            } else {
                out.x = source.tx;
                out.y = source.ty;
                out.z = 1.0_f64;
            }
        }
    }
}

// Source: upstream/packages/geometry/src/matrix.ts:64 (sha256:033c69954a33bd187d9ab058459f001c2c79e5e7d2c3de715796c8a250d9e67d)
pub fn copy_matrix_row_from_vector3(out: &mut MatrixLike, row: f64, source: &Vector3Like) -> () {
    if (row > 2.0_f64) {
        panic!("{}", "generated Flight function threw");
    } else {
        if (row == 0.0_f64) {
            out.a = source.x;
            out.c = source.y;
            out.tx = source.z;
        } else {
            if (row == 1.0_f64) {
                out.b = source.x;
                out.d = source.y;
                out.ty = source.z;
            }
        }
    }
}

// Source: upstream/packages/geometry/src/matrix.ts:81 (sha256:dcb2255c730e20d4f15e343046e98af0963398e923c505869174de3a9c332da0)
pub fn copy_matrix_row_to_vector3(out: &mut Vector3Like, row: f64, source: &MatrixLike) -> () {
    if (row > 2.0_f64) {
        panic!("{}", "generated Flight function threw");
    } else {
        if (row == 0.0_f64) {
            out.x = source.a;
            out.y = source.c;
            out.z = source.tx;
        } else {
            if (row == 1.0_f64) {
                out.x = source.b;
                out.y = source.d;
                out.z = source.ty;
            } else {
                out.x = 0.0_f64;
                out.y = 0.0_f64;
                out.z = 1.0_f64;
            }
        }
    }
}

// Source: upstream/packages/geometry/src/matrix.ts:99 (sha256:0dd9fbb07cfa575d7ccb20ff833694b48f02b206f72906f76eba7ae1e1208f64)
pub fn create_gradient_transform_matrix(
    width: f64,
    height: f64,
    rotation: Option<f64>,
    tx: Option<f64>,
    ty: Option<f64>,
) -> Matrix {
    let rotation = rotation.unwrap_or(0.0_f64);
    let tx = tx.unwrap_or(0.0_f64);
    let ty = ty.unwrap_or(0.0_f64);
    let mut out = create_matrix(None, None, None, None, None, None);
    set_gradient_transform_matrix(&mut out, width, height, Some(rotation), Some(tx), Some(ty));
    return (out).clone();
}

// Source: upstream/packages/geometry/src/matrix.ts:123 (sha256:8c42ee692d5afe71d3726d329431375adda696bd7d1f9ffa103c47a4c5e31704)
pub fn create_matrix(
    a: Option<f64>,
    b: Option<f64>,
    c: Option<f64>,
    d: Option<f64>,
    tx: Option<f64>,
    ty: Option<f64>,
) -> Matrix {
    return create_entity(Some(Matrix {
        __flight_identity: std::sync::Arc::new(()),
        a: (a).unwrap_or(1.0_f64),
        b: (b).unwrap_or(0.0_f64),
        c: (c).unwrap_or(0.0_f64),
        d: (d).unwrap_or(1.0_f64),
        tx: (tx).unwrap_or(0.0_f64),
        ty: (ty).unwrap_or(0.0_f64),
    }));
}

// Source: upstream/packages/geometry/src/matrix.ts:127 (sha256:49e85917c9c271567d2953da1c3dbc000a7b35440d2dd3f2ca85f0da9cb3345b)
pub fn create_transform_matrix(
    scale_x: f64,
    scale_y: f64,
    rotation: Option<f64>,
    tx: Option<f64>,
    ty: Option<f64>,
) -> Matrix {
    let rotation = rotation.unwrap_or(0.0_f64);
    let tx = tx.unwrap_or(0.0_f64);
    let ty = ty.unwrap_or(0.0_f64);
    let mut out = create_matrix(None, None, None, None, None, None);
    set_transform_matrix(
        &mut out,
        scale_x,
        scale_y,
        Some(rotation),
        Some(tx),
        Some(ty),
    );
    return (out).clone();
}

// Source: upstream/packages/geometry/src/matrix.ts:139 (sha256:c9d73338df7aab825e46232685ce9fc6e9decebfca5773a7f5b5aa30d8479928)
pub fn equals_matrix(
    a: Option<MatrixLike>,
    b: Option<MatrixLike>,
    compare_translation: Option<bool>,
) -> bool {
    let compare_translation = compare_translation.unwrap_or(true);
    if (a == b) {
        return true;
    }
    if ((a).is_none() || (b).is_none()) {
        return false;
    }
    return ((((((!compare_translation)
        || ((a.as_ref().unwrap().tx == b.as_ref().unwrap().tx)
            && (a.as_ref().unwrap().ty == b.as_ref().unwrap().ty)))
        && (a.as_ref().unwrap().a == b.as_ref().unwrap().a))
        && (a.as_ref().unwrap().b == b.as_ref().unwrap().b))
        && (a.as_ref().unwrap().c == b.as_ref().unwrap().c))
        && (a.as_ref().unwrap().d == b.as_ref().unwrap().d));
}

// Source: upstream/packages/geometry/src/matrix.ts:163 (sha256:c021a7a92fa7399f6b1e4aa2477bbdef2c560d530531c10546d366d91caf740c)
pub fn inverse_matrix(out: &mut MatrixLike, source: &MatrixLike) -> bool {
    let a = source.a;
    let b = source.b;
    let c = source.c;
    let d = source.d;
    let tx = source.tx;
    let ty = source.ty;
    let det = ((a * d) - (c * b));
    if (det == 0.0_f64) {
        out.a = {
            out.b = {
                out.c = {
                    out.d = 0.0_f64;
                    out.d
                };
                out.c
            };
            out.b
        };
        out.tx = (-tx);
        out.ty = (-ty);
        return false;
    }
    let inv_det = (1.0_f64 / det);
    let out_a = (d * inv_det);
    let out_b = ((-b) * inv_det);
    let out_c = ((-c) * inv_det);
    let out_d = (a * inv_det);
    out.a = out_a;
    out.b = out_b;
    out.c = out_c;
    out.d = out_d;
    out.tx = (-((out_a * tx) + (out_c * ty)));
    out.ty = (-((out_b * tx) + (out_d * ty)));
    return true;
}

// Source: upstream/packages/geometry/src/matrix.ts:196 (sha256:9e7f3725a29805a0cb8b0e183b2bae6d552de8935bbf76e511f1047acae5a6e3)
pub fn inverse_matrix_transform_point(
    out: &mut Vector2Like,
    matrix: &MatrixLike,
    point: &Vector2Like,
) -> () {
    inverse_matrix_transform_point_xy(out, matrix, point.x, point.y);
}

// Source: upstream/packages/geometry/src/matrix.ts:204 (sha256:0945005b329ccf99d8c5ac291f34972cb0875a1682f888708db45f5ee2cf1ec5)
pub fn inverse_matrix_transform_point_xy(
    out: &mut Vector2Like,
    source: &MatrixLike,
    x: f64,
    y: f64,
) -> () {
    let norm = ((source.a * source.d) - (source.b * source.c));
    if (norm == 0.0_f64) {
        out.x = (-source.tx);
        out.y = (-source.ty);
    } else {
        let px = ((1.0_f64 / norm) * ((source.c * (source.ty - y)) + (source.d * (x - source.tx))));
        out.y = ((1.0_f64 / norm) * ((source.a * (y - source.ty)) + (source.b * (source.tx - x))));
        out.x = px;
    }
}

// Source: upstream/packages/geometry/src/matrix.ts:226 (sha256:1a8b8af520177bbedc283cb44822312e123ced462c1e098fcd8f0462da23a5ce)
pub fn inverse_matrix_transform_vector(
    out: &mut Vector2Like,
    matrix: &MatrixLike,
    vector: &Vector2Like,
) -> () {
    inverse_matrix_transform_vector_xy(out, matrix, vector.x, vector.y);
}

// Source: upstream/packages/geometry/src/matrix.ts:234 (sha256:f61560663ecc50bc5d35ce3e8f923dcee791c96041344839ad6c2c6b8715555f)
pub fn inverse_matrix_transform_vector_xy(
    out: &mut Vector2Like,
    source: &MatrixLike,
    x: f64,
    y: f64,
) -> () {
    let norm = ((source.a * source.d) - (source.b * source.c));
    if (norm == 0.0_f64) {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
    } else {
        let px = ((1.0_f64 / norm) * ((source.d * x) - (source.c * y)));
        out.y = ((1.0_f64 / norm) * (((-source.b) * x) + (source.a * y)));
        out.x = px;
    }
}

// Source: upstream/packages/geometry/src/matrix.ts:260 (sha256:021750a5df3b2beb74e9e805692baaa2312b3c0a16d884cdba836fef72e710f3)
pub fn matrix_transform_bounds(
    out: &mut RectangleLike,
    source: &MatrixLike,
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
) -> () {
    let a = source.a;
    let b = source.b;
    let c = source.c;
    let d = source.d;
    if ((ax == bx) && (ay == by)) {
        out.x = source.tx;
        out.y = source.ty;
        out.width = 0.0_f64;
        out.height = 0.0_f64;
        return;
    }
    let mut tx0 = ((a * ax) + (c * ay));
    let mut tx1 = tx0;
    let mut ty0 = ((b * ax) + (d * ay));
    let mut ty1 = ty0;
    let mut tx = ((a * bx) + (c * ay));
    let mut ty = ((b * bx) + (d * ay));
    if (tx < tx0) {
        tx0 = tx;
    }
    if (ty < ty0) {
        ty0 = ty;
    }
    if (tx > tx1) {
        tx1 = tx;
    }
    if (ty > ty1) {
        ty1 = ty;
    }
    tx = ((a * bx) + (c * by));
    ty = ((b * bx) + (d * by));
    if (tx < tx0) {
        tx0 = tx;
    }
    if (ty < ty0) {
        ty0 = ty;
    }
    if (tx > tx1) {
        tx1 = tx;
    }
    if (ty > ty1) {
        ty1 = ty;
    }
    tx = ((a * ax) + (c * by));
    ty = ((b * ax) + (d * by));
    if (tx < tx0) {
        tx0 = tx;
    }
    if (ty < ty0) {
        ty0 = ty;
    }
    if (tx > tx1) {
        tx1 = tx;
    }
    if (ty > ty1) {
        ty1 = ty;
    }
    out.x = (tx0 + source.tx);
    out.y = (ty0 + source.ty);
    out.width = (tx1 - tx0);
    out.height = (ty1 - ty0);
}

// Source: upstream/packages/geometry/src/matrix.ts:314 (sha256:45f938d4c8d9beca19d39c07032d7b13c24c59f306c801cabfb3f14c93a1861c)
pub fn matrix_transform_bounds_vector2(
    out: &mut RectangleLike,
    matrix: &MatrixLike,
    a: &Vector2Like,
    b: &Vector2Like,
) -> () {
    matrix_transform_bounds(out, matrix, a.x, a.y, b.x, b.y);
}

// Source: upstream/packages/geometry/src/matrix.ts:327 (sha256:16bbeda9bc34da7facc9f272c9137fa8b56307a8b89899870e51aada2651c8d7)
pub fn matrix_transform_point(
    out: &mut Vector2Like,
    matrix: &MatrixLike,
    point: &Vector2Like,
) -> () {
    matrix_transform_point_xy(out, matrix, point.x, point.y);
}

// Source: upstream/packages/geometry/src/matrix.ts:338 (sha256:21336cb8910f1acb4692d68a18e1fcb7820b1d0b8a02c322f76289e067bc639e)
pub fn matrix_transform_point_xy(out: &mut Vector2Like, source: &MatrixLike, x: f64, y: f64) -> () {
    out.x = (((x * source.a) + (y * source.c)) + source.tx);
    out.y = (((x * source.b) + (y * source.d)) + source.ty);
}

// Source: upstream/packages/geometry/src/matrix.ts:353 (sha256:3b777157deb027c06e7d1f003f029c4320da9897f6fa7a5d0f7d55b99567c50b)
pub fn matrix_transform_rectangle(
    out: &mut RectangleLike,
    matrix: &MatrixLike,
    source: &RectangleLike,
) -> () {
    matrix_transform_bounds(
        out,
        matrix,
        source.x,
        source.y,
        (source.x + source.width),
        (source.y + source.height),
    );
}

// Source: upstream/packages/geometry/src/matrix.ts:370 (sha256:4ccf309f68ab81c6eee016a411da214f1019ade0ac4ec5fe16133918861074fe)
pub fn matrix_transform_vector(
    out: &mut Vector2Like,
    matrix: &MatrixLike,
    vector: &Vector2Like,
) -> () {
    matrix_transform_vector_xy(out, matrix, vector.x, vector.y);
}

// Source: upstream/packages/geometry/src/matrix.ts:378 (sha256:c2f1458fa9b76d0b6cd21ede251926d579677ff5fc6d8d9dfd44a9bcd0dc2ef4)
pub fn matrix_transform_vector_xy(
    out: &mut Vector2Like,
    source: &MatrixLike,
    x: f64,
    y: f64,
) -> () {
    out.x = ((x * source.a) + (y * source.c));
    out.y = ((x * source.b) + (y * source.d));
}

// Source: upstream/packages/geometry/src/matrix.ts:388 (sha256:202f4e1396559a00cb5d86ecaa86ce585ab9955ee116b873d429432b123a18c5)
pub fn multiply_matrix(out: &mut MatrixLike, a: &MatrixLike, b: &MatrixLike) -> () {
    let a1 = a.a;
    let b1 = a.b;
    let tx1 = a.tx;
    let c1 = a.c;
    let d1 = a.d;
    let ty1 = a.ty;
    let a2 = b.a;
    let b2 = b.b;
    let tx2 = b.tx;
    let c2 = b.c;
    let d2 = b.d;
    let ty2 = b.ty;
    out.a = ((a1 * a2) + (c1 * b2));
    out.b = ((b1 * a2) + (d1 * b2));
    out.tx = (((a1 * tx2) + (c1 * ty2)) + tx1);
    out.c = ((a1 * c2) + (c1 * d2));
    out.d = ((b1 * c2) + (d1 * d2));
    out.ty = (((b1 * tx2) + (d1 * ty2)) + ty1);
}

// Source: upstream/packages/geometry/src/matrix.ts:417 (sha256:12c6347160802707944dfcb74ece7ca5d6d5cc6cfe296adb9a1267903087486d)
pub fn rotate_matrix(out: &mut MatrixLike, source: &MatrixLike, theta: f64) -> () {
    let cos = (theta).cos();
    let sin = (theta).sin();
    let mut a1 = ((source.a * cos) - (source.b * sin));
    out.b = ((source.a * sin) + (source.b * cos));
    out.a = a1;
    let mut c1 = ((source.c * cos) - (source.d * sin));
    out.d = ((source.c * sin) + (source.d * cos));
    out.c = c1;
    let mut tx1 = ((source.tx * cos) - (source.ty * sin));
    out.ty = ((source.tx * sin) + (source.ty * cos));
    out.tx = tx1;
}

// Source: upstream/packages/geometry/src/matrix.ts:450 (sha256:39f5659f12ee3ed08e18e2547d3446ff8817f854b3765ee7f3ef0a0755da6d9e)
pub fn scale_matrix(out: &mut MatrixLike, source: &MatrixLike, sx: f64, sy: f64) -> () {
    out.a = (source.a * sx);
    out.b = (source.b * sy);
    out.c = (source.c * sx);
    out.d = (source.d * sy);
    out.tx = (source.tx * sx);
    out.ty = (source.ty * sy);
}

// Source: upstream/packages/geometry/src/matrix.ts:472 (sha256:63ac57572d30259517af9ed7f6bed322d4e1845ea253fb79fbe12203983e4608)
pub fn set_gradient_transform_matrix(
    out: &mut MatrixLike,
    width: f64,
    height: f64,
    rotation: Option<f64>,
    tx: Option<f64>,
    ty: Option<f64>,
) -> () {
    let rotation = rotation.unwrap_or(0.0_f64);
    let tx = tx.unwrap_or(0.0_f64);
    let ty = ty.unwrap_or(0.0_f64);
    out.a = (width / 1638.4_f64);
    out.d = (height / 1638.4_f64);
    if (rotation != 0.0_f64) {
        let cos = (rotation).cos();
        let sin = (rotation).sin();
        out.b = (sin * out.d);
        out.c = ((-sin) * out.a);
        out.a *= cos;
        out.d *= cos;
    } else {
        out.b = 0.0_f64;
        out.c = 0.0_f64;
    }
    out.tx = (tx + (width / 2.0_f64));
    out.ty = (ty + (height / 2.0_f64));
}

// Source: upstream/packages/geometry/src/matrix.ts:501 (sha256:58a550dd4388f993c35d13773d652bd3767b6f57ecf4f539b5c1745eca5ff651)
pub fn set_matrix(out: &mut MatrixLike, a: f64, b: f64, c: f64, d: f64, tx: f64, ty: f64) -> () {
    out.a = a;
    out.b = b;
    out.c = c;
    out.d = d;
    out.tx = tx;
    out.ty = ty;
}

// Source: upstream/packages/geometry/src/matrix.ts:510 (sha256:ab06e48deda4fc6356ba12aedb00b5c53ca1b170d8c85ebaefca0e3d3ae56f1f)
pub fn set_matrix_from_float32_array(out: &mut MatrixLike, offset: f64, source: &Vec<f32>) -> () {
    out.a = (source[offset as usize] as f64);
    out.b = (source[(offset + 1.0_f64) as usize] as f64);
    out.c = (source[(offset + 2.0_f64) as usize] as f64);
    out.d = (source[(offset + 3.0_f64) as usize] as f64);
    out.tx = (source[(offset + 4.0_f64) as usize] as f64);
    out.ty = (source[(offset + 5.0_f64) as usize] as f64);
}

// Source: upstream/packages/geometry/src/matrix.ts:519 (sha256:6082b8297cadd9e1a42e300a15b9166bb02865f97a87990e2df72e7db5d195e5)
pub fn set_matrix_from_matrix3(out: &mut MatrixLike, source: &Matrix3Like) -> () {
    set_matrix(
        out,
        (source.m[0.0_f64 as usize] as f64),
        (source.m[3.0_f64 as usize] as f64),
        (source.m[1.0_f64 as usize] as f64),
        (source.m[4.0_f64 as usize] as f64),
        (source.m[6.0_f64 as usize] as f64),
        (source.m[7.0_f64 as usize] as f64),
    );
}

// Source: upstream/packages/geometry/src/matrix.ts:525 (sha256:abc1c832dfa335af5bf8bb416b954fb05e1292492358bf6ed86860c59ae32fc4)
pub fn set_matrix_from_matrix4(out: &mut MatrixLike, source: &Matrix4Like) -> () {
    out.a = (source.m[0.0_f64 as usize] as f64);
    out.b = (source.m[4.0_f64 as usize] as f64);
    out.tx = (source.m[12.0_f64 as usize] as f64);
    out.c = (source.m[1.0_f64 as usize] as f64);
    out.d = (source.m[5.0_f64 as usize] as f64);
    out.ty = (source.m[13.0_f64 as usize] as f64);
}

// Source: upstream/packages/geometry/src/matrix.ts:542 (sha256:5487d6a18e8ac0ad3f7b95c715dacf602df7b015f27fec301d8d24dc85f558e4)
pub fn set_matrix_identity(out: &mut MatrixLike) -> () {
    set_matrix(out, 1.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64);
}

// Source: upstream/packages/geometry/src/matrix.ts:551 (sha256:ba0670e7942cf8867a3f68111edef25eebd0c0cc3ac5d0f145fa2548461d68b5)
pub fn set_transform_matrix(
    out: &mut MatrixLike,
    scale_x: f64,
    scale_y: f64,
    rotation: Option<f64>,
    tx: Option<f64>,
    ty: Option<f64>,
) -> () {
    let rotation = rotation.unwrap_or(0.0_f64);
    let tx = tx.unwrap_or(0.0_f64);
    let ty = ty.unwrap_or(0.0_f64);
    if (rotation != 0.0_f64) {
        let cos = (rotation).cos();
        let sin = (rotation).sin();
        out.a = (cos * scale_x);
        out.b = (sin * scale_y);
        out.c = ((-sin) * scale_x);
        out.d = (cos * scale_y);
    } else {
        out.a = scale_x;
        out.b = 0.0_f64;
        out.c = 0.0_f64;
        out.d = scale_y;
    }
    out.tx = tx;
    out.ty = ty;
}

// Source: upstream/packages/geometry/src/matrix.ts:587 (sha256:342e6ba9e2ae8f9095029c1f8a73499d0584af1d6c20c22ad592a5d9dc8fa5b5)
pub fn translate_matrix(out: &mut MatrixLike, source: &MatrixLike, dx: f64, dy: f64) -> () {
    out.tx = (source.tx + dx);
    out.ty = (source.ty + dy);
}

// Source: upstream/packages/geometry/src/matrix.ts:595 (sha256:d15418396ab926bbc0fb0cef2fa663d39c4b653203d2f3656dab1cc66468d489)
pub fn translate_matrix_by_vector(
    out: &mut MatrixLike,
    matrix: &MatrixLike,
    vector: &Vector2Like,
) -> () {
    translate_matrix_by_vector_xy(out, matrix, vector.x, vector.y);
}

// Source: upstream/packages/geometry/src/matrix.ts:599 (sha256:ed4685c468d2fdd4131642d0c19047aa26a8ebae4ef322aa3d4490bb79a52b6a)
pub fn translate_matrix_by_vector_xy(
    out: &mut MatrixLike,
    source: &MatrixLike,
    x: f64,
    y: f64,
) -> () {
    out.tx = ((source.tx + (source.a * x)) + (source.c * y));
    out.ty = ((source.ty + (source.b * x)) + (source.d * y));
}

// Source: upstream/packages/geometry/src/matrix.ts:604 (sha256:c3ef28cb3fc57f071dd65fbe87d806d870eec6652dbf7fb223dc08b9cda4cb71)
pub fn write_matrix_to_float32_array(out: &mut Vec<f32>, offset: f64, source: &MatrixLike) -> () {
    out[offset as usize] = (source.a) as f32;
    out[(offset + 1.0_f64) as usize] = (source.b) as f32;
    out[(offset + 2.0_f64) as usize] = (source.c) as f32;
    out[(offset + 3.0_f64) as usize] = (source.d) as f32;
    out[(offset + 4.0_f64) as usize] = (source.tx) as f32;
    out[(offset + 5.0_f64) as usize] = (source.ty) as f32;
}
