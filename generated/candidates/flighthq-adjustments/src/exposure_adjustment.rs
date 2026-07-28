// @generated from upstream/packages/adjustments/src/exposureAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ExposureAdjustment;

// Source: upstream/packages/adjustments/src/exposureAdjustment.ts:9 (sha256:a244d4a1c4c5edfabd326c03a596be32c1c8c399f83425daa656837c35eeb160)
#[derive(Clone)]
struct CreateExposureAdjustmentRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateExposureAdjustmentRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_exposure_adjustment(options: Option<ExposureAdjustment>) -> ExposureAdjustment {
    let options = options.unwrap_or(ExposureAdjustment {
        __flight_identity: std::sync::Arc::new(()),
        exposure: None,
    });
    let m = (2.0_f64).powf((options.exposure).unwrap_or(0.0_f64));
    let color_matrix = vec![
        m, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, m, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
        0.0_f64, m, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
    ];
    return ExposureAdjustment {
        kind: "ExposureAdjustment".to_owned(),
        color_matrix: color_matrix,
        ..((options).clone()).clone()
    };
}
