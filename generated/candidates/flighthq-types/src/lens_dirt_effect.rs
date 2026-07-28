// @generated from upstream/packages/types/src/LensDirtEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/LensDirtEffect.ts:5 (sha256:e7bd18362130d985500ce1df2f7602792550595aa8ca8c015abc295b26b00ed7)
#[derive(Clone)]
pub struct LensDirtEffect {
    pub kind: String,
    pub intensity: Option<f64>,
    pub threshold: Option<f64>,
    pub seed: Option<f64>,
}
