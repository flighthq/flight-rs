// @generated from upstream/packages/types/src/GradientBevelEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EffectSourceMode;

// Source: upstream/packages/types/src/GradientBevelEffect.ts:7 (sha256:76e90209baf6a5c6e39df6d8af199bc57e81f9812b2b9e6407b949000f3c38ab)
#[derive(Clone)]
pub struct GradientBevelEffect {
    pub kind: String,
    pub alphas: Vec<f64>,
    pub angle: Option<f64>,
    pub bevel_type: Option<String>,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
    pub colors: Vec<f64>,
    pub distance: Option<f64>,
    pub quality: Option<f64>,
    pub ratios: Vec<f64>,
    pub source_mode: Option<EffectSourceMode>,
    pub strength: Option<f64>,
}
