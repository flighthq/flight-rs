// @generated from upstream/packages/types/src/DropShadowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EffectSourceMode;

// Source: upstream/packages/types/src/DropShadowEffect.ts:7 (sha256:6848511a980718c7082335e4839bb93de5a772a3a8714f0f1a720796bc2ca393)
#[derive(Clone)]
pub struct DropShadowEffect {
    pub kind: String,
    pub alpha: Option<f64>,
    pub angle: Option<f64>,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
    pub color: Option<f64>,
    pub distance: Option<f64>,
    pub quality: Option<f64>,
    pub source_mode: Option<EffectSourceMode>,
    pub strength: Option<f64>,
}
