// @generated from upstream/packages/scene-gl/src/glShadedModifierSnippet.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_gl_scene_runtime;
use flighthq_shading::{
    ModifierDefinition, create_modifier_registry, register_modifier, resolve_modifier,
};
use flighthq_types::{
    BlendMode, DisplayObjectClipHooks, GlRenderState, ImageResource, Matrix, Modifier,
    ModifierKind, ModifierSlot, Sampler, SceneGraphSyncPolicy, SceneResourceRef, TextureColorSpace,
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

// Source: upstream/packages/scene-gl/src/glShadedModifierSnippet.ts:15 (sha256:c55c0fbae15cde96e2ce51f2d43151c3936ac8108b82ff046a55ee7c33ff316a)
#[derive(Clone)]
pub struct GlModifierBindContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub acquire_modifier_texture_unit:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>>,
    pub index: f64,
    pub program: crate::OpaqueHostValue,
    pub state: GlRenderState,
}
impl PartialEq for GlModifierBindContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glShadedModifierSnippet.ts:35 (sha256:5b8fa686689304cc7c5bd81ab2388af3da8b5d13c0ad9e562b57b35d54c4a8a9)
#[derive(Clone)]
pub struct GlModifierSnippet {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: ModifierKind,
    pub slot: ModifierSlot,
    pub get_define_signature: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Modifier) -> String + Send + 'static>>>,
    >,
    pub bind: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(Modifier, GlModifierBindContext) -> () + Send + 'static>,
            >,
        >,
    >,
    pub contribution:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>>>,
    pub declarations: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>>>,
    >,
}
impl PartialEq for GlModifierSnippet {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glShadedModifierSnippet.ts:47 (sha256:7bd4529eda7ea4ff89ac8dfe8f8e2643e62d9cbe3affc2e062b3b6bcf04eb135)
pub fn register_gl_modifier_snippet(state: &mut GlRenderState, snippet: &GlModifierSnippet) -> () {
    let mut runtime = get_gl_scene_runtime(state);
    if ((runtime.modifier_snippet_registry).clone()).is_none() {
        runtime.modifier_snippet_registry = Some(create_modifier_registry());
    }
    register_modifier(runtime.modifier_snippet_registry.as_mut().unwrap(), &{
        let __flight_source = &(snippet);
        ModifierDefinition {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            slot: (__flight_source.slot).clone(),
            get_define_signature: (__flight_source.get_define_signature).clone(),
        }
    });
}

// Source: upstream/packages/scene-gl/src/glShadedModifierSnippet.ts:56 (sha256:ecb89c9c60b6d756d83b36a9c082cbafc26b76f580f536ba8b7208988bd6c97c)
pub fn resolve_gl_modifier_snippet(
    state: &mut GlRenderState,
    kind: ModifierKind,
) -> Option<GlModifierSnippet> {
    let registry = (get_gl_scene_runtime(state).modifier_snippet_registry).clone();
    if (registry).is_none() {
        return None;
    }
    return resolve_modifier(&registry.as_ref().unwrap(), (kind).clone());
}
