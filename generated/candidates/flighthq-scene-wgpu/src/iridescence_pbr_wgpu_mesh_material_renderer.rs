// @generated from upstream/packages/scene-wgpu/src/iridescencePbrWgpuMeshMaterialRenderer.ts; do not edit.
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
use flighthq_render_wgpu::get_wgpu_render_state_runtime;
use flighthq_types::{
    BlendMode, Camera, ColorTransform, DisplayObjectClipHooks,
    IRIDESCENCE_PBR_MATERIAL_KIND as iridescence_pbr_material_kind_constant, ImageResource,
    IridescencePbrMaterial, Material, Matrix, MeshGeometry, Sampler, SceneGraphSyncPolicy,
    SceneLightBlock, SceneRenderProxy, SceneResourceRef, TextureColorSpace, TextureFilter,
    TextureWrap, Vector2, WgpuMeshMaterialRenderer, WgpuRenderState,
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

// Source: upstream/packages/scene-wgpu/src/iridescencePbrWgpuMeshMaterialRenderer.ts:36 (sha256:6b72d676bdb46efacfa4f7a56fc7bed9d5bab9e16888fc37ff7619fd73bf5cfe)
pub static IRIDESCENCE_PBR_WGPU_MESH_MATERIAL_RENDERER: std::sync::LazyLock<
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
            let iridescence = material;
            let standard = if (iridescence).is_some() {
                Some((iridescence.as_ref().unwrap().standard).clone())
            } else {
                None
            };
            let mut key = build_wgpu_pbr_standard_define_key(
                ((standard).clone()).clone(),
                (iridescence).clone(),
            );
            key.iridescence_enabled = true;
            let format = ((state_runtime.inner.lock().unwrap().current_color_format).clone())
                .unwrap_or((state.format).clone());
            let pipeline =
                ensure_wgpu_pbr_pipeline((state).clone(), (key).clone(), (format).clone());
            write_wgpu_frame_uniform(&mut state, &camera, &lights);
            let binding = ensure_wgpu_pbr_material_bind_group(
                &mut state,
                &pipeline,
                (iridescence).unwrap_or((*FALLBACK_MATERIAL).clone()),
                ((standard).clone()).clone(),
            );
            let mut out = get_wgpu_pbr_material_scratch();
            write_wgpu_pbr_standard_block(
                &mut out,
                ((standard).clone()).clone(),
                if (iridescence).is_some() {
                    iridescence.as_ref().unwrap().alpha_cutoff
                } else {
                    0.5_f64
                },
            );
            out.fill((0.0_f64) as f32);
            if (iridescence).is_some() {
                out[28.0_f64 as usize] = (iridescence.as_ref().unwrap().iridescence) as f32;
                out[29.0_f64 as usize] = (iridescence.as_ref().unwrap().iridescence_ior) as f32;
                out[30.0_f64 as usize] = ((iridescence.as_ref().unwrap().iridescence_thickness_min
                    + iridescence.as_ref().unwrap().iridescence_thickness_max)
                    * 0.5_f64) as f32;
            } else {
                out[28.0_f64 as usize] = (0.0_f64) as f32;
                out[29.0_f64 as usize] = (1.3_f64) as f32;
                out[30.0_f64 as usize] = (250.0_f64) as f32;
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

// Source: upstream/packages/scene-wgpu/src/iridescencePbrWgpuMeshMaterialRenderer.ts:82 (sha256:1f78ec876b70b4c441c0fed13102f66e7a4125ef626786b0f234041e115ebdb9)
pub fn register_iridescence_pbr_wgpu_material(state: &mut WgpuRenderState) -> () {
    register_wgpu_mesh_material_renderer(
        state,
        (iridescence_pbr_material_kind_constant).to_owned(),
        &IRIDESCENCE_PBR_WGPU_MESH_MATERIAL_RENDERER,
    );
}

// Source: upstream/packages/scene-wgpu/src/iridescencePbrWgpuMeshMaterialRenderer.ts:86 (sha256:8bb66511675fc2174c146782bdb28a185297200cf3990972c35f4b1652921409)
static FALLBACK_MATERIAL: std::sync::LazyLock<IridescencePbrMaterial> =
    std::sync::LazyLock::new(|| IridescencePbrMaterial {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
    });
