// @generated from upstream/packages/types/src/WgpuEffectPipeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, WgpuEffectBlendMode};

// Source: upstream/packages/types/src/WgpuEffectPipeline.ts:4 (sha256:88051e62299fdf59a4b86236534b70e068ca4176b378f17e0da62a494899f868)
#[derive(Clone, Default)]
pub struct WgpuEffectPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub pipeline: crate::OpaqueHostValue,
    pub blend_mode: WgpuEffectBlendMode,
    pub compile_for_format: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(crate::OpaqueHostValue) -> crate::OpaqueHostValue + Send + 'static>,
            >,
        >,
    >,
    pub variants: Option<Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>>,
}
impl PartialEq for WgpuEffectPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuEffectPipeline {
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
