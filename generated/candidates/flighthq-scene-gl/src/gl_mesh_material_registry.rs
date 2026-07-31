// @generated from upstream/packages/scene-gl/src/glMeshMaterialRegistry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_gl_scene_runtime;
use flighthq_types::{
    BlendMode, DEFAULT_MATERIAL_KIND as default_material_kind_constant, DisplayObjectClipHooks,
    GlMeshMaterialRenderer, GlRenderState, ImageResource, Kind, Material, Matrix, Sampler,
    SceneGraphSyncPolicy, SceneResourceRef, TextureColorSpace, TextureFilter, TextureWrap, Vector2,
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

// Source: upstream/packages/scene-gl/src/glMeshMaterialRegistry.ts:9 (sha256:c85940822db300fe2a7606abeb9e1d38d9dac7a31109c95da992b51d0e139d92)
pub fn get_gl_mesh_material_renderer(
    state: &mut GlRenderState,
    kind: Kind,
) -> Option<GlMeshMaterialRenderer> {
    return get_gl_scene_runtime(state)
        .material_registry
        .iter()
        .find(|(key, _)| key == &(kind).clone())
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/scene-gl/src/glMeshMaterialRegistry.ts:17 (sha256:45bfffada07722f6f1967fdd553f68d76d195e3e61b27fa5984c0cc04ed52ad3)
pub fn register_gl_mesh_material_renderer(
    state: &mut GlRenderState,
    kind: Kind,
    renderer: &GlMeshMaterialRenderer,
) -> () {
    {
        let __flight_key = (kind).clone();
        let __flight_value = (*renderer).clone();
        if let Some((_, value)) = get_gl_scene_runtime(state)
            .material_registry
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            get_gl_scene_runtime(state)
                .material_registry
                .push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/scene-gl/src/glMeshMaterialRegistry.ts:29 (sha256:090fdc2925cd9ffd26193853527a9a3a38ea7f9be328ee2be7207c4e1e45c781)
pub fn resolve_gl_mesh_material_renderer(
    state: &mut GlRenderState,
    material: Option<Material>,
) -> Option<GlMeshMaterialRenderer> {
    if (material).is_some() {
        let renderer = get_gl_scene_runtime(state)
            .material_registry
            .iter()
            .find(|(key, _)| key == &(material.as_ref().unwrap().kind).clone())
            .map(|(_, value)| value.clone());
        if (renderer).is_some() {
            return Some((renderer.as_ref().unwrap()).clone());
        }
    }
    return get_gl_scene_runtime(state)
        .material_registry
        .iter()
        .find(|(key, _)| key == &(default_material_kind_constant).to_owned())
        .map(|(_, value)| value.clone());
}
