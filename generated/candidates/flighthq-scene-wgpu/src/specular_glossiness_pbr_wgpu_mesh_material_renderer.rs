// @generated from upstream/packages/scene-wgpu/src/specularGlossinessPbrWgpuMeshMaterialRenderer.ts; do not edit.
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
    Material, Matrix, MeshGeometry,
    SPECULAR_GLOSSINESS_PBR_MATERIAL_KIND as specular_glossiness_pbr_material_kind_constant,
    Sampler, SceneGraphSyncPolicy, SceneLightBlock, SceneRenderProxy, SceneResourceRef,
    SpecularGlossinessPbrMaterial, StandardPbrMaterialProperties, TextureColorSpace, TextureFilter,
    TextureWrap, Vector2, WgpuMeshMaterialRenderer, WgpuRenderState,
};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

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

// Source: upstream/packages/scene-wgpu/src/specularGlossinessPbrWgpuMeshMaterialRenderer.ts:52 (sha256:92e5c038cd8aead4baca52ef479990e310dcf334ea830e5f52edc6149a68636b)
pub static SPECULAR_GLOSSINESS_PBR_WGPU_MESH_MATERIAL_RENDERER: std::sync::LazyLock<
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
            let spec_gloss = material;
            let standard = if (spec_gloss).is_some() {
                Some(convert_specular_glossiness_to_standard(
                    &spec_gloss.as_ref().unwrap(),
                ))
            } else {
                None
            };
            let key = build_wgpu_pbr_standard_define_key(
                ((standard).clone()).clone(),
                (spec_gloss).clone(),
            );
            let format = ((state_runtime.inner.lock().unwrap().current_color_format).clone())
                .unwrap_or((state.format).clone());
            let pipeline =
                ensure_wgpu_pbr_pipeline((state).clone(), (key).clone(), (format).clone());
            write_wgpu_frame_uniform(&mut state, &camera, &lights);
            let binding = ensure_wgpu_pbr_material_bind_group(
                &mut state,
                &pipeline,
                (spec_gloss).unwrap_or((*FALLBACK_MATERIAL).clone()),
                ((standard).clone()).clone(),
            );
            let mut out = get_wgpu_pbr_material_scratch();
            write_wgpu_pbr_standard_block(
                &mut out,
                ((standard).clone()).clone(),
                if (spec_gloss).is_some() {
                    spec_gloss.as_ref().unwrap().alpha_cutoff
                } else {
                    0.5_f64
                },
            );
            out.fill((0.0_f64) as f32);
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

// Source: upstream/packages/scene-wgpu/src/specularGlossinessPbrWgpuMeshMaterialRenderer.ts:88 (sha256:1bbb6c80a9ac0ea1b200866b0f359c204415d6b39779db18195d118fccf259b1)
pub fn register_specular_glossiness_pbr_wgpu_material(state: &mut WgpuRenderState) -> () {
    register_wgpu_mesh_material_renderer(
        state,
        (specular_glossiness_pbr_material_kind_constant).to_owned(),
        &SPECULAR_GLOSSINESS_PBR_WGPU_MESH_MATERIAL_RENDERER,
    );
}

// Source: upstream/packages/scene-wgpu/src/specularGlossinessPbrWgpuMeshMaterialRenderer.ts:101 (sha256:ac988c88fb2b74b7f2ac47272c3b195a5450f584af48bfbd672589af951f4a35)
fn convert_specular_glossiness_to_standard(
    material: &SpecularGlossinessPbrMaterial,
) -> StandardPbrMaterialProperties {
    unpack_color_to_linear(&mut _DIFFUSE_SCRATCH, material.diffuse);
    unpack_color_to_linear(&mut _SPECULAR_SCRATCH, material.specular);
    let specular_brightness = ((_SPECULAR_SCRATCH[0.0_f64 as usize].clone())
        .max(_SPECULAR_SCRATCH[1.0_f64 as usize].clone()))
    .max(_SPECULAR_SCRATCH[2.0_f64 as usize].clone());
    let one_minus_specular_strength = (1.0_f64 - specular_brightness);
    let diffuse_brightness = ((_DIFFUSE_SCRATCH[0.0_f64 as usize].clone())
        .max(_DIFFUSE_SCRATCH[1.0_f64 as usize].clone()))
    .max(_DIFFUSE_SCRATCH[2.0_f64 as usize].clone());
    let metallic = solve_metallic(
        diffuse_brightness,
        specular_brightness,
        one_minus_specular_strength,
    );
    let denom = (1.0_f64 - DIELECTRIC_SPECULAR).max(0.0001_f64);
    let r = lerp(
        ((_DIFFUSE_SCRATCH[0.0_f64 as usize].clone() * one_minus_specular_strength) / denom),
        _SPECULAR_SCRATCH[0.0_f64 as usize].clone(),
        metallic,
    );
    let g = lerp(
        ((_DIFFUSE_SCRATCH[1.0_f64 as usize].clone() * one_minus_specular_strength) / denom),
        _SPECULAR_SCRATCH[1.0_f64 as usize].clone(),
        metallic,
    );
    let b = lerp(
        ((_DIFFUSE_SCRATCH[2.0_f64 as usize].clone() * one_minus_specular_strength) / denom),
        _SPECULAR_SCRATCH[2.0_f64 as usize].clone(),
        metallic,
    );
    return StandardPbrMaterialProperties {
        __flight_identity: std::sync::Arc::new(()),
        base_color: pack_linear_rgba(r, g, b, _DIFFUSE_SCRATCH[3.0_f64 as usize].clone()),
        base_color_map: (material.diffuse_map).clone(),
        emissive: material.emissive,
        emissive_map: (material.emissive_map).clone(),
        emissive_strength: material.emissive_strength,
        metallic: metallic,
        metallic_roughness_map: None,
        normal_map: (material.normal_map).clone(),
        normal_scale: material.normal_scale,
        occlusion_map: (material.occlusion_map).clone(),
        occlusion_strength: material.occlusion_strength,
        roughness: (1.0_f64 - material.glossiness),
    };
}

// Source: upstream/packages/scene-wgpu/src/specularGlossinessPbrWgpuMeshMaterialRenderer.ts:135 (sha256:cd0b0c7f12d62073921747b45ae6e65432836c014bfddce7b6618c8baafc1450)
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    return (a + ((b - a) * t));
}

// Source: upstream/packages/scene-wgpu/src/specularGlossinessPbrWgpuMeshMaterialRenderer.ts:141 (sha256:626a62e8ce825f13d669ae542bb99d554ef9e1c4c6ea66eb51587029fa9494eb)
fn pack_linear_rgba(r: f64, g: f64, b: f64, a: f64) -> f64 {
    let mut to_byte: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new(move |linear: f64| -> f64 {
            let clamped = ((linear).max(0.0_f64)).min(1.0_f64);
            let srgb = if (clamped <= 0.0031308_f64) {
                (clamped * 12.92_f64)
            } else {
                ((1.055_f64 * (clamped).powf((1.0_f64 / 2.4_f64))) - 0.055_f64)
            };
            return (__flight_js_to_i32((srgb * 255.0_f64).round()) & __flight_js_to_i32(255.0_f64))
                as f64;
        })
            as Box<dyn FnMut(f64) -> f64 + Send + 'static>));
    let alpha = (__flight_js_to_i32((((a).max(0.0_f64)).min(1.0_f64) * 255.0_f64).round())
        & __flight_js_to_i32(255.0_f64)) as f64;
    return (__flight_js_to_u32(
        (__flight_js_to_i32(
            (__flight_js_to_i32(
                (__flight_js_to_i32(
                    __flight_js_to_i32({
                        let __flight_callback = (to_byte).clone();
                        let __flight_result = __flight_callback.lock().unwrap()(r);
                        __flight_result
                    })
                    .wrapping_shl((__flight_js_to_u32(24.0_f64) & 31)) as f64,
                ) | __flight_js_to_i32(
                    __flight_js_to_i32({
                        let __flight_callback = (to_byte).clone();
                        let __flight_result = __flight_callback.lock().unwrap()(g);
                        __flight_result
                    })
                    .wrapping_shl((__flight_js_to_u32(16.0_f64) & 31)) as f64,
                )) as f64,
            ) | __flight_js_to_i32(
                __flight_js_to_i32({
                    let __flight_callback = (to_byte).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(b);
                    __flight_result
                })
                .wrapping_shl((__flight_js_to_u32(8.0_f64) & 31)) as f64,
            )) as f64,
        ) | __flight_js_to_i32(alpha)) as f64,
    ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/scene-wgpu/src/specularGlossinessPbrWgpuMeshMaterialRenderer.ts:154 (sha256:a1c15f5161cfc62eb7be16df30b9216d1c76369868335a877e1e596db421e329)
fn solve_metallic(diffuse: f64, specular: f64, one_minus_specular_strength: f64) -> f64 {
    if (specular < DIELECTRIC_SPECULAR) {
        return 0.0_f64;
    }
    let a = DIELECTRIC_SPECULAR;
    let b = ((((diffuse * one_minus_specular_strength) / (1.0_f64 - DIELECTRIC_SPECULAR))
        + specular)
        - (2.0_f64 * DIELECTRIC_SPECULAR));
    let c = (DIELECTRIC_SPECULAR - specular);
    let discriminant = ((b * b) - ((4.0_f64 * a) * c)).max(0.0_f64);
    return ((((-b) + (discriminant).sqrt()) / (2.0_f64 * a)).max(0.0_f64)).min(1.0_f64);
}

// Source: upstream/packages/scene-wgpu/src/specularGlossinessPbrWgpuMeshMaterialRenderer.ts:163 (sha256:6eafeff718d709fcec713ab73bf33641a15f4332172b7fc86e7d4648e9f4863c)
const DIELECTRIC_SPECULAR: f64 = 0.04_f64;

// Source: upstream/packages/scene-wgpu/src/specularGlossinessPbrWgpuMeshMaterialRenderer.ts:164 (sha256:f268f08b5d87f2a97870208969bdb046dfe0f8a12b12723347a3aa4edaa9b0c9)
static FALLBACK_MATERIAL: std::sync::LazyLock<SpecularGlossinessPbrMaterial> =
    std::sync::LazyLock::new(|| SpecularGlossinessPbrMaterial {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
    });

// Source: upstream/packages/scene-wgpu/src/specularGlossinessPbrWgpuMeshMaterialRenderer.ts:165 (sha256:a4ef48e76217347d58a822891df9f1c1e3e5c1a7656b5d36194626db150f49e2)
static _DIFFUSE_SCRATCH: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/scene-wgpu/src/specularGlossinessPbrWgpuMeshMaterialRenderer.ts:166 (sha256:b3c7903d68fb17d9324e095697f04ccf2ec3d5509bcdc2f7b2d6c3a634e77f25)
static _SPECULAR_SCRATCH: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));
