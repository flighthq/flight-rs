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
    return ColorScaleBiasAdjustment {
        __flight_identity: std::sync::Arc::new(()),
        kind: "ColorScaleBiasAdjustment".to_owned(),
        color_scale_bias: (value).clone(),
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
