// @generated from upstream/packages/types/src/RegistryCatalog.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, RequirementFacet};

// Source: upstream/packages/types/src/RegistryCatalog.ts:7 (sha256:c07292691d0d6993f70b9fa7dc7d3a6492cf5cd5c205c2aaff99343d7d1aec05)
#[derive(Clone, Default)]
pub struct RegistryCatalogEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub backend: String,
    pub facet: RequirementFacet,
    pub implementation_import: String,
    pub implementation_symbol: String,
    pub kind: Kind,
    pub registrar_import: String,
    pub registrar_symbol: String,
}
impl PartialEq for RegistryCatalogEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RegistryCatalog.ts:18 (sha256:892e19dcdb6b8738da74754dfde8302f697b0f2a80e50e7ae89f82ca40b46abf)
#[derive(Clone, Default)]
pub struct RegistryCatalog {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub entries: Vec<RegistryCatalogEntry>,
}
impl PartialEq for RegistryCatalog {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
