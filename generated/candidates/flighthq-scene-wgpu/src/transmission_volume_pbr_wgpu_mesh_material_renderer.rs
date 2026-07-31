// @generated from upstream/packages/scene-wgpu/src/transmissionVolumePbrWgpuMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    WgpuMeshPipeline, begin_wgpu_mesh_draw, build_wgpu_pbr_standard_define_key,
    draw_wgpu_mesh_subset, ensure_wgpu_pbr_material_bind_group, ensure_wgpu_pbr_pipeline,
    get_wgpu_pbr_material_scratch, register_wgpu_mesh_material_renderer, write_wgpu_frame_uniform,
    write_wgpu_pbr_material_uniform, write_wgpu_pbr_standard_block,
};
use flighthq_color::{LinearColor, unpack_color_to_linear};
use flighthq_render_wgpu::get_wgpu_render_state_runtime;
use flighthq_types::{
    BlendMode, Camera, ColorTransform, DisplayObjectClipHooks, ImageResource, LinearColor,
    Material, Matrix, MeshGeometry, Sampler, SceneGraphSyncPolicy, SceneLightBlock,
    SceneRenderProxy, SceneResourceRef,
    TRANSMISSION_VOLUME_PBR_MATERIAL_KIND as transmission_volume_pbr_material_kind_constant,
    TextureColorSpace, TextureFilter, TextureWrap, TransmissionVolumePbrMaterial, Vector2,
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

// Source: upstream/packages/scene-wgpu/src/transmissionVolumePbrWgpuMeshMaterialRenderer.ts:46 (sha256:419d9bc59af11a995f1af490d4c4b9be0bb4900a746fb22ef329ca17b2f67228)
pub static TRANSMISSION_VOLUME_PBR_WGPU_MESH_MATERIAL_RENDERER: std::sync::LazyLock<
    WgpuMeshMaterialRenderer,
> = std::sync::LazyLock::new(|| WgpuMeshMaterialRenderer {
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
            let transmission = material;
            let standard = if (transmission).is_some() {
                Some((transmission.as_ref().unwrap().standard).clone())
            } else {
                None
            };
            let mut key = build_wgpu_pbr_standard_define_key(
                ((standard).clone()).clone(),
                (transmission).clone(),
            );
            key.transmission_enabled = true;
            let format = ((state_runtime.inner.lock().unwrap().current_color_format).clone())
                .unwrap_or((state.format).clone());
            let pipeline =
                ensure_wgpu_pbr_pipeline((state).clone(), (key).clone(), (format).clone());
            write_wgpu_frame_uniform(&mut state, &camera, &lights);
            let binding = ensure_wgpu_pbr_material_bind_group(
                &mut state,
                &pipeline,
                (transmission).unwrap_or((*FALLBACK_MATERIAL).clone()),
                ((standard).clone()).clone(),
            );
            let mut out = get_wgpu_pbr_material_scratch();
            write_wgpu_pbr_standard_block(
                &mut out,
                ((standard).clone()).clone(),
                if (transmission).is_some() {
                    transmission.as_ref().unwrap().alpha_cutoff
                } else {
                    0.5_f64
                },
            );
            out.fill((0.0_f64) as f32);
            if (transmission).is_some() {
                unpack_color_to_linear(
                    &mut _COLOR_SCRATCH,
                    transmission.as_ref().unwrap().attenuation_color,
                );
                out[44.0_f64 as usize] = (transmission.as_ref().unwrap().transmission) as f32;
                out[45.0_f64 as usize] = (_COLOR_SCRATCH[0.0_f64 as usize].clone()) as f32;
                out[46.0_f64 as usize] = (_COLOR_SCRATCH[1.0_f64 as usize].clone()) as f32;
                out[47.0_f64 as usize] = (_COLOR_SCRATCH[2.0_f64 as usize].clone()) as f32;
            } else {
                out[44.0_f64 as usize] = (0.0_f64) as f32;
                out[45.0_f64 as usize] = (1.0_f64) as f32;
                out[46.0_f64 as usize] = (1.0_f64) as f32;
                out[47.0_f64 as usize] = (1.0_f64) as f32;
            }
            write_wgpu_pbr_material_uniform(&state, &binding);
            begin_wgpu_mesh_draw(&mut state, &{
                let __flight_source = &(pipeline);
                WgpuMeshPipeline {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
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

// Source: upstream/packages/scene-wgpu/src/transmissionVolumePbrWgpuMeshMaterialRenderer.ts:96 (sha256:6f55e5f9f15d937f822c37fee1b6646524261339f6025560c728a73d83ee5532)
pub fn register_transmission_volume_pbr_wgpu_material(state: &mut WgpuRenderState) -> () {
    register_wgpu_mesh_material_renderer(
        state,
        (transmission_volume_pbr_material_kind_constant).to_owned(),
        &TRANSMISSION_VOLUME_PBR_WGPU_MESH_MATERIAL_RENDERER,
    );
}

// Source: upstream/packages/scene-wgpu/src/transmissionVolumePbrWgpuMeshMaterialRenderer.ts:104 (sha256:7b690302b132ad50cbe2feda1968af504bca4761a8a4f3dcd88b2efa9817cd0e)
static FALLBACK_MATERIAL: std::sync::LazyLock<TransmissionVolumePbrMaterial> =
    std::sync::LazyLock::new(|| TransmissionVolumePbrMaterial {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
    });

// Source: upstream/packages/scene-wgpu/src/transmissionVolumePbrWgpuMeshMaterialRenderer.ts:105 (sha256:6a9f830968bc6954bccfb97bb82ed041f446aefa0c2a2b179ed5e284309f551a)
static _COLOR_SCRATCH: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));
