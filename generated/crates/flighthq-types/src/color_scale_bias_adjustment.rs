// @generated from upstream/packages/types/src/ColorScaleBiasAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AdjustmentKind, ColorScaleBiasLike};

// Source: upstream/packages/types/src/ColorScaleBiasAdjustment.ts:7 (sha256:3278880a1d9211064f698564ca44537606ecece7981d51d39f1d1c3458fe3541)
#[derive(Clone, Default)]
pub struct ColorScaleBiasAdjustment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: AdjustmentKind,
    pub color_matrix: Vec<f64>,
    pub color_scale_bias: ColorScaleBiasLike,
}
impl PartialEq for ColorScaleBiasAdjustment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
