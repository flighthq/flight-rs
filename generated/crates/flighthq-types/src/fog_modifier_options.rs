// @generated from upstream/packages/types/src/FogModifierOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::FogModifierMode;

// Source: upstream/packages/types/src/FogModifierOptions.ts:3 (sha256:a140958cfd3e17565cf886b6ec71cf5ad24d26795dee44c1741d2a55287472e4)
#[derive(Clone, Default)]
pub struct FogModifierOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color: f64,
    pub mode: Option<FogModifierMode>,
    pub near: Option<f64>,
    pub far: Option<f64>,
    pub density: Option<f64>,
}
impl PartialEq for FogModifierOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
