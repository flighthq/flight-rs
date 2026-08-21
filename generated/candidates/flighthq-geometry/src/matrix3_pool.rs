// @generated from upstream/packages/geometry/src/matrix3Pool.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GEOMETRY_POOL_RELEASE_GUARD as geometry_pool_release_guard_constant, create_matrix3,
    set_matrix3_identity,
};
use flighthq_types::Matrix3;

// Source: upstream/packages/geometry/src/matrix3Pool.ts:7 (sha256:a4c453e42f6fd62cf16922f2f5239214799ca4803d9e752b1b77c861cc2ad944)
pub fn acquire_identity_matrix3() -> Matrix3 {
    let mut m = acquire_matrix3();
    set_matrix3_identity(&mut m);
    return m;
}

// Source: upstream/packages/geometry/src/matrix3Pool.ts:14 (sha256:bdea5998e745539ac70083f26a63bdfc461d4735b2dbce85428dc802243c223e)
pub fn acquire_matrix3() -> Matrix3 {
    let mut m: Option<Matrix3> = None;
    if ((POOL.lock().unwrap().len() as f64) > 0.0_f64) {
        m = Some(POOL.lock().unwrap().pop().unwrap());
    } else {
        m = Some(create_matrix3(
            None, None, None, None, None, None, None, None, None,
        ));
    }
    return ((m).clone().unwrap()).clone();
}

// Source: upstream/packages/geometry/src/matrix3Pool.ts:26 (sha256:4ed143326344ac48c209876b7ad6b9f5243c92a41142f25091d8fad8e88cf00a)
pub fn clear_matrix3_pool() -> () {
    POOL.lock().unwrap().clear();
}

// Source: upstream/packages/geometry/src/matrix3Pool.ts:30 (sha256:3714db64cdc523bda1f7560621bcebb5670e1c2be0de4b57b7645c597c0f705e)
pub fn release_matrix3(m: &mut Matrix3) -> () {
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
            __flight_callback.lock().unwrap()("releaseMatrix3".to_owned())
        };
    }
    *flighthq_types::FlightEntity::__flight_entity_runtime(m)
        .lock()
        .unwrap() = None;
    POOL.lock().unwrap().push(((*m).clone()).clone());
}

// Source: upstream/packages/geometry/src/matrix3Pool.ts:37 (sha256:5c5c60435dde2e285ee890d9a930048239788a8bd6084e0dbc2c0b10235d8c59)
static POOL: std::sync::LazyLock<std::sync::Mutex<Vec<Matrix3>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));
