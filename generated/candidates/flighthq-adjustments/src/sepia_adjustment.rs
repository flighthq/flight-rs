// @generated from upstream/packages/adjustments/src/sepiaAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::initialize_color_matrix_adjustment;
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{ColorBlindType, ColorScaleBiasLike, EntityConstruction, SepiaAdjustment};

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

// Source: upstream/packages/adjustments/src/sepiaAdjustment.ts:6 (sha256:8d748cd56769d7c5a30d0035f5340956ed9c16ad042115dc8801ef5546adaca6)
#[derive(Clone, Default)]
struct CreateSepiaAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateSepiaAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_sepia_adjustment(options: Option<FlightOmitRecord2968336371>) -> SepiaAdjustment {
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
    initialize_sepia_adjustment(
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

// Source: upstream/packages/adjustments/src/sepiaAdjustment.ts:16 (sha256:d61807a8ac6ef4c82bddd24e0569a3abd6317a738bf602a0bc4a581c76f8b2f4)
#[derive(Clone, Default)]
struct InitializeSepiaAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for InitializeSepiaAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn initialize_sepia_adjustment(
    out: EntityConstruction<SepiaAdjustment>,
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
    initialize_color_matrix_adjustment((out).clone(), "SepiaAdjustment".to_owned(), &color_matrix);
    crate::host_set("host.intensity", intensity);
}
