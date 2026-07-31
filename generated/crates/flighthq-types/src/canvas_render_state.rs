// @generated from upstream/packages/types/src/CanvasRenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, DisplayObjectClipHooks, EntityRuntime, ImageResource, Kind, Matrix, RenderProxy2D,
    SceneGraphSyncPolicy,
};

// Source: upstream/packages/types/src/CanvasRenderState.ts:8 (sha256:2d3ed80aeffa1af698defe21cc96fededc24c5de7d4a233df2684315565006c5)
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
    pub display_object_clip_hooks: Option<DisplayObjectClipHooks>,
    pub pixel_ratio: f64,
    pub render_alpha: f64,
    pub render_blend_mode: Option<BlendMode>,
    pub render_transform2_d: Option<Matrix>,
    pub scene_graph_sync_policy: SceneGraphSyncPolicy,
    pub round_pixels: bool,
    pub apply_blend_mode: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(crate::OpaqueHostValue, Option<BlendMode>) -> () + Send + 'static>,
            >,
        >,
    >,
    pub canvas_css_filter_resolver: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(crate::OpaqueHostValue, RenderProxy2D) -> Option<String>
                        + Send
                        + 'static,
                >,
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

// Source: upstream/packages/types/src/CanvasRenderState.ts:23 (sha256:b889957a28ba70a783459bf89658abe277f2c59e9dc40952d4b934de00035ed8)
#[derive(Clone, Default)]
pub struct CanvasRenderStateRuntimeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub element: crate::OpaqueHostValue,
    pub version: f64,
}
impl PartialEq for CanvasRenderStateRuntimeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[doc(hidden)]
pub struct CanvasRenderStateRuntimeStorage {
    pub image_resource_element_cache: Option<Vec<(ImageResource, CanvasRenderStateRuntimeRecord1)>>,
    pub material_renderer_map: Option<Vec<(Kind, crate::OpaqueHostValue)>>,
}
impl Default for CanvasRenderStateRuntimeStorage {
    fn default() -> Self {
        Self {
            image_resource_element_cache: Default::default(),
            material_renderer_map: Default::default(),
        }
    }
}
pub type CanvasRenderStateRuntime = crate::EntityRuntime;
