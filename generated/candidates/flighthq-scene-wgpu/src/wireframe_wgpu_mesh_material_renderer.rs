// @generated from upstream/packages/scene-wgpu/src/wireframeWgpuMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    WgpuMeshPipeline, begin_wgpu_mesh_draw, bind_wgpu_wireframe_color,
    ensure_wgpu_wireframe_pipeline, ensure_wgpu_wireframe_upload, get_wgpu_scene_runtime,
    register_wgpu_mesh_material_renderer, write_wgpu_draw_uniform, write_wgpu_frame_uniform,
};
use flighthq_color::{LinearColor, unpack_color_to_linear};
use flighthq_render_wgpu::get_wgpu_render_state_runtime;
use flighthq_types::{
    BlendMode, Camera, ColorTransform, DisplayObjectClipHooks, ImageResource, LinearColor,
    Material, Matrix, MeshGeometry, Sampler, SceneGraphSyncPolicy, SceneLightBlock,
    SceneRenderProxy, SceneResourceRef, TextureColorSpace, TextureFilter, TextureWrap, Vector2,
    WIREFRAME_MATERIAL_KIND as wireframe_material_kind_constant, WgpuMeshMaterialRenderer,
    WgpuRenderState, WireframeMaterial,
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

// Source: upstream/packages/scene-wgpu/src/wireframeWgpuMeshMaterialRenderer.ts:29 (sha256:86cf2c3619ea707c016212bef51e55cc8fe2ab9b6900cf4990511d5a7edcbc1f)
pub static WIREFRAME_WGPU_MESH_MATERIAL_RENDERER: std::sync::LazyLock<WgpuMeshMaterialRenderer> =
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
                let wireframe = material;
                let format = ((state_runtime.inner.lock().unwrap().current_color_format).clone())
                    .unwrap_or((state.format).clone());
                let pipeline = ensure_wgpu_wireframe_pipeline((state).clone(), (format).clone());
                write_wgpu_frame_uniform(&mut state, &camera, &_lights);
                let mut group: crate::OpaqueHostValue;
                if (wireframe).is_none() {
                    group = bind_wgpu_wireframe_color(
                        &mut state,
                        &pipeline,
                        (FALLBACK_MATERIAL).clone(),
                        &WHITE,
                    );
                } else {
                    unpack_color_to_linear(&mut _SCRATCH, wireframe.as_ref().unwrap().color);
                    group = bind_wgpu_wireframe_color(
                        &mut state,
                        &pipeline,
                        (wireframe.as_ref().unwrap()).clone(),
                        &_SCRATCH,
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
                let state_runtime = get_wgpu_render_state_runtime(&state);
                let pass = (state_runtime.inner.lock().unwrap().render_pass).clone();
                let scene = get_wgpu_scene_runtime(&mut state);
                if ((pass).is_none()) || (((scene.active_mesh_pipeline).clone()).is_none()) {
                    return;
                }
                if (proxy.subset.index_count == 0.0_f64) {
                    return;
                }
                let upload = ensure_wgpu_wireframe_upload(&mut state, &mut geometry);
                if (upload).is_none() {
                    return;
                }
                let draw_bind_group = write_wgpu_draw_uniform(&mut state, &proxy);
                (*_DYNAMIC_OFFSETS.lock().unwrap())[0.0_f64 as usize] =
                    (scene.pending_draw_offset) as u32;
                crate::host_value::<()>("host.setBindGroup");
                crate::host_value::<()>("host.setVertexBuffer");
                crate::host_value::<()>("host.setIndexBuffer");
                crate::host_value::<()>("host.drawIndexed");
            },
        )
            as Box<
                dyn FnMut(WgpuRenderState, SceneRenderProxy, MeshGeometry) -> () + Send + 'static,
            >)),
    });

// Source: upstream/packages/scene-wgpu/src/wireframeWgpuMeshMaterialRenderer.ts:84 (sha256:a25fff1412989863197d3641b184ead95dc89679d38b9d301f4d4df8902c9546)
pub fn register_wireframe_wgpu_material(state: &mut WgpuRenderState) -> () {
    register_wgpu_mesh_material_renderer(
        state,
        (wireframe_material_kind_constant).to_owned(),
        &WIREFRAME_WGPU_MESH_MATERIAL_RENDERER,
    );
}

// Source: upstream/packages/scene-wgpu/src/wireframeWgpuMeshMaterialRenderer.ts:88 (sha256:ce25130beb5b2bbfc44750860410f83189e1726fa06593e75aef7cda6cd9f857)
static _SCRATCH: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/scene-wgpu/src/wireframeWgpuMeshMaterialRenderer.ts:89 (sha256:58f9b06eb92a298e62fc3daaeeb389221c18a43a05437c9efca88f9e2187010d)
static _DYNAMIC_OFFSETS: std::sync::LazyLock<std::sync::Mutex<Vec<u32>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0_u32; (1.0_f64) as usize]));

// Source: upstream/packages/scene-wgpu/src/wireframeWgpuMeshMaterialRenderer.ts:90 (sha256:aff9b994c4ec1ff829784b9cbcec2e44480de30e79309b95921d6d455229dd18)
static WHITE: std::sync::LazyLock<LinearColor> =
    std::sync::LazyLock::new(|| vec![1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64]);

// Source: upstream/packages/scene-wgpu/src/wireframeWgpuMeshMaterialRenderer.ts:91 (sha256:53b892d4e36765e1c2477bd7eaf40f934febcd5cabc1e6cd79654a2dd7a1720b)
static FALLBACK_MATERIAL: std::sync::LazyLock<WireframeMaterial> =
    std::sync::LazyLock::new(|| WireframeMaterial {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
    });
