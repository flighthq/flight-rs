// @generated from upstream/packages/geometry/src/quaternionPool.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GEOMETRY_POOL_RELEASE_GUARD as geometry_pool_release_guard_constant, create_quaternion,
};
use flighthq_types::Quaternion;

// Source: upstream/packages/geometry/src/quaternionPool.ts:7 (sha256:12e464e66f6986a6574de18ef38ca91d2884472fb91e8c6178b610e8af2c90ef)
pub fn acquire_identity_quaternion() -> Quaternion {
    let mut q = acquire_quaternion();
    q.x = 0.0_f64;
    q.y = 0.0_f64;
    q.z = 0.0_f64;
    q.w = 1.0_f64;
    return q;
}

// Source: upstream/packages/geometry/src/quaternionPool.ts:17 (sha256:fdaf6cf5940150d9823fa57f5eb9ef18c40b6b31b76513417704fd93ca52526f)
pub fn acquire_quaternion() -> Quaternion {
    let mut q: Option<Quaternion> = None;
    if ((POOL.lock().unwrap().len() as f64) > 0.0_f64) {
        q = Some(POOL.lock().unwrap().pop().unwrap());
    } else {
        q = Some(create_quaternion(None, None, None, None));
    }
    return ((q).clone().unwrap()).clone();
}

// Source: upstream/packages/geometry/src/quaternionPool.ts:29 (sha256:42c2235c62fd6902d6e94635533f3d260398d635a72dc455c61b8f6ef19f259c)
pub fn clear_quaternion_pool() -> () {
    POOL.lock().unwrap().clear();
}

// Source: upstream/packages/geometry/src/quaternionPool.ts:33 (sha256:6cced54bcd6ae466162872d0dfd775231cefc0a6ce12d5dd9012abc802d87306)
pub fn release_quaternion(q: &mut Quaternion) -> () {
    if false {
        return;
    }
    if ((geometry_pool_release_guard_constant).is_some())
        && ({
            let __flight_value = (*q).clone();
            (POOL.lock().unwrap())
                .iter()
                .any(|item| item == &__flight_value)
        })
    {
        geometry_pool_release_guard_constant("releaseQuaternion");
    }
    *flighthq_types::FlightEntity::__flight_entity_runtime(q)
        .lock()
        .unwrap() = None;
    POOL.lock().unwrap().push(((*q).clone()).clone());
}

// Source: upstream/packages/geometry/src/quaternionPool.ts:40 (sha256:14ada2a06a9cf5a7e82f6f6689826cf1394f15f43a5f7874e5d0c9357ade9881)
static POOL: std::sync::LazyLock<std::sync::Mutex<Vec<Quaternion>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));
