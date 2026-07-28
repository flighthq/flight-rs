// @generated from upstream/packages/spring/src/isSpringSettled.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_math::{approx_equal, approx_zero};
use flighthq_types::Spring;

// Source: upstream/packages/spring/src/isSpringSettled.ts:11 (sha256:f2157ae0080b64a24b7351e2c81d14b1a3274fb15508b84f9a799877a51f7747)
pub fn is_spring_settled(
    spring: &Spring,
    target: f64,
    position_epsilon: Option<f64>,
    velocity_epsilon: Option<f64>,
) -> bool {
    let position_epsilon = position_epsilon.unwrap_or(SPRING_SETTLE_EPSILON);
    let velocity_epsilon = velocity_epsilon.unwrap_or(SPRING_SETTLE_EPSILON);
    return (approx_equal(spring.value, target, Some(position_epsilon))
        && approx_zero(spring.velocity, Some(velocity_epsilon)));
}

// Source: upstream/packages/spring/src/isSpringSettled.ts:22 (sha256:83a7110b7bcfde35c0048d0a9576dec659df1c651ad9c049fc983206022260d1)
const SPRING_SETTLE_EPSILON: f64 = 0.001_f64;
