// @generated from upstream/packages/geometry/src/vector2.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{Vector2, Vector2Like, Vector3Like};

// Source: upstream/packages/geometry/src/vector2.ts:4 (sha256:6a4da271037a57882ce076db0c5b88dc9f1882a8e9430a3e149b543c3d1027aa)
pub fn add_vector2(out: &mut Vector2Like, a: &Vector2Like, b: &Vector2Like) -> () {
    let ax = a.x;
    let ay = a.y;
    let bx = b.x;
    let by = b.y;
    out.x = (ax + bx);
    out.y = (ay + by);
}

// Source: upstream/packages/geometry/src/vector2.ts:18 (sha256:cd8405a8b8a464d84c28193b7eca2cae22848e2b871eb9b905a9cec6e1a1c376)
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

// Source: upstream/packages/geometry/src/vector2.ts:34 (sha256:109378da4bc5c9e6e98eb8d757340d490f4581dda821bdd22b25c12e8aa960b7)
pub fn clone_vector2(source: &Vector2Like) -> Vector2 {
    return create_vector2(Some(source.x), Some(source.y));
}

// Source: upstream/packages/geometry/src/vector2.ts:38 (sha256:d41b6271f1c18e29b10550f16052750a4dd046a05e891127dc49099a3aa6b5d6)
pub fn copy_vector2(out: &mut Vector2Like, source: &Vector2Like) -> () {
    let x = source.x;
    let y = source.y;
    out.x = x;
    out.y = y;
}

// Source: upstream/packages/geometry/src/vector2.ts:58 (sha256:93293dbf88b52d52f5ecaa6d3fd4852c4f4ee5d01f6d3c28557078645e1ee9c3)
pub fn create_vector2(x: Option<f64>, y: Option<f64>) -> Vector2 {
    return create_entity(Some(Vector2 {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        x: (x).unwrap_or(0.0_f64),
        y: (y).unwrap_or(0.0_f64),
    }));
}

// Source: upstream/packages/geometry/src/vector2.ts:62 (sha256:9980e29a450311b3962b3822f71230f3ed03764f8812a1f96365eecf5563a220)
pub fn create_vector2_from_polar(length: f64, angle: f64) -> Vector2 {
    let mut out = create_vector2(None, None);
    {
        out.x = (length * (angle).cos());
        out.y = (length * (angle).sin());
    };
    return out;
}

// Source: upstream/packages/geometry/src/vector2.ts:75 (sha256:39e72f35c72cd4816cc3b905cbdb3505c595a563855421bf3041303e886df321)
pub fn divide_vector2(out: &mut Vector2Like, source: &Vector2Like, divisor: &Vector2Like) -> () {
    let sx = source.x;
    let sy = source.y;
    let dx = divisor.x;
    let dy = divisor.y;
    out.x = if (dx != 0.0_f64) { (sx / dx) } else { 0.0_f64 };
    out.y = if (dy != 0.0_f64) { (sy / dy) } else { 0.0_f64 };
}

// Source: upstream/packages/geometry/src/vector2.ts:84 (sha256:4adbf64e66f4408ed79128b453a4822c134d6c286abac5f00e9f4eb7d2c18db4)
pub fn equals_vector2(a: &Option<Vector2Like>, b: &Option<Vector2Like>) -> bool {
    if ((a).is_none()) || ((b).is_none()) {
        return false;
    }
    return (a == b)
        || ((a.as_ref().unwrap().x == b.as_ref().unwrap().x)
            && (a.as_ref().unwrap().y == b.as_ref().unwrap().y));
}

// Source: upstream/packages/geometry/src/vector2.ts:97 (sha256:0df84bc722d03b719a934358655744e55915787b161ee1aca2469341e4ae03b1)
pub fn get_vector2_angle_between(a: &Vector2Like, b: &Vector2Like) -> f64 {
    let la = get_vector2_length(a);
    let lb = get_vector2_length(b);
    if (la == 0.0_f64) || (lb == 0.0_f64) {
        return f64::NAN;
    }
    let _dot = (get_vector2_dot(a, b) / (la * lb));
    return ((1.0_f64).min((-1.0_f64).max(_dot))).acos();
}

// Source: upstream/packages/geometry/src/vector2.ts:108 (sha256:76a7bdac7776b511211587dae76c656142a3044b35f5ad8df6a0fab88553b0d6)
pub fn get_vector2_distance(a: &Vector2Like, b: &Vector2Like) -> f64 {
    let dx = (a.x - b.x);
    let dy = (a.y - b.y);
    return ((dx * dx) + (dy * dy)).sqrt();
}

// Source: upstream/packages/geometry/src/vector2.ts:119 (sha256:a19030a7a77d74c779c71747278ef36e859b0ddc9bacdcc841e198d20e0136c0)
pub fn get_vector2_distance_squared(a: &Vector2Like, b: &Vector2Like) -> f64 {
    let dx = (a.x - b.x);
    let dy = (a.y - b.y);
    return ((dx).powf(2.0_f64) + (dy).powf(2.0_f64));
}

// Source: upstream/packages/geometry/src/vector2.ts:132 (sha256:26972ed82af38ac231ec4aa8964415f74c4a6b9f07c79ce264060c3952904e51)
pub fn get_vector2_dot(a: &Vector2Like, b: &Vector2Like) -> f64 {
    return ((a.x * b.x) + (a.y * b.y));
}

// Source: upstream/packages/geometry/src/vector2.ts:136 (sha256:3da3dfc672cd2f542053fb088e0921c3a0c77991add62d409c989108547c1a3f)
pub fn get_vector2_length(source: &Vector2Like) -> f64 {
    return ((source.x).powf(2.0_f64) + (source.y).powf(2.0_f64)).sqrt();
}

// Source: upstream/packages/geometry/src/vector2.ts:140 (sha256:e6e6c7833c83aab1722b3ded109ebd13bcb5e6ee6ad05cca3f7e11dfc6bd8ab1)
pub fn get_vector2_length_squared(source: &Vector2Like) -> f64 {
    return ((source.x).powf(2.0_f64) + (source.y).powf(2.0_f64));
}

// Source: upstream/packages/geometry/src/vector2.ts:147 (sha256:28d928ba40312a8c699289b502cdc8429fb66d2ac397ce7e8d633e93696fdc79)
pub fn interpolate_vector2(out: &mut Vector2Like, a: &Vector2Like, b: &Vector2Like, t: f64) -> () {
    let ax = a.x;
    let ay = a.y;
    let bx = b.x;
    let by = b.y;
    out.x = (ax + (t * (bx - ax)));
    out.y = (ay + (t * (by - ay)));
}

// Source: upstream/packages/geometry/src/vector2.ts:166 (sha256:a6933a095ba238480004b6f06bd059b040d80e66beaf61527ee5b389749b29f5)
pub fn max_vector2(out: &mut Vector2Like, a: &Vector2Like, b: &Vector2Like) -> () {
    let ax = a.x;
    let ay = a.y;
    let bx = b.x;
    let by = b.y;
    out.x = if (ax > bx) { ax } else { bx };
    out.y = if (ay > by) { ay } else { by };
}

// Source: upstream/packages/geometry/src/vector2.ts:180 (sha256:6b5d73189b3d9db21189428b0b78ac594f8d77cdcda5eab77016634fd4bc6202)
pub fn min_vector2(out: &mut Vector2Like, a: &Vector2Like, b: &Vector2Like) -> () {
    let ax = a.x;
    let ay = a.y;
    let bx = b.x;
    let by = b.y;
    out.x = if (ax < bx) { ax } else { bx };
    out.y = if (ay < by) { ay } else { by };
}

// Source: upstream/packages/geometry/src/vector2.ts:194 (sha256:038f7dc54d4c2f6fe240145efbdac447dc197d12b11f30db25da8b8345d3cb27)
pub fn multiply_vector2(out: &mut Vector2Like, a: &Vector2Like, b: &Vector2Like) -> () {
    let ax = a.x;
    let ay = a.y;
    let bx = b.x;
    let by = b.y;
    out.x = (ax * bx);
    out.y = (ay * by);
}

// Source: upstream/packages/geometry/src/vector2.ts:210 (sha256:c58d535fe2785827d73fab7ec370d7aa03ffe2132d6f1e737dc70b652a639974)
pub fn near_equals_vector2(a: &Vector2Like, b: &Vector2Like, tolerance: Option<f64>) -> bool {
    let tolerance = tolerance.unwrap_or(0.000001_f64);
    return ((a.x - b.x).abs() < tolerance) && ((a.y - b.y).abs() < tolerance);
}

// Source: upstream/packages/geometry/src/vector2.ts:223 (sha256:88292aa73dcd9dc14125a2898bce31fc20b8a4797e58231da01904c5900e606c)
pub fn negate_vector2(out: &mut Vector2Like, source: &Vector2Like) -> () {
    let x = source.x;
    let y = source.y;
    out.x = (x * (-1.0_f64));
    out.y = (y * (-1.0_f64));
}

// Source: upstream/packages/geometry/src/vector2.ts:236 (sha256:2abc6bf214a7d1759ff074b510f8f30ee7dc7f747e68306767da93eaea26af7f)
pub fn normalize_vector2(out: &mut Vector2Like, source: &Vector2Like) -> f64 {
    let x = source.x;
    let y = source.y;
    let l = ((x).powf(2.0_f64) + (y).powf(2.0_f64)).sqrt();
    if (l != 0.0_f64) {
        out.x = (x / l);
        out.y = (y / l);
    } else {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
    }
    return l;
}

// Source: upstream/packages/geometry/src/vector2.ts:252 (sha256:f00881f0e8e3f23de4fdbf54224c530d2b80571f6d89727f7d6f5faa72b4f499)
pub fn offset_vector2(out: &mut Vector2Like, source: &Vector2Like, dx: f64, dy: f64) -> () {
    let x = source.x;
    let y = source.y;
    out.x = (x + dx);
    out.y = (y + dy);
}

// Source: upstream/packages/geometry/src/vector2.ts:267 (sha256:807c8ec888a22e185b2273c8b0518cd6f24edaa769f40d32ea8aeb42463c250d)
pub fn reflect_vector2(out: &mut Vector2Like, incident: &Vector2Like, normal: &Vector2Like) -> () {
    let ix = incident.x;
    let iy = incident.y;
    let nx = normal.x;
    let ny = normal.y;
    let two_dot = (2.0_f64 * ((ix * nx) + (iy * ny)));
    out.x = (ix - (two_dot * nx));
    out.y = (iy - (two_dot * ny));
}

// Source: upstream/packages/geometry/src/vector2.ts:281 (sha256:6acec74c3d8a0496a7d85441291e0ebc0a53af8b74f296d47e14f2fb3c73c3ac)
pub fn scale_vector2(out: &mut Vector2Like, source: &Vector2Like, scalar: f64) -> () {
    let x = source.x;
    let y = source.y;
    out.x = (x * scalar);
    out.y = (y * scalar);
}

// Source: upstream/packages/geometry/src/vector2.ts:288 (sha256:54531df9218cfe0696a971c073c886ece549e45e630e3d42c4bd9fef7e739267)
pub fn scale_vector2_to_length(out: &mut Vector2Like, source: &Vector2Like, length: f64) -> () {
    let x = source.x;
    let y = source.y;
    let current_length = ((x).powf(2.0_f64) + (y).powf(2.0_f64)).sqrt();
    if (current_length == 0.0_f64) {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
    } else {
        let scale = (length / current_length);
        out.x = (x * scale);
        out.y = (y * scale);
    }
}

// Source: upstream/packages/geometry/src/vector2.ts:302 (sha256:58ee30325d5821904a67a9f1d17a80b633deae1f92fa74e8738b42901ab4bafa)
pub fn set_vector2(out: &mut Vector2Like, x: f64, y: f64) -> () {
    out.x = x;
    out.y = y;
}

// Source: upstream/packages/geometry/src/vector2.ts:307 (sha256:fb787edf266e8dce0f43a9998fea34d53b7ae6146d309ff1864642def8725cf0)
pub fn set_vector2_from_float32_array(out: &mut Vector2Like, offset: f64, source: &Vec<f32>) -> () {
    out.x = (source[offset as usize] as f64);
    out.y = (source[(offset + 1.0_f64) as usize] as f64);
}

// Source: upstream/packages/geometry/src/vector2.ts:312 (sha256:1a4372c4ed9bc4fafa13a949c7f15118512c479431fe7cce2510533d5b94ef03)
pub fn set_vector2_from_polar(out: &mut Vector2Like, length: f64, angle: f64) -> () {
    out.x = (length * (angle).cos());
    out.y = (length * (angle).sin());
}

// Source: upstream/packages/geometry/src/vector2.ts:322 (sha256:1482fc21a46f01395895d5d5d55963bb8bb15b61ae2a921d14c644b666c6a503)
pub fn set_vector2_from_vector3(out: &mut Vector2Like, source: &Vector3Like) -> () {
    out.x = source.x;
    out.y = source.y;
}

// Source: upstream/packages/geometry/src/vector2.ts:327 (sha256:614277406b6402761ca1e4ddeb757773732b3723af8f8a5387d63bbf19dafc7a)
pub fn subtract_vector2(out: &mut Vector2Like, source: &Vector2Like, other: &Vector2Like) -> () {
    let sx = source.x;
    let sy = source.y;
    let ox = other.x;
    let oy = other.y;
    out.x = (sx - ox);
    out.y = (sy - oy);
}

// Source: upstream/packages/geometry/src/vector2.ts:336 (sha256:1d08a88fe7c17024fd191fbcd6bdb871e294e56c324d674656f018164d1dfdf3)
pub fn write_vector2_to_float32_array(out: &mut Vec<f32>, offset: f64, source: &Vector2Like) -> () {
    out[offset as usize] = (source.x) as f32;
    out[(offset + 1.0_f64) as usize] = (source.y) as f32;
}

// Source: upstream/packages/geometry/src/vector2.ts:341 (sha256:b39963657b67e6c16269d2259f828c9e82951345ef085c209e584221caee7fcb)
pub static VECTOR2_X_AXIS: std::sync::LazyLock<Vector2> =
    std::sync::LazyLock::new(|| create_vector2(Some(1.0_f64), Some(0.0_f64)));

// Source: upstream/packages/geometry/src/vector2.ts:342 (sha256:dd1c3f8e2d077145cd2681c8c92bc06b1074abbd3aabf8422c6936055e7a76e5)
pub static VECTOR2_Y_AXIS: std::sync::LazyLock<Vector2> =
    std::sync::LazyLock::new(|| create_vector2(Some(0.0_f64), Some(1.0_f64)));
