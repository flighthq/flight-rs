// @generated from upstream/packages/types/src/DisplacementEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/DisplacementEffect.ts:5 (sha256:d044c5db42b3c9fd414c55ee4d38b0ab52d2e6cba2906645096d521ef78f57f3)
#[derive(Clone)]
pub struct DisplacementEffect {
    pub kind: String,
    pub intensity: Option<f64>,
    pub frequency: Option<f64>,
    pub seed: Option<f64>,
}
