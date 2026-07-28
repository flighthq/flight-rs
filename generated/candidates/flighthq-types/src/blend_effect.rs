// @generated from upstream/packages/types/src/BlendEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::AdvancedBlendMode;

// Source: upstream/packages/types/src/BlendEffect.ts:12 (sha256:d63118c410116509a903cf1414b9561e31bee6107c718e648d79b5c96fb3fd3b)
#[derive(Clone)]
pub struct BlendEffect {
    pub kind: String,
    pub mode: AdvancedBlendMode,
    pub backdrop_key: Option<String>,
    pub opacity: Option<f64>,
}
