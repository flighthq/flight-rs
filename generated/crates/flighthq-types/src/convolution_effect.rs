// @generated from upstream/packages/types/src/ConvolutionEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/ConvolutionEffect.ts:8 (sha256:bde9a7679c21f48a1a9479c7bfcd6dd049d9255cee1c88feaecc530e7fbfb5fc)
#[derive(Clone, Default)]
pub struct ConvolutionEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub matrix: Vec<f64>,
    pub matrix_x: f64,
    pub matrix_y: f64,
    pub bias: Option<f64>,
    pub clamp: Option<bool>,
    pub color: Option<f64>,
    pub divisor: Option<f64>,
    pub preserve_alpha: Option<bool>,
}
impl PartialEq for ConvolutionEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
