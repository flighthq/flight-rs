// @generated from upstream/packages/types/src/SmaaEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/SmaaEffect.ts:3 (sha256:0e637abe8dc7a9abf1cb6d7c806714f410db9194c50f27fc7fc800a244157ab6)
#[derive(Clone, Default)]
pub struct SmaaEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub threshold: Option<f64>,
}
impl PartialEq for SmaaEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
