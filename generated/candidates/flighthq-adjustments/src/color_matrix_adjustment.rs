// @generated from upstream/packages/adjustments/src/colorMatrixAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::COLOR_MATRIX_LENGTH as color_matrix_length_constant;
use flighthq_types::{AdjustmentKind, ColorMatrixAdjustment};

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
pub struct FlightPartialRecord182769320 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub color_matrix: Option<Vec<f64>>,
}
impl PartialEq for FlightPartialRecord182769320 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/colorMatrixAdjustment.ts:5 (sha256:765e9636ff705c8f3cef389c914d2ed1ddec7ac43ff87751f904fe711d0ccfd4)
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
    return ColorMatrixAdjustment {
        __flight_identity: std::sync::Arc::new(()),
        kind: "ColorMatrixAdjustment".to_owned(),
        color_matrix: {
            let mut __flight_array = Vec::new();
            __flight_array.extend((color_matrix).iter().cloned());
            __flight_array
        },
    };
}

// Source: upstream/packages/adjustments/src/colorMatrixAdjustment.ts:16 (sha256:bcb99d30a02350b0fe0cd77861c66f0d1347a1b34f69effc36574a2181e98898)
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

// Source: upstream/packages/adjustments/src/colorMatrixAdjustment.ts:22 (sha256:ae984486a3ac2724e48ba3d0fcbab65099e12d56c0de5f73166c3bfb1aeb4fd7)
pub fn is_color_matrix_adjustment(operation: &SharedStructuralRecord1) -> bool {
    return (get_adjustment_color_matrix(operation)).is_some();
}
