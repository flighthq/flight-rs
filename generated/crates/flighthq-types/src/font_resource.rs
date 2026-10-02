// @generated from upstream/packages/types/src/FontResource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/FontResource.ts:3 (sha256:d208d8cc9be6c9bfd46c3010d294c570ae0f770cebfa1357666f43a2650ca516)
#[derive(Clone, Default)]
pub struct FontResource {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub family: String,
    pub face: Option<crate::OpaqueHostValue>,
}
impl PartialEq for FontResource {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for FontResource {
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

// Source: upstream/packages/types/src/FontResource.ts:8 (sha256:50b0b605c4a59647c487994d45d99039aa098de090df70939b4e58a2635be502)
#[derive(Clone, Default)]
pub struct FontUrl {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub format: Option<String>,
    pub url: String,
}
impl PartialEq for FontUrl {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
