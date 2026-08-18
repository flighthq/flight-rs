// @generated from upstream/packages/types/src/ResourceLoadCounts.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ResourceLoadCounts.ts:5 (sha256:f877d5fb92c0d6d948be3012ca157ced774b80dc33171776e1dbafeb80dc3aed)
#[derive(Clone, Default)]
pub struct ResourceLoadCounts {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub settled_items: f64,
    pub in_flight_items: f64,
    pub queued_items: f64,
    pub total_items: f64,
}
impl PartialEq for ResourceLoadCounts {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
