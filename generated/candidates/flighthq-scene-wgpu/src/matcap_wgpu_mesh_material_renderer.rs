// @generated from upstream/packages/scene-wgpu/src/matcapWgpuMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    WgpuMatcapDefineKey, WgpuMeshPipeline, begin_wgpu_mesh_draw, bind_wgpu_matcap_surface,
    draw_wgpu_mesh_subset, ensure_wgpu_matcap_pipeline, register_wgpu_mesh_material_renderer,
    write_wgpu_frame_uniform,
};
use flighthq_color::{LinearColor, unpack_color_to_linear};
use flighthq_render_wgpu::get_wgpu_render_state_runtime;
use flighthq_types::{
    BlendMode, Camera, ColorTransform, DisplayObjectClipHooks, ImageResource, LinearColor,
    MATCAP_MATERIAL_KIND as matcap_material_kind_constant, MatcapMaterial, Material, Matrix,
    MeshGeometry, Sampler, SceneGraphSyncPolicy, SceneLightBlock, SceneRenderProxy,
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

// Source: upstream/packages/scene-wgpu/src/matcapWgpuMeshMaterialRenderer.ts:28 (sha256:aef00d6a6f42deff966211e6052ff92e05b1d8546cee30870436517bf2abc6d0)
pub static MATCAP_WGPU_MESH_MATERIAL_RENDERER: std::sync::LazyLock<WgpuMeshMaterialRenderer> =
    std::sync::LazyLock::new(|| WgpuMeshMaterialRenderer {
        __flight_identity: std::sync::Arc::new(()),
        bind: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut state: WgpuRenderState,
                  material: Option<Material>,
                  _lights: SceneLightBlock,
                  camera: Camera|
                  -> () {
                let state_runtime = get_wgpu_render_state_runtime(&state);
                let pass = (state_runtime.inner.lock().unwrap().render_pass).clone();
                if (pass).is_none() {
                    return;
                }
                let matcap = material;
                let format = ((state_runtime.inner.lock().unwrap().current_color_format).clone())
                    .unwrap_or((state.format).clone());
                let pipeline = ensure_wgpu_matcap_pipeline(
                    (state).clone(),
                    define_key_for_material(((matcap).clone()).clone()),
                    (format).clone(),
                );
                write_wgpu_frame_uniform(&mut state, &camera, &_lights);
                let mut group: crate::OpaqueHostValue;
                if (matcap).is_none() {
                    group = bind_wgpu_matcap_surface(
                        &mut state,
                        &pipeline,
                        (FALLBACK_MATERIAL).clone(),
                        &WHITE,
                        0.5_f64,
                    );
                } else {
                    unpack_color_to_linear(&mut _SCRATCH, matcap.as_ref().unwrap().tint);
                    group = bind_wgpu_matcap_surface(
                        &mut state,
                        &pipeline,
                        (matcap.as_ref().unwrap()).clone(),
                        &_SCRATCH,
                        matcap.as_ref().unwrap().alpha_cutoff,
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

// Source: upstream/packages/scene-wgpu/src/matcapWgpuMeshMaterialRenderer.ts:63 (sha256:b952ba9c016a0d1026f9c5c296b4239b3b9dbd5c045f88993bd20fdb173e728b)
pub fn register_matcap_wgpu_material(state: &mut WgpuRenderState) -> () {
    register_wgpu_mesh_material_renderer(
        state,
        (matcap_material_kind_constant).to_owned(),
        &MATCAP_WGPU_MESH_MATERIAL_RENDERER,
    );
}

// Source: upstream/packages/scene-wgpu/src/matcapWgpuMeshMaterialRenderer.ts:67 (sha256:e0da71d33c492d1eb7a96b0172f32846c64a30791d3069f4a864b7d4ece9d2bd)
fn define_key_for_material(material: Option<MatcapMaterial>) -> WgpuMatcapDefineKey {
    return WgpuMatcapDefineKey {
        __flight_identity: std::sync::Arc::new(()),
        alpha_mask_enabled: ((material).is_some())
            && ((material.as_ref().unwrap().alpha_mode).clone() == "mask"),
        double_sided: ((material).is_some()) && (material.as_ref().unwrap().double_sided),
        has_matcap: false,
    };
}

// Source: upstream/packages/scene-wgpu/src/matcapWgpuMeshMaterialRenderer.ts:75 (sha256:ce25130beb5b2bbfc44750860410f83189e1726fa06593e75aef7cda6cd9f857)
static _SCRATCH: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/scene-wgpu/src/matcapWgpuMeshMaterialRenderer.ts:76 (sha256:aff9b994c4ec1ff829784b9cbcec2e44480de30e79309b95921d6d455229dd18)
static WHITE: std::sync::LazyLock<LinearColor> =
    std::sync::LazyLock::new(|| vec![1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64]);

// Source: upstream/packages/scene-wgpu/src/matcapWgpuMeshMaterialRenderer.ts:77 (sha256:42ca900e53f2c5d47b244734be81aacd85e726a184c172964c0cfa5fa9d57f8b)
static FALLBACK_MATERIAL: std::sync::LazyLock<MatcapMaterial> =
    std::sync::LazyLock::new(|| MatcapMaterial {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
    });
