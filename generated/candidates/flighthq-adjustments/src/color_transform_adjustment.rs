// @generated from upstream/packages/adjustments/src/colorTransformAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{ColorTransform, ColorTransformAdjustment};

// Source: upstream/packages/adjustments/src/colorTransformAdjustment.ts:8 (sha256:955e299a6cd1f2578492afb1bfba9b9480fdcba8b92c4c10f75950e1c42abb34)
pub fn create_color_transform_adjustment(
    color_transform: &ColorTransform,
) -> ColorTransformAdjustment {
    let color_matrix = vec![
        color_transform.red_multiplier,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        color_transform.red_offset,
        0.0_f64,
        color_transform.green_multiplier,
        0.0_f64,
        0.0_f64,
        color_transform.green_offset,
        0.0_f64,
        0.0_f64,
        color_transform.blue_multiplier,
        0.0_f64,
        color_transform.blue_offset,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        color_transform.alpha_multiplier,
        color_transform.alpha_offset,
    ];
    return ColorTransformAdjustment {
        __flight_identity: std::sync::Arc::new(()),
        kind: "ColorTransformAdjustment".to_owned(),
        color_transform: (*color_transform).clone(),
        color_matrix: (color_matrix).clone(),
        ..Default::default()
    };
}
