// @generated from upstream/packages/types/src/BitmapConvolutionOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::BitmapEdgeMode;

// Source: upstream/packages/types/src/BitmapConvolutionOptions.ts:3 (sha256:36622be26b0d6d74ad2b9612c9d5c08e151474680298a719a6cb4f5e3099003e)
#[derive(Clone, Default)]
pub struct BitmapConvolutionOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bias: Option<f64>,
    pub edge: Option<BitmapEdgeMode>,
    pub divisor: Option<f64>,
    pub matrix: Vec<f64>,
    pub matrix_x: f64,
    pub matrix_y: f64,
    pub preserve_alpha: Option<bool>,
}
impl PartialEq for BitmapConvolutionOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
