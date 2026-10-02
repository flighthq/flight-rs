// @generated from upstream/packages/adjustments/src/brightnessContrastAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::initialize_color_matrix_adjustment;
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    BrightnessContrastAdjustment, ColorBlindType, ColorScaleBiasLike, EntityConstruction,
};

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

// Source: upstream/packages/adjustments/src/brightnessContrastAdjustment.ts:6 (sha256:64634f824372a0c74eb99c2af074c9fa2dbef64a99302a279d2ef28947088803)
#[derive(Clone, Default)]
struct CreateBrightnessContrastAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateBrightnessContrastAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_brightness_contrast_adjustment(
    options: Option<FlightOmitRecord2968336371>,
) -> BrightnessContrastAdjustment {
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
    initialize_brightness_contrast_adjustment(
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

// Source: upstream/packages/adjustments/src/brightnessContrastAdjustment.ts:21 (sha256:9aa94eec409a146ed7a5e9cc237aaef88de1b5eca19f9f3507445470f3475f94)
#[derive(Clone, Default)]
struct InitializeBrightnessContrastAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for InitializeBrightnessContrastAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn initialize_brightness_contrast_adjustment(
    out: EntityConstruction<BrightnessContrastAdjustment>,
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
    let brightness = (options.brightness).unwrap_or(0.0_f64);
    let contrast = (options.contrast).unwrap_or(1.0_f64);
    let s = contrast;
    let o = ((brightness * contrast) + (0.5_f64 * (1.0_f64 - contrast)));
    let color_matrix = vec![
        s, 0.0_f64, 0.0_f64, 0.0_f64, o, 0.0_f64, s, 0.0_f64, 0.0_f64, o, 0.0_f64, 0.0_f64, s,
        0.0_f64, o, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
    ];
    initialize_color_matrix_adjustment(
        (out).clone(),
        "BrightnessContrastAdjustment".to_owned(),
        &color_matrix,
    );
    crate::host_set("host.brightness", brightness);
    crate::host_set("host.contrast", contrast);
}
