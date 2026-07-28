// @generated from upstream/packages/types/src/TiltShiftEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TiltShiftEffect.ts:3 (sha256:fbcbf389afcd6233e11df0aa41d734c8fa5b4c26a8c9ed22d9e3e46a707e24a6)
#[derive(Clone)]
pub struct TiltShiftEffect {
    pub kind: String,
    pub center: Option<f64>,
    pub width: Option<f64>,
    pub blur: Option<f64>,
}
