// @generated from upstream/packages/types/src/DitherEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/DitherEffect.ts:3 (sha256:debeb7293ff5e4aa30615c6662deb9265998d09e7a503e253016e5f9458a78b6)
#[derive(Clone, Default)]
pub struct DitherEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub levels: Option<f64>,
}
impl PartialEq for DitherEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
