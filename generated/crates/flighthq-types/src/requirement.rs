// @generated from upstream/packages/types/src/Requirement.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, RequirementFacet};

// Source: upstream/packages/types/src/Requirement.ts:6 (sha256:6c0abbca38fb4e58ed608773c6e6d182b4d2c2b128b40c7466834f2cf5adcb74)
#[derive(Clone, Default)]
pub struct Requirement {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub facet: RequirementFacet,
    pub key: Kind,
}
impl PartialEq for Requirement {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Requirement.ts:13 (sha256:19329e715973f5b6a5cba7f8a116877c581db46c89df896ccd3aeebe2f47f691)
#[derive(Clone, Default)]
pub struct RequirementSet {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub covers: Vec<RequirementFacet>,
    pub requirements: Vec<Requirement>,
}
impl PartialEq for RequirementSet {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
