// @generated from upstream/packages/types/src/EmissiveModifierOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EmissiveModifierFacing, Texture};

// Source: upstream/packages/types/src/EmissiveModifierOptions.ts:4 (sha256:3178f9ac65a057f14a0f654c4380a341fe76b57d865832040c2b7e3f3a6bf79c)
#[derive(Clone, Default)]
pub struct EmissiveModifierOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color: f64,
    pub strength: Option<f64>,
    pub mask: Option<Texture>,
    pub facing: Option<EmissiveModifierFacing>,
    pub facing_softness: Option<f64>,
}
impl PartialEq for EmissiveModifierOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
