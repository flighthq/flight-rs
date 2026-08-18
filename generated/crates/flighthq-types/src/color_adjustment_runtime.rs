// @generated from upstream/packages/types/src/ColorAdjustmentRuntime.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Adjustment, ColorScaleBias};

// Source: upstream/packages/types/src/ColorAdjustmentRuntime.ts:6 (sha256:6af421b0c66043312dce3e7248225fbb43b0df05efaec7f3186f4f863b33dd93)
#[derive(Clone, Default)]
pub struct ColorAdjustmentRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub color_adjustments_unsupported: bool,
    pub resolved_color_matrix: Option<Vec<f64>>,
    pub resolved_color_scale_bias: Option<ColorScaleBias>,
}
impl PartialEq for ColorAdjustmentRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
