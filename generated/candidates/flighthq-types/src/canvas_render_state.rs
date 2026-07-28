// @generated from upstream/packages/types/src/CanvasRenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, DisplayObjectClipHooks, Matrix, RenderProxy2D, RenderState, Renderable,
    SceneGraphSyncPolicy,
};

// Source: upstream/packages/types/src/CanvasRenderState.ts:8 (sha256:2d3ed80aeffa1af698defe21cc96fededc24c5de7d4a233df2684315565006c5)
#[derive(Clone)]
pub struct CanvasRenderState {
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
            dyn Fn(crate::OpaqueHostValue, Option<BlendMode>) -> () + Send + Sync + 'static,
        >,
    >,
    pub canvas_css_filter_resolver: Option<
        std::sync::Arc<
            dyn Fn(crate::OpaqueHostValue, RenderProxy2D) -> Option<String> + Send + Sync + 'static,
        >,
    >,
    pub canvas: crate::OpaqueHostValue,
    pub context: crate::OpaqueHostValue,
    pub context_attributes: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/CanvasRenderState.ts:23 (sha256:b889957a28ba70a783459bf89658abe277f2c59e9dc40952d4b934de00035ed8)
#[derive(Clone)]
pub struct CanvasRenderStateRuntime {
    pub binding: Option<crate::OpaqueHostValue>,
    pub color_adjustment_channel_mixing_guard:
        Option<std::sync::Arc<dyn Fn(RenderState, Renderable) -> () + Send + Sync + 'static>>,
    pub current_frame_id: f64,
    pub render_adapt_hook: Option<
        std::sync::Arc<
            dyn Fn(RenderState, Renderable, RenderProxy2D) -> () + Send + Sync + 'static,
        >,
    >,
    pub render_proxy_adapter_map: crate::OpaqueHostValue,
    pub render_proxy_map: crate::OpaqueHostValue,
    pub renderer_map: crate::OpaqueHostValue,
    pub renderer_map_id: f64,
    pub temp_stack: Vec<Renderable>,
    pub current_blend_mode: Option<BlendMode>,
    pub image_smoothing_enabled: bool,
    pub image_smoothing_quality: crate::OpaqueHostValue,
    pub image_resource_element_cache: Option<crate::OpaqueHostValue>,
    pub material_renderer_map: Option<crate::OpaqueHostValue>,
}
