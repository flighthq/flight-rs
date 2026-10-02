// @generated from upstream/packages/adjustments/src/grayscaleAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::initialize_color_matrix_adjustment;
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{ColorBlindType, ColorScaleBiasLike, EntityConstruction, GrayscaleAdjustment};

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

// Source: upstream/packages/adjustments/src/grayscaleAdjustment.ts:6 (sha256:4fdae9b31f55a702e21f6c00ecddf10b4dff6461449ac12e8ab1c2b13abc2215)
#[derive(Clone, Default)]
struct CreateGrayscaleAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateGrayscaleAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_grayscale_adjustment(
    options: Option<FlightOmitRecord2968336371>,
) -> GrayscaleAdjustment {
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
    initialize_grayscale_adjustment(
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

// Source: upstream/packages/adjustments/src/grayscaleAdjustment.ts:17 (sha256:2beb7b26409399ce6feb75de45816f0597a089a984f23842cefae7c65688038c)
#[derive(Clone, Default)]
struct InitializeGrayscaleAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for InitializeGrayscaleAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn initialize_grayscale_adjustment(
    out: EntityConstruction<GrayscaleAdjustment>,
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
    initialize_color_matrix_adjustment(
        (out).clone(),
        "GrayscaleAdjustment".to_owned(),
        &color_matrix,
    );
    crate::host_set("host.intensity", intensity);
}
