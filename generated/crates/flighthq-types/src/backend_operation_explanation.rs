// @generated from upstream/packages/types/src/BackendOperationExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BackendOperationExplanation.ts:15 (sha256:5af0c42f09f96e7d1604c62ab82547d7d9c5578ad19b22bc551bb037fa21e0fd)
#[derive(Clone, Default)]
pub struct BackendOperationExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub implemented: bool,
    pub layer: String,
    pub operation: String,
}
impl PartialEq for BackendOperationExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
