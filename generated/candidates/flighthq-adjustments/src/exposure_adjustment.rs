// @generated from upstream/packages/adjustments/src/exposureAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::initialize_color_matrix_adjustment;
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{ColorBlindType, ColorScaleBiasLike, EntityConstruction, ExposureAdjustment};

#[derive(Clone, Default)]
pub struct FlightOmitRecord2968336371 {
    pub __flight_identity: std::sync::Arc<()>,
    pub intensity: Option<f64>,
    pub exposure: Option<f64>,
    pub color_scale_bias: ColorScaleBiasLike,
    pub type_: Option<ColorBlindType>,
    pub matrix: Vec<f64>,
    pub brightness: Option<f64>,
    pub contrast: Option<f64>,
}
impl PartialEq for FlightOmitRecord2968336371 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/exposureAdjustment.ts:6 (sha256:b664337299c575e7c3c9804f82fa44b758eeafa2569f635d4abe0f3aa5aa2b20)
#[derive(Clone, Default)]
struct CreateExposureAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateExposureAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_exposure_adjustment(
    options: Option<FlightOmitRecord2968336371>,
) -> ExposureAdjustment {
    let options = options.unwrap_or(FlightOmitRecord2968336371 {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        exposure: None,
        color_scale_bias: Default::default(),
        type_: None,
        matrix: Default::default(),
        brightness: None,
        contrast: None,
    });
    let mut out = allocate_entity();
    initialize_exposure_adjustment(
        (out).clone(),
        Some({
            let __flight_source = &((options).clone());
            FlightOmitRecord2968336371 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                intensity: __flight_source.intensity,
                exposure: __flight_source.exposure,
                color_scale_bias: (__flight_source.color_scale_bias).clone(),
                type_: (__flight_source.type_).clone(),
                matrix: (__flight_source.matrix).clone(),
                brightness: __flight_source.brightness,
                contrast: __flight_source.contrast,
            }
        }),
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/adjustments/src/exposureAdjustment.ts:20 (sha256:c399085cf15b6e5fa423b2bd59aae2e73d1362f1ec2fa7a08d040684240be207)
#[derive(Clone, Default)]
struct InitializeExposureAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for InitializeExposureAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn initialize_exposure_adjustment(
    out: EntityConstruction<ExposureAdjustment>,
    options: Option<FlightOmitRecord2968336371>,
) -> () {
    let options = options.unwrap_or(FlightOmitRecord2968336371 {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        exposure: None,
        color_scale_bias: Default::default(),
        type_: None,
        matrix: Default::default(),
        brightness: None,
        contrast: None,
    });
    let exposure = (options.exposure).unwrap_or(0.0_f64);
    let m = (2.0_f64).powf(exposure);
    let color_matrix = vec![
        m, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, m, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
        0.0_f64, m, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
    ];
    initialize_color_matrix_adjustment(
        (out).clone(),
        "ExposureAdjustment".to_owned(),
        &(((color_matrix).clone())
            .iter()
            .map(|__flight_value| *__flight_value)
            .collect::<Vec<_>>()),
    );
    crate::host_set("host.exposure", exposure);
}
