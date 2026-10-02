// @generated from upstream/packages/adjustments/src/invertAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::initialize_color_matrix_adjustment;
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{ColorBlindType, ColorScaleBiasLike, EntityConstruction, InvertAdjustment};

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

// Source: upstream/packages/adjustments/src/invertAdjustment.ts:6 (sha256:69ba7ea613e6d72eb7abef050ff3cab87a023368af8db3e061f9cba7e2180afc)
#[derive(Clone, Default)]
struct CreateInvertAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateInvertAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_invert_adjustment(options: Option<FlightOmitRecord2968336371>) -> InvertAdjustment {
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
    initialize_invert_adjustment(
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

// Source: upstream/packages/adjustments/src/invertAdjustment.ts:17 (sha256:b93f52c3757b3d9bccaa2cb53f7a9c1e8e4d916ea9d72430edd649187af69f0c)
#[derive(Clone, Default)]
struct InitializeInvertAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for InitializeInvertAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn initialize_invert_adjustment(
    out: EntityConstruction<InvertAdjustment>,
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
    let intensity = (options.intensity).unwrap_or(1.0_f64);
    let s = (1.0_f64 - (2.0_f64 * intensity));
    let o = intensity;
    let color_matrix = vec![
        s, 0.0_f64, 0.0_f64, 0.0_f64, o, 0.0_f64, s, 0.0_f64, 0.0_f64, o, 0.0_f64, 0.0_f64, s,
        0.0_f64, o, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
    ];
    initialize_color_matrix_adjustment((out).clone(), "InvertAdjustment".to_owned(), &color_matrix);
    crate::host_set("host.intensity", intensity);
}
