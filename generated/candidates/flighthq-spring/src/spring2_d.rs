// @generated from upstream/packages/spring/src/spring2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_spring, is_spring_settled, update_spring};
use flighthq_types::{Spring2D, SpringConfig};

// Source: upstream/packages/spring/src/spring2D.ts:7 (sha256:e7a66857916f0e01030467f8a74f796aa21d69f0cbb1a9556b9da8a27ede4560)
pub fn create_spring2_d(
    value_x: Option<f64>,
    value_y: Option<f64>,
    velocity_x: Option<f64>,
    velocity_y: Option<f64>,
) -> Spring2D {
    let value_x = value_x.unwrap_or(0.0_f64);
    let value_y = value_y.unwrap_or(0.0_f64);
    let velocity_x = velocity_x.unwrap_or(0.0_f64);
    let velocity_y = velocity_y.unwrap_or(0.0_f64);
    return Spring2D {
        __flight_identity: std::sync::Arc::new(()),
        x: create_spring(Some(value_x), Some(velocity_x)),
        y: create_spring(Some(value_y), Some(velocity_y)),
    };
}

// Source: upstream/packages/spring/src/spring2D.ts:18 (sha256:21d177fe51890c0e040f69d32d67786beae49d53c99e7d6269f387cff75cdd1b)
pub fn is_spring2_d_settled(
    spring2_d: &Spring2D,
    target_x: f64,
    target_y: f64,
    position_epsilon: Option<f64>,
    velocity_epsilon: Option<f64>,
) -> bool {
    return (is_spring_settled(
        &spring2_d.x,
        target_x,
        Some((position_epsilon).clone().unwrap()),
        Some((velocity_epsilon).clone().unwrap()),
    )) && (is_spring_settled(
        &spring2_d.y,
        target_y,
        Some((position_epsilon).clone().unwrap()),
        Some((velocity_epsilon).clone().unwrap()),
    ));
}

// Source: upstream/packages/spring/src/spring2D.ts:34 (sha256:0f9ad297213840451af063ea6c55e381e7df9daacc603f38c20c44f3a916d31d)
pub fn update_spring2_d(
    spring2_d: &mut Spring2D,
    target_x: f64,
    target_y: f64,
    config: &SpringConfig,
    delta_time: f64,
) -> () {
    update_spring(&mut spring2_d.x, target_x, config, delta_time);
    update_spring(&mut spring2_d.y, target_y, config, delta_time);
}
