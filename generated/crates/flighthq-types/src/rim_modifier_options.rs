// @generated from upstream/packages/types/src/RimModifierOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/RimModifierOptions.ts:1 (sha256:c076fae4fd9f9ece6653588781777d568b00d47e04987ea940980c1f583938bd)
#[derive(Clone, Default)]
pub struct RimModifierOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color: f64,
    pub power: Option<f64>,
    pub intensity: Option<f64>,
    pub bias: Option<f64>,
}
impl PartialEq for RimModifierOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
