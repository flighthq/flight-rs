// @generated from upstream/packages/geometry/src/vector2Pool.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_vector2;
use flighthq_types::Vector2;

// Source: upstream/packages/geometry/src/vector2Pool.ts:5 (sha256:d9cef3452fec1c0c47a7cf7c5f99df72b430ee2ffeafc13d3b1e1f86cb920695)
pub fn acquire_empty_vector2() -> Vector2 {
    let mut v = acquire_vector2();
    v.x = 0.0_f64;
    v.y = 0.0_f64;
    return (v).clone();
}

// Source: upstream/packages/geometry/src/vector2Pool.ts:12 (sha256:ef001613245ab8bd9b0f2e4662379929788bf0740263e1dbfb7501b89b760567)
pub fn acquire_vector2() -> Vector2 {
    return if ((POOL.lock().unwrap().len() as f64) > 0.0_f64) {
        POOL.lock()
            .unwrap()
            .pop()
            .expect("TypeScript Array.pop returned undefined")
    } else {
        create_vector2(None, None)
    };
}

// Source: upstream/packages/geometry/src/vector2Pool.ts:16 (sha256:377dd1486d138ebf42a7e77b2bb5661485af6a0695a1c8008e90b4daa021e9cc)
pub fn clear_vector2_pool() -> () {
    POOL.lock().unwrap().clear();
}

// Source: upstream/packages/geometry/src/vector2Pool.ts:20 (sha256:6273c89c5ef0f1c36b177e86f64a261c9a5feddbad4ba03de90678019f6a3fc4)
pub fn release_vector2(v: &Vector2) -> () {
    if false {
        return;
    }
    POOL.lock().unwrap().push(((*v).clone()).clone());
}

// Source: upstream/packages/geometry/src/vector2Pool.ts:25 (sha256:c458e447488e2cf8b9003671a302059aecd72096890d2c4bf7c798ca708b7312)
static POOL: std::sync::LazyLock<std::sync::Mutex<Vec<Vector2>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));
