// @generated from upstream/packages/adjustments/src/colorAdjustmentResolution.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{fuse_color_matrices, get_adjustment_color_matrix};
use flighthq_types::{Adjustment, AdjustmentKind, ColorTransform};

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

// Source: upstream/packages/adjustments/src/colorAdjustmentResolution.ts:7 (sha256:0d9005b392e96e7e3bf5396796623af018d5f60bd48d2cd29325d7eba60a0f03)
pub const COLOR_ADJUSTMENT_NONE: f64 = 0.0_f64;

// Source: upstream/packages/adjustments/src/colorAdjustmentResolution.ts:8 (sha256:a361f5b587c35bc70665bd802cecb271700bea03cd6b68fcbfd3c6a9ed0d0dc8)
pub const COLOR_ADJUSTMENT_AFFINE: f64 = 1.0_f64;

// Source: upstream/packages/adjustments/src/colorAdjustmentResolution.ts:9 (sha256:a6c176b1432f701faf5961f8611eb3599296cc96019b2c14970165abe73257b2)
pub const COLOR_ADJUSTMENT_CHANNEL_MIXING: f64 = 2.0_f64;

// Source: upstream/packages/adjustments/src/colorAdjustmentResolution.ts:14 (sha256:b5c2b0957af3de020edc081efdcf8b5ea62997e1c7674f942ca104c049022b55)
pub fn is_affine_color_matrix(matrix: &Vec<f64>) -> bool {
    return (((((((((((matrix[1.0_f64 as usize].clone() == 0.0_f64)
        && (matrix[2.0_f64 as usize].clone() == 0.0_f64))
        && (matrix[3.0_f64 as usize].clone() == 0.0_f64))
        && (matrix[5.0_f64 as usize].clone() == 0.0_f64))
        && (matrix[7.0_f64 as usize].clone() == 0.0_f64))
        && (matrix[8.0_f64 as usize].clone() == 0.0_f64))
        && (matrix[10.0_f64 as usize].clone() == 0.0_f64))
        && (matrix[11.0_f64 as usize].clone() == 0.0_f64))
        && (matrix[13.0_f64 as usize].clone() == 0.0_f64))
        && (matrix[15.0_f64 as usize].clone() == 0.0_f64))
        && (matrix[16.0_f64 as usize].clone() == 0.0_f64))
        && (matrix[17.0_f64 as usize].clone() == 0.0_f64);
}

// Source: upstream/packages/adjustments/src/colorAdjustmentResolution.ts:43 (sha256:3fb78491e79650aebe321a41d474a8e227b4eb8c6065d805d3687fee0c1b35fc)
#[derive(Clone)]
struct OperationContextRecord2 {
    __flight_identity: std::sync::Arc<()>,
    kind: String,
}
impl PartialEq for OperationContextRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn resolve_color_adjustments_color_transform(
    adjustments: Option<Vec<Adjustment>>,
    out: &mut ColorTransform,
) -> f64 {
    if ((adjustments).is_none()) || ((adjustments.as_ref().unwrap().len() as f64) == 0.0_f64) {
        return COLOR_ADJUSTMENT_NONE;
    }
    let mut matrices: Vec<Vec<f64>> = vec![];
    let mut inlineable = true;
    {
        let mut i = 0.0_f64;
        while (i < (adjustments.as_ref().unwrap().len() as f64)) {
            let matrix = get_adjustment_color_matrix(&OperationContextRecord2 {
                __flight_identity: std::sync::Arc::clone(
                    &(adjustments.as_ref().unwrap()[i as usize]).__flight_identity,
                ),
                kind: ((adjustments.as_ref().unwrap()[i as usize]).kind).clone(),
            });
            if (matrix).is_none() {
                inlineable = false;
            } else {
                matrices.push(((matrix.as_ref().unwrap()).clone()).clone());
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let fused = fuse_color_matrices(&matrices);
    out.red_multiplier = fused[0.0_f64 as usize].clone();
    out.green_multiplier = fused[6.0_f64 as usize].clone();
    out.blue_multiplier = fused[12.0_f64 as usize].clone();
    out.alpha_multiplier = fused[18.0_f64 as usize].clone();
    out.red_offset = fused[4.0_f64 as usize].clone();
    out.green_offset = fused[9.0_f64 as usize].clone();
    out.blue_offset = fused[14.0_f64 as usize].clone();
    out.alpha_offset = fused[19.0_f64 as usize].clone();
    return if (inlineable) && (is_affine_color_matrix(&fused)) {
        COLOR_ADJUSTMENT_AFFINE
    } else {
        COLOR_ADJUSTMENT_CHANNEL_MIXING
    };
}
