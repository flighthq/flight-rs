// @generated from upstream/packages/types/src/FilmGrainEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/FilmGrainEffect.ts:3 (sha256:9a00b56b3964f4300679fc03b08efc716144f91285ef67e7f80fc772a723c568)
#[derive(Clone)]
pub struct FilmGrainEffect {
    pub kind: String,
    pub intensity: Option<f64>,
    pub size: Option<f64>,
    pub seed: Option<f64>,
}
