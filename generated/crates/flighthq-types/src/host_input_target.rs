// @generated from upstream/packages/types/src/HostInputTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/HostInputTarget.ts:7 (sha256:05b0769b27f24a8b4a88398fab618b188a8ead1101d9456baaa8a681d8b8f505)
#[derive(Clone, Default)]
pub struct InputTargetHandle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub __brand: String,
}
impl PartialEq for InputTargetHandle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for InputTargetHandle {
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

// Source: upstream/packages/types/src/HostInputTarget.ts:14 (sha256:09452d4bce90771a8e125ac231ae1c1c9ccd4d9038e378ba175514fd57d8a84c)
#[derive(Clone)]
pub struct HostInputTargetCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub prepare:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputTargetHandle) -> () + Send + 'static>>>,
}
impl PartialEq for HostInputTargetCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
