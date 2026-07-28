// @generated from upstream/packages/geometry/src/matrixPool.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_matrix, set_matrix_identity};
use flighthq_types::Matrix;

// Source: upstream/packages/geometry/src/matrixPool.ts:5 (sha256:b621da13fc5904993d2c4723a44f7f68c39189e08b82d6cd1bc982a2b2be3674)
pub fn acquire_identity_matrix() -> Matrix {
    let mut m = acquire_matrix();
    set_matrix_identity(&mut m);
    return (m).clone();
}

// Source: upstream/packages/geometry/src/matrixPool.ts:11 (sha256:ec4e435d2cc5bcd4e2e39b0e10f9a3795156068449372817c4e01fce612faf75)
pub fn acquire_matrix() -> Matrix {
    let mut m: Option<Matrix> = None;
    if ((POOL.lock().unwrap().len() as f64) > 0.0_f64) {
        m = Some(
            POOL.lock()
                .unwrap()
                .pop()
                .expect("TypeScript Array.pop returned undefined"),
        );
    } else {
        m = Some(create_matrix(None, None, None, None, None, None));
    }
    return (m).clone().unwrap();
}

// Source: upstream/packages/geometry/src/matrixPool.ts:23 (sha256:0d2a5eb0a5c0a2c92b3cf517f54845b0db45f42f752dd3f21824cf59d707c238)
pub fn clear_matrix_pool() -> () {
    POOL.lock().unwrap().clear();
}

// Source: upstream/packages/geometry/src/matrixPool.ts:27 (sha256:11033c5be4c6d61b214e7df0a976c2e4adc32692c357d54050a51b938f97d506)
pub fn release_matrix(m: &Matrix) -> () {
    if false {
        return;
    }
    POOL.lock().unwrap().push(((*m).clone()).clone());
}

// Source: upstream/packages/geometry/src/matrixPool.ts:32 (sha256:ba19f44d741a21c53f2ad5beb89993429ce9a3cce6f95feec76fbc9db089ad50)
static POOL: std::sync::LazyLock<std::sync::Mutex<Vec<Matrix>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));
