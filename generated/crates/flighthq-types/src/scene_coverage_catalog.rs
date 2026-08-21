// @generated from upstream/packages/types/src/SceneCoverageCatalog.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, RenderRegistry};

// Source: upstream/packages/types/src/SceneCoverageCatalog.ts:6 (sha256:359fc1ad0b03454fe4b30b9b03e6c0d3168dc09661565639c4b852f15805603b)
#[derive(Clone, Default)]
pub struct CatalogRegistration {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub module: String,
    pub registrar: String,
}
impl PartialEq for CatalogRegistration {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SceneCoverageCatalog.ts:13 (sha256:61e5a2afeb5fc4305782f3ec26802e743184324e02741a4f319c17f6fa1a4f71)
#[derive(Clone)]
pub struct CatalogEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub registrations: Vec<CatalogRegistration>,
    pub registry: RenderRegistry,
}
impl PartialEq for CatalogEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SceneCoverageCatalog.ts:21 (sha256:f86c06b102cf62d7abb7fc03ef8fe2432a7a7144474b3ec2080bf673456fb8ae)
pub type SceneCoverageCatalog = Vec<CatalogEntry>;
