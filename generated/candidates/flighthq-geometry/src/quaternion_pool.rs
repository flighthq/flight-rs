// @generated from upstream/packages/geometry/src/quaternionPool.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_quaternion;
use flighthq_types::Quaternion;

// Source: upstream/packages/geometry/src/quaternionPool.ts:5 (sha256:12e464e66f6986a6574de18ef38ca91d2884472fb91e8c6178b610e8af2c90ef)
pub fn acquire_identity_quaternion() -> Quaternion {
    let mut q = acquire_quaternion();
    q.x = 0.0_f64;
    q.y = 0.0_f64;
    q.z = 0.0_f64;
    q.w = 1.0_f64;
    return q;
}

// Source: upstream/packages/geometry/src/quaternionPool.ts:14 (sha256:fdaf6cf5940150d9823fa57f5eb9ef18c40b6b31b76513417704fd93ca52526f)
pub fn acquire_quaternion() -> Quaternion {
    let mut q: Option<Quaternion> = None;
    if ((POOL.lock().unwrap().len() as f64) > 0.0_f64) {
        q = Some(
            POOL.lock()
                .unwrap()
                .pop()
                .expect("TypeScript Array.pop returned undefined"),
        );
    } else {
        q = Some(create_quaternion(None, None, None, None));
    }
    return ((q).clone().unwrap()).clone();
}

// Source: upstream/packages/geometry/src/quaternionPool.ts:26 (sha256:42c2235c62fd6902d6e94635533f3d260398d635a72dc455c61b8f6ef19f259c)
pub fn clear_quaternion_pool() -> () {
    POOL.lock().unwrap().clear();
}

// Source: upstream/packages/geometry/src/quaternionPool.ts:30 (sha256:49935c2310cffbc7e5238fac903b7c490751c128d631c820c603610f421e6c54)
pub fn release_quaternion(q: &Quaternion) -> () {
    if false {
        return;
    }
    POOL.lock().unwrap().push(((*q).clone()).clone());
}

// Source: upstream/packages/geometry/src/quaternionPool.ts:35 (sha256:14ada2a06a9cf5a7e82f6f6689826cf1394f15f43a5f7874e5d0c9357ade9881)
static POOL: std::sync::LazyLock<std::sync::Mutex<Vec<Quaternion>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));
