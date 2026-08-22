// @generated from upstream/packages/types/src/BackendExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BackendExplanation.ts:1 (sha256:ff9baccdc11a19a08078228c8985c8dc19daaf673f2fbfbd08ea4e7135672f36)
#[derive(Clone, Default)]
pub struct BackendExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub layer: String,
    pub viability: String,
}
impl PartialEq for BackendExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
