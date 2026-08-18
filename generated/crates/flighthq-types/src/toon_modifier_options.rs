// @generated from upstream/packages/types/src/ToonModifierOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ToonModifierOptions.ts:1 (sha256:d7d97edb945a99e12f016a5cecf37e6b874d9b5b38f459c0e5a09568992b6233)
#[derive(Clone, Default)]
pub struct ToonModifierOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub steps: f64,
    pub smoothness: Option<f64>,
}
impl PartialEq for ToonModifierOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
