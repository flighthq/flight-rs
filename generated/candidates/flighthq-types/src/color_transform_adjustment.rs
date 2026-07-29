// @generated from upstream/packages/types/src/ColorTransformAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ColorBlindType;
use crate::{AdjustmentKind, ColorTransform};

// Source: upstream/packages/types/src/ColorTransformAdjustment.ts:10 (sha256:c31fc4c470375aaa6b620ec0028148ba1a6cbcfc3eca340bff502ed3fa6a0899)
#[derive(Clone, Default)]
pub struct ColorTransformAdjustment {
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
impl PartialEq for ColorTransformAdjustment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
