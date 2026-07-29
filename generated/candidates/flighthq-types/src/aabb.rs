// @generated from upstream/packages/types/src/Aabb.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Vector3, Vector3Like};

// Source: upstream/packages/types/src/Aabb.ts:7 (sha256:fddcf6f4384ac64540b2d5e5773987e8bd13b153e3415c0fc5624922679fdd33)
#[derive(Clone, Default)]
pub struct Aabb {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub max: Vector3,
    pub min: Vector3,
}
impl PartialEq for Aabb {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Aabb {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
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

// Source: upstream/packages/types/src/Aabb.ts:12 (sha256:b297ec83d7e3724ec14478784fcfb1cfdd769c0f27970ff606ef16e542de48c2)
#[derive(Clone, Default)]
pub struct AabbLike {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub max: Vector3Like,
    pub min: Vector3Like,
}
impl PartialEq for AabbLike {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
