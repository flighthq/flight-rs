// @generated from upstream/packages/scene-wgpu/src/vertexColorWgpuMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    WgpuMeshPipeline, WgpuUnlitDefineKey, begin_wgpu_mesh_draw, bind_wgpu_unlit_surface,
    draw_wgpu_mesh_subset, ensure_wgpu_unlit_pipeline, register_wgpu_mesh_material_renderer,
    write_wgpu_frame_uniform,
};
use flighthq_color::{LinearColor, unpack_color_to_linear};
use flighthq_render_wgpu::get_wgpu_render_state_runtime;
use flighthq_types::{
    BlendMode, Camera, ColorTransform, DisplayObjectClipHooks, ImageResource, LinearColor,
    Material, Matrix, MeshGeometry, Sampler, SceneGraphSyncPolicy, SceneLightBlock,
    SceneRenderProxy, SceneResourceRef, TextureColorSpace, TextureFilter, TextureWrap,
    VERTEX_COLOR_MATERIAL_KIND as vertex_color_material_kind_constant, Vector2,
    VertexColorMaterial, WgpuMeshMaterialRenderer, WgpuRenderState,
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

// Source: upstream/packages/scene-wgpu/src/vertexColorWgpuMeshMaterialRenderer.ts:26 (sha256:8097afe050df6a93d26aed00c4f2da1508e9ee5a06fb68abe5278b76b042189a)
pub static VERTEX_COLOR_WGPU_MESH_MATERIAL_RENDERER: std::sync::LazyLock<WgpuMeshMaterialRenderer> =
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
                let vertex_color = material;
                let format = ((state_runtime.inner.lock().unwrap().current_color_format).clone())
                    .unwrap_or((state.format).clone());
                let pipeline = ensure_wgpu_unlit_pipeline(
                    (state).clone(),
                    define_key_for_material(((vertex_color).clone()).clone()),
                    (format).clone(),
                );
                write_wgpu_frame_uniform(&mut state, &camera, &_lights);
                let mut group: crate::OpaqueHostValue;
                if (vertex_color).is_none() {
                    group = bind_wgpu_unlit_surface(
                        &mut state,
                        &pipeline,
                        (FALLBACK_MATERIAL).clone(),
                        &WHITE,
                        1.0_f64,
                        0.5_f64,
                        None,
                    );
                } else {
                    unpack_color_to_linear(&mut _SCRATCH, vertex_color.as_ref().unwrap().tint);
                    group = bind_wgpu_unlit_surface(
                        &mut state,
                        &pipeline,
                        (vertex_color.as_ref().unwrap()).clone(),
                        &_SCRATCH,
                        1.0_f64,
                        vertex_color.as_ref().unwrap().alpha_cutoff,
                        None,
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

// Source: upstream/packages/scene-wgpu/src/vertexColorWgpuMeshMaterialRenderer.ts:64 (sha256:890b7da0f20863f09a7db0fe56a117faee90f8680eb3bf1f3a73932e1aaac6d5)
pub fn register_vertex_color_wgpu_material(state: &mut WgpuRenderState) -> () {
    register_wgpu_mesh_material_renderer(
        state,
        (vertex_color_material_kind_constant).to_owned(),
        &VERTEX_COLOR_WGPU_MESH_MATERIAL_RENDERER,
    );
}

// Source: upstream/packages/scene-wgpu/src/vertexColorWgpuMeshMaterialRenderer.ts:68 (sha256:b3740eede70a9830ecb2f5fc69b3fa9ad5e524049c735483a5642fac916e508c)
fn define_key_for_material(material: Option<VertexColorMaterial>) -> WgpuUnlitDefineKey {
    return WgpuUnlitDefineKey {
        __flight_identity: std::sync::Arc::new(()),
        alpha_mask_enabled: ((material).is_some())
            && ((material.as_ref().unwrap().alpha_mode).clone() == "mask"),
        double_sided: ((material).is_some()) && (material.as_ref().unwrap().double_sided),
        has_color_map: false,
    };
}

// Source: upstream/packages/scene-wgpu/src/vertexColorWgpuMeshMaterialRenderer.ts:76 (sha256:ce25130beb5b2bbfc44750860410f83189e1726fa06593e75aef7cda6cd9f857)
static _SCRATCH: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/scene-wgpu/src/vertexColorWgpuMeshMaterialRenderer.ts:77 (sha256:aff9b994c4ec1ff829784b9cbcec2e44480de30e79309b95921d6d455229dd18)
static WHITE: std::sync::LazyLock<LinearColor> =
    std::sync::LazyLock::new(|| vec![1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64]);

// Source: upstream/packages/scene-wgpu/src/vertexColorWgpuMeshMaterialRenderer.ts:78 (sha256:0b11f319f393eb9c46f417ee392fa1a2a0c70cc2c51855089c87d14ea951f20b)
static FALLBACK_MATERIAL: std::sync::LazyLock<VertexColorMaterial> =
    std::sync::LazyLock::new(|| VertexColorMaterial {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
    });
