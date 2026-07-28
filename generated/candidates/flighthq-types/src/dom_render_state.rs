// @generated from upstream/packages/types/src/DomRenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, DisplayObjectClipHooks, DomStageRectangle, ImageResource, Kind, Matrix, PathWinding,
    RenderProxy, RenderProxy2D, RenderProxyAdapter, RenderState, Renderable, Renderer,
    SceneGraphSyncPolicy,
};

// Source: upstream/packages/types/src/DomRenderState.ts:8 (sha256:7dd771caabb5913c54dc523f2804bfcdff548e84aca709060279660fb26ea9af)
#[derive(Clone, Default)]
pub struct DomRenderState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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
    pub dom_css_filter_resolver: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RenderProxy2D) -> Option<String> + Send + 'static>>,
        >,
    >,
    pub element: crate::OpaqueHostValue,
}
impl PartialEq for DomRenderState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/DomRenderState.ts:20 (sha256:95a2d3164f536c2bae6c8c55ccfe6be59858dcb9485508c252f453ccb8a8fceb)
#[derive(Clone, Default)]
pub struct DomRenderStateRuntimeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub element: crate::OpaqueHostValue,
    pub version: f64,
}
impl PartialEq for DomRenderStateRuntimeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct DomRenderStateRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub color_adjustment_channel_mixing_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RenderState, Renderable) -> () + Send + 'static>>,
        >,
    >,
    pub current_frame_id: f64,
    pub render_adapt_hook: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(RenderState, Renderable, RenderProxy2D) -> () + Send + 'static>,
            >,
        >,
    >,
    pub render_proxy_adapter_map: Vec<(Renderable, RenderProxyAdapter)>,
    pub render_proxy_map: Vec<(Renderable, RenderProxy)>,
    pub renderer_map: Vec<(Kind, Renderer)>,
    pub renderer_map_id: f64,
    pub temp_stack: Vec<Renderable>,
    pub current_blend_mode: Option<BlendMode>,
    pub dom_clip_hooks: Option<DomClipHooks>,
    pub dom_clip_stack: Vec<DomClipEntry>,
    pub dom_current_element: Option<crate::OpaqueHostValue>,
    pub dom_element_map: Vec<(RenderProxy2D, crate::OpaqueHostValue)>,
    pub image_resource_element_cache: Option<Vec<(ImageResource, DomRenderStateRuntimeRecord1)>>,
    pub dom_next_order_list: Vec<RenderProxy2D>,
    pub dom_order_length: f64,
    pub dom_order_list: Vec<RenderProxy2D>,
}
impl PartialEq for DomRenderStateRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/DomRenderState.ts:49 (sha256:38fa448f3ab74d60dcd04814fef23fdd7681955b2281907978b8d6bc1f47c017)
#[derive(Clone, Default)]
pub struct DomClipContourEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub contours: Vec<Vec<f64>>,
    pub kind: String,
    pub winding: PathWinding,
}
impl PartialEq for DomClipContourEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/DomRenderState.ts:59 (sha256:0157b8f85d24205cd3bc07aa3a9e2e2d1c01fbb0a6740d86be014c35bee41d65)
pub type DomClipEntry = crate::FlightUnion2<DomClipContourEntry, DomStageRectangle>;

// Source: upstream/packages/types/src/DomRenderState.ts:64 (sha256:60fd7cdcf4fa1ac76ed25cb9780162dc456844bc2439b386623c4addf7067d88)
#[derive(Clone)]
pub struct DomClipHooks {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub apply: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(DomRenderState, RenderProxy2D) -> () + Send + 'static>>,
    >,
}
impl PartialEq for DomClipHooks {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
