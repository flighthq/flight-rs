// @generated from upstream/packages/types/src/FilmGrainEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/FilmGrainEffect.ts:3 (sha256:9a00b56b3964f4300679fc03b08efc716144f91285ef67e7f80fc772a723c568)
#[derive(Clone, Default)]
pub struct FilmGrainEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub intensity: Option<f64>,
    pub size: Option<f64>,
    pub seed: Option<f64>,
}
impl PartialEq for FilmGrainEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
