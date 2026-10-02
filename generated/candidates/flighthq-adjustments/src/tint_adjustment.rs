// @generated from upstream/packages/adjustments/src/tintAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::initialize_color_matrix_adjustment;
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{EntityConstruction, TintAdjustment};

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

// Source: upstream/packages/adjustments/src/tintAdjustment.ts:6 (sha256:f7d727adb91ab9498bdfdeb7dff2713bc0088ddb881c35d0ad5032ffbaf10d02)
pub fn create_tint_adjustment(rgba: f64) -> TintAdjustment {
    let mut out = allocate_entity();
    initialize_tint_adjustment((out).clone(), rgba);
    return finish_entity((out).clone());
}

// Source: upstream/packages/adjustments/src/tintAdjustment.ts:16 (sha256:e9315a662742fa485393a30ded24f12b4be42d31a7ec78f102cf6097b06f539e)
pub fn initialize_tint_adjustment(out: EntityConstruction<TintAdjustment>, rgba: f64) -> () {
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
    initialize_color_matrix_adjustment((out).clone(), "TintAdjustment".to_owned(), &color_matrix);
}
