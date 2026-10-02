// @generated from upstream/packages/types/src/CanvasRenderPass.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CanvasRenderState, CanvasRenderTarget, EntityRuntime};

// Source: upstream/packages/types/src/CanvasRenderPass.ts:13 (sha256:54ae0acf1996baf1061bc710385971235ba3db869302c1436ac06fadaa921f33)
#[derive(Clone, Default)]
pub struct CanvasRenderPass {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub context: crate::OpaqueHostValue,
    pub saved: CanvasSavedPassState,
    pub state: CanvasRenderState,
    pub target: CanvasRenderTarget,
    pub viewport: CanvasRenderPassViewport,
}
impl PartialEq for CanvasRenderPass {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for CanvasRenderPass {
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

// Source: upstream/packages/types/src/CanvasRenderPass.ts:22 (sha256:a3020d92a6483cdb242ad8b3cd273dcd7a7c1b08eec3e96c0bca48ec10ef4f1c)
#[derive(Clone, Default)]
pub struct CanvasRenderPassViewport {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub width: f64,
}
impl PartialEq for CanvasRenderPassViewport {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CanvasRenderPass.ts:30 (sha256:6932bad7bb9233959bff942ac53cb23cf5048f40632df5d25d5e3057eb3b3326)
#[derive(Clone, Default)]
pub struct CanvasSavedPassState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub canvas: Option<crate::OpaqueHostValue>,
    pub context: Option<crate::OpaqueHostValue>,
    pub current_alpha: f64,
    pub current_blend_mode: CanvasSavedBlendMode,
    pub target: Option<CanvasRenderTarget>,
}
impl PartialEq for CanvasSavedPassState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CanvasRenderPass.ts:40 (sha256:c4992a8bae23e1b0467ad978b4a8aeb193241df99a629a3b71972bfda364f6ef)
pub type CanvasSavedBlendMode = crate::OpaqueHostValue;
