// @generated from upstream/packages/types/src/WgpuRenderPass.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, WgpuRenderState, WgpuRenderTarget, WgpuScissorRect};

// Source: upstream/packages/types/src/WgpuRenderPass.ts:15 (sha256:a5b846e34c4148276abf83fd70b09d74cf2cc41e0bdcfdec8d95e56391c0962e)
#[derive(Clone, Default)]
pub struct WgpuRenderPass {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub color_view: crate::OpaqueHostValue,
    pub encoder: Option<crate::OpaqueHostValue>,
    pub owns_frame: bool,
    pub saved: WgpuSavedPassState,
    pub state: WgpuRenderState,
    pub target: WgpuRenderTarget,
    pub viewport: WgpuRenderPassViewport,
}
impl PartialEq for WgpuRenderPass {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuRenderPass {
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

// Source: upstream/packages/types/src/WgpuRenderPass.ts:33 (sha256:5853e16c369948e36da20ed6cbc23f8d256e3601d128de031b3b03c35be8b4a4)
#[derive(Clone, Default)]
pub struct WgpuRenderPassViewport {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub width: f64,
}
impl PartialEq for WgpuRenderPassViewport {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderPass.ts:40 (sha256:37ef6e99198a6257d50381d43791ba6b33f1a908a96938443678150ece086304)
#[derive(Clone, Default)]
pub struct WgpuSavedPassState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub clip_forms: Vec<String>,
    pub color_format: Option<crate::OpaqueHostValue>,
    pub current_mask_depth: f64,
    pub current_scissor_rect: Option<WgpuScissorRect>,
    pub mask_write_mode: bool,
    pub render_target: Option<WgpuRenderTarget>,
    pub render_target_viewport: Option<WgpuRenderPassViewport>,
    pub scissor_stack: Vec<WgpuScissorRect>,
}
impl PartialEq for WgpuSavedPassState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
