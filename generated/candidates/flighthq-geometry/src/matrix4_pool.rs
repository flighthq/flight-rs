// @generated from upstream/packages/geometry/src/matrix4Pool.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_matrix4, set_matrix4_identity};
use flighthq_types::Matrix4;

// Source: upstream/packages/geometry/src/matrix4Pool.ts:5 (sha256:dc23a958dc6804f79e1cbe11c475c1a539fc5224b4d3d23c7eb75e102de25ae2)
pub fn acquire_identity_matrix4() -> Matrix4 {
    let mut m = acquire_matrix4();
    set_matrix4_identity(&mut m);
    return m;
}

// Source: upstream/packages/geometry/src/matrix4Pool.ts:11 (sha256:dfa0a47c06a0a1fe5f08a9a7a6309957a5b70f4e9a048b2fa817014212a0e660)
pub fn acquire_matrix4() -> Matrix4 {
    let mut m: Option<Matrix4> = None;
    if ((POOL.lock().unwrap().len() as f64) > 0.0_f64) {
        m = Some(
            POOL.lock()
                .unwrap()
                .pop()
                .expect("TypeScript Array.pop returned undefined"),
        );
    } else {
        m = Some(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ));
    }
    return (m).clone().unwrap();
}

// Source: upstream/packages/geometry/src/matrix4Pool.ts:23 (sha256:5dcaf987f95f442d7dd2d9033eb31f460d0e123626f8ec1561c25215e75b0ffe)
pub fn clear_matrix4_pool() -> () {
    POOL.lock().unwrap().clear();
}

// Source: upstream/packages/geometry/src/matrix4Pool.ts:27 (sha256:ed142868bef4f39656ca86a7a70401ff6fc85e91d8d75b2a2d6aac0d3d8b94a1)
pub fn release_matrix4(m: &Matrix4) -> () {
    if false {
        return;
    }
    POOL.lock().unwrap().push(((*m).clone()).clone());
}

// Source: upstream/packages/geometry/src/matrix4Pool.ts:32 (sha256:d6c94774691a1db7519e1331ea8f2f53343d9865e48260bc7845a428c8651d8c)
static POOL: std::sync::LazyLock<std::sync::Mutex<Vec<Matrix4>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));
