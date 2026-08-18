// @generated from upstream/packages/types/src/InteractionHitEligibility.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/InteractionHitEligibility.ts:2 (sha256:5373a10b5f3a1eae036ebf74729b0d3b5bae9f04591360b61e3a63b80b53f8c3)
#[derive(Clone, Default)]
pub struct InteractionHitEligibility {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub eligible: bool,
    pub has_eligible_in_subtree: bool,
}
impl PartialEq for InteractionHitEligibility {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
