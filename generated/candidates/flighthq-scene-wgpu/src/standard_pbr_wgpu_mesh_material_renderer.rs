// @generated from upstream/packages/scene-wgpu/src/standardPbrWgpuMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    WgpuMaterialBinding, WgpuMeshPipeline, WgpuPbrDefineKey, WgpuPbrPipeline, begin_wgpu_mesh_draw,
    draw_wgpu_mesh_subset, ensure_wgpu_pbr_pipeline, get_wgpu_scene_runtime, is_wgpu_texture_ready,
    stash_wgpu_uv_transform, write_wgpu_frame_uniform,
};
use flighthq_color::{LinearColor, unpack_color_to_linear};
use flighthq_render_wgpu::get_wgpu_render_state_runtime;
use flighthq_types::{
    BlendMode, Camera, ColorTransform, DisplayObjectClipHooks, ImageResource, LinearColor,
    Material, Matrix, MeshGeometry, Sampler, SceneGraphSyncPolicy, SceneLightBlock,
    SceneRenderProxy, SceneResourceRef, StandardPbrMaterial, StandardPbrMaterialProperties,
    SurfaceMaterial, TextureColorSpace, TextureFilter, TextureWrap, Vector2,
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

// Source: upstream/packages/scene-wgpu/src/standardPbrWgpuMeshMaterialRenderer.ts:35 (sha256:152ab22750185f3564f45a90663b683f034ecdee1bca32e9c5295038467fbb21)
pub const WGPU_PBR_MATERIAL_UNIFORM_FLOATS: f64 = 48.0_f64;

// Source: upstream/packages/scene-wgpu/src/standardPbrWgpuMeshMaterialRenderer.ts:43 (sha256:5ee57f58ebf40758031669974ce534db04721220031801bab3c4547d93386aa6)
pub fn build_wgpu_pbr_standard_define_key(
    standard: Option<StandardPbrMaterialProperties>,
    surface: Option<SurfaceMaterial>,
) -> WgpuPbrDefineKey {
    return WgpuPbrDefineKey {
        __flight_identity: std::sync::Arc::new(()),
        alpha_mask_enabled: ((surface).is_some())
            && ((surface.as_ref().unwrap().alpha_mode).clone() == "mask"),
        anisotropy_enabled: false,
        clearcoat_enabled: false,
        double_sided: ((surface).is_some()) && (surface.as_ref().unwrap().double_sided),
        has_base_color_map: ((standard).is_some())
            && (is_wgpu_texture_ready(
                ((standard.as_ref().unwrap().base_color_map).clone()).clone(),
            )),
        has_emissive_map: ((standard).is_some())
            && (is_wgpu_texture_ready(((standard.as_ref().unwrap().emissive_map).clone()).clone())),
        has_metallic_roughness_map: ((standard).is_some())
            && (is_wgpu_texture_ready(
                ((standard.as_ref().unwrap().metallic_roughness_map).clone()).clone(),
            )),
        has_normal_map: ((standard).is_some())
            && (is_wgpu_texture_ready(((standard.as_ref().unwrap().normal_map).clone()).clone())),
        has_occlusion_map: ((standard).is_some())
            && (is_wgpu_texture_ready(
                ((standard.as_ref().unwrap().occlusion_map).clone()).clone(),
            )),
        iridescence_enabled: false,
        sheen_enabled: false,
        specular_enabled: false,
        subsurface_enabled: false,
        transmission_enabled: false,
    };
}

// Source: upstream/packages/scene-wgpu/src/standardPbrWgpuMeshMaterialRenderer.ts:74 (sha256:6bdd3a6a83cd16211e3e5b461fb0c4133da85c4f2ec67285cf7a79cb4c2861f3)
pub fn ensure_wgpu_pbr_material_bind_group(
    state: &mut WgpuRenderState,
    pipeline: &WgpuPbrPipeline,
    key: crate::OpaqueHostValue,
    standard: Option<StandardPbrMaterialProperties>,
) -> WgpuMaterialBinding {
    let mut scene = get_wgpu_scene_runtime(state);
    let mut binding: Option<WgpuMaterialBinding> = scene
        .material_bind_groups
        .iter()
        .find(|(key, _)| key == &(key).clone())
        .map(|(_, value)| value.clone());
    if (binding).is_none() {
        let buffer = crate::host_value::<()>("host.createBuffer");
        let bind_group = crate::host_value::<()>("host.createBindGroup");
        binding = Some(WgpuMaterialBinding {
            __flight_identity: std::sync::Arc::new(()),
            bind_group: bind_group,
            buffer: buffer,
        });
        {
            let __flight_key = (key).clone();
            let __flight_value = (binding).clone().unwrap();
            if let Some((_, value)) = scene
                .material_bind_groups
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                scene
                    .material_bind_groups
                    .push((__flight_key, __flight_value));
            }
        };
    }
    stash_wgpu_uv_transform(
        state,
        (if (standard).is_some() {
            (standard.as_ref().unwrap().base_color_map).clone()
        } else {
            None
        })
        .clone(),
    );
    return ((binding).clone().unwrap()).clone();
}

// Source: upstream/packages/scene-wgpu/src/standardPbrWgpuMeshMaterialRenderer.ts:124 (sha256:f30192394fceae6c2c9e83f6afc92abc12c72dc3d100a1891fd79b646770bdca)
pub fn get_wgpu_pbr_material_scratch() -> Vec<f32> {
    return (*_MATERIAL_SCRATCH.lock().unwrap()).clone();
}

// Source: upstream/packages/scene-wgpu/src/standardPbrWgpuMeshMaterialRenderer.ts:130 (sha256:17021ca6782c9964a728c9d81d864b8ca61962e2270318e41dd8944fee9a76fe)
pub fn write_wgpu_pbr_material_uniform(
    state: &WgpuRenderState,
    binding: &WgpuMaterialBinding,
) -> () {
    crate::host_value::<()>("host.writeBuffer");
}

// Source: upstream/packages/scene-wgpu/src/standardPbrWgpuMeshMaterialRenderer.ts:140 (sha256:85e11cc0621a3c11a8b73e628504f0e7bf91b084125edcbdbbc98b6e2599402e)
pub fn write_wgpu_pbr_standard_block(
    out: &mut Vec<f32>,
    standard: Option<StandardPbrMaterialProperties>,
    alpha_cutoff: f64,
) -> () {
    if (standard).is_none() {
        out[0.0_f64 as usize] = (1.0_f64) as f32;
        out[1.0_f64 as usize] = (1.0_f64) as f32;
        out[2.0_f64 as usize] = (1.0_f64) as f32;
        out[3.0_f64 as usize] = (1.0_f64) as f32;
        out[4.0_f64 as usize] = (0.0_f64) as f32;
        out[5.0_f64 as usize] = (0.0_f64) as f32;
        out[6.0_f64 as usize] = (0.0_f64) as f32;
        out[7.0_f64 as usize] = (0.0_f64) as f32;
        out[8.0_f64 as usize] = (0.0_f64) as f32;
        out[9.0_f64 as usize] = (1.0_f64) as f32;
        out[10.0_f64 as usize] = (1.0_f64) as f32;
        out[11.0_f64 as usize] = (1.0_f64) as f32;
        out[12.0_f64 as usize] = (alpha_cutoff) as f32;
        out[13.0_f64 as usize] = (0.0_f64) as f32;
        out[14.0_f64 as usize] = (0.0_f64) as f32;
        out[15.0_f64 as usize] = (0.0_f64) as f32;
        return;
    }
    unpack_color_to_linear(&mut _COLOR_SCRATCH, standard.as_ref().unwrap().base_color);
    out[0.0_f64 as usize] = (_COLOR_SCRATCH[0.0_f64 as usize].clone()) as f32;
    out[1.0_f64 as usize] = (_COLOR_SCRATCH[1.0_f64 as usize].clone()) as f32;
    out[2.0_f64 as usize] = (_COLOR_SCRATCH[2.0_f64 as usize].clone()) as f32;
    out[3.0_f64 as usize] = (_COLOR_SCRATCH[3.0_f64 as usize].clone()) as f32;
    unpack_color_to_linear(&mut _COLOR_SCRATCH, standard.as_ref().unwrap().emissive);
    let strength = standard.as_ref().unwrap().emissive_strength;
    out[4.0_f64 as usize] = (_COLOR_SCRATCH[0.0_f64 as usize].clone() * strength) as f32;
    out[5.0_f64 as usize] = (_COLOR_SCRATCH[1.0_f64 as usize].clone() * strength) as f32;
    out[6.0_f64 as usize] = (_COLOR_SCRATCH[2.0_f64 as usize].clone() * strength) as f32;
    out[7.0_f64 as usize] = (0.0_f64) as f32;
    out[8.0_f64 as usize] = (standard.as_ref().unwrap().metallic) as f32;
    out[9.0_f64 as usize] = (standard.as_ref().unwrap().roughness) as f32;
    out[10.0_f64 as usize] = (standard.as_ref().unwrap().normal_scale) as f32;
    out[11.0_f64 as usize] = (standard.as_ref().unwrap().occlusion_strength) as f32;
    out[12.0_f64 as usize] = (alpha_cutoff) as f32;
    out[13.0_f64 as usize] = (0.0_f64) as f32;
    out[14.0_f64 as usize] = (0.0_f64) as f32;
    out[15.0_f64 as usize] = (0.0_f64) as f32;
}

// Source: upstream/packages/scene-wgpu/src/standardPbrWgpuMeshMaterialRenderer.ts:205 (sha256:2c84ce8e58933d75bace00cb67dba6511e2925d650f3c09c7bcf7c66df868131)
pub static STANDARD_PBR_WGPU_MESH_MATERIAL_RENDERER: std::sync::LazyLock<WgpuMeshMaterialRenderer> =
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
                let pbr = material;
                let format = ((state_runtime.inner.lock().unwrap().current_color_format).clone())
                    .unwrap_or((state.format).clone());
                let pipeline = ensure_wgpu_pbr_pipeline(
                    (state).clone(),
                    build_wgpu_pbr_standard_define_key((pbr).clone(), (pbr).clone()),
                    (format).clone(),
                );
                write_wgpu_frame_uniform(&mut state, &camera, &lights);
                let binding = ensure_wgpu_pbr_material_bind_group(
                    &mut state,
                    &pipeline,
                    (pbr).unwrap_or((*FALLBACK_MATERIAL).clone()),
                    (pbr).clone(),
                );
                write_wgpu_pbr_standard_block(
                    &mut (*_MATERIAL_SCRATCH.lock().unwrap()),
                    (pbr).clone(),
                    if (pbr).is_some() {
                        pbr.as_ref().unwrap().alpha_cutoff
                    } else {
                        0.5_f64
                    },
                );
                (*_MATERIAL_SCRATCH.lock().unwrap()).fill((0.0_f64) as f32);
                write_wgpu_pbr_material_uniform(&state, &binding);
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

// Source: upstream/packages/scene-wgpu/src/standardPbrWgpuMeshMaterialRenderer.ts:238 (sha256:4c53d8bb363369690c711312b95164e6ae7681a3cd49ac7459aadf31505f964d)
static FALLBACK_MATERIAL: std::sync::LazyLock<StandardPbrMaterial> =
    std::sync::LazyLock::new(|| StandardPbrMaterial {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
    });

// Source: upstream/packages/scene-wgpu/src/standardPbrWgpuMeshMaterialRenderer.ts:240 (sha256:6a9f830968bc6954bccfb97bb82ed041f446aefa0c2a2b179ed5e284309f551a)
static _COLOR_SCRATCH: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/scene-wgpu/src/standardPbrWgpuMeshMaterialRenderer.ts:241 (sha256:4970e8019b290e80215db20ed020e0468bf121807df3b02b7449d416fe77ba87)
static _MATERIAL_SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<f32>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![0.0_f32; (WGPU_PBR_MATERIAL_UNIFORM_FLOATS) as usize])
    });
