// @generated from upstream/packages/types/src/ChromaticAberrationEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/ChromaticAberrationEffect.ts:3 (sha256:fa93abec91ebb99b2582060786d84b1f62223be7892adf18c50407ffac76ed2c)
#[derive(Clone, Default)]
pub struct ChromaticAberrationEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub intensity: Option<f64>,
    pub radial: Option<bool>,
}
impl PartialEq for ChromaticAberrationEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
