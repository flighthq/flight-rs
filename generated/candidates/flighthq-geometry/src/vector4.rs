// @generated from upstream/packages/geometry/src/vector4.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{Vector3Like, Vector4, Vector4Like};

// Source: upstream/packages/geometry/src/vector4.ts:8 (sha256:1474299a981bfae4269804752a33d5b086de31fb4828bffa2f73c7a74309ebf0)
pub fn add_vector4(out: &mut Vector4Like, a: &Vector4Like, b: &Vector4Like) -> () {
    let ax = a.x;
    let ay = a.y;
    let az = a.z;
    let aw = a.w;
    let bx = b.x;
    let by = b.y;
    let bz = b.z;
    let bw = b.w;
    out.x = (ax + bx);
    out.y = (ay + by);
    out.z = (az + bz);
    out.w = (aw + bw);
}

// Source: upstream/packages/geometry/src/vector4.ts:28 (sha256:b1f4fe384fd702b6883f5516abd87d00cfdeeacd20f2772da9b0fd4282c3a352)
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

// Source: upstream/packages/geometry/src/vector4.ts:52 (sha256:7b6ceec0f42419dcc628adf3ab1499aed8ddf88681f895e4b4a966d0f34193fb)
pub fn clone_vector4(source: &Vector4Like) -> Vector4 {
    return create_vector4(
        Some(source.x),
        Some(source.y),
        Some(source.z),
        Some(source.w),
    );
}

// Source: upstream/packages/geometry/src/vector4.ts:59 (sha256:e008939f7b0f4e5d731729d7944fa78909d0a3f529d4a16fcb5082e41f0187c0)
pub fn copy_vector4(out: &mut Vector4Like, source: &Vector4Like) -> () {
    let x = source.x;
    let y = source.y;
    let z = source.z;
    let w = source.w;
    out.x = x;
    out.y = y;
    out.z = z;
    out.w = w;
}

// Source: upstream/packages/geometry/src/vector4.ts:88 (sha256:4554d1e32d87ac5c7123542b7586521e102d50134059acbaf14b3979886898d5)
pub fn create_vector4(x: Option<f64>, y: Option<f64>, z: Option<f64>, w: Option<f64>) -> Vector4 {
    return create_entity(Some(Vector4 {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        x: (x).clone().unwrap_or(0.0_f64),
        y: (y).clone().unwrap_or(0.0_f64),
        z: (z).clone().unwrap_or(0.0_f64),
        w: (w).clone().unwrap_or(0.0_f64),
    }));
}

// Source: upstream/packages/geometry/src/vector4.ts:99 (sha256:721ed30dee364ce92e1dad368c8044ba72ab73784aa925f365b5486b91edfed8)
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

// Source: upstream/packages/geometry/src/vector4.ts:114 (sha256:951809f69c2b7365e6e41f9006c58849de58dc9cf77b174b1f47d731d0f87433)
pub fn equals_vector4(a: &Option<Vector4Like>, b: &Option<Vector4Like>) -> bool {
    if ((a).is_none()) || ((b).is_none()) {
        return false;
    }
    return (a == b)
        || ((((a.as_ref().unwrap().x == b.as_ref().unwrap().x)
            && (a.as_ref().unwrap().y == b.as_ref().unwrap().y))
            && (a.as_ref().unwrap().z == b.as_ref().unwrap().z))
            && (a.as_ref().unwrap().w == b.as_ref().unwrap().w));
}

// Source: upstream/packages/geometry/src/vector4.ts:129 (sha256:4e1c2aa604d686cc5e2d5adc7055472c26fed3d9547cd06c7ee786d7203d2638)
pub fn get_vector4_angle_between(a: &Vector4Like, b: &Vector4Like) -> f64 {
    let la = get_vector4_length(a);
    let lb = get_vector4_length(b);
    if (la == 0.0_f64) || (lb == 0.0_f64) {
        return f64::NAN;
    }
    let _dot = (get_vector4_dot(a, b) / (la * lb));
    return ((1.0_f64).min((-1.0_f64).max(_dot))).acos();
}

// Source: upstream/packages/geometry/src/vector4.ts:143 (sha256:247b51f2a5edbed19118f76c4798d319c6d72783f5b5235427f3c865b2ed3c1f)
pub fn get_vector4_distance(a: &Vector4Like, b: &Vector4Like) -> f64 {
    let x: f64 = (b.x - a.x);
    let y: f64 = (b.y - a.y);
    let z: f64 = (b.z - a.z);
    let w: f64 = (b.w - a.w);
    return ((((x).powf(2.0_f64) + (y).powf(2.0_f64)) + (z).powf(2.0_f64)) + (w).powf(2.0_f64))
        .sqrt();
}

// Source: upstream/packages/geometry/src/vector4.ts:157 (sha256:0c4d408656595a2f4e0476d5ff5656560a0ea9398459d6e2584dc0230f5836fe)
pub fn get_vector4_distance_squared(a: &Vector4Like, b: &Vector4Like) -> f64 {
    let x: f64 = (b.x - a.x);
    let y: f64 = (b.y - a.y);
    let z: f64 = (b.z - a.z);
    let w: f64 = (b.w - a.w);
    return ((((x).powf(2.0_f64) + (y).powf(2.0_f64)) + (z).powf(2.0_f64)) + (w).powf(2.0_f64));
}

// Source: upstream/packages/geometry/src/vector4.ts:173 (sha256:f9aa63ae1755096c9b73b1663643a66e0205096e09b64ac8e6f950a2a87f068d)
pub fn get_vector4_dot(a: &Vector4Like, b: &Vector4Like) -> f64 {
    return ((((a.x * b.x) + (a.y * b.y)) + (a.z * b.z)) + (a.w * b.w));
}

// Source: upstream/packages/geometry/src/vector4.ts:182 (sha256:cab79e96e8290f394689cfc06d8ab8e0fc2498bd3febd3b9e2ac00fb7a76aeaf)
pub fn get_vector4_length(source: &Vector4Like) -> f64 {
    return ((((source.x).powf(2.0_f64) + (source.y).powf(2.0_f64)) + (source.z).powf(2.0_f64))
        + (source.w).powf(2.0_f64))
    .sqrt();
}

// Source: upstream/packages/geometry/src/vector4.ts:192 (sha256:d50dff8ac0933c340edaed60a2f25cb9fae48ea06b9d275f324f6b83a37a16da)
pub fn get_vector4_length_squared(source: &Vector4Like) -> f64 {
    return ((((source.x).powf(2.0_f64) + (source.y).powf(2.0_f64)) + (source.z).powf(2.0_f64))
        + (source.w).powf(2.0_f64));
}

// Source: upstream/packages/geometry/src/vector4.ts:202 (sha256:739ab591ca9f83aec2e951826dcedcf9f1316e0e0716902b4c1a8dbef66d44ea)
pub fn interpolate_vector4(out: &mut Vector4Like, a: &Vector4Like, b: &Vector4Like, t: f64) -> () {
    let ax = a.x;
    let ay = a.y;
    let az = a.z;
    let aw = a.w;
    let bx = b.x;
    let by = b.y;
    let bz = b.z;
    let bw = b.w;
    out.x = (ax + (t * (bx - ax)));
    out.y = (ay + (t * (by - ay)));
    out.z = (az + (t * (bz - az)));
    out.w = (aw + (t * (bw - aw)));
}

// Source: upstream/packages/geometry/src/vector4.ts:227 (sha256:a424e82a47e2bd575767822a383dccf1135264c4f7da6b6d7f46a25db0cad3f2)
pub fn max_vector4(out: &mut Vector4Like, a: &Vector4Like, b: &Vector4Like) -> () {
    let ax = a.x;
    let ay = a.y;
    let az = a.z;
    let aw = a.w;
    let bx = b.x;
    let by = b.y;
    let bz = b.z;
    let bw = b.w;
    out.x = if (ax > bx) { ax } else { bx };
    out.y = if (ay > by) { ay } else { by };
    out.z = if (az > bz) { az } else { bz };
    out.w = if (aw > bw) { aw } else { bw };
}

// Source: upstream/packages/geometry/src/vector4.ts:247 (sha256:a2ca40dba789edd3fa029b00f7632d0e151cff5d05902d79f40a828feb34907d)
pub fn min_vector4(out: &mut Vector4Like, a: &Vector4Like, b: &Vector4Like) -> () {
    let ax = a.x;
    let ay = a.y;
    let az = a.z;
    let aw = a.w;
    let bx = b.x;
    let by = b.y;
    let bz = b.z;
    let bw = b.w;
    out.x = if (ax < bx) { ax } else { bx };
    out.y = if (ay < by) { ay } else { by };
    out.z = if (az < bz) { az } else { bz };
    out.w = if (aw < bw) { aw } else { bw };
}

// Source: upstream/packages/geometry/src/vector4.ts:267 (sha256:10ff150f6ca5af983781596c1afbd4c7a0aef11fbdea36d648f13286f658f724)
pub fn multiply_vector4(out: &mut Vector4Like, a: &Vector4Like, b: &Vector4Like) -> () {
    let ax = a.x;
    let ay = a.y;
    let az = a.z;
    let aw = a.w;
    let bx = b.x;
    let by = b.y;
    let bz = b.z;
    let bw = b.w;
    out.x = (ax * bx);
    out.y = (ay * by);
    out.z = (az * bz);
    out.w = (aw * bw);
}

// Source: upstream/packages/geometry/src/vector4.ts:289 (sha256:63f5d9d5746929775ccff51ec9c00bd6a698275c5d0898fbddd8ef497bc0c21d)
pub fn near_equals_vector4(a: &Vector4Like, b: &Vector4Like, tolerance: Option<f64>) -> bool {
    let tolerance = tolerance.unwrap_or(0.000001_f64);
    return ((((a.x - b.x).abs() < tolerance) && ((a.y - b.y).abs() < tolerance))
        && ((a.z - b.z).abs() < tolerance))
        && ((a.w - b.w).abs() < tolerance);
}

// Source: upstream/packages/geometry/src/vector4.ts:307 (sha256:61a3d628d4405e9fe98cc63ff92b9d9defee9782da1d1cdee99e487b77ccec21)
pub fn negate_vector4(out: &mut Vector4Like, source: &Vector4Like) -> () {
    let x = source.x;
    let y = source.y;
    let z = source.z;
    let w = source.w;
    out.x = (x * (-1.0_f64));
    out.y = (y * (-1.0_f64));
    out.z = (z * (-1.0_f64));
    out.w = (w * (-1.0_f64));
}

// Source: upstream/packages/geometry/src/vector4.ts:324 (sha256:82f933182f895437a4935669691d0884c0db6d2a77fc1dcbed69b6d00b36823d)
pub fn normalize_vector4(out: &mut Vector4Like, source: &Vector4Like) -> f64 {
    let x = source.x;
    let y = source.y;
    let z = source.z;
    let w = source.w;
    let l =
        ((((x).powf(2.0_f64) + (y).powf(2.0_f64)) + (z).powf(2.0_f64)) + (w).powf(2.0_f64)).sqrt();
    if (l != 0.0_f64) {
        out.x = (x / l);
        out.y = (y / l);
        out.z = (z / l);
        out.w = (w / l);
    } else {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        out.z = 0.0_f64;
        out.w = 0.0_f64;
    }
    return l;
}

// Source: upstream/packages/geometry/src/vector4.ts:349 (sha256:caa77337f8644477d113ef85c52cc21e3af95eb1397313fca059571a436aa480)
pub fn offset_vector4(
    out: &mut Vector4Like,
    source: &Vector4Like,
    dx: f64,
    dy: f64,
    dz: f64,
    dw: f64,
) -> () {
    let x = source.x;
    let y = source.y;
    let z = source.z;
    let w = source.w;
    out.x = (x + dx);
    out.y = (y + dy);
    out.z = (z + dz);
    out.w = (w + dw);
}

// Source: upstream/packages/geometry/src/vector4.ts:371 (sha256:a15413833f6f891d3f809da7e4fd5f0945829f03e5392eec342d039b4469793c)
pub fn project_vector4(out: &mut Vector3Like, source: &Vector4Like) -> () {
    out.x = (source.x / source.w);
    out.y = (source.y / source.w);
    out.z = (source.z / source.w);
}

// Source: upstream/packages/geometry/src/vector4.ts:385 (sha256:95297422af18f8121c9877607958b634b18e431bbd1a9f3b9df8915ab677d485)
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

// Source: upstream/packages/geometry/src/vector4.ts:405 (sha256:492dda6089998d9741cd53e934fd93a3ed12b8912a9d1ad785d725f967ee1cb4)
pub fn scale_vector4(out: &mut Vector4Like, source: &Vector4Like, scalar: f64) -> () {
    let x = source.x;
    let y = source.y;
    let z = source.z;
    let w = source.w;
    out.x = (x * scalar);
    out.y = (y * scalar);
    out.z = (z * scalar);
    out.w = (w * scalar);
}

// Source: upstream/packages/geometry/src/vector4.ts:419 (sha256:e38065fe68ccf0b701233e7de13a055122028dd4f9c434568a7c0ae05fdddb78)
pub fn set_vector4(out: &mut Vector4Like, x: f64, y: f64, z: f64, w: f64) -> () {
    out.x = x;
    out.y = y;
    out.z = z;
    out.w = w;
}

// Source: upstream/packages/geometry/src/vector4.ts:429 (sha256:03d2237981389f311e26fdcec4911d4f48c7df006c65da7919afddd4db308b7d)
pub fn set_vector4_from_float32_array(out: &mut Vector4Like, offset: f64, source: &Vec<f32>) -> () {
    out.x = (source[offset as usize] as f64);
    out.y = (source[(offset + 1.0_f64) as usize] as f64);
    out.z = (source[(offset + 2.0_f64) as usize] as f64);
    out.w = (source[(offset + 3.0_f64) as usize] as f64);
}

// Source: upstream/packages/geometry/src/vector4.ts:442 (sha256:09e4240d19aa68631d6d137401289afbe4625b2bec6042350aab00442450ceec)
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

// Source: upstream/packages/geometry/src/vector4.ts:456 (sha256:b659ccee859f8f2ca5380785d28ed80bf9eb2064659606685c73f5f86faa3690)
pub fn subtract_vector4(out: &mut Vector4Like, source: &Vector4Like, other: &Vector4Like) -> () {
    let sx = source.x;
    let sy = source.y;
    let sz = source.z;
    let sw = source.w;
    let ox = other.x;
    let oy = other.y;
    let oz = other.z;
    let ow = other.w;
    out.x = (sx - ox);
    out.y = (sy - oy);
    out.z = (sz - oz);
    out.w = (sw - ow);
}

// Source: upstream/packages/geometry/src/vector4.ts:474 (sha256:ed031d0bc33b9eadb32524a3a80ee1090cdb697ee3fcc733b58d784ca79fabb9)
pub fn write_vector4_to_float32_array(out: &mut Vec<f32>, offset: f64, source: &Vector4Like) -> () {
    out[offset as usize] = (source.x) as f32;
    out[(offset + 1.0_f64) as usize] = (source.y) as f32;
    out[(offset + 2.0_f64) as usize] = (source.z) as f32;
    out[(offset + 3.0_f64) as usize] = (source.w) as f32;
}

// Source: upstream/packages/geometry/src/vector4.ts:481 (sha256:7d47ae771aab8ba8ebecca7cdf91637f2dbc04c7d45f53ffb0ce608cf72b2a0e)
pub static VECTOR4_W_UNIT: std::sync::LazyLock<Vector4> = std::sync::LazyLock::new(|| {
    create_vector4(Some(0.0_f64), Some(0.0_f64), Some(0.0_f64), Some(1.0_f64))
});

// Source: upstream/packages/geometry/src/vector4.ts:482 (sha256:b0a3f725d0a69a81bf597931d8e2aae3327d6ecd694e95ae0f83b21bf07366fa)
pub static VECTOR4_X_AXIS: std::sync::LazyLock<Vector4> = std::sync::LazyLock::new(|| {
    create_vector4(Some(1.0_f64), Some(0.0_f64), Some(0.0_f64), Some(0.0_f64))
});

// Source: upstream/packages/geometry/src/vector4.ts:483 (sha256:1fa7153cdfdd18887c04f47972048f1cfd74e9cff2b6cb9da3f3ad23dd25e5b4)
pub static VECTOR4_Y_AXIS: std::sync::LazyLock<Vector4> = std::sync::LazyLock::new(|| {
    create_vector4(Some(0.0_f64), Some(1.0_f64), Some(0.0_f64), Some(0.0_f64))
});

// Source: upstream/packages/geometry/src/vector4.ts:484 (sha256:819c425cd78a8204d9780ef745d5bce4dc0982c706a1ed7cd1fd7f022fb1acde)
pub static VECTOR4_Z_AXIS: std::sync::LazyLock<Vector4> = std::sync::LazyLock::new(|| {
    create_vector4(Some(0.0_f64), Some(0.0_f64), Some(1.0_f64), Some(0.0_f64))
});
