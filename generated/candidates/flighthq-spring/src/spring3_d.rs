// @generated from upstream/packages/spring/src/spring3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_spring, is_spring_settled, update_spring};
use flighthq_types::{Spring3D, SpringConfig};

// Source: upstream/packages/spring/src/spring3D.ts:9 (sha256:e63c95968b368a2bf29be838595731363f2ff0227e8d2cfb7c80139e0b670ccc)
pub fn create_spring3_d(
    value_x: Option<f64>,
    value_y: Option<f64>,
    value_z: Option<f64>,
    velocity_x: Option<f64>,
    velocity_y: Option<f64>,
    velocity_z: Option<f64>,
) -> Spring3D {
    let value_x = value_x.unwrap_or(0.0_f64);
    let value_y = value_y.unwrap_or(0.0_f64);
    let value_z = value_z.unwrap_or(0.0_f64);
    let velocity_x = velocity_x.unwrap_or(0.0_f64);
    let velocity_y = velocity_y.unwrap_or(0.0_f64);
    let velocity_z = velocity_z.unwrap_or(0.0_f64);
    return Spring3D {
        __flight_identity: std::sync::Arc::new(()),
        x: create_spring(Some(value_x), Some(velocity_x)),
        y: create_spring(Some(value_y), Some(velocity_y)),
        z: create_spring(Some(value_z), Some(velocity_z)),
    };
}

// Source: upstream/packages/spring/src/spring3D.ts:26 (sha256:147240fa35bacf7ffa93ff7aaea2c3e6abe1f9344e4bbf2b1f63c83c29ebaf4e)
pub fn is_spring3_d_settled(
    spring3_d: &Spring3D,
    target_x: f64,
    target_y: f64,
    target_z: f64,
    position_epsilon: Option<f64>,
    velocity_epsilon: Option<f64>,
) -> bool {
    return ((is_spring_settled(
        &spring3_d.x,
        target_x,
        Some((position_epsilon).clone().unwrap()),
        Some((velocity_epsilon).clone().unwrap()),
    ) && is_spring_settled(
        &spring3_d.y,
        target_y,
        Some((position_epsilon).clone().unwrap()),
        Some((velocity_epsilon).clone().unwrap()),
    )) && is_spring_settled(
        &spring3_d.z,
        target_z,
        Some((position_epsilon).clone().unwrap()),
        Some((velocity_epsilon).clone().unwrap()),
    ));
}

// Source: upstream/packages/spring/src/spring3D.ts:43 (sha256:16d17faf58d366eaae2c3e71ff998929760565c70e04685b27913fcdd16bef22)
pub fn update_spring3_d(
    spring3_d: &mut Spring3D,
    target_x: f64,
    target_y: f64,
    target_z: f64,
    config: &SpringConfig,
    delta_time: f64,
) -> () {
    update_spring(&mut spring3_d.x, target_x, config, delta_time);
    update_spring(&mut spring3_d.y, target_y, config, delta_time);
    update_spring(&mut spring3_d.z, target_z, config, delta_time);
}
