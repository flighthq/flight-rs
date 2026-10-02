// @generated from upstream/packages/types/src/Requirement.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Kind, RequirementFacet};

// Source: upstream/packages/types/src/Requirement.ts:7 (sha256:6c0abbca38fb4e58ed608773c6e6d182b4d2c2b128b40c7466834f2cf5adcb74)
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

// Source: upstream/packages/types/src/Requirement.ts:14 (sha256:9befc6bb3d93b76fb5a6674be0a1b061bcf221c9718a41fc1a94358721a10584)
#[derive(Clone, Default)]
pub struct RequirementSet {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub covers: Vec<RequirementFacet>,
    pub requirements: Vec<Requirement>,
}
impl PartialEq for RequirementSet {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RequirementSet {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}
