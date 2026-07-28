// @generated from upstream/packages/types/src/InnerShadowEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::InnerEffectSourceMode;

// Source: upstream/packages/types/src/InnerShadowEffect.ts:7 (sha256:7183cdb448684c12099a20b237230f777d4b82a482de25e13c022cf188053e0b)
#[derive(Clone)]
pub struct InnerShadowEffect {
    pub kind: String,
    pub alpha: Option<f64>,
    pub angle: Option<f64>,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
    pub color: Option<f64>,
    pub distance: Option<f64>,
    pub quality: Option<f64>,
    pub source_mode: Option<InnerEffectSourceMode>,
    pub strength: Option<f64>,
}
