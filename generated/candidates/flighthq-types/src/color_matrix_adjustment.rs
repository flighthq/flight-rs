// @generated from upstream/packages/types/src/ColorMatrixAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::AdjustmentKind;
use crate::{ColorBlindType, ColorTransform};

// Source: upstream/packages/types/src/ColorMatrixAdjustment.ts:9 (sha256:ddbd2cd1f56b8106535ff2ed70999f9d688030cfca652d1eeb30dc1feca81430)
#[derive(Clone, Default)]
pub struct ColorMatrixAdjustment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: AdjustmentKind,
    pub color_matrix: Vec<f64>,
    pub intensity: Option<f64>,
    pub exposure: Option<f64>,
    pub color_transform: ColorTransform,
    pub type_: Option<ColorBlindType>,
    pub matrix: Vec<f64>,
    pub brightness: Option<f64>,
    pub contrast: Option<f64>,
}
impl PartialEq for ColorMatrixAdjustment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
