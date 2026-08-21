// @generated from upstream/packages/adjustments/src/grayscaleAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::GrayscaleAdjustment;

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

// Source: upstream/packages/adjustments/src/grayscaleAdjustment.ts:6 (sha256:c5def2b47e2ea5ec0351a431b3d66ef5a279c4f282694cb747b52790d7f73466)
#[derive(Clone, Default)]
struct CreateGrayscaleAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateGrayscaleAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_grayscale_adjustment(options: Option<FlightOmitRecord1>) -> GrayscaleAdjustment {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
    });
    let intensity = (options.intensity).clone().unwrap_or(1.0_f64);
    let k = intensity;
    let j = (1.0_f64 - intensity);
    let lr = (0.2126_f64 * k);
    let lg = (0.7152_f64 * k);
    let lb = (0.0722_f64 * k);
    let color_matrix = vec![
        (j + lr),
        lg,
        lb,
        0.0_f64,
        0.0_f64,
        lr,
        (j + lg),
        lb,
        0.0_f64,
        0.0_f64,
        lr,
        lg,
        (j + lb),
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
        GrayscaleAdjustment {
            __flight_identity: std::sync::Arc::new(()),
            kind: "GrayscaleAdjustment".to_owned(),
            color_matrix: (color_matrix).clone(),
            intensity: __flight_spread_1.intensity,
            ..Default::default()
        }
    };
}
