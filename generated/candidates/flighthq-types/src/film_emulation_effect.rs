// @generated from upstream/packages/types/src/FilmEmulationEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/FilmEmulationEffect.ts:2 (sha256:e9d3989afcb32606cd87e5b1370d32b966a924f8e797e26c268758ee68e5cbba)
#[derive(Clone)]
pub struct FilmEmulationEffect {
    pub kind: String,
    pub gate_weave: Option<f64>,
    pub grain_intensity: Option<f64>,
    pub halation_radius: Option<f64>,
    pub halation_strength: Option<f64>,
}
