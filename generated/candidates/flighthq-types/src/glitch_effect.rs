// @generated from upstream/packages/types/src/GlitchEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GlitchEffect.ts:5 (sha256:2ca3b14ea2ba238108237a31c7a3a2764abd2981b1ab0e441f8977b7fc5e5df2)
#[derive(Clone)]
pub struct GlitchEffect {
    pub kind: String,
    pub intensity: Option<f64>,
    pub block_size: Option<f64>,
    pub color_shift: Option<f64>,
    pub seed: Option<f64>,
}
