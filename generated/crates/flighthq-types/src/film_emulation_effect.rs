// @generated from upstream/packages/types/src/FilmEmulationEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/FilmEmulationEffect.ts:2 (sha256:e9d3989afcb32606cd87e5b1370d32b966a924f8e797e26c268758ee68e5cbba)
#[derive(Clone, Default)]
pub struct FilmEmulationEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub gate_weave: Option<f64>,
    pub grain_intensity: Option<f64>,
    pub halation_radius: Option<f64>,
    pub halation_strength: Option<f64>,
}
impl PartialEq for FilmEmulationEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
