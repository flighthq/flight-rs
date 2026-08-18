// @generated from upstream/packages/adjustments/src/tintAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::TintAdjustment;

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

// Source: upstream/packages/adjustments/src/tintAdjustment.ts:7 (sha256:2212210f21fb1c49874d050d027d5b1b9f9ef816ff613ed3a4910c1c3ef8cd16)
pub fn create_tint_adjustment(rgba: f64) -> TintAdjustment {
    let red_scale = ((__flight_js_to_i32(
        (__flight_js_to_u32(rgba) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        / 255.0_f64);
    let green_scale = ((__flight_js_to_i32(
        (__flight_js_to_u32(rgba) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        / 255.0_f64);
    let blue_scale = ((__flight_js_to_i32(
        (__flight_js_to_u32(rgba) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        / 255.0_f64);
    let alpha_scale =
        ((__flight_js_to_i32(rgba) & __flight_js_to_i32(255.0_f64)) as f64 / 255.0_f64);
    let color_matrix = vec![
        red_scale,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        green_scale,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        blue_scale,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        alpha_scale,
        0.0_f64,
    ];
    return TintAdjustment {
        __flight_identity: std::sync::Arc::new(()),
        kind: "TintAdjustment".to_owned(),
        color_matrix: (color_matrix).clone(),
        ..Default::default()
    };
}
