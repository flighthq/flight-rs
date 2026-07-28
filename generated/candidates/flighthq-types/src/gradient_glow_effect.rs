// @generated from upstream/packages/types/src/GradientGlowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EffectSourceMode;

// Source: upstream/packages/types/src/GradientGlowEffect.ts:7 (sha256:095a13e1c53d9734759a7a788007e08fb2ad1347e36f353e48ba2e6b730c44ae)
#[derive(Clone)]
pub struct GradientGlowEffect {
    pub kind: String,
    pub alphas: Vec<f64>,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
    pub colors: Vec<f64>,
    pub quality: Option<f64>,
    pub ratios: Vec<f64>,
    pub source_mode: Option<EffectSourceMode>,
    pub strength: Option<f64>,
}
