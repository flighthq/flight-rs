// @generated from upstream/packages/adjustments/src/colorScaleBiasAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{ColorScaleBias, ColorScaleBiasAdjustment, ColorScaleBiasLike};

// Source: upstream/packages/adjustments/src/colorScaleBiasAdjustment.ts:3 (sha256:6dfac73af83815a3820176bc8913267661f2f7a1da605ee647d1725f78ba5c1b)
pub fn create_color_scale_bias_adjustment(
    color_scale_bias: &ColorScaleBiasLike,
) -> ColorScaleBiasAdjustment {
    let value = {
        let __flight_entity_spread = color_scale_bias;
        ColorScaleBias {
            __flight_entity_runtime: std::sync::Arc::new(std::sync::Mutex::new(
                __flight_entity_spread
                    .__flight_entity_runtime
                    .lock()
                    .unwrap()
                    .clone(),
            )),
            __flight_identity: std::sync::Arc::new(()),
            ..__flight_entity_spread.clone()
        }
    };
    return ColorScaleBiasAdjustment {
        __flight_identity: std::sync::Arc::new(()),
        kind: "ColorScaleBiasAdjustment".to_owned(),
        color_scale_bias: value,
        color_matrix: vec![
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
        ],
        ..Default::default()
    };
}
