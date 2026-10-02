// @generated from upstream/packages/types/src/RequirementCodegen.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Requirement, RequirementCatalogEntry};

// Source: upstream/packages/types/src/RequirementCodegen.ts:6 (sha256:b24443e3fecbe9f8892d864f1a8a7b3ca2d15fbbbfbd665faafa660f10a6d317)
#[derive(Clone, Default)]
pub struct RequirementCodegenPlan {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub backend: String,
    pub declined: Vec<RequirementDeclination>,
    pub entries: Vec<RequirementCatalogEntry>,
    pub unresolved: Vec<Requirement>,
}
impl PartialEq for RequirementCodegenPlan {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RequirementCodegen.ts:21 (sha256:7ad87890abc99d784d7e9f62590b5d9e84ee2b74b0e458615eea0c77c750026f)
#[derive(Clone, Default)]
pub struct RequirementDeclination {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub requirement: Requirement,
}
impl PartialEq for RequirementDeclination {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
