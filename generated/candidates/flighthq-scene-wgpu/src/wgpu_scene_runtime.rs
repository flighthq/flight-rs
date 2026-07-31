// @generated from upstream/packages/scene-wgpu/src/wgpuSceneRuntime.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::WgpuMeshPipeline;
use flighthq_types::{
    BlendMode, ColorTransform, DisplayObjectClipHooks, ImageResource, Kind, Matrix, Matrix4,
    Sampler, SceneGraphSyncPolicy, SceneResourceRef, TextureColorSpace, TextureFilter, TextureWrap,
    Vector2, WgpuMeshMaterialRenderer, WgpuRenderState,
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

// Source: upstream/packages/scene-wgpu/src/wgpuSceneRuntime.ts:12 (sha256:2ca2f56e80c0ab31d8f59ef937dcaef6c3f132b3d1bfb594340e5f8986271ace)
#[derive(Clone, Default)]
pub struct WgpuSceneShadow {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub depth_texture: crate::OpaqueHostValue,
    pub depth_view: crate::OpaqueHostValue,
    pub matrix: Matrix4,
}
impl PartialEq for WgpuSceneShadow {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuSceneRuntime.ts:27 (sha256:405003f22dcf3115f21383114e7db8d2cdac5ec370f204e3a369302e653c1f39)
#[derive(Clone, Default)]
pub struct WgpuSceneIbl {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub brdf_lut: crate::OpaqueHostValue,
    pub brdf_lut_view: crate::OpaqueHostValue,
    pub intensity: f64,
    pub irradiance_cube: crate::OpaqueHostValue,
    pub irradiance_cube_view: crate::OpaqueHostValue,
    pub prefiltered_cube: crate::OpaqueHostValue,
    pub prefiltered_cube_view: crate::OpaqueHostValue,
    pub prefiltered_mip_count: f64,
}
impl PartialEq for WgpuSceneIbl {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuSceneRuntime.ts:49 (sha256:30eb1a86b5c8ac7cf541811075de6adb894797956bc8fe24eb9f01b69c92f95c)
#[derive(Clone, Default)]
pub struct WgpuSceneRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub active_mesh_pipeline: Option<WgpuMeshPipeline>,
    pub draw_bind_group: Option<crate::OpaqueHostValue>,
    pub draw_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub frame_bind_group: Option<crate::OpaqueHostValue>,
    pub frame_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub frame_buffer: Option<crate::OpaqueHostValue>,
    pub environment_source_cube: Option<crate::OpaqueHostValue>,
    pub environment_source_cube_view: Option<crate::OpaqueHostValue>,
    pub ibl: Option<WgpuSceneIbl>,
    pub ibl_dummy_cube_texture: Option<crate::OpaqueHostValue>,
    pub ibl_dummy_cube_view: Option<crate::OpaqueHostValue>,
    pub ibl_dummy_lut_texture: Option<crate::OpaqueHostValue>,
    pub ibl_dummy_lut_view: Option<crate::OpaqueHostValue>,
    pub ibl_sample_bind_group: Option<crate::OpaqueHostValue>,
    pub ibl_sample_cube_view: Option<crate::OpaqueHostValue>,
    pub ibl_sample_layout: Option<crate::OpaqueHostValue>,
    pub ibl_sampler: Option<crate::OpaqueHostValue>,
    pub ibl_uniform_buffer: Option<crate::OpaqueHostValue>,
    pub material_bind_groups: Vec<(crate::OpaqueHostValue, WgpuMaterialBinding)>,
    pub pbr_sample_bind_group: Option<crate::OpaqueHostValue>,
    pub pbr_sample_ibl_cube_view: Option<crate::OpaqueHostValue>,
    pub pbr_sample_layout: Option<crate::OpaqueHostValue>,
    pub pbr_sample_shadow_view: Option<crate::OpaqueHostValue>,
    pub material_registry: Vec<(Kind, WgpuMeshMaterialRenderer)>,
    pub pending_draw_offset: f64,
    pub pending_uv_transform: Vec<f32>,
    pub pipeline_cache: Vec<(String, WgpuMeshPipeline)>,
    pub placeholder_view: Option<crate::OpaqueHostValue>,
    pub shadow: Option<WgpuSceneShadow>,
    pub shadow_comparison_sampler: Option<crate::OpaqueHostValue>,
    pub shadow_depth_pipeline: Option<crate::OpaqueHostValue>,
    pub shadow_dummy_texture: Option<crate::OpaqueHostValue>,
    pub shadow_dummy_view: Option<crate::OpaqueHostValue>,
    pub shadow_sample_bind_group: Option<crate::OpaqueHostValue>,
    pub shadow_sample_layout: Option<crate::OpaqueHostValue>,
    pub shadow_sample_view: Option<crate::OpaqueHostValue>,
    pub shadow_uniform_buffer: Option<crate::OpaqueHostValue>,
    pub upload_cache: Vec<(crate::OpaqueHostValue, WgpuMeshUpload)>,
}
impl PartialEq for WgpuSceneRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuSceneRuntime.ts:109 (sha256:cbc9c781b4cded30325cdce6000d11306dcff67643027add179bf2d2c85ffc86)
#[derive(Clone, Default)]
pub struct WgpuMeshUpload {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub index_buffer: Option<crate::OpaqueHostValue>,
    pub index_count: f64,
    pub index_format: crate::OpaqueHostValue,
    pub version: f64,
    pub vertex_buffer: crate::OpaqueHostValue,
}
impl PartialEq for WgpuMeshUpload {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuSceneRuntime.ts:120 (sha256:6f5e1908d8c844c1f5117b103f5ee7a84a53a50f96b7a77b08661541f79f3459)
#[derive(Clone, Default)]
pub struct WgpuMaterialBinding {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bind_group: crate::OpaqueHostValue,
    pub buffer: crate::OpaqueHostValue,
}
impl PartialEq for WgpuMaterialBinding {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuSceneRuntime.ts:128 (sha256:2617f0d91c8915e642dd6b8bedf66c99f8e71c3747d69c930cb68d712eafc4ae)
pub fn get_wgpu_scene_runtime(state: &mut WgpuRenderState) -> WgpuSceneRuntime {
    let mut state_runtime = ({
        let __flight_runtime = flighthq_types::FlightEntity::__flight_entity_runtime(state)
            .lock()
            .unwrap()
            .clone()
            .expect("entity runtime was read before initialization");
        __flight_runtime
    })
    .clone();
    let mut scene = (*SCENE_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (scene).is_none() {
        scene = Some(WgpuSceneRuntime {
            __flight_identity: std::sync::Arc::new(()),
            active_mesh_pipeline: None,
            draw_bind_group: None,
            draw_bind_group_layout: None,
            frame_bind_group: None,
            frame_bind_group_layout: None,
            frame_buffer: None,
            environment_source_cube: None,
            environment_source_cube_view: None,
            ibl: None,
            ibl_dummy_cube_texture: None,
            ibl_dummy_cube_view: None,
            ibl_dummy_lut_texture: None,
            ibl_dummy_lut_view: None,
            ibl_sample_bind_group: None,
            ibl_sample_cube_view: None,
            ibl_sample_layout: None,
            ibl_sampler: None,
            ibl_uniform_buffer: None,
            material_bind_groups: Vec::new(),
            pbr_sample_bind_group: None,
            pbr_sample_ibl_cube_view: None,
            pbr_sample_layout: None,
            pbr_sample_shadow_view: None,
            material_registry: Vec::new(),
            pending_draw_offset: 0.0_f64,
            pending_uv_transform: (vec![
                1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64,
            ])
            .iter()
            .map(|value| (*value) as f32)
            .collect(),
            pipeline_cache: Vec::new(),
            placeholder_view: None,
            shadow: None,
            shadow_comparison_sampler: None,
            shadow_depth_pipeline: None,
            shadow_dummy_texture: None,
            shadow_dummy_view: None,
            shadow_sample_bind_group: None,
            shadow_sample_layout: None,
            shadow_sample_view: None,
            shadow_uniform_buffer: None,
            upload_cache: Vec::new(),
        });
        {
            let __flight_key = (*state).clone();
            let __flight_value = (scene).clone().unwrap();
            if let Some((_, value)) = (*SCENE_RUNTIMES.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*SCENE_RUNTIMES.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
        {
            let __flight_runtime = state_runtime;
            let __flight_value = Some((scene.as_mut().unwrap().material_registry).clone());
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage
                .wgpu_render_state_runtime
                .scene_mesh_material_registry = __flight_value;
        };
        {
            let __flight_runtime = state_runtime;
            let __flight_value = Some((scene.as_mut().unwrap().upload_cache).clone());
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.scene_mesh_upload_cache = __flight_value;
        };
    }
    return ((scene).clone().unwrap()).clone();
}

// Source: upstream/packages/scene-wgpu/src/wgpuSceneRuntime.ts:182 (sha256:a1aba56eeb007ebab644d9f53f95d7392ea79e5080900c9a82a088ea341cfb8c)
static SCENE_RUNTIMES: std::sync::LazyLock<
    std::sync::Mutex<Vec<(WgpuRenderState, WgpuSceneRuntime)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
