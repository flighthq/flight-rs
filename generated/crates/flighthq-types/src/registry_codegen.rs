// @generated from upstream/packages/types/src/RegistryCodegen.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{RegistryCatalogEntry, Requirement};

// Source: upstream/packages/types/src/RegistryCodegen.ts:6 (sha256:0f84424a2597131e0c5b72a3f86f1a3a3e8f5a561f2217212123a055e0aed8ce)
#[derive(Clone, Default)]
pub struct RegistryCodegenPlan {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub backend: String,
    pub entries: Vec<RegistryCatalogEntry>,
    pub unresolved: Vec<Requirement>,
}
impl PartialEq for RegistryCodegenPlan {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
