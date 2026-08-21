// @generated from upstream/packages/adjustments/src/invertAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::InvertAdjustment;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub intensity: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/invertAdjustment.ts:6 (sha256:f0cece1b700cc75703c05ccd0a1038b0ffede4d98335b96e8a981fb13ab494bb)
#[derive(Clone, Default)]
struct CreateInvertAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateInvertAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_invert_adjustment(options: Option<FlightOmitRecord1>) -> InvertAdjustment {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
    });
    let intensity = (options.intensity).clone().unwrap_or(1.0_f64);
    let s = (1.0_f64 - (2.0_f64 * intensity));
    let o = intensity;
    let color_matrix = vec![
        s, 0.0_f64, 0.0_f64, 0.0_f64, o, 0.0_f64, s, 0.0_f64, 0.0_f64, o, 0.0_f64, 0.0_f64, s,
        0.0_f64, o, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
    ];
    return {
        let __flight_spread_1 = options;
        InvertAdjustment {
            __flight_identity: std::sync::Arc::new(()),
            kind: "InvertAdjustment".to_owned(),
            color_matrix: (color_matrix).clone(),
            intensity: __flight_spread_1.intensity,
            ..Default::default()
        }
    };
}
