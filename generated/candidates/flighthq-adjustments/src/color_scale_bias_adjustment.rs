// @generated from upstream/packages/adjustments/src/colorScaleBiasAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::initialize_color_matrix_adjustment;
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    ColorScaleBias, ColorScaleBiasAdjustment, ColorScaleBiasLike, EntityConstruction,
};

// Source: upstream/packages/adjustments/src/colorScaleBiasAdjustment.ts:6 (sha256:fa68f3a4836e52573a8fcc4d5b4d53dd1defe78cbb2b40bcb87946e8832aadd5)
pub fn create_color_scale_bias_adjustment(
    color_scale_bias: &ColorScaleBiasLike,
) -> ColorScaleBiasAdjustment {
    let mut out = allocate_entity();
    initialize_color_scale_bias_adjustment((out).clone(), color_scale_bias);
    return finish_entity((out).clone());
}

// Source: upstream/packages/adjustments/src/colorScaleBiasAdjustment.ts:12 (sha256:167a08168d0b618a2f9e5a9f7edb32b54f92ce3cc6d87af3ea9799d7e12a7d57)
pub fn initialize_color_scale_bias_adjustment(
    out: EntityConstruction<ColorScaleBiasAdjustment>,
    color_scale_bias: &ColorScaleBiasLike,
) -> () {
    let value = {
        let __flight_spread_0 = (*color_scale_bias).clone();
        ColorScaleBias {
            __flight_identity: std::sync::Arc::new(()),
            __flight_entity_runtime: std::sync::Arc::new(std::sync::Mutex::new(
                __flight_spread_0
                    .__flight_entity_runtime
                    .lock()
                    .unwrap()
                    .clone(),
            )),
            __flight_entity_snapshot: __flight_spread_0
                .__flight_entity_snapshot
                .clone()
                .or_else(|| Some(std::sync::Arc::new(__flight_spread_0.clone()))),
            alpha_scale: __flight_spread_0.alpha_scale,
            alpha_bias: __flight_spread_0.alpha_bias,
            blue_scale: __flight_spread_0.blue_scale,
            blue_bias: __flight_spread_0.blue_bias,
            green_scale: __flight_spread_0.green_scale,
            green_bias: __flight_spread_0.green_bias,
            red_scale: __flight_spread_0.red_scale,
            red_bias: __flight_spread_0.red_bias,
        }
    };
    let color_matrix = vec![
        value.red_scale,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        value.red_bias,
        0.0_f64,
        value.green_scale,
        0.0_f64,
        0.0_f64,
        value.green_bias,
        0.0_f64,
        0.0_f64,
        value.blue_scale,
        0.0_f64,
        value.blue_bias,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        value.alpha_scale,
        value.alpha_bias,
    ];
    initialize_color_matrix_adjustment(
        (out).clone(),
        "ColorScaleBiasAdjustment".to_owned(),
        &color_matrix,
    );
    crate::host_set("host.colorScaleBias", value);
}
