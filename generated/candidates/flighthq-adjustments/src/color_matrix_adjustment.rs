// @generated from upstream/packages/adjustments/src/colorMatrixAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{COLOR_MATRIX_LENGTH as color_matrix_length_constant, initialize_adjustment};
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    AdjustmentKind, ColorBlindType, ColorMatrixAdjustment, ColorScaleBiasLike, EntityConstruction,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord1762611335 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub color_matrix: Option<Vec<f64>>,
    pub intensity: Option<f64>,
    pub exposure: Option<f64>,
    pub color_scale_bias: Option<ColorScaleBiasLike>,
    pub type_: Option<ColorBlindType>,
    pub matrix: Option<Vec<f64>>,
    pub brightness: Option<f64>,
    pub contrast: Option<f64>,
}
impl PartialEq for FlightPartialRecord1762611335 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/colorMatrixAdjustment.ts:7 (sha256:3f013edd497d9b9fedc3dfdc58483f652b75f7738ff6e0a45b024542015857cc)
pub fn create_color_matrix_adjustment(color_matrix: &Vec<f64>) -> ColorMatrixAdjustment {
    if ((color_matrix.len() as f64) != color_matrix_length_constant) {
        panic!(
            "{}",
            format!(
                "Color matrix must contain {} values.",
                color_matrix_length_constant
            )
        );
    }
    let mut out = allocate_entity();
    initialize_color_matrix_adjustment(
        (out).clone(),
        "ColorMatrixAdjustment".to_owned(),
        &(({
            let mut __flight_array = Vec::new();
            __flight_array.extend((color_matrix).iter().cloned());
            __flight_array
        })
        .iter()
        .map(|__flight_value| *__flight_value)
        .collect::<Vec<_>>()),
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/adjustments/src/colorMatrixAdjustment.ts:20 (sha256:bcb99d30a02350b0fe0cd77861c66f0d1347a1b34f69effc36574a2181e98898)
pub fn get_adjustment_color_matrix(operation: &SharedStructuralRecord1) -> Option<Vec<f64>> {
    let matrix = None::<Vec<f64>>;
    return if ((matrix).is_some())
        && ((matrix.as_ref().unwrap().len() as f64) == color_matrix_length_constant)
    {
        (matrix).clone()
    } else {
        None
    };
}

// Source: upstream/packages/adjustments/src/colorMatrixAdjustment.ts:25 (sha256:715d242ecc07ed30460b98d7678c7dae1e1a95d0e4a8ac903507f56d5ea868bc)
pub fn initialize_color_matrix_adjustment<T: Clone>(
    out: EntityConstruction<T>,
    kind: AdjustmentKind,
    color_matrix: &Vec<f64>,
) -> () {
    initialize_adjustment((out).clone(), (kind).clone());
    crate::host_set("host.colorMatrix", color_matrix);
}

// Source: upstream/packages/adjustments/src/colorMatrixAdjustment.ts:35 (sha256:ae984486a3ac2724e48ba3d0fcbab65099e12d56c0de5f73166c3bfb1aeb4fd7)
pub fn is_color_matrix_adjustment(operation: &SharedStructuralRecord1) -> bool {
    return (get_adjustment_color_matrix(operation)).is_some();
}
