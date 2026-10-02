// @generated from upstream/packages/types/src/CanvasEffectState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    CanvasRenderState, CanvasTextureRenderTarget, ColorLutCache, Effect, EffectStateOptions,
    EntityRuntime, HostCanvasCapability,
};

// Source: upstream/packages/types/src/CanvasEffectState.ts:16 (sha256:73b6b2f3e540b3efcea0762438525162f8395ab5b2adcec65733383f13c30464)
#[derive(Clone)]
pub struct CanvasEffectContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub state: CanvasRenderState,
    pub source: CanvasTextureRenderTarget,
    pub dest: CanvasTextureRenderTarget,
    pub pool: CanvasRenderTargetPool,
}
impl PartialEq for CanvasEffectContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CanvasEffectState.ts:27 (sha256:3db066921777e0b8ebac7954e0079c31c3b37023fe8f7a6ff6986a345e0ba6ff)
pub type CanvasEffectRunner = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(CanvasEffectContext, Effect) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/CanvasEffectState.ts:33 (sha256:8b712e35f556bbaf4205e68af2e95989d28cca2be56896561d87efaee8ab530b)
#[derive(Clone)]
pub struct CanvasRenderTargetPool {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub canvas_host: HostCanvasCapability,
    pub free: Vec<CanvasTextureRenderTarget>,
    pub in_use: Vec<CanvasTextureRenderTarget>,
}
impl PartialEq for CanvasRenderTargetPool {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for CanvasRenderTargetPool {
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

// Source: upstream/packages/types/src/CanvasEffectState.ts:43 (sha256:18ab37215889fb4c9d2fe3e9db926a0f8fc967146214474c66202f4ea60719be)
#[derive(Clone)]
pub struct CanvasEffectState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub options: EffectStateOptions,
    pub scene_target: Option<CanvasTextureRenderTarget>,
    pub pool: CanvasRenderTargetPool,
    pub lut_cache: ColorLutCache,
}
impl PartialEq for CanvasEffectState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
