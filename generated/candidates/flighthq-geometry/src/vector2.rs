// @generated from upstream/packages/geometry/src/vector2.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{Vector2, Vector2Like, Vector3Like};

// Source: upstream/packages/geometry/src/vector2.ts:4 (sha256:e08dad3f97fd08748ad9ecd7b5b418e802e839b3edb359a41664c947e1747bc6)
pub fn add_vector2(out: &mut Vector2Like, a: &Vector2Like, b: &Vector2Like) -> () {
    out.x = (a.x + b.x);
    out.y = (a.y + b.y);
}

// Source: upstream/packages/geometry/src/vector2.ts:14 (sha256:cd8405a8b8a464d84c28193b7eca2cae22848e2b871eb9b905a9cec6e1a1c376)
pub fn clamp_vector2(
    out: &mut Vector2Like,
    value: &Vector2Like,
    min: &Vector2Like,
    max: &Vector2Like,
) -> () {
    let vx = value.x;
    let vy = value.y;
    let min_x = min.x;
    let min_y = min.y;
    let max_x = max.x;
    let max_y = max.y;
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
}

// Source: upstream/packages/geometry/src/vector2.ts:30 (sha256:109378da4bc5c9e6e98eb8d757340d490f4581dda821bdd22b25c12e8aa960b7)
pub fn clone_vector2(source: &Vector2Like) -> Vector2 {
    return create_vector2(Some(source.x), Some(source.y));
}

// Source: upstream/packages/geometry/src/vector2.ts:34 (sha256:43d7e3b29591a240b4a3ba9a0a4b07e600af5ed3764ef55efcc47df474bce9eb)
pub fn copy_vector2(out: &mut Vector2Like, source: &Vector2Like) -> () {
    out.x = source.x;
    out.y = source.y;
}

// Source: upstream/packages/geometry/src/vector2.ts:52 (sha256:93293dbf88b52d52f5ecaa6d3fd4852c4f4ee5d01f6d3c28557078645e1ee9c3)
pub fn create_vector2(x: Option<f64>, y: Option<f64>) -> Vector2 {
    return create_entity(Some(Vector2 {
        __flight_identity: std::sync::Arc::new(()),
        x: (x).unwrap_or(0.0_f64),
        y: (y).unwrap_or(0.0_f64),
    }));
}

// Source: upstream/packages/geometry/src/vector2.ts:56 (sha256:9980e29a450311b3962b3822f71230f3ed03764f8812a1f96365eecf5563a220)
pub fn create_vector2_from_polar(length: f64, angle: f64) -> Vector2 {
    let mut out = create_vector2(None, None);
    set_vector2_from_polar(&mut out, length, angle);
    return (out).clone();
}

// Source: upstream/packages/geometry/src/vector2.ts:69 (sha256:39e72f35c72cd4816cc3b905cbdb3505c595a563855421bf3041303e886df321)
pub fn divide_vector2(out: &mut Vector2Like, source: &Vector2Like, divisor: &Vector2Like) -> () {
    let sx = source.x;
    let sy = source.y;
    let dx = divisor.x;
    let dy = divisor.y;
    out.x = if (dx != 0.0_f64) { (sx / dx) } else { 0.0_f64 };
    out.y = if (dy != 0.0_f64) { (sy / dy) } else { 0.0_f64 };
}

// Source: upstream/packages/geometry/src/vector2.ts:78 (sha256:4adbf64e66f4408ed79128b453a4822c134d6c286abac5f00e9f4eb7d2c18db4)
pub fn equals_vector2(a: Option<Vector2Like>, b: Option<Vector2Like>) -> bool {
    if ((a).is_none() || (b).is_none()) {
        return false;
    }
    return ((a == b)
        || ((a.as_ref().unwrap().x == b.as_ref().unwrap().x)
            && (a.as_ref().unwrap().y == b.as_ref().unwrap().y)));
}

// Source: upstream/packages/geometry/src/vector2.ts:91 (sha256:0df84bc722d03b719a934358655744e55915787b161ee1aca2469341e4ae03b1)
pub fn get_vector2_angle_between(a: &Vector2Like, b: &Vector2Like) -> f64 {
    let la = get_vector2_length(a);
    let lb = get_vector2_length(b);
    if ((la == 0.0_f64) || (lb == 0.0_f64)) {
        return f64::NAN;
    }
    let _dot = (get_vector2_dot(a, b) / (la * lb));
    return ((1.0_f64).min((-1.0_f64).max(_dot))).acos();
}

// Source: upstream/packages/geometry/src/vector2.ts:102 (sha256:76a7bdac7776b511211587dae76c656142a3044b35f5ad8df6a0fab88553b0d6)
pub fn get_vector2_distance(a: &Vector2Like, b: &Vector2Like) -> f64 {
    let dx = (a.x - b.x);
    let dy = (a.y - b.y);
    return ((dx * dx) + (dy * dy)).sqrt();
}

// Source: upstream/packages/geometry/src/vector2.ts:113 (sha256:a19030a7a77d74c779c71747278ef36e859b0ddc9bacdcc841e198d20e0136c0)
pub fn get_vector2_distance_squared(a: &Vector2Like, b: &Vector2Like) -> f64 {
    let dx = (a.x - b.x);
    let dy = (a.y - b.y);
    return ((dx).powf(2.0_f64) + (dy).powf(2.0_f64));
}

// Source: upstream/packages/geometry/src/vector2.ts:126 (sha256:26972ed82af38ac231ec4aa8964415f74c4a6b9f07c79ce264060c3952904e51)
pub fn get_vector2_dot(a: &Vector2Like, b: &Vector2Like) -> f64 {
    return ((a.x * b.x) + (a.y * b.y));
}

// Source: upstream/packages/geometry/src/vector2.ts:130 (sha256:3da3dfc672cd2f542053fb088e0921c3a0c77991add62d409c989108547c1a3f)
pub fn get_vector2_length(source: &Vector2Like) -> f64 {
    return ((source.x).powf(2.0_f64) + (source.y).powf(2.0_f64)).sqrt();
}

// Source: upstream/packages/geometry/src/vector2.ts:134 (sha256:e6e6c7833c83aab1722b3ded109ebd13bcb5e6ee6ad05cca3f7e11dfc6bd8ab1)
pub fn get_vector2_length_squared(source: &Vector2Like) -> f64 {
    return ((source.x).powf(2.0_f64) + (source.y).powf(2.0_f64));
}

// Source: upstream/packages/geometry/src/vector2.ts:141 (sha256:78e3f4b0c2769c1f9299395c170293f97882cb39fce81bc5d98b28158b22f43f)
pub fn interpolate_vector2(out: &mut Vector2Like, a: &Vector2Like, b: &Vector2Like, t: f64) -> () {
    out.x = (a.x + (t * (b.x - a.x)));
    out.y = (a.y + (t * (b.y - a.y)));
}

// Source: upstream/packages/geometry/src/vector2.ts:156 (sha256:11cc6ff1008a7c09c31f4f6e4ed8d6b0e22224b2cb06d289b5eb6fbe75e78181)
pub fn max_vector2(out: &mut Vector2Like, a: &Vector2Like, b: &Vector2Like) -> () {
    out.x = if (a.x > b.x) { a.x } else { b.x };
    out.y = if (a.y > b.y) { a.y } else { b.y };
}

// Source: upstream/packages/geometry/src/vector2.ts:166 (sha256:753deaab109a9a8421640e6c00ff94a0e137ebc449caf926dd5ec3913b494b9e)
pub fn min_vector2(out: &mut Vector2Like, a: &Vector2Like, b: &Vector2Like) -> () {
    out.x = if (a.x < b.x) { a.x } else { b.x };
    out.y = if (a.y < b.y) { a.y } else { b.y };
}

// Source: upstream/packages/geometry/src/vector2.ts:176 (sha256:1b20f4d1d018ce5306327c6fb706c97935a1b50b9e79dbd12e33f7c49be6adb1)
pub fn multiply_vector2(out: &mut Vector2Like, a: &Vector2Like, b: &Vector2Like) -> () {
    out.x = (a.x * b.x);
    out.y = (a.y * b.y);
}

// Source: upstream/packages/geometry/src/vector2.ts:188 (sha256:c58d535fe2785827d73fab7ec370d7aa03ffe2132d6f1e737dc70b652a639974)
pub fn near_equals_vector2(a: &Vector2Like, b: &Vector2Like, tolerance: Option<f64>) -> bool {
    let tolerance = tolerance.unwrap_or(0.000001_f64);
    return (((a.x - b.x).abs() < tolerance) && ((a.y - b.y).abs() < tolerance));
}

// Source: upstream/packages/geometry/src/vector2.ts:201 (sha256:75466adb581b30681d3c28eaf2953da23ffdd14a355b9a62292bea722b2031f7)
pub fn negate_vector2(out: &mut Vector2Like, source: &Vector2Like) -> () {
    out.x = (source.x * (-1.0_f64));
    out.y = (source.y * (-1.0_f64));
}

// Source: upstream/packages/geometry/src/vector2.ts:212 (sha256:5c4e16235016a1782bd0c41cc7e1f0fcaf4b0518796246ca4d80e4f805580377)
pub fn normalize_vector2(out: &mut Vector2Like, source: &Vector2Like) -> f64 {
    let l = get_vector2_length(source);
    if (l != 0.0_f64) {
        out.x = (source.x / l);
        out.y = (source.y / l);
    } else {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
    }
    return l;
}

// Source: upstream/packages/geometry/src/vector2.ts:226 (sha256:4a60194bb8bd3d3bf35f8feca3a8d965665d3a8bfcf9767cdf1fd258f4bf35da)
pub fn offset_vector2(out: &mut Vector2Like, source: &Vector2Like, dx: f64, dy: f64) -> () {
    out.x = (source.x + dx);
    out.y = (source.y + dy);
}

// Source: upstream/packages/geometry/src/vector2.ts:239 (sha256:807c8ec888a22e185b2273c8b0518cd6f24edaa769f40d32ea8aeb42463c250d)
pub fn reflect_vector2(out: &mut Vector2Like, incident: &Vector2Like, normal: &Vector2Like) -> () {
    let ix = incident.x;
    let iy = incident.y;
    let nx = normal.x;
    let ny = normal.y;
    let two_dot = (2.0_f64 * ((ix * nx) + (iy * ny)));
    out.x = (ix - (two_dot * nx));
    out.y = (iy - (two_dot * ny));
}

// Source: upstream/packages/geometry/src/vector2.ts:253 (sha256:1844759db3836e97a73b898b7f40681f88e5f30d8cc0ce1943dd6651095cb5fe)
pub fn scale_vector2(out: &mut Vector2Like, source: &Vector2Like, scalar: f64) -> () {
    out.x = (source.x * scalar);
    out.y = (source.y * scalar);
}

// Source: upstream/packages/geometry/src/vector2.ts:258 (sha256:adc88b9b72739da8389b03381127985f0b9d7649d0df421019ee676a9df8fd84)
pub fn scale_vector2_to_length(out: &mut Vector2Like, source: &Vector2Like, length: f64) -> () {
    let current_length = get_vector2_length(source);
    if (current_length == 0.0_f64) {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
    } else {
        let scale = (length / current_length);
        out.x = (source.x * scale);
        out.y = (source.y * scale);
    }
}

// Source: upstream/packages/geometry/src/vector2.ts:270 (sha256:58ee30325d5821904a67a9f1d17a80b633deae1f92fa74e8738b42901ab4bafa)
pub fn set_vector2(out: &mut Vector2Like, x: f64, y: f64) -> () {
    out.x = x;
    out.y = y;
}

// Source: upstream/packages/geometry/src/vector2.ts:275 (sha256:fb787edf266e8dce0f43a9998fea34d53b7ae6146d309ff1864642def8725cf0)
pub fn set_vector2_from_float32_array(out: &mut Vector2Like, offset: f64, source: &Vec<f32>) -> () {
    out.x = (source[offset as usize] as f64);
    out.y = (source[(offset + 1.0_f64) as usize] as f64);
}

// Source: upstream/packages/geometry/src/vector2.ts:280 (sha256:1a4372c4ed9bc4fafa13a949c7f15118512c479431fe7cce2510533d5b94ef03)
pub fn set_vector2_from_polar(out: &mut Vector2Like, length: f64, angle: f64) -> () {
    out.x = (length * (angle).cos());
    out.y = (length * (angle).sin());
}

// Source: upstream/packages/geometry/src/vector2.ts:290 (sha256:1482fc21a46f01395895d5d5d55963bb8bb15b61ae2a921d14c644b666c6a503)
pub fn set_vector2_from_vector3(out: &mut Vector2Like, source: &Vector3Like) -> () {
    out.x = source.x;
    out.y = source.y;
}

// Source: upstream/packages/geometry/src/vector2.ts:295 (sha256:f4809ec33248d03700fb4a53b2002022fc1f62f2841816003c3d5d7df30cb5a4)
pub fn subtract_vector2(out: &mut Vector2Like, source: &Vector2Like, other: &Vector2Like) -> () {
    out.x = (source.x - other.x);
    out.y = (source.y - other.y);
}

// Source: upstream/packages/geometry/src/vector2.ts:300 (sha256:1d08a88fe7c17024fd191fbcd6bdb871e294e56c324d674656f018164d1dfdf3)
pub fn write_vector2_to_float32_array(out: &mut Vec<f32>, offset: f64, source: &Vector2Like) -> () {
    out[offset as usize] = (source.x) as f32;
    out[(offset + 1.0_f64) as usize] = (source.y) as f32;
}

// Source: upstream/packages/geometry/src/vector2.ts:305 (sha256:b39963657b67e6c16269d2259f828c9e82951345ef085c209e584221caee7fcb)
pub static VECTOR2_X_AXIS: std::sync::LazyLock<Vector2> =
    std::sync::LazyLock::new(|| create_vector2(Some(1.0_f64), Some(0.0_f64)));

// Source: upstream/packages/geometry/src/vector2.ts:306 (sha256:dd1c3f8e2d077145cd2681c8c92bc06b1074abbd3aabf8422c6936055e7a76e5)
pub static VECTOR2_Y_AXIS: std::sync::LazyLock<Vector2> =
    std::sync::LazyLock::new(|| create_vector2(Some(0.0_f64), Some(1.0_f64)));
