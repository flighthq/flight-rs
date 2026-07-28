// @generated from upstream/packages/geometry/src/vector4Pool.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_vector4;
use flighthq_types::Vector4;

// Source: upstream/packages/geometry/src/vector4Pool.ts:5 (sha256:47d5347f98111cd250476d131c5fb0a5a7bb97dac50604ecddb3a50bdff43294)
pub fn acquire_empty_vector4() -> Vector4 {
    let mut v = acquire_vector4();
    v.x = 0.0_f64;
    v.y = 0.0_f64;
    v.z = 0.0_f64;
    v.w = 0.0_f64;
    return (v).clone();
}

// Source: upstream/packages/geometry/src/vector4Pool.ts:14 (sha256:b1eec3910d7ed174efee5fa385e7e02c6c9534fbeceb1aea6c3f6435d28173c8)
pub fn acquire_vector4() -> Vector4 {
    let mut v: Option<Vector4> = None;
    if ((POOL.lock().unwrap().len() as f64) > 0.0_f64) {
        v = Some(
            POOL.lock()
                .unwrap()
                .pop()
                .expect("TypeScript Array.pop returned undefined"),
        );
    } else {
        v = Some(create_vector4(None, None, None, None));
    }
    return (v).clone().unwrap();
}

// Source: upstream/packages/geometry/src/vector4Pool.ts:26 (sha256:86eab05ee014e148c7c62c525d86678270380b8b833618fa745b3f4343648e24)
pub fn clear_vector4_pool() -> () {
    POOL.lock().unwrap().clear();
}

// Source: upstream/packages/geometry/src/vector4Pool.ts:30 (sha256:a33a4f90dea880c7c9992c04c693d0f03893d64479339e8472f7d2d1fa04c493)
pub fn release_vector4(v: &Vector4) -> () {
    if false {
        return;
    }
    POOL.lock().unwrap().push(((*v).clone()).clone());
}

// Source: upstream/packages/geometry/src/vector4Pool.ts:35 (sha256:75a1141bcaa075fa5c60298f9cb16d2fb251df175e032fe152df55ad95a8eacf)
static POOL: std::sync::LazyLock<std::sync::Mutex<Vec<Vector4>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));
