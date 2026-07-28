// @generated from upstream/packages/geometry/src/rectanglePool.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_rectangle;
use flighthq_types::Rectangle;

// Source: upstream/packages/geometry/src/rectanglePool.ts:5 (sha256:bf37551ebb0f30d95cecae5c30dbea02b06c693ac0cfde383b1f5b7325ba2c0e)
pub fn acquire_empty_rectangle() -> Rectangle {
    let mut r = acquire_rectangle();
    r.x = 0.0_f64;
    r.y = 0.0_f64;
    r.width = 0.0_f64;
    r.height = 0.0_f64;
    return r;
}

// Source: upstream/packages/geometry/src/rectanglePool.ts:14 (sha256:6fc408cfc41ec5a0acd2b2f6cb93ca91f5e8f96237e0db62d8e53dec85b42e28)
pub fn acquire_rectangle() -> Rectangle {
    let mut r: Option<Rectangle> = None;
    if ((POOL.lock().unwrap().len() as f64) > 0.0_f64) {
        r = Some(
            POOL.lock()
                .unwrap()
                .pop()
                .expect("TypeScript Array.pop returned undefined"),
        );
    } else {
        r = Some(create_rectangle(None, None, None, None));
    }
    return (r).clone().unwrap();
}

// Source: upstream/packages/geometry/src/rectanglePool.ts:26 (sha256:56b0c9740f3479e101b1cbfd9680abdd5dca829693b3baaa668dff47b1a288f2)
pub fn clear_rectangle_pool() -> () {
    POOL.lock().unwrap().clear();
}

// Source: upstream/packages/geometry/src/rectanglePool.ts:30 (sha256:f7aa705ca8785b1a7a900014b156296bc2bb31ebe8ab72e0d468a18e6041aabd)
pub fn release_rectangle(r: &Rectangle) -> () {
    if false {
        return;
    }
    POOL.lock().unwrap().push(((*r).clone()).clone());
}

// Source: upstream/packages/geometry/src/rectanglePool.ts:35 (sha256:d6b14a8f9e617b0d19c806af0c2801a5482914efc71a5357f465334ecebbdcaa)
static POOL: std::sync::LazyLock<std::sync::Mutex<Vec<Rectangle>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));
