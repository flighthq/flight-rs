// @generated from upstream/packages/geometry/src/rectanglePool.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GEOMETRY_POOL_RELEASE_GUARD as geometry_pool_release_guard_constant, create_rectangle,
};
use flighthq_types::Rectangle;

// Source: upstream/packages/geometry/src/rectanglePool.ts:7 (sha256:bf37551ebb0f30d95cecae5c30dbea02b06c693ac0cfde383b1f5b7325ba2c0e)
pub fn acquire_empty_rectangle() -> Rectangle {
    let mut r = acquire_rectangle();
    r.x = 0.0_f64;
    r.y = 0.0_f64;
    r.width = 0.0_f64;
    r.height = 0.0_f64;
    return r;
}

// Source: upstream/packages/geometry/src/rectanglePool.ts:17 (sha256:6fc408cfc41ec5a0acd2b2f6cb93ca91f5e8f96237e0db62d8e53dec85b42e28)
pub fn acquire_rectangle() -> Rectangle {
    let mut r: Option<Rectangle> = None;
    if ((POOL.lock().unwrap().len() as f64) > 0.0_f64) {
        r = Some(POOL.lock().unwrap().pop().unwrap());
    } else {
        r = Some(create_rectangle(None, None, None, None));
    }
    return ((r).clone().unwrap()).clone();
}

// Source: upstream/packages/geometry/src/rectanglePool.ts:29 (sha256:56b0c9740f3479e101b1cbfd9680abdd5dca829693b3baaa668dff47b1a288f2)
pub fn clear_rectangle_pool() -> () {
    POOL.lock().unwrap().clear();
}

// Source: upstream/packages/geometry/src/rectanglePool.ts:33 (sha256:afe52d8a3f5ea6b3f8deb6e9ed17f098fa26865b2eaa9064dd3d856a8f62d2c1)
pub fn release_rectangle(r: &mut Rectangle) -> () {
    if false {
        return;
    }
    if ((geometry_pool_release_guard_constant).is_some())
        && ({
            let __flight_value = (*r).clone();
            (POOL.lock().unwrap())
                .iter()
                .any(|item| item == &__flight_value)
        })
    {
        geometry_pool_release_guard_constant("releaseRectangle");
    }
    *flighthq_types::FlightEntity::__flight_entity_runtime(r)
        .lock()
        .unwrap() = None;
    POOL.lock().unwrap().push(((*r).clone()).clone());
}

// Source: upstream/packages/geometry/src/rectanglePool.ts:40 (sha256:d6b14a8f9e617b0d19c806af0c2801a5482914efc71a5357f465334ecebbdcaa)
static POOL: std::sync::LazyLock<std::sync::Mutex<Vec<Rectangle>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));
