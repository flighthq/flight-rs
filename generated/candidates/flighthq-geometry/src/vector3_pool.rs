// @generated from upstream/packages/geometry/src/vector3Pool.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_vector3;
use flighthq_types::Vector3;

// Source: upstream/packages/geometry/src/vector3Pool.ts:5 (sha256:d88b323b3c245cbe0df494598d7d643348ac6e6af241fced8706c433bb4cae97)
pub fn acquire_empty_vector3() -> Vector3 {
    let mut v = acquire_vector3();
    v.x = 0.0_f64;
    v.y = 0.0_f64;
    v.z = 0.0_f64;
    return v;
}

// Source: upstream/packages/geometry/src/vector3Pool.ts:13 (sha256:62507464f065ce6aeb4fc017635760dd6860882fe7aa7d585c52278530ef6053)
pub fn acquire_vector3() -> Vector3 {
    let mut v: Option<Vector3> = None;
    if ((POOL.lock().unwrap().len() as f64) > 0.0_f64) {
        v = Some(
            POOL.lock()
                .unwrap()
                .pop()
                .expect("TypeScript Array.pop returned undefined"),
        );
    } else {
        v = Some(create_vector3(None, None, None));
    }
    return ((v).clone().unwrap()).clone();
}

// Source: upstream/packages/geometry/src/vector3Pool.ts:25 (sha256:ec63d294ec9dcef62efe3b95cc6ccf7aa93f5ad79c03340dc8fc64da56c266dd)
pub fn clear_vector3_pool() -> () {
    POOL.lock().unwrap().clear();
}

// Source: upstream/packages/geometry/src/vector3Pool.ts:29 (sha256:0c0f3c7206ff3795288987836fb49d1ed214697e303fca529893dab227acb6e2)
pub fn release_vector3(v: &Vector3) -> () {
    if false {
        return;
    }
    POOL.lock().unwrap().push(((*v).clone()).clone());
}

// Source: upstream/packages/geometry/src/vector3Pool.ts:34 (sha256:55515b4cbfd519f7a9daef59eefdf22126bc5d50f242b0ada542a01d26b80c11)
static POOL: std::sync::LazyLock<std::sync::Mutex<Vec<Vector3>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));
