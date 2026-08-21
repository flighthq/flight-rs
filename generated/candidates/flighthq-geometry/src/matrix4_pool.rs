// @generated from upstream/packages/geometry/src/matrix4Pool.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GEOMETRY_POOL_RELEASE_GUARD as geometry_pool_release_guard_constant, create_matrix4,
    set_matrix4_identity,
};
use flighthq_types::Matrix4;

// Source: upstream/packages/geometry/src/matrix4Pool.ts:7 (sha256:dc23a958dc6804f79e1cbe11c475c1a539fc5224b4d3d23c7eb75e102de25ae2)
pub fn acquire_identity_matrix4() -> Matrix4 {
    let mut m = acquire_matrix4();
    set_matrix4_identity(&mut m);
    return m;
}

// Source: upstream/packages/geometry/src/matrix4Pool.ts:14 (sha256:dfa0a47c06a0a1fe5f08a9a7a6309957a5b70f4e9a048b2fa817014212a0e660)
pub fn acquire_matrix4() -> Matrix4 {
    let mut m: Option<Matrix4> = None;
    if ((POOL.lock().unwrap().len() as f64) > 0.0_f64) {
        m = Some(POOL.lock().unwrap().pop().unwrap());
    } else {
        m = Some(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ));
    }
    return ((m).clone().unwrap()).clone();
}

// Source: upstream/packages/geometry/src/matrix4Pool.ts:26 (sha256:5dcaf987f95f442d7dd2d9033eb31f460d0e123626f8ec1561c25215e75b0ffe)
pub fn clear_matrix4_pool() -> () {
    POOL.lock().unwrap().clear();
}

// Source: upstream/packages/geometry/src/matrix4Pool.ts:30 (sha256:6b2ef3ad9ac5d58d2d004e2aad7dbd345c1a1a6aabac55762ed16063c1a51a3e)
pub fn release_matrix4(m: &mut Matrix4) -> () {
    if false {
        return;
    }
    if (((*geometry_pool_release_guard_constant.lock().unwrap()).clone()).is_some())
        && ({
            let __flight_value = (*m).clone();
            (POOL.lock().unwrap())
                .iter()
                .any(|item| item == &__flight_value)
        })
    {
        {
            let __flight_callback = (*geometry_pool_release_guard_constant.lock().unwrap())
                .clone()
                .unwrap();
            __flight_callback.lock().unwrap()("releaseMatrix4".to_owned())
        };
    }
    *flighthq_types::FlightEntity::__flight_entity_runtime(m)
        .lock()
        .unwrap() = None;
    POOL.lock().unwrap().push(((*m).clone()).clone());
}

// Source: upstream/packages/geometry/src/matrix4Pool.ts:37 (sha256:d6c94774691a1db7519e1331ea8f2f53343d9865e48260bc7845a428c8651d8c)
static POOL: std::sync::LazyLock<std::sync::Mutex<Vec<Matrix4>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));
