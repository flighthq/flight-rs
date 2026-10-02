// @generated from upstream/packages/types/src/RenderCacheAdapter.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    EntityRuntime, NodeAny, RenderCache, RenderCacheAdapterSignals, RenderProxy2D, RenderState,
};

// Source: upstream/packages/types/src/RenderCacheAdapter.ts:9 (sha256:008a3d5320477366a10b00818fe48df133d37e34c2437fa2f367d5753b747d15)
#[derive(Clone)]
pub struct RenderCacheAdapter {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub adapt: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(RenderState, NodeAny, RenderProxy2D) -> Option<bool> + Send + 'static>,
        >,
    >,
    pub cache: Option<RenderCache>,
    pub signals: Option<RenderCacheAdapterSignals>,
}
impl PartialEq for RenderCacheAdapter {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RenderCacheAdapter {
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
