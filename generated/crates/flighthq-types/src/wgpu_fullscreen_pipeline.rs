// @generated from upstream/packages/types/src/WgpuFullscreenPipeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/WgpuFullscreenPipeline.ts:3 (sha256:4791dae3ba87e08d15850be274f2854d99a8a1d4fb332a3d80a90ffee6ad00ab)
#[derive(Clone, Default)]
pub struct WgpuFullscreenPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub pipeline: crate::OpaqueHostValue,
    pub pipeline_layout: crate::OpaqueHostValue,
    pub uniform_bind_group_layout: crate::OpaqueHostValue,
    pub texture_bind_group_layouts: Vec<crate::OpaqueHostValue>,
}
impl PartialEq for WgpuFullscreenPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuFullscreenPipeline {
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
