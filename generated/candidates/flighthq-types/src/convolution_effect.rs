// @generated from upstream/packages/types/src/ConvolutionEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ConvolutionEffect.ts:8 (sha256:bde9a7679c21f48a1a9479c7bfcd6dd049d9255cee1c88feaecc530e7fbfb5fc)
#[derive(Clone)]
pub struct ConvolutionEffect {
    pub kind: String,
    pub matrix: Vec<f64>,
    pub matrix_x: f64,
    pub matrix_y: f64,
    pub bias: Option<f64>,
    pub clamp: Option<bool>,
    pub color: Option<f64>,
    pub divisor: Option<f64>,
    pub preserve_alpha: Option<bool>,
}
