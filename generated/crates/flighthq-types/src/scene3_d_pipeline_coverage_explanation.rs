// @generated from upstream/packages/types/src/Scene3DPipelineCoverageExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/Scene3DPipelineCoverageExplanation.ts:3 (sha256:049ad5335da2e6635a761e59ab9057c1e1302ca66f4581afaf55d9f65855ce17)
#[derive(Clone, Default)]
pub struct Scene3DPipelineCoverageExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub registered_kinds: Vec<Kind>,
    pub registered_material_kinds: Vec<Kind>,
    pub uncovered_kinds: Vec<Kind>,
    pub uncovered_material_kinds: Vec<Kind>,
    pub unused_material_registrations: Vec<Kind>,
    pub unused_registrations: Vec<Kind>,
    pub used_kinds: Vec<Kind>,
    pub used_material_kinds: Vec<Kind>,
}
impl PartialEq for Scene3DPipelineCoverageExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
