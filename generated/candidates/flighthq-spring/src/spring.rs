// @generated from upstream/packages/spring/src/spring.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_math::{TAU as tau_constant, approx_equal, approx_zero};
use flighthq_types::{Spring, SpringConfig};

// Source: upstream/packages/spring/src/spring.ts:7 (sha256:d9b331b68b5f051193afed57c3c6f7279d4cefb29f44522bad2d0111c0416c3d)
pub fn create_spring(value: Option<f64>, velocity: Option<f64>) -> Spring {
    let value = value.unwrap_or(0.0_f64);
    let velocity = velocity.unwrap_or(0.0_f64);
    return Spring {
        __flight_identity: std::sync::Arc::new(()),
        value: value,
        velocity: velocity,
    };
}

// Source: upstream/packages/spring/src/spring.ts:18 (sha256:f2157ae0080b64a24b7351e2c81d14b1a3274fb15508b84f9a799877a51f7747)
pub fn is_spring_settled(
    spring: &Spring,
    target: f64,
    position_epsilon: Option<f64>,
    velocity_epsilon: Option<f64>,
) -> bool {
    let position_epsilon = position_epsilon.unwrap_or(SPRING_SETTLE_EPSILON);
    let velocity_epsilon = velocity_epsilon.unwrap_or(SPRING_SETTLE_EPSILON);
    return (approx_equal(spring.value, target, Some(position_epsilon)))
        && (approx_zero(spring.velocity, Some(velocity_epsilon)));
}

// Source: upstream/packages/spring/src/spring.ts:29 (sha256:83a7110b7bcfde35c0048d0a9576dec659df1c651ad9c049fc983206022260d1)
const SPRING_SETTLE_EPSILON: f64 = 0.001_f64;

// Source: upstream/packages/spring/src/spring.ts:34 (sha256:da600212dac053b5d1dcfd49737a2ff1acbb45d83859137d29e614f3c1daf086)
pub fn reset_spring(spring: &mut Spring, value: f64, velocity: Option<f64>) -> () {
    let velocity = velocity.unwrap_or(0.0_f64);
    spring.value = value;
    spring.velocity = velocity;
}

// Source: upstream/packages/spring/src/spring.ts:56 (sha256:0e3162737b3e6452bec76ef14eaaeaa8033cdc5685d2e17682355eec1a379524)
pub fn update_spring(
    spring: &mut Spring,
    target: f64,
    config: &SpringConfig,
    delta_time: f64,
) -> () {
    if (delta_time <= 0.0_f64) {
        return;
    }
    let frequency = config.frequency;
    if (frequency <= 0.0_f64) {
        return;
    }
    let value = spring.value;
    let velocity = spring.velocity;
    let damping_ratio = if (config.damping_ratio < 0.0_f64) {
        0.0_f64
    } else {
        config.damping_ratio
    };
    let omega = (tau_constant * frequency);
    let c0 = (value - target);
    let mut pos_pos_coef: f64;
    let mut pos_vel_coef: f64;
    let mut vel_pos_coef: f64;
    let mut vel_vel_coef: f64;
    if (damping_ratio > (1.0_f64 + CRITICAL_BAND)) {
        let zb = (omega * ((damping_ratio * damping_ratio) - 1.0_f64).sqrt());
        let za = ((-omega) * damping_ratio);
        let z1 = (za - zb);
        let z2 = (za + zb);
        let e1 = (z1 * delta_time).exp();
        let e2 = (z2 * delta_time).exp();
        let inv_denominator = (1.0_f64 / (z2 - z1));
        pos_pos_coef = (((z2 * e1) - (z1 * e2)) * inv_denominator);
        pos_vel_coef = ((e2 - e1) * inv_denominator);
        vel_pos_coef = (((z1 * z2) * (e1 - e2)) * inv_denominator);
        vel_vel_coef = (((z2 * e2) - (z1 * e1)) * inv_denominator);
    } else {
        if (damping_ratio < (1.0_f64 - CRITICAL_BAND)) {
            let alpha = (damping_ratio * omega);
            let beta = (omega * (1.0_f64 - (damping_ratio * damping_ratio)).sqrt());
            let envelope = ((-alpha) * delta_time).exp();
            let cosine = (beta * delta_time).cos();
            let sine = (beta * delta_time).sin();
            let inv_beta = (1.0_f64 / beta);
            pos_pos_coef = (envelope * (cosine + ((alpha * inv_beta) * sine)));
            pos_vel_coef = ((envelope * inv_beta) * sine);
            vel_pos_coef = (((((-envelope) * omega) * omega) * inv_beta) * sine);
            vel_vel_coef = (envelope * (cosine - ((alpha * inv_beta) * sine)));
        } else {
            let envelope = ((-omega) * delta_time).exp();
            let omega_dt = (omega * delta_time);
            pos_pos_coef = (envelope * (1.0_f64 + omega_dt));
            pos_vel_coef = (envelope * delta_time);
            vel_pos_coef = ((((-envelope) * omega) * omega) * delta_time);
            vel_vel_coef = (envelope * (1.0_f64 - omega_dt));
        }
    }
    spring.value = ((target + (pos_pos_coef * c0)) + (pos_vel_coef * velocity));
    spring.velocity = ((vel_pos_coef * c0) + (vel_vel_coef * velocity));
}

// Source: upstream/packages/spring/src/spring.ts:124 (sha256:51c2a5cdd6e48553aa10b13b75c63f98d5ecaf1afaee2d668a152c7d479ef9f7)
const CRITICAL_BAND: f64 = 0.0001_f64;
