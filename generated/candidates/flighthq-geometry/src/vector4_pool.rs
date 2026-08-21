// @generated from upstream/packages/geometry/src/vector4Pool.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GEOMETRY_POOL_RELEASE_GUARD as geometry_pool_release_guard_constant, create_vector4};
use flighthq_types::Vector4;

// Source: upstream/packages/geometry/src/vector4Pool.ts:7 (sha256:47d5347f98111cd250476d131c5fb0a5a7bb97dac50604ecddb3a50bdff43294)
pub fn acquire_empty_vector4() -> Vector4 {
    let mut v = acquire_vector4();
    v.x = 0.0_f64;
    v.y = 0.0_f64;
    v.z = 0.0_f64;
    v.w = 0.0_f64;
    return v;
}

// Source: upstream/packages/geometry/src/vector4Pool.ts:17 (sha256:b1eec3910d7ed174efee5fa385e7e02c6c9534fbeceb1aea6c3f6435d28173c8)
pub fn acquire_vector4() -> Vector4 {
    let mut v: Option<Vector4> = None;
    if ((POOL.lock().unwrap().len() as f64) > 0.0_f64) {
        v = Some(POOL.lock().unwrap().pop().unwrap());
    } else {
        v = Some(create_vector4(None, None, None, None));
    }
    return ((v).clone().unwrap()).clone();
}

// Source: upstream/packages/geometry/src/vector4Pool.ts:29 (sha256:86eab05ee014e148c7c62c525d86678270380b8b833618fa745b3f4343648e24)
pub fn clear_vector4_pool() -> () {
    POOL.lock().unwrap().clear();
}

// Source: upstream/packages/geometry/src/vector4Pool.ts:33 (sha256:f9cb440b775599d687a5dd6a2f5d77f561b4d42aae118c15b704ed1e1f38f95c)
pub fn release_vector4(v: &mut Vector4) -> () {
    if false {
        return;
    }
    if (((*geometry_pool_release_guard_constant.lock().unwrap()).clone()).is_some())
        && ({
            let __flight_value = (*v).clone();
            (POOL.lock().unwrap())
                .iter()
                .any(|item| item == &__flight_value)
        })
    {
        {
            let __flight_callback = (*geometry_pool_release_guard_constant.lock().unwrap())
                .clone()
                .unwrap();
            __flight_callback.lock().unwrap()("releaseVector4".to_owned())
        };
    }
    *flighthq_types::FlightEntity::__flight_entity_runtime(v)
        .lock()
        .unwrap() = None;
    POOL.lock().unwrap().push(((*v).clone()).clone());
}

// Source: upstream/packages/geometry/src/vector4Pool.ts:40 (sha256:75a1141bcaa075fa5c60298f9cb16d2fb251df175e032fe152df55ad95a8eacf)
static POOL: std::sync::LazyLock<std::sync::Mutex<Vec<Vector4>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));
