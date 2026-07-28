// @generated from upstream/packages/types/src/GodRaysEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GodRaysEffect.ts:3 (sha256:200f20a7b556d1c3a1c4880fde41f35aba28c6d41c03cf434ac1c39eb00f2275)
#[derive(Clone)]
pub struct GodRaysEffect {
    pub kind: String,
    pub center_x: Option<f64>,
    pub center_y: Option<f64>,
    pub density: Option<f64>,
    pub decay: Option<f64>,
    pub weight: Option<f64>,
    pub exposure: Option<f64>,
    pub samples: Option<f64>,
}
