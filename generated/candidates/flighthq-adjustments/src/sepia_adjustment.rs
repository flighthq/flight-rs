// @generated from upstream/packages/adjustments/src/sepiaAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::SepiaAdjustment;

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub intensity: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/sepiaAdjustment.ts:5 (sha256:b67e0c7ed9fbb4d1eeb6be54adfcc307d4b9e7daf89050454664eaba40353180)
#[derive(Clone)]
struct CreateSepiaAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateSepiaAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_sepia_adjustment(options: Option<FlightOmitRecord1>) -> SepiaAdjustment {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
    });
    let k = (options.intensity).unwrap_or(1.0_f64);
    let j = (1.0_f64 - k);
    let color_matrix = vec![
        (j + (0.393_f64 * k)),
        (0.769_f64 * k),
        (0.189_f64 * k),
        0.0_f64,
        0.0_f64,
        (0.349_f64 * k),
        (j + (0.686_f64 * k)),
        (0.168_f64 * k),
        0.0_f64,
        0.0_f64,
        (0.272_f64 * k),
        (0.534_f64 * k),
        (j + (0.131_f64 * k)),
        0.0_f64,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        1.0_f64,
        0.0_f64,
    ];
    return {
        let __flight_spread_1 = options;
        SepiaAdjustment {
            __flight_identity: std::sync::Arc::new(()),
            kind: "SepiaAdjustment".to_owned(),
            color_matrix: (color_matrix).clone(),
            intensity: __flight_spread_1.intensity,
        }
    };
}
