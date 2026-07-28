// @generated from upstream/packages/geometry/src/vector4.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{Vector3Like, Vector4, Vector4Like};

// Source: upstream/packages/geometry/src/vector4.ts:8 (sha256:e26996b7b7752050d97a673913bb3f9d21df58d915638443e3700fe59a48d750)
pub fn add_vector4(out: &mut Vector4Like, a: &Vector4Like, b: &Vector4Like) -> () {
    out.x = (a.x + b.x);
    out.y = (a.y + b.y);
    out.z = (a.z + b.z);
    out.w = (a.w + b.w);
}

// Source: upstream/packages/geometry/src/vector4.ts:20 (sha256:b1f4fe384fd702b6883f5516abd87d00cfdeeacd20f2772da9b0fd4282c3a352)
pub fn clamp_vector4(
    out: &mut Vector4Like,
    value: &Vector4Like,
    min: &Vector4Like,
    max: &Vector4Like,
) -> () {
    let vx = value.x;
    let vy = value.y;
    let vz = value.z;
    let vw = value.w;
    let min_x = min.x;
    let min_y = min.y;
    let min_z = min.z;
    let min_w = min.w;
    let max_x = max.x;
    let max_y = max.y;
    let max_z = max.z;
    let max_w = max.w;
    out.x = if (vx < min_x) {
        min_x
    } else {
        if (vx > max_x) { max_x } else { vx }
    };
    out.y = if (vy < min_y) {
        min_y
    } else {
        if (vy > max_y) { max_y } else { vy }
    };
    out.z = if (vz < min_z) {
        min_z
    } else {
        if (vz > max_z) { max_z } else { vz }
    };
    out.w = if (vw < min_w) {
        min_w
    } else {
        if (vw > max_w) { max_w } else { vw }
    };
}

// Source: upstream/packages/geometry/src/vector4.ts:44 (sha256:7b6ceec0f42419dcc628adf3ab1499aed8ddf88681f895e4b4a966d0f34193fb)
pub fn clone_vector4(source: &Vector4Like) -> Vector4 {
    return create_vector4(
        Some(source.x),
        Some(source.y),
        Some(source.z),
        Some(source.w),
    );
}

// Source: upstream/packages/geometry/src/vector4.ts:51 (sha256:e6aa9dcea24557a66edb18fbb59f160ccf997153e749a21d4488a127a26a9df1)
pub fn copy_vector4(out: &mut Vector4Like, source: &Vector4Like) -> () {
    out.x = source.x;
    out.y = source.y;
    out.z = source.z;
    out.w = source.w;
}

// Source: upstream/packages/geometry/src/vector4.ts:76 (sha256:4554d1e32d87ac5c7123542b7586521e102d50134059acbaf14b3979886898d5)
pub fn create_vector4(x: Option<f64>, y: Option<f64>, z: Option<f64>, w: Option<f64>) -> Vector4 {
    return create_entity(Some(Vector4 {
        __flight_identity: std::sync::Arc::new(()),
        x: (x).unwrap_or(0.0_f64),
        y: (y).unwrap_or(0.0_f64),
        z: (z).unwrap_or(0.0_f64),
        w: (w).unwrap_or(0.0_f64),
    }));
}

// Source: upstream/packages/geometry/src/vector4.ts:87 (sha256:721ed30dee364ce92e1dad368c8044ba72ab73784aa925f365b5486b91edfed8)
pub fn divide_vector4(out: &mut Vector4Like, source: &Vector4Like, divisor: &Vector4Like) -> () {
    let sx = source.x;
    let sy = source.y;
    let sz = source.z;
    let sw = source.w;
    let dx = divisor.x;
    let dy = divisor.y;
    let dz = divisor.z;
    let dw = divisor.w;
    out.x = if (dx != 0.0_f64) { (sx / dx) } else { 0.0_f64 };
    out.y = if (dy != 0.0_f64) { (sy / dy) } else { 0.0_f64 };
    out.z = if (dz != 0.0_f64) { (sz / dz) } else { 0.0_f64 };
    out.w = if (dw != 0.0_f64) { (sw / dw) } else { 0.0_f64 };
}

// Source: upstream/packages/geometry/src/vector4.ts:102 (sha256:a946b6736f0922e8a43277db83b2f615e6557d7bed4f0f8e9987a96a62bba13f)
pub fn equals_vector4(a: Option<Vector4Like>, b: Option<Vector4Like>) -> bool {
    if ((a).is_none() || (b).is_none()) {
        return false;
    }
    return ((((a.as_ref().unwrap().x == b.as_ref().unwrap().x)
        && (a.as_ref().unwrap().y == b.as_ref().unwrap().y))
        && (a.as_ref().unwrap().z == b.as_ref().unwrap().z))
        && (a.as_ref().unwrap().w == b.as_ref().unwrap().w));
}

// Source: upstream/packages/geometry/src/vector4.ts:115 (sha256:4e1c2aa604d686cc5e2d5adc7055472c26fed3d9547cd06c7ee786d7203d2638)
pub fn get_vector4_angle_between(a: &Vector4Like, b: &Vector4Like) -> f64 {
    let la = get_vector4_length(a);
    let lb = get_vector4_length(b);
    if ((la == 0.0_f64) || (lb == 0.0_f64)) {
        return f64::NAN;
    }
    let _dot = (get_vector4_dot(a, b) / (la * lb));
    return ((1.0_f64).min((-1.0_f64).max(_dot))).acos();
}

// Source: upstream/packages/geometry/src/vector4.ts:129 (sha256:247b51f2a5edbed19118f76c4798d319c6d72783f5b5235427f3c865b2ed3c1f)
pub fn get_vector4_distance(a: &Vector4Like, b: &Vector4Like) -> f64 {
    let x: f64 = (b.x - a.x);
    let y: f64 = (b.y - a.y);
    let z: f64 = (b.z - a.z);
    let w: f64 = (b.w - a.w);
    return ((((x).powf(2.0_f64) + (y).powf(2.0_f64)) + (z).powf(2.0_f64)) + (w).powf(2.0_f64))
        .sqrt();
}

// Source: upstream/packages/geometry/src/vector4.ts:143 (sha256:0c4d408656595a2f4e0476d5ff5656560a0ea9398459d6e2584dc0230f5836fe)
pub fn get_vector4_distance_squared(a: &Vector4Like, b: &Vector4Like) -> f64 {
    let x: f64 = (b.x - a.x);
    let y: f64 = (b.y - a.y);
    let z: f64 = (b.z - a.z);
    let w: f64 = (b.w - a.w);
    return ((((x).powf(2.0_f64) + (y).powf(2.0_f64)) + (z).powf(2.0_f64)) + (w).powf(2.0_f64));
}

// Source: upstream/packages/geometry/src/vector4.ts:159 (sha256:f9aa63ae1755096c9b73b1663643a66e0205096e09b64ac8e6f950a2a87f068d)
pub fn get_vector4_dot(a: &Vector4Like, b: &Vector4Like) -> f64 {
    return ((((a.x * b.x) + (a.y * b.y)) + (a.z * b.z)) + (a.w * b.w));
}

// Source: upstream/packages/geometry/src/vector4.ts:168 (sha256:cab79e96e8290f394689cfc06d8ab8e0fc2498bd3febd3b9e2ac00fb7a76aeaf)
pub fn get_vector4_length(source: &Vector4Like) -> f64 {
    return ((((source.x).powf(2.0_f64) + (source.y).powf(2.0_f64)) + (source.z).powf(2.0_f64))
        + (source.w).powf(2.0_f64))
    .sqrt();
}

// Source: upstream/packages/geometry/src/vector4.ts:178 (sha256:d50dff8ac0933c340edaed60a2f25cb9fae48ea06b9d275f324f6b83a37a16da)
pub fn get_vector4_length_squared(source: &Vector4Like) -> f64 {
    return ((((source.x).powf(2.0_f64) + (source.y).powf(2.0_f64)) + (source.z).powf(2.0_f64))
        + (source.w).powf(2.0_f64));
}

// Source: upstream/packages/geometry/src/vector4.ts:188 (sha256:654c672b10bfd843e978234637a541dbc584c035d4b03cac49bb7995def767fa)
pub fn interpolate_vector4(out: &mut Vector4Like, a: &Vector4Like, b: &Vector4Like, t: f64) -> () {
    let ax = a.x;
    let ay = a.y;
    let az = a.z;
    let aw = a.w;
    out.x = (ax + (t * (b.x - ax)));
    out.y = (ay + (t * (b.y - ay)));
    out.z = (az + (t * (b.z - az)));
    out.w = (aw + (t * (b.w - aw)));
}

// Source: upstream/packages/geometry/src/vector4.ts:209 (sha256:e06a658354c0a93c270dc3cc36bcfe97e3f7234a4e695525e8c8f0a0cb063c77)
pub fn max_vector4(out: &mut Vector4Like, a: &Vector4Like, b: &Vector4Like) -> () {
    out.x = if (a.x > b.x) { a.x } else { b.x };
    out.y = if (a.y > b.y) { a.y } else { b.y };
    out.z = if (a.z > b.z) { a.z } else { b.z };
    out.w = if (a.w > b.w) { a.w } else { b.w };
}

// Source: upstream/packages/geometry/src/vector4.ts:221 (sha256:0a5103e4a11775c32cbaff9ed47dbc09d946d3dc7cf03747fdf10e165e9cd552)
pub fn min_vector4(out: &mut Vector4Like, a: &Vector4Like, b: &Vector4Like) -> () {
    out.x = if (a.x < b.x) { a.x } else { b.x };
    out.y = if (a.y < b.y) { a.y } else { b.y };
    out.z = if (a.z < b.z) { a.z } else { b.z };
    out.w = if (a.w < b.w) { a.w } else { b.w };
}

// Source: upstream/packages/geometry/src/vector4.ts:233 (sha256:31b75074c26066a7b39df9f80d435e1dedb79adfe68aaec8d4f8862203368fe3)
pub fn multiply_vector4(out: &mut Vector4Like, a: &Vector4Like, b: &Vector4Like) -> () {
    out.x = (a.x * b.x);
    out.y = (a.y * b.y);
    out.z = (a.z * b.z);
    out.w = (a.w * b.w);
}

// Source: upstream/packages/geometry/src/vector4.ts:247 (sha256:63f5d9d5746929775ccff51ec9c00bd6a698275c5d0898fbddd8ef497bc0c21d)
pub fn near_equals_vector4(a: &Vector4Like, b: &Vector4Like, tolerance: Option<f64>) -> bool {
    let tolerance = tolerance.unwrap_or(0.000001_f64);
    return (((((a.x - b.x).abs() < tolerance) && ((a.y - b.y).abs() < tolerance))
        && ((a.z - b.z).abs() < tolerance))
        && ((a.w - b.w).abs() < tolerance));
}

// Source: upstream/packages/geometry/src/vector4.ts:265 (sha256:f41123c0f8678870845c03e597ee9bc8ae0ba95b5f00392a1f0ce1cc7f539fd0)
pub fn negate_vector4(out: &mut Vector4Like, source: &Vector4Like) -> () {
    out.x = (source.x * (-1.0_f64));
    out.y = (source.y * (-1.0_f64));
    out.z = (source.z * (-1.0_f64));
    out.w = (source.w * (-1.0_f64));
}

// Source: upstream/packages/geometry/src/vector4.ts:278 (sha256:b42f7b3ec507b627f559d7ce539fa75e667844dd2699c623d82e33b679c526cd)
pub fn normalize_vector4(out: &mut Vector4Like, source: &Vector4Like) -> f64 {
    let l = get_vector4_length(source);
    if (l != 0.0_f64) {
        out.x = (source.x / l);
        out.y = (source.y / l);
        out.z = (source.z / l);
        out.w = (source.w / l);
    } else {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        out.z = 0.0_f64;
        out.w = 0.0_f64;
    }
    return l;
}

// Source: upstream/packages/geometry/src/vector4.ts:299 (sha256:8daf5c711ca4d410d387c26af5373d837d7f4c74b38080d8b124590411678797)
pub fn offset_vector4(
    out: &mut Vector4Like,
    source: &Vector4Like,
    dx: f64,
    dy: f64,
    dz: f64,
    dw: f64,
) -> () {
    out.x = (source.x + dx);
    out.y = (source.y + dy);
    out.z = (source.z + dz);
    out.w = (source.w + dw);
}

// Source: upstream/packages/geometry/src/vector4.ts:317 (sha256:a15413833f6f891d3f809da7e4fd5f0945829f03e5392eec342d039b4469793c)
pub fn project_vector4(out: &mut Vector3Like, source: &Vector4Like) -> () {
    out.x = (source.x / source.w);
    out.y = (source.y / source.w);
    out.z = (source.z / source.w);
}

// Source: upstream/packages/geometry/src/vector4.ts:331 (sha256:95297422af18f8121c9877607958b634b18e431bbd1a9f3b9df8915ab677d485)
pub fn reflect_vector4(out: &mut Vector4Like, incident: &Vector4Like, normal: &Vector4Like) -> () {
    let ix = incident.x;
    let iy = incident.y;
    let iz = incident.z;
    let iw = incident.w;
    let nx = normal.x;
    let ny = normal.y;
    let nz = normal.z;
    let nw = normal.w;
    let two_dot = (2.0_f64 * ((((ix * nx) + (iy * ny)) + (iz * nz)) + (iw * nw)));
    out.x = (ix - (two_dot * nx));
    out.y = (iy - (two_dot * ny));
    out.z = (iz - (two_dot * nz));
    out.w = (iw - (two_dot * nw));
}

// Source: upstream/packages/geometry/src/vector4.ts:351 (sha256:c0b9106ad55a8a79ad63b8aac311621dc0651de4efa3a6ed6ed3a57560c93649)
pub fn scale_vector4(out: &mut Vector4Like, source: &Vector4Like, scalar: f64) -> () {
    out.x = (source.x * scalar);
    out.y = (source.y * scalar);
    out.z = (source.z * scalar);
    out.w = (source.w * scalar);
}

// Source: upstream/packages/geometry/src/vector4.ts:361 (sha256:e38065fe68ccf0b701233e7de13a055122028dd4f9c434568a7c0ae05fdddb78)
pub fn set_vector4(out: &mut Vector4Like, x: f64, y: f64, z: f64, w: f64) -> () {
    out.x = x;
    out.y = y;
    out.z = z;
    out.w = w;
}

// Source: upstream/packages/geometry/src/vector4.ts:371 (sha256:03d2237981389f311e26fdcec4911d4f48c7df006c65da7919afddd4db308b7d)
pub fn set_vector4_from_float32_array(out: &mut Vector4Like, offset: f64, source: &Vec<f32>) -> () {
    out.x = (source[offset as usize] as f64);
    out.y = (source[(offset + 1.0_f64) as usize] as f64);
    out.z = (source[(offset + 2.0_f64) as usize] as f64);
    out.w = (source[(offset + 3.0_f64) as usize] as f64);
}

// Source: upstream/packages/geometry/src/vector4.ts:384 (sha256:09e4240d19aa68631d6d137401289afbe4625b2bec6042350aab00442450ceec)
pub fn set_vector4_from_vector3(out: &mut Vector4Like, source: &Vector3Like, w: Option<f64>) -> () {
    let w = w.unwrap_or(0.0_f64);
    let x = source.x;
    let y = source.y;
    let z = source.z;
    out.x = x;
    out.y = y;
    out.z = z;
    out.w = w;
}

// Source: upstream/packages/geometry/src/vector4.ts:398 (sha256:26f7ea77fffee7159eb73f4417a0af1d1aa029d32eb5d4d95e34a056147edefc)
pub fn subtract_vector4(out: &mut Vector4Like, source: &Vector4Like, other: &Vector4Like) -> () {
    out.x = (source.x - other.x);
    out.y = (source.y - other.y);
    out.z = (source.z - other.z);
    out.w = (source.w - other.w);
}

// Source: upstream/packages/geometry/src/vector4.ts:408 (sha256:ed031d0bc33b9eadb32524a3a80ee1090cdb697ee3fcc733b58d784ca79fabb9)
pub fn write_vector4_to_float32_array(out: &mut Vec<f32>, offset: f64, source: &Vector4Like) -> () {
    out[offset as usize] = (source.x) as f32;
    out[(offset + 1.0_f64) as usize] = (source.y) as f32;
    out[(offset + 2.0_f64) as usize] = (source.z) as f32;
    out[(offset + 3.0_f64) as usize] = (source.w) as f32;
}

// Source: upstream/packages/geometry/src/vector4.ts:415 (sha256:7d47ae771aab8ba8ebecca7cdf91637f2dbc04c7d45f53ffb0ce608cf72b2a0e)
pub static VECTOR4_W_UNIT: std::sync::LazyLock<Vector4> = std::sync::LazyLock::new(|| {
    create_vector4(Some(0.0_f64), Some(0.0_f64), Some(0.0_f64), Some(1.0_f64))
});

// Source: upstream/packages/geometry/src/vector4.ts:416 (sha256:b0a3f725d0a69a81bf597931d8e2aae3327d6ecd694e95ae0f83b21bf07366fa)
pub static VECTOR4_X_AXIS: std::sync::LazyLock<Vector4> = std::sync::LazyLock::new(|| {
    create_vector4(Some(1.0_f64), Some(0.0_f64), Some(0.0_f64), Some(0.0_f64))
});

// Source: upstream/packages/geometry/src/vector4.ts:417 (sha256:1fa7153cdfdd18887c04f47972048f1cfd74e9cff2b6cb9da3f3ad23dd25e5b4)
pub static VECTOR4_Y_AXIS: std::sync::LazyLock<Vector4> = std::sync::LazyLock::new(|| {
    create_vector4(Some(0.0_f64), Some(1.0_f64), Some(0.0_f64), Some(0.0_f64))
});

// Source: upstream/packages/geometry/src/vector4.ts:418 (sha256:819c425cd78a8204d9780ef745d5bce4dc0982c706a1ed7cd1fd7f022fb1acde)
pub static VECTOR4_Z_AXIS: std::sync::LazyLock<Vector4> = std::sync::LazyLock::new(|| {
    create_vector4(Some(0.0_f64), Some(0.0_f64), Some(1.0_f64), Some(0.0_f64))
});
