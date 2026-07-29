// @generated from upstream/packages/geometry/src/vector3.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{Vector2Like, Vector3, Vector3Like, Vector4Like};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub m: Vec<f32>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/geometry/src/vector3.ts:8 (sha256:c5afbbcaba93725b6dbbe95332676d5c775ccb24c3f125bcd7e53295fceb8abd)
pub fn add_vector3(out: &mut Vector3Like, a: &Vector3Like, b: &Vector3Like) -> () {
    out.x = (a.x + b.x);
    out.y = (a.y + b.y);
    out.z = (a.z + b.z);
}

// Source: upstream/packages/geometry/src/vector3.ts:19 (sha256:bf4d96491c8418f65a6411a14ec8d0d1aa31690ea2b0e34635ef232562b1c0e7)
pub fn clamp_vector3(
    out: &mut Vector3Like,
    value: &Vector3Like,
    min: &Vector3Like,
    max: &Vector3Like,
) -> () {
    let vx = value.x;
    let vy = value.y;
    let vz = value.z;
    let min_x = min.x;
    let min_y = min.y;
    let min_z = min.z;
    let max_x = max.x;
    let max_y = max.y;
    let max_z = max.z;
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
}

// Source: upstream/packages/geometry/src/vector3.ts:39 (sha256:939c4a3639ba2af60dba17ce32017319d6081f4b9df15c62d8a551acc5ce6792)
pub fn clone_vector3(source: &Vector3Like) -> Vector3 {
    return create_vector3(Some(source.x), Some(source.y), Some(source.z));
}

// Source: upstream/packages/geometry/src/vector3.ts:46 (sha256:198db84efe7444f9079461f65e94f7207e050d615062fa58df5814b1fc811f68)
pub fn copy_vector3(out: &mut Vector3Like, source: &Vector3Like) -> () {
    out.x = source.x;
    out.y = source.y;
    out.z = source.z;
}

// Source: upstream/packages/geometry/src/vector3.ts:73 (sha256:8223d8aa5ec4379f054e164afeac9007b95daae916b591cd6b7a17f0a683f243)
pub fn create_vector3(x: Option<f64>, y: Option<f64>, z: Option<f64>) -> Vector3 {
    return create_entity(Some(Vector3 {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        x: (x).unwrap_or(0.0_f64),
        y: (y).unwrap_or(0.0_f64),
        z: (z).unwrap_or(0.0_f64),
    }));
}

// Source: upstream/packages/geometry/src/vector3.ts:84 (sha256:0693b3d9077e187d51772f4871aaf17cf69c85560d6ac66c73320d3e834c4894)
pub fn create_vector3_from_spherical(radius: f64, theta: f64, phi: f64) -> Vector3 {
    let mut out = create_vector3(None, None, None);
    set_vector3_from_spherical(&mut out, radius, theta, phi);
    return out;
}

// Source: upstream/packages/geometry/src/vector3.ts:95 (sha256:6d0843add672a56ea17f712227f5da6ed773302c18e2c02b01f644e033b0890f)
pub fn cross_vector3(out: &mut Vector3Like, source: &Vector3Like, other: &Vector3Like) -> () {
    let x = ((source.y * other.z) - (source.z * other.y));
    let y = ((source.z * other.x) - (source.x * other.z));
    let z = ((source.x * other.y) - (source.y * other.x));
    out.x = x;
    out.y = y;
    out.z = z;
}

// Source: upstream/packages/geometry/src/vector3.ts:111 (sha256:b3bcfc1e705823613d7850a956318fe9553349e29cce236462ee298c2d7850ac)
pub fn divide_vector3(out: &mut Vector3Like, source: &Vector3Like, divisor: &Vector3Like) -> () {
    let sx = source.x;
    let sy = source.y;
    let sz = source.z;
    let dx = divisor.x;
    let dy = divisor.y;
    let dz = divisor.z;
    out.x = if (dx != 0.0_f64) { (sx / dx) } else { 0.0_f64 };
    out.y = if (dy != 0.0_f64) { (sy / dy) } else { 0.0_f64 };
    out.z = if (dz != 0.0_f64) { (sz / dz) } else { 0.0_f64 };
}

// Source: upstream/packages/geometry/src/vector3.ts:123 (sha256:ca2ed710b8cd020bb999d7b5efa0fe89199a4d5ea0f2e0514b877010b241a626)
pub fn equals_vector3(a: Option<Vector3Like>, b: Option<Vector3Like>) -> bool {
    if ((a).is_none()) || ((b).is_none()) {
        return false;
    }
    return ((a.as_ref().unwrap().x == b.as_ref().unwrap().x)
        && (a.as_ref().unwrap().y == b.as_ref().unwrap().y))
        && (a.as_ref().unwrap().z == b.as_ref().unwrap().z);
}

// Source: upstream/packages/geometry/src/vector3.ts:136 (sha256:1ccdc2a834984cecd4db6ae9862e0440e99e9a3eebb5f0e5541c44bcf7ce2130)
pub fn get_vector3_angle_between(a: &Vector3Like, b: &Vector3Like) -> f64 {
    let la = get_vector3_length(a);
    let lb = get_vector3_length(b);
    if (la == 0.0_f64) || (lb == 0.0_f64) {
        return f64::NAN;
    }
    let _dot = (get_vector3_dot(a, b) / (la * lb));
    return ((1.0_f64).min((-1.0_f64).max(_dot))).acos();
}

// Source: upstream/packages/geometry/src/vector3.ts:150 (sha256:b19b75a68d9d8153567980a9f68d11b9327d437829628275c7b9cde9fb4ca9a6)
pub fn get_vector3_distance(a: &Vector3Like, b: &Vector3Like) -> f64 {
    let x: f64 = (b.x - a.x);
    let y: f64 = (b.y - a.y);
    let z: f64 = (b.z - a.z);
    return (((x).powf(2.0_f64) + (y).powf(2.0_f64)) + (z).powf(2.0_f64)).sqrt();
}

// Source: upstream/packages/geometry/src/vector3.ts:163 (sha256:1978e7fd9d447217db792ad29d5458a73aba0cce87854cd2cdc410b7312c62d6)
pub fn get_vector3_distance_squared(a: &Vector3Like, b: &Vector3Like) -> f64 {
    let x: f64 = (b.x - a.x);
    let y: f64 = (b.y - a.y);
    let z: f64 = (b.z - a.z);
    return (((x).powf(2.0_f64) + (y).powf(2.0_f64)) + (z).powf(2.0_f64));
}

// Source: upstream/packages/geometry/src/vector3.ts:178 (sha256:6a948fafca3f83cb254f3eee87de6ad8764b23b4c4f50e456f54938cb12524f5)
pub fn get_vector3_dot(a: &Vector3Like, b: &Vector3Like) -> f64 {
    return (((a.x * b.x) + (a.y * b.y)) + (a.z * b.z));
}

// Source: upstream/packages/geometry/src/vector3.ts:187 (sha256:b9807669c3ed486c269c818c5ae150f82f5adb4908ed74e8c715a4be612c0525)
pub fn get_vector3_length(source: &Vector3Like) -> f64 {
    return (((source.x).powf(2.0_f64) + (source.y).powf(2.0_f64)) + (source.z).powf(2.0_f64))
        .sqrt();
}

// Source: upstream/packages/geometry/src/vector3.ts:197 (sha256:32cb6a673ac4dc436a693cf68bd4d8090ff40b3f95492524f65fd9ca2d7bbb63)
pub fn get_vector3_length_squared(source: &Vector3Like) -> f64 {
    return (((source.x).powf(2.0_f64) + (source.y).powf(2.0_f64)) + (source.z).powf(2.0_f64));
}

// Source: upstream/packages/geometry/src/vector3.ts:209 (sha256:7f3c81893b6ad5a81d00e34d33f5b997b882d5b533d5a09ab2c4f15a571f247f)
pub fn get_vector3_spherical(out: &mut Vector3Like, source: &Vector3Like) -> () {
    let x = source.x;
    let y = source.y;
    let z = source.z;
    let radius = (((x * x) + (y * y)) + (z * z)).sqrt();
    if (radius == 0.0_f64) {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        out.z = 0.0_f64;
        return;
    }
    out.x = radius;
    out.y = ((1.0_f64).min((-1.0_f64).max((y / radius)))).acos();
    out.z = (z).atan2(x);
}

// Source: upstream/packages/geometry/src/vector3.ts:231 (sha256:069fdd54c8cf1084b5633385085e31c91f42b1067aec4bfab1bc387ed69de971)
pub fn interpolate_vector3(out: &mut Vector3Like, a: &Vector3Like, b: &Vector3Like, t: f64) -> () {
    let ax = a.x;
    let ay = a.y;
    let az = a.z;
    out.x = (ax + (t * (b.x - ax)));
    out.y = (ay + (t * (b.y - ay)));
    out.z = (az + (t * (b.z - az)));
}

// Source: upstream/packages/geometry/src/vector3.ts:250 (sha256:72afa1f02502c64202e3eec21f5840815f4da59d268bc8d2bf0416c7318e18c4)
pub fn max_vector3(out: &mut Vector3Like, a: &Vector3Like, b: &Vector3Like) -> () {
    out.x = if (a.x > b.x) { a.x } else { b.x };
    out.y = if (a.y > b.y) { a.y } else { b.y };
    out.z = if (a.z > b.z) { a.z } else { b.z };
}

// Source: upstream/packages/geometry/src/vector3.ts:261 (sha256:8b2b2670f04fa30e5aef777b6c3edaeaa80dfcd98aa26a4fcce077080bae6c3c)
pub fn min_vector3(out: &mut Vector3Like, a: &Vector3Like, b: &Vector3Like) -> () {
    out.x = if (a.x < b.x) { a.x } else { b.x };
    out.y = if (a.y < b.y) { a.y } else { b.y };
    out.z = if (a.z < b.z) { a.z } else { b.z };
}

// Source: upstream/packages/geometry/src/vector3.ts:272 (sha256:da6761e739475294e2e7c5f19b7718773b97a1e2944a6687ed5db36981358c31)
pub fn multiply_vector3(out: &mut Vector3Like, a: &Vector3Like, b: &Vector3Like) -> () {
    out.x = (a.x * b.x);
    out.y = (a.y * b.y);
    out.z = (a.z * b.z);
}

// Source: upstream/packages/geometry/src/vector3.ts:285 (sha256:aba72e090a837a6ea12bbe18871b17e610c285c023b0df53d930eefe7ebc7cf4)
pub fn near_equals_vector3(a: &Vector3Like, b: &Vector3Like, tolerance: Option<f64>) -> bool {
    let tolerance = tolerance.unwrap_or(0.000001_f64);
    return (((a.x - b.x).abs() < tolerance) && ((a.y - b.y).abs() < tolerance))
        && ((a.z - b.z).abs() < tolerance);
}

// Source: upstream/packages/geometry/src/vector3.ts:298 (sha256:d08305da36f51946cc97f8ef328d41ef6c0397e8cb23ab344b26d3705e2f29c1)
pub fn negate_vector3(out: &mut Vector3Like, source: &Vector3Like) -> () {
    out.x = (source.x * (-1.0_f64));
    out.y = (source.y * (-1.0_f64));
    out.z = (source.z * (-1.0_f64));
}

// Source: upstream/packages/geometry/src/vector3.ts:310 (sha256:4a211fbefd5c9d4a6a75ef726c04a66ab05e3d7a1bb1d1cd9d9980c3f54340c8)
pub fn normalize_vector3(out: &mut Vector3Like, source: &Vector3Like) -> f64 {
    let l = get_vector3_length(source);
    if (l != 0.0_f64) {
        out.x = (source.x / l);
        out.y = (source.y / l);
        out.z = (source.z / l);
    } else {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        out.z = 0.0_f64;
    }
    return l;
}

// Source: upstream/packages/geometry/src/vector3.ts:329 (sha256:9e5039f5343f7b94403743e01f3cad628e9668ba546b814991b77dc9ee56f52f)
pub fn offset_vector3(
    out: &mut Vector3Like,
    source: &Vector3Like,
    dx: f64,
    dy: f64,
    dz: f64,
) -> () {
    out.x = (source.x + dx);
    out.y = (source.y + dy);
    out.z = (source.z + dz);
}

// Source: upstream/packages/geometry/src/vector3.ts:345 (sha256:0204923bff962a4a208e01a80a418aeae0ff2badfb1ddcee50fa1dfc2de5854c)
pub fn project_vector3(out: &mut Vector2Like, source: &Vector3Like) -> () {
    out.x = (source.x / source.z);
    out.y = (source.y / source.z);
}

// Source: upstream/packages/geometry/src/vector3.ts:358 (sha256:adea76822f132986d6c88e4e5b9835a326a781e3a5727a28b366ebbb95155391)
pub fn reflect_vector3(out: &mut Vector3Like, incident: &Vector3Like, normal: &Vector3Like) -> () {
    let ix = incident.x;
    let iy = incident.y;
    let iz = incident.z;
    let nx = normal.x;
    let ny = normal.y;
    let nz = normal.z;
    let two_dot = (2.0_f64 * (((ix * nx) + (iy * ny)) + (iz * nz)));
    out.x = (ix - (two_dot * nx));
    out.y = (iy - (two_dot * ny));
    out.z = (iz - (two_dot * nz));
}

// Source: upstream/packages/geometry/src/vector3.ts:375 (sha256:e0df4e5f333dacb2fcebd4be260d39e251d3f31694b26cf87651789337186aa0)
pub fn scale_vector3(out: &mut Vector3Like, source: &Vector3Like, scalar: f64) -> () {
    out.x = (source.x * scalar);
    out.y = (source.y * scalar);
    out.z = (source.z * scalar);
}

// Source: upstream/packages/geometry/src/vector3.ts:384 (sha256:f80a68021d6ca5a57ec8a7e4bab5f0464946aa3ee18b4734823731f72204d30f)
pub fn set_vector3(out: &mut Vector3Like, x: f64, y: f64, z: f64) -> () {
    out.x = x;
    out.y = y;
    out.z = z;
}

// Source: upstream/packages/geometry/src/vector3.ts:393 (sha256:cee3f379b1f03b7a671eef54d39f11474d3c7ec2d015b2b11425a3f56b1bdd47)
pub fn set_vector3_from_float32_array(out: &mut Vector3Like, offset: f64, source: &Vec<f32>) -> () {
    out.x = (source[offset as usize] as f64);
    out.y = (source[(offset + 1.0_f64) as usize] as f64);
    out.z = (source[(offset + 2.0_f64) as usize] as f64);
}

// Source: upstream/packages/geometry/src/vector3.ts:403 (sha256:94e8f81d0cc311a17786d44b95923eb528a3a93eee66779a4f2ebad7a61e3e8a)
pub fn set_vector3_from_spherical(out: &mut Vector3Like, radius: f64, theta: f64, phi: f64) -> () {
    let sin_theta = (theta).sin();
    out.x = ((radius * sin_theta) * (phi).cos());
    out.y = (radius * (theta).cos());
    out.z = ((radius * sin_theta) * (phi).sin());
}

// Source: upstream/packages/geometry/src/vector3.ts:417 (sha256:5c3f5a6bb34add3c8590043bc41490e298c744933384a0382fe0437b21f82003)
pub fn set_vector3_from_vector4(out: &mut Vector3Like, source: &Vector4Like) -> () {
    let x = source.x;
    let y = source.y;
    let z = source.z;
    out.x = x;
    out.y = y;
    out.z = z;
}

// Source: upstream/packages/geometry/src/vector3.ts:430 (sha256:a9cea85761e765aa7114dcb7e62a47a50454cadb292b139f5d96182049b836ba)
pub fn subtract_vector3(out: &mut Vector3Like, source: &Vector3Like, other: &Vector3Like) -> () {
    out.x = (source.x - other.x);
    out.y = (source.y - other.y);
    out.z = (source.z - other.z);
}

// Source: upstream/packages/geometry/src/vector3.ts:442 (sha256:f50576dcfa1a61bb5656385e925c57f235594b69e15866cf37f65adbd8fd88df)
pub fn transform_vector3_by_matrix3(
    out: &mut Vector3Like,
    source: &Vector3Like,
    matrix: &SharedStructuralRecord1,
) -> () {
    let x = source.x;
    let y = source.y;
    let z = source.z;
    out.x = ((((matrix.m[0.0_f64 as usize] as f64) * x)
        + ((matrix.m[3.0_f64 as usize] as f64) * y))
        + ((matrix.m[6.0_f64 as usize] as f64) * z));
    out.y = ((((matrix.m[1.0_f64 as usize] as f64) * x)
        + ((matrix.m[4.0_f64 as usize] as f64) * y))
        + ((matrix.m[7.0_f64 as usize] as f64) * z));
    out.z = ((((matrix.m[2.0_f64 as usize] as f64) * x)
        + ((matrix.m[5.0_f64 as usize] as f64) * y))
        + ((matrix.m[8.0_f64 as usize] as f64) * z));
}

// Source: upstream/packages/geometry/src/vector3.ts:459 (sha256:fa4be79da40a17d3e76854a8a5b940f925e50ee9e815762b3b6d1a7b8126d6a3)
pub fn write_vector3_to_float32_array(out: &mut Vec<f32>, offset: f64, source: &Vector3Like) -> () {
    out[offset as usize] = (source.x) as f32;
    out[(offset + 1.0_f64) as usize] = (source.y) as f32;
    out[(offset + 2.0_f64) as usize] = (source.z) as f32;
}

// Source: upstream/packages/geometry/src/vector3.ts:465 (sha256:e3c15b39d55a065fed8f1e1d6a0adb07d8d414d0363170b61e3f7e167f21ca8e)
pub static VECTOR3_X_AXIS: std::sync::LazyLock<Vector3> =
    std::sync::LazyLock::new(|| create_vector3(Some(1.0_f64), Some(0.0_f64), Some(0.0_f64)));

// Source: upstream/packages/geometry/src/vector3.ts:466 (sha256:e780be74616bd91e25953cb3df5522d158958ad2cb728d1eb75f2e3ac89900d2)
pub static VECTOR3_Y_AXIS: std::sync::LazyLock<Vector3> =
    std::sync::LazyLock::new(|| create_vector3(Some(0.0_f64), Some(1.0_f64), Some(0.0_f64)));

// Source: upstream/packages/geometry/src/vector3.ts:467 (sha256:668bf65a765394262a10a59681f1cd79fc84dfa3741547bdddc82f7b6335c59f)
pub static VECTOR3_Z_AXIS: std::sync::LazyLock<Vector3> =
    std::sync::LazyLock::new(|| create_vector3(Some(0.0_f64), Some(0.0_f64), Some(1.0_f64)));
