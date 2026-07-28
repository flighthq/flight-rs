// @generated from upstream/packages/adjustments/src/brightnessContrastAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::BrightnessContrastAdjustment;

// Source: upstream/packages/adjustments/src/brightnessContrastAdjustment.ts:10 (sha256:076aa1b0b6b9f74f032d3f8d445eb38f4934ff5691844e38cc37840962b869c4)
#[derive(Clone)]
struct CreateBrightnessContrastAdjustmentRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateBrightnessContrastAdjustmentRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_brightness_contrast_adjustment(
    options: Option<BrightnessContrastAdjustment>,
) -> BrightnessContrastAdjustment {
    let options = options.unwrap_or(BrightnessContrastAdjustment {
        __flight_identity: std::sync::Arc::new(()),
        brightness: None,
        contrast: None,
    });
    let brightness = (options.brightness).unwrap_or(0.0_f64);
    let contrast = (options.contrast).unwrap_or(1.0_f64);
    let s = contrast;
    let o = (255.0_f64 * ((brightness * contrast) + (0.5_f64 * (1.0_f64 - contrast))));
    let color_matrix = vec![
        s, 0.0_f64, 0.0_f64, 0.0_f64, o, 0.0_f64, s, 0.0_f64, 0.0_f64, o, 0.0_f64, 0.0_f64, s,
        0.0_f64, o, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
    ];
    return BrightnessContrastAdjustment {
        kind: "BrightnessContrastAdjustment".to_owned(),
        color_matrix: (color_matrix).clone(),
        ..((options).clone()).clone()
    };
}
