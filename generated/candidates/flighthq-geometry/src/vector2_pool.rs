// @generated from upstream/packages/geometry/src/vector2Pool.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GEOMETRY_POOL_RELEASE_GUARD as geometry_pool_release_guard_constant, create_vector2};
use flighthq_types::Vector2;

// Source: upstream/packages/geometry/src/vector2Pool.ts:7 (sha256:d9cef3452fec1c0c47a7cf7c5f99df72b430ee2ffeafc13d3b1e1f86cb920695)
pub fn acquire_empty_vector2() -> Vector2 {
    let mut v = acquire_vector2();
    v.x = 0.0_f64;
    v.y = 0.0_f64;
    return v;
}

// Source: upstream/packages/geometry/src/vector2Pool.ts:15 (sha256:ef001613245ab8bd9b0f2e4662379929788bf0740263e1dbfb7501b89b760567)
pub fn acquire_vector2() -> Vector2 {
    return if ((POOL.lock().unwrap().len() as f64) > 0.0_f64) {
        POOL.lock().unwrap().pop().unwrap()
    } else {
        create_vector2(None, None)
    };
}

// Source: upstream/packages/geometry/src/vector2Pool.ts:19 (sha256:377dd1486d138ebf42a7e77b2bb5661485af6a0695a1c8008e90b4daa021e9cc)
pub fn clear_vector2_pool() -> () {
    POOL.lock().unwrap().clear();
}

// Source: upstream/packages/geometry/src/vector2Pool.ts:23 (sha256:c2226edf1227fd11fa41639cf130a66f8cf799848610767dc306c4b85d9df92c)
pub fn release_vector2(v: &mut Vector2) -> () {
    if false {
        return;
    }
    if ((geometry_pool_release_guard_constant).is_some())
        && ({
            let __flight_value = (*v).clone();
            (POOL.lock().unwrap())
                .iter()
                .any(|item| item == &__flight_value)
        })
    {
        geometry_pool_release_guard_constant("releaseVector2");
    }
    *flighthq_types::FlightEntity::__flight_entity_runtime(v)
        .lock()
        .unwrap() = None;
    POOL.lock().unwrap().push(((*v).clone()).clone());
}

// Source: upstream/packages/geometry/src/vector2Pool.ts:30 (sha256:c458e447488e2cf8b9003671a302059aecd72096890d2c4bf7c798ca708b7312)
static POOL: std::sync::LazyLock<std::sync::Mutex<Vec<Vector2>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));
