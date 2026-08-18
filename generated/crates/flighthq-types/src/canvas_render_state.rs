// @generated from upstream/packages/types/src/CanvasRenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, CanvasMaterialRenderer, EntityRuntime, Kind, Matrix, RenderProxy2D,
    RenderRegistrySignals, Scene2DClipHooks, Scene3DGraphSyncPolicy,
};

// Source: upstream/packages/types/src/CanvasRenderState.ts:10 (sha256:2d3ed80aeffa1af698defe21cc96fededc24c5de7d4a233df2684315565006c5)
#[derive(Clone, Default)]
pub struct CanvasRenderState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub allow_smoothing: bool,
    pub background_color: f64,
    pub background_color_rgba: Vec<f64>,
    pub background_color_string: String,
    pub current_clip_depth: f64,
    pub display_object_clip_hooks: Option<Scene2DClipHooks>,
    pub pixel_ratio: f64,
    pub render_alpha: f64,
    pub render_blend_mode: Option<BlendMode>,
    pub render_transform2_d: Option<Matrix>,
    pub scene_graph_sync_policy: Scene3DGraphSyncPolicy,
    pub round_pixels: bool,
    pub apply_blend_mode: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(CanvasRenderState, Option<BlendMode>) -> () + Send + 'static>,
            >,
        >,
    >,
    pub canvas_css_filter_resolver: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(CanvasRenderState, RenderProxy2D) -> Option<String> + Send + 'static>,
            >,
        >,
    >,
    pub canvas: crate::OpaqueHostValue,
    pub context: crate::OpaqueHostValue,
    pub context_attributes: crate::OpaqueHostValue,
}
impl PartialEq for CanvasRenderState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for CanvasRenderState {
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

// Source: upstream/packages/types/src/CanvasRenderState.ts:25 (sha256:be617f3f8f9f830df9da893eee28eeb9740c2f15a2e4462f451c6d3191c0ca99)
#[derive(Clone)]
pub struct CanvasRenderStateRuntimeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clear: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub signals: RenderRegistrySignals,
}
impl PartialEq for CanvasRenderStateRuntimeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[doc(hidden)]
pub struct CanvasRenderStateRuntimeStorage {
    pub material_renderer_map: Option<Vec<(Kind, CanvasMaterialRenderer)>>,
}
impl Default for CanvasRenderStateRuntimeStorage {
    fn default() -> Self {
        Self {
            material_renderer_map: Default::default(),
        }
    }
}
pub type CanvasRenderStateRuntime = crate::EntityRuntime;
