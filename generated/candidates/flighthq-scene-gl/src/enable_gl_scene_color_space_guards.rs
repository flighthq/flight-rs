// @generated from upstream/packages/scene-gl/src/enableGlSceneColorSpaceGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_gl_scene_runtime;
use flighthq_log::log_once;
use flighthq_types::{
    BlendMode, DisplayObjectClipHooks, GlRenderState, ImageResource, LogData, LogDataProvider,
    LogLevel, Matrix, Sampler, SceneGraphSyncPolicy, SceneResourceRef, TextureColorSpace,
    TextureFilter, TextureWrap, Vector2,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub allow_smoothing: Option<bool>,
    pub background_color: Option<f64>,
    pub background_color_rgba: Option<Vec<f64>>,
    pub background_color_string: Option<String>,
    pub current_clip_depth: Option<f64>,
    pub display_object_clip_hooks: Option<DisplayObjectClipHooks>,
    pub pixel_ratio: Option<f64>,
    pub render_alpha: Option<f64>,
    pub render_blend_mode: Option<BlendMode>,
    pub render_transform2_d: Option<Matrix>,
    pub scene_graph_sync_policy: Option<SceneGraphSyncPolicy>,
    pub round_pixels: Option<bool>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub anisotropy: Option<f64>,
    pub mag_filter: Option<TextureFilter>,
    pub min_filter: Option<TextureFilter>,
    pub mipmaps: Option<bool>,
    pub wrap_u: Option<TextureWrap>,
    pub wrap_v: Option<TextureWrap>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub uv_offset: Option<Vector2>,
    pub uv_rotation: Option<f64>,
    pub uv_scale: Option<Vector2>,
    pub color_space: Option<TextureColorSpace>,
    pub image: Option<ImageResource>,
    pub resource: Option<SceneResourceRef>,
    pub sampler: Option<Sampler>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/enableGlSceneColorSpaceGuards.ts:8 (sha256:207ef4c4cc1d69d2aaf0295c6799156e1dd32e863f69968f9b1ab5231a7dccd8)
pub fn are_gl_scene_color_space_guards_enabled(state: &mut GlRenderState) -> bool {
    return ((get_gl_scene_runtime(state).color_space_guard).clone()).is_some();
}

// Source: upstream/packages/scene-gl/src/enableGlSceneColorSpaceGuards.ts:18 (sha256:cbf3d65df6d66a169e58539a22c37b107f74d6130536bb39c6558467a6cd2d02)
pub fn enable_gl_scene_color_space_guards(state: &mut GlRenderState) -> () {
    get_gl_scene_runtime(state).color_space_guard = Some(std::sync::Arc::new(
        std::sync::Mutex::new(Box::new(move || -> () { warn_gl_scene_drawn_to_canvas() })
            as Box<dyn FnMut() -> () + Send + 'static>),
    ));
}

// Source: upstream/packages/scene-gl/src/enableGlSceneColorSpaceGuards.ts:22 (sha256:0f5220a955a896b5ce7d17b5d8e0d49cdc92b0b17b0999d5f333d9a646b7b288)
#[derive(Clone, Default)]
struct WarnGlSceneDrawnToCanvasRecord4 {
    __flight_identity: std::sync::Arc<()>,
    message: String,
}
impl PartialEq for WarnGlSceneDrawnToCanvasRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn warn_gl_scene_drawn_to_canvas() -> () {
    log_once(
        "scene-gl:scene-drawn-to-canvas-unencoded".to_owned(),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::OpaqueHostValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), crate::OpaqueHostValue::String("drawGlScene: scene drawn directly to the canvas — linear radiance is not sRGB-encoded (output will be dark). Render into a target and present with presentGlScene, or draw through the effect pipeline.".to_owned())));
            __flight_record
        }))),
        Some(("scene-gl".to_owned()).clone()),
    );
}
