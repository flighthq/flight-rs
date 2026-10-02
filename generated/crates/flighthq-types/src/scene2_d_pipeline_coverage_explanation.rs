// @generated from upstream/packages/types/src/Scene2DPipelineCoverageExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/Scene2DPipelineCoverageExplanation.ts:3 (sha256:5b19031e6be486220c10f9b9843ca77ecca4fbe1efb9545bd69d34b7fdd20462)
#[derive(Clone, Default)]
pub struct Scene2DPipelineCoverageExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub registered_kinds: Vec<Kind>,
    pub uncovered_kinds: Vec<Kind>,
    pub unused_registrations: Vec<Kind>,
    pub used_kinds: Vec<Kind>,
}
impl PartialEq for Scene2DPipelineCoverageExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
