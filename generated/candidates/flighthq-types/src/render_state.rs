// @generated from upstream/packages/types/src/RenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, DisplayObjectClipHooks, Kind, Matrix, RenderProxy, RenderProxy2D,
    RenderProxyAdapter, Renderable, Renderer,
};

// Source: upstream/packages/types/src/RenderState.ts:18 (sha256:40b439a7e330c14642efe20a24021547cc65e5abe1162e5479dc8ed5b45a1752)
pub type SceneGraphSyncPolicy = String;

// Source: upstream/packages/types/src/RenderState.ts:20 (sha256:824f9ece3ebf65a6e609c2e05d8f066e4c5cec4b5983b2d84169f77ea69680fd)
#[derive(Clone)]
pub struct RenderState {
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
}
impl PartialEq for RenderState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderState.ts:42 (sha256:12eafb27ebd3efdcc20e516b8a329a06026b353134ab8badbafeedc31aba8220)
#[derive(Clone)]
pub struct RenderStateRuntime {
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
}
impl PartialEq for RenderStateRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
