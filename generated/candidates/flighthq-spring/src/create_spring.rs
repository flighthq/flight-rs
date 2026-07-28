// @generated from upstream/packages/spring/src/createSpring.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Spring;

// Source: upstream/packages/spring/src/createSpring.ts:6 (sha256:d9b331b68b5f051193afed57c3c6f7279d4cefb29f44522bad2d0111c0416c3d)
pub fn create_spring(value: Option<f64>, velocity: Option<f64>) -> Spring {
    let value = value.unwrap_or(0.0_f64);
    let velocity = velocity.unwrap_or(0.0_f64);
    return Spring {
        __flight_identity: std::sync::Arc::new(()),
        value: value,
        velocity: velocity,
    };
}
