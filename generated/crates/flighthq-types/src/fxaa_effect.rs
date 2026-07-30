// @generated from upstream/packages/types/src/FxaaEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/FxaaEffect.ts:3 (sha256:ae1e123ab3a75fa71fdbac65ec37ade6e9e1fd25175b99f28971c829d5f234ee)
#[derive(Clone, Default)]
pub struct FxaaEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub edge_threshold: Option<f64>,
    pub subpixel: Option<f64>,
}
impl PartialEq for FxaaEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
