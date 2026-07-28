// @generated from upstream/packages/spring/src/resetSpring.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Spring;

// Source: upstream/packages/spring/src/resetSpring.ts:6 (sha256:da600212dac053b5d1dcfd49737a2ff1acbb45d83859137d29e614f3c1daf086)
pub fn reset_spring(spring: &mut Spring, value: f64, velocity: Option<f64>) -> () {
    let velocity = velocity.unwrap_or(0.0_f64);
    spring.value = value;
    spring.velocity = velocity;
}
