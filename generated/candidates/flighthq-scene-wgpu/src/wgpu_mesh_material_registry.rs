// @generated from upstream/packages/scene-wgpu/src/wgpuMeshMaterialRegistry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_wgpu_scene_runtime;
use flighthq_types::{
    BlendMode, ColorTransform, DEFAULT_MATERIAL_KIND as default_material_kind_constant,
    DisplayObjectClipHooks, ImageResource, Kind, Material, Matrix, Sampler, SceneGraphSyncPolicy,
    SceneResourceRef, TextureColorSpace, TextureFilter, TextureWrap, Vector2,
    WgpuMeshMaterialRenderer, WgpuRenderState,
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

#[derive(Clone, Default)]
pub struct FlightPartialRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshMaterialRegistry.ts:9 (sha256:6ca468bc4b74de0bc1ded737330595408e436e4afaf7fa52768b46ba076c6893)
pub fn get_wgpu_mesh_material_renderer(
    state: &mut WgpuRenderState,
    kind: Kind,
) -> Option<WgpuMeshMaterialRenderer> {
    return get_wgpu_scene_runtime(state)
        .material_registry
        .iter()
        .find(|(key, _)| key == &(kind).clone())
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshMaterialRegistry.ts:17 (sha256:3bfa5b9a0b252583e1ff71c254e36060a2073597c29327e2b6afc5e06102f06a)
pub fn register_wgpu_mesh_material_renderer(
    state: &mut WgpuRenderState,
    kind: Kind,
    renderer: &WgpuMeshMaterialRenderer,
) -> () {
    {
        let __flight_key = (kind).clone();
        let __flight_value = (*renderer).clone();
        if let Some((_, value)) = get_wgpu_scene_runtime(state)
            .material_registry
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            get_wgpu_scene_runtime(state)
                .material_registry
                .push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuMeshMaterialRegistry.ts:29 (sha256:525c2615a7c6b034b5549b156f8a29333bf8bfba78d7fa70bf0cc88215907d11)
pub fn resolve_wgpu_mesh_material_renderer(
    state: &mut WgpuRenderState,
    material: Option<Material>,
) -> Option<WgpuMeshMaterialRenderer> {
    if (material).is_some() {
        let renderer = get_wgpu_scene_runtime(state)
            .material_registry
            .iter()
            .find(|(key, _)| key == &(material.as_ref().unwrap().kind).clone())
            .map(|(_, value)| value.clone());
        if (renderer).is_some() {
            return Some((renderer.as_ref().unwrap()).clone());
        }
    }
    return get_wgpu_scene_runtime(state)
        .material_registry
        .iter()
        .find(|(key, _)| key == &(default_material_kind_constant).to_owned())
        .map(|(_, value)| value.clone());
}
