// @generated from upstream/packages/adjustments/src/colorMatrixAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::COLOR_MATRIX_LENGTH as color_matrix_length_constant;
use flighthq_types::AdjustmentKind;

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub color_matrix: Option<Vec<f64>>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/colorMatrixAdjustment.ts:9 (sha256:bcb99d30a02350b0fe0cd77861c66f0d1347a1b34f69effc36574a2181e98898)
#[derive(Clone)]
struct GetAdjustmentColorMatrixRecord2 {
    __flight_identity: std::sync::Arc<()>,
    kind: String,
}
impl PartialEq for GetAdjustmentColorMatrixRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn get_adjustment_color_matrix(
    operation: &GetAdjustmentColorMatrixRecord2,
) -> Option<Vec<f64>> {
    let matrix = (operation.color_matrix).clone();
    return if ((matrix).is_some()
        && ((matrix.as_ref().unwrap().len() as f64) == color_matrix_length_constant))
    {
        (matrix).clone()
    } else {
        None
    };
}

// Source: upstream/packages/adjustments/src/colorMatrixAdjustment.ts:15 (sha256:ae984486a3ac2724e48ba3d0fcbab65099e12d56c0de5f73166c3bfb1aeb4fd7)
#[derive(Clone)]
struct IsColorMatrixAdjustmentRecord2 {
    __flight_identity: std::sync::Arc<()>,
    kind: String,
}
impl PartialEq for IsColorMatrixAdjustmentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn is_color_matrix_adjustment(operation: &IsColorMatrixAdjustmentRecord2) -> bool {
    return (get_adjustment_color_matrix(operation)).is_some();
}
