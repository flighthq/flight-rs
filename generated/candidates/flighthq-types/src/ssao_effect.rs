// @generated from upstream/packages/types/src/SsaoEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SsaoEffect.ts:3 (sha256:c4feec58172d610c6635234d248dea594bbfc0948c9cb586b7af2a3c0335fbd6)
#[derive(Clone)]
pub struct SsaoEffect {
    pub kind: String,
    pub radius: Option<f64>,
    pub intensity: Option<f64>,
    pub bias: Option<f64>,
    pub samples: Option<f64>,
}
