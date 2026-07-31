// @generated from upstream/packages/scene-wgpu/src/phongWgpuMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    WgpuClassicDefineKey, WgpuMeshPipeline, begin_wgpu_mesh_draw, bind_wgpu_classic_surface,
    draw_wgpu_mesh_subset, ensure_wgpu_classic_pipeline, is_wgpu_texture_ready,
    register_wgpu_mesh_material_renderer, write_wgpu_frame_uniform,
};
use flighthq_color::{LinearColor, unpack_color_to_linear};
use flighthq_render_wgpu::get_wgpu_render_state_runtime;
use flighthq_types::{
    BlendMode, Camera, ColorTransform, DisplayObjectClipHooks, ImageResource, LinearColor,
    Material, Matrix, MeshGeometry, PHONG_MATERIAL_KIND as phong_material_kind_constant,
    PhongMaterial, Sampler, SceneGraphSyncPolicy, SceneLightBlock, SceneRenderProxy,
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

#[derive(Clone, Default)]
pub struct ModuleSynthesizedRecord58771532 {
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for ModuleSynthesizedRecord58771532 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/phongWgpuMeshMaterialRenderer.ts:29 (sha256:982eb4d3b9a0e87de4852d0f12f0e03fec6044515a5ffcd4697e63cf1deb0065)
pub static PHONG_WGPU_MESH_MATERIAL_RENDERER: std::sync::LazyLock<WgpuMeshMaterialRenderer> =
    std::sync::LazyLock::new(|| WgpuMeshMaterialRenderer {
        __flight_identity: std::sync::Arc::new(()),
        bind: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut state: WgpuRenderState,
                  material: Option<Material>,
                  lights: SceneLightBlock,
                  camera: Camera|
                  -> () {
                let state_runtime = get_wgpu_render_state_runtime(&state);
                let pass = (state_runtime.inner.lock().unwrap().render_pass).clone();
                if (pass).is_none() {
                    return;
                }
                let phong = material;
                let format = ((state_runtime.inner.lock().unwrap().current_color_format).clone())
                    .unwrap_or((state.format).clone());
                let pipeline = ensure_wgpu_classic_pipeline(
                    (state).clone(),
                    define_key_for_material(((phong).clone()).clone()),
                    (format).clone(),
                );
                write_wgpu_frame_uniform(&mut state, &camera, &lights);
                let mut group: crate::OpaqueHostValue;
                if (phong).is_none() {
                    group = bind_wgpu_classic_surface(
                        &mut state,
                        &pipeline,
                        (FALLBACK_MATERIAL).clone(),
                        &WHITE,
                        &WHITE,
                        32.0_f64,
                        0.5_f64,
                        None,
                        None,
                        None,
                    );
                } else {
                    unpack_color_to_linear(&mut _DIFFUSE, phong.as_ref().unwrap().diffuse);
                    unpack_color_to_linear(&mut _SPECULAR, phong.as_ref().unwrap().specular);
                    group = bind_wgpu_classic_surface(
                        &mut state,
                        &pipeline,
                        (phong.as_ref().unwrap()).clone(),
                        &_DIFFUSE,
                        &_SPECULAR,
                        phong.as_ref().unwrap().shininess,
                        phong.as_ref().unwrap().alpha_cutoff,
                        ((phong.as_ref().unwrap().diffuse_map).clone()).clone(),
                        ((phong.as_ref().unwrap().specular_map).clone()).clone(),
                        ((phong.as_ref().unwrap().normal_map).clone()).clone(),
                    );
                }
                begin_wgpu_mesh_draw(&mut state, &{
                    let __flight_source = &(pipeline);
                    WgpuMeshPipeline {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        has_ibl_group: __flight_source.has_ibl_group,
                        has_pbr_sample_group: __flight_source.has_pbr_sample_group,
                        has_shadow_group: __flight_source.has_shadow_group,
                        material_bind_group_layout: (__flight_source.material_bind_group_layout)
                            .clone(),
                        pipeline: (__flight_source.pipeline).clone(),
                    }
                });
                crate::host_value::<()>("host.setBindGroup");
            },
        )
            as Box<
                dyn FnMut(WgpuRenderState, Option<Material>, SceneLightBlock, Camera) -> ()
                    + Send
                    + 'static,
            >)),
        draw: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut state: WgpuRenderState,
                  proxy: SceneRenderProxy,
                  mut geometry: MeshGeometry|
                  -> () {
                draw_wgpu_mesh_subset(&mut state, &proxy, &mut geometry);
            },
        )
            as Box<
                dyn FnMut(WgpuRenderState, SceneRenderProxy, MeshGeometry) -> () + Send + 'static,
            >)),
    });

// Source: upstream/packages/scene-wgpu/src/phongWgpuMeshMaterialRenderer.ts:76 (sha256:57ae2cb00b964edc7682e5eca043f07263c768805826d78987963cc6fc17d1c3)
pub fn register_phong_wgpu_material(state: &mut WgpuRenderState) -> () {
    register_wgpu_mesh_material_renderer(
        state,
        (phong_material_kind_constant).to_owned(),
        &PHONG_WGPU_MESH_MATERIAL_RENDERER,
    );
}

// Source: upstream/packages/scene-wgpu/src/phongWgpuMeshMaterialRenderer.ts:82 (sha256:6bf0ffc8e082779eb9ec2982a2fa6265bae341717baec90d78ff393e14be0f96)
fn define_key_for_material(material: Option<PhongMaterial>) -> WgpuClassicDefineKey {
    return WgpuClassicDefineKey {
        __flight_identity: std::sync::Arc::new(()),
        alpha_mask_enabled: ((material).is_some())
            && ((material.as_ref().unwrap().alpha_mode).clone() == "mask"),
        double_sided: ((material).is_some()) && (material.as_ref().unwrap().double_sided),
        has_diffuse_map: ((material).is_some())
            && (is_wgpu_texture_ready(((material.as_ref().unwrap().diffuse_map).clone()).clone())),
        has_normal_map: ((material).is_some())
            && (is_wgpu_texture_ready(((material.as_ref().unwrap().normal_map).clone()).clone())),
        has_specular_map: ((material).is_some())
            && (is_wgpu_texture_ready(((material.as_ref().unwrap().specular_map).clone()).clone())),
        lighting_model: "phong".to_owned(),
    };
}

// Source: upstream/packages/scene-wgpu/src/phongWgpuMeshMaterialRenderer.ts:93 (sha256:b7e409ce22a12972b4b057f68eb91a3bc2d9187e4ec8eec67df87fa9bc49932b)
static _DIFFUSE: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/scene-wgpu/src/phongWgpuMeshMaterialRenderer.ts:94 (sha256:8623fc911004e0754ac4656ba25a30709d07c5321542d175fcc1e6bacdd91e65)
static _SPECULAR: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/scene-wgpu/src/phongWgpuMeshMaterialRenderer.ts:95 (sha256:aff9b994c4ec1ff829784b9cbcec2e44480de30e79309b95921d6d455229dd18)
static WHITE: std::sync::LazyLock<LinearColor> =
    std::sync::LazyLock::new(|| vec![1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64]);

// Source: upstream/packages/scene-wgpu/src/phongWgpuMeshMaterialRenderer.ts:96 (sha256:f789da6c120ba8de9f1b4876eec64f70081fa87507195b4c744e64f7e5a981c9)
static FALLBACK_MATERIAL: std::sync::LazyLock<PhongMaterial> =
    std::sync::LazyLock::new(|| PhongMaterial {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
    });
