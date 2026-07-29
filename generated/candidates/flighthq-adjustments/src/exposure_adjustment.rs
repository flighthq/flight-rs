// @generated from upstream/packages/adjustments/src/exposureAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ExposureAdjustment;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub intensity: Option<f64>,
    pub exposure: Option<f64>,
    pub color_transform: ColorTransform,
    pub type_: Option<ColorBlindType>,
    pub matrix: Vec<f64>,
    pub brightness: Option<f64>,
    pub contrast: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/exposureAdjustment.ts:9 (sha256:a244d4a1c4c5edfabd326c03a596be32c1c8c399f83425daa656837c35eeb160)
#[derive(Clone, Default)]
struct CreateExposureAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateExposureAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_exposure_adjustment(options: Option<FlightOmitRecord1>) -> ExposureAdjustment {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        exposure: None,
        type_: None,
        brightness: None,
        contrast: None,
    });
    let m = (2.0_f64).powf((options.exposure).unwrap_or(0.0_f64));
    let color_matrix = vec![
        m, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, m, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
        0.0_f64, m, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
    ];
    return {
        let __flight_spread_1 = options;
        ExposureAdjustment {
            __flight_identity: std::sync::Arc::new(()),
            kind: "ExposureAdjustment".to_owned(),
            color_matrix: color_matrix,
            intensity: __flight_spread_1.intensity,
            exposure: __flight_spread_1.exposure,
            color_transform: (__flight_spread_1.color_transform).clone(),
            type_: (__flight_spread_1.type_).clone(),
            matrix: (__flight_spread_1.matrix).clone(),
            brightness: __flight_spread_1.brightness,
            contrast: __flight_spread_1.contrast,
            ..Default::default()
        }
    };
}
