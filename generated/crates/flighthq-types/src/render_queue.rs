// @generated from upstream/packages/types/src/RenderQueue.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, RenderProxy};

// Source: upstream/packages/types/src/RenderQueue.ts:3 (sha256:41b60b0e39f163a9d167fdc350c63919b2c488d8a7eaa98037125721aab0dad4)
pub type RenderSortKey = f64;

// Source: upstream/packages/types/src/RenderQueue.ts:4 (sha256:a2ce11b93d508a020e1780615c583c770d72c77a9c3f109ac625fdfa54d086a6)
#[derive(Clone, Default)]
pub struct RenderQueueEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub proxy: RenderProxy,
    pub sort_key: RenderSortKey,
}
impl PartialEq for RenderQueueEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderQueue.ts:8 (sha256:227422be1bd51c4064a929d2fa7aa7008279dba22a6419e8ebfb4543acbd323e)
#[derive(Clone, Default)]
pub struct RenderQueue {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub entries: Vec<RenderQueueEntry>,
    pub entry_count: f64,
}
impl PartialEq for RenderQueue {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RenderQueue {
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
