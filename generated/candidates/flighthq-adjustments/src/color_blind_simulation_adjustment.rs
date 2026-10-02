// @generated from upstream/packages/adjustments/src/colorBlindSimulationAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::initialize_color_matrix_adjustment;
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    ColorBlindSimulationAdjustment, ColorBlindType, ColorScaleBiasLike, EntityConstruction,
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

// Source: upstream/packages/adjustments/src/colorBlindSimulationAdjustment.ts:11 (sha256:8e697c2f35509dbe8215f440bb0698902320d3ae79d9a9b27f868abaa5212a17)
#[derive(Clone, Default)]
struct CreateColorBlindSimulationAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateColorBlindSimulationAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_color_blind_simulation_adjustment(
    options: Option<FlightOmitRecord2968336371>,
) -> ColorBlindSimulationAdjustment {
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
    initialize_color_blind_simulation_adjustment(
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

// Source: upstream/packages/adjustments/src/colorBlindSimulationAdjustment.ts:28 (sha256:995bd0526b5955c8e8e9c3e63166fbb31ecfee99171179b08104fc23bf2bc453)
#[derive(Clone, Default)]
struct InitializeColorBlindSimulationAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for InitializeColorBlindSimulationAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn initialize_color_blind_simulation_adjustment(
    out: EntityConstruction<ColorBlindSimulationAdjustment>,
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
    let type_: ColorBlindType = ((options.type_).clone()).unwrap_or("deuteranopia".to_owned());
    let m = (COLOR_BLIND_MATRICES
        .iter()
        .find(|(entry_key, _)| entry_key == &(type_).clone())
        .map(|(_, value)| value.clone()))
    .expect("TypeScript Record key was absent");
    let color_matrix = vec![
        m[0.0_f64 as usize].clone(),
        m[1.0_f64 as usize].clone(),
        m[2.0_f64 as usize].clone(),
        0.0_f64,
        0.0_f64,
        m[3.0_f64 as usize].clone(),
        m[4.0_f64 as usize].clone(),
        m[5.0_f64 as usize].clone(),
        0.0_f64,
        0.0_f64,
        m[6.0_f64 as usize].clone(),
        m[7.0_f64 as usize].clone(),
        m[8.0_f64 as usize].clone(),
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
        "ColorBlindSimulationAdjustment".to_owned(),
        &color_matrix,
    );
    crate::host_set("host.type", type_);
}

// Source: upstream/packages/adjustments/src/colorBlindSimulationAdjustment.ts:46 (sha256:614a91fc3257d3208502a48c370680c1ba02007be2d0238596b7243f03b20c3c)
static COLOR_BLIND_MATRICES: std::sync::LazyLock<Vec<(ColorBlindType, Vec<f64>)>> =
    std::sync::LazyLock::new(|| {
        let mut __flight_record = Vec::new();
        __flight_record.push((
            "protanopia".to_owned(),
            vec![
                0.567_f64, 0.433_f64, 0.0_f64, 0.558_f64, 0.442_f64, 0.0_f64, 0.0_f64, 0.242_f64,
                0.758_f64,
            ],
        ));
        __flight_record.push((
            "protanomaly".to_owned(),
            vec![
                0.817_f64, 0.183_f64, 0.0_f64, 0.333_f64, 0.667_f64, 0.0_f64, 0.0_f64, 0.125_f64,
                0.875_f64,
            ],
        ));
        __flight_record.push((
            "deuteranopia".to_owned(),
            vec![
                0.625_f64, 0.375_f64, 0.0_f64, 0.7_f64, 0.3_f64, 0.0_f64, 0.0_f64, 0.3_f64, 0.7_f64,
            ],
        ));
        __flight_record.push((
            "deuteranomaly".to_owned(),
            vec![
                0.8_f64, 0.2_f64, 0.0_f64, 0.258_f64, 0.742_f64, 0.0_f64, 0.0_f64, 0.142_f64,
                0.858_f64,
            ],
        ));
        __flight_record.push((
            "tritanopia".to_owned(),
            vec![
                0.95_f64, 0.05_f64, 0.0_f64, 0.0_f64, 0.433_f64, 0.567_f64, 0.0_f64, 0.475_f64,
                0.525_f64,
            ],
        ));
        __flight_record.push((
            "tritanomaly".to_owned(),
            vec![
                0.967_f64, 0.033_f64, 0.0_f64, 0.0_f64, 0.733_f64, 0.267_f64, 0.0_f64, 0.183_f64,
                0.817_f64,
            ],
        ));
        __flight_record.push((
            "achromatopsia".to_owned(),
            vec![
                0.299_f64, 0.587_f64, 0.114_f64, 0.299_f64, 0.587_f64, 0.114_f64, 0.299_f64,
                0.587_f64, 0.114_f64,
            ],
        ));
        __flight_record.push((
            "achromatomaly".to_owned(),
            vec![
                0.618_f64, 0.32_f64, 0.062_f64, 0.163_f64, 0.775_f64, 0.062_f64, 0.163_f64,
                0.32_f64, 0.516_f64,
            ],
        ));
        __flight_record
    });
