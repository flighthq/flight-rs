// @generated from upstream/packages/scene-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GlLitProgram, GlMeshProgram, begin_gl_mesh_draw, bind_gl_mesh_light_block,
    bind_gl_pbr_standard_block, build_gl_pbr_standard_define_key, draw_gl_mesh_subset,
    ensure_gl_pbr_program, get_gl_scene_runtime, register_gl_mesh_material_renderer,
    set_gl_mesh_camera_position, set_gl_mesh_view_projection,
};
use flighthq_color::{LinearColor, unpack_color_to_linear};
use flighthq_types::{
    BlendMode, Camera, DisplayObjectClipHooks, GlMeshMaterialRenderer, GlRenderState,
    ImageResource, LinearColor, Material, Matrix, MeshGeometry,
    SPECULAR_GLOSSINESS_PBR_MATERIAL_KIND as specular_glossiness_pbr_material_kind_constant,
    Sampler, SceneGraphSyncPolicy, SceneLightBlock, SceneRenderProxy, SceneResourceRef,
    SpecularGlossinessPbrMaterial, StandardPbrMaterialProperties, TextureColorSpace, TextureFilter,
    TextureWrap, Vector2,
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

// Source: upstream/packages/scene-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:46 (sha256:82a385b777364f97e514d85dd590c8b343181728d57637c49193f822b1a6e03c)
pub static SPECULAR_GLOSSINESS_PBR_GL_MESH_MATERIAL_RENDERER: std::sync::LazyLock<
    GlMeshMaterialRenderer,
> = std::sync::LazyLock::new(|| GlMeshMaterialRenderer {
    __flight_identity: std::sync::Arc::new(()),
    bind: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
        move |mut state: GlRenderState,
              material: Option<Material>,
              lights: SceneLightBlock,
              camera: Camera|
              -> () {
            let gl = (state.gl).clone();
            let spec_gloss = material;
            let standard = if (spec_gloss).is_some() {
                Some(convert_specular_glossiness_to_standard(
                    &spec_gloss.as_ref().unwrap(),
                ))
            } else {
                None
            };
            let mut program = ensure_gl_pbr_program(
                &mut state,
                &build_gl_pbr_standard_define_key(
                    ((standard).clone()).clone(),
                    ((spec_gloss).is_some())
                        && ((spec_gloss.as_ref().unwrap().alpha_mode).clone() == "mask"),
                ),
            );
            begin_gl_mesh_draw(
                &mut state,
                &{
                    let __flight_source = &(program);
                    GlMeshProgram {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        loc_object_alpha: (__flight_source.loc_object_alpha).clone(),
                        loc_joint_texture: (__flight_source.loc_joint_texture).clone(),
                        loc_model: (__flight_source.loc_model).clone(),
                        loc_normal_matrix: (__flight_source.loc_normal_matrix).clone(),
                        loc_uv_transform: (__flight_source.loc_uv_transform).clone(),
                        loc_view_projection: (__flight_source.loc_view_projection).clone(),
                        program: (__flight_source.program).clone(),
                    }
                },
                ((spec_gloss).is_some()) && (spec_gloss.as_ref().unwrap().double_sided),
            );
            set_gl_mesh_view_projection(
                (gl).clone(),
                ((program.loc_view_projection).clone()).clone(),
                &camera,
            );
            set_gl_mesh_camera_position(
                (gl).clone(),
                ((program.loc_camera_position).clone()).clone(),
                &camera,
            );
            bind_gl_mesh_light_block(
                &mut state,
                &{
                    let __flight_source = &(program);
                    GlLitProgram {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        loc_object_alpha: (__flight_source.loc_object_alpha).clone(),
                        loc_joint_texture: (__flight_source.loc_joint_texture).clone(),
                        loc_model: (__flight_source.loc_model).clone(),
                        loc_normal_matrix: (__flight_source.loc_normal_matrix).clone(),
                        loc_uv_transform: (__flight_source.loc_uv_transform).clone(),
                        loc_view_projection: (__flight_source.loc_view_projection).clone(),
                        program: (__flight_source.program).clone(),
                        loc_ambient_count: (__flight_source.loc_ambient_count).clone(),
                        loc_ambient_radiance: (__flight_source.loc_ambient_radiance).clone(),
                        loc_camera_position: (__flight_source.loc_camera_position).clone(),
                        loc_directional: (__flight_source.loc_directional).clone(),
                        loc_directional_count: (__flight_source.loc_directional_count).clone(),
                        loc_directional_radiance: (__flight_source.loc_directional_radiance)
                            .clone(),
                        loc_hemisphere_count: (__flight_source.loc_hemisphere_count).clone(),
                        loc_hemisphere_lights: (__flight_source.loc_hemisphere_lights).clone(),
                        loc_ibl_brdf: (__flight_source.loc_ibl_brdf).clone(),
                        loc_ibl_enabled: (__flight_source.loc_ibl_enabled).clone(),
                        loc_ibl_intensity: (__flight_source.loc_ibl_intensity).clone(),
                        loc_ibl_irradiance: (__flight_source.loc_ibl_irradiance).clone(),
                        loc_ibl_max_mip: (__flight_source.loc_ibl_max_mip).clone(),
                        loc_ibl_prefiltered: (__flight_source.loc_ibl_prefiltered).clone(),
                        loc_point_count: (__flight_source.loc_point_count).clone(),
                        loc_point_lights: (__flight_source.loc_point_lights).clone(),
                        loc_shadow_enabled: (__flight_source.loc_shadow_enabled).clone(),
                        loc_shadow_map: (__flight_source.loc_shadow_map).clone(),
                        loc_shadow_matrix: (__flight_source.loc_shadow_matrix).clone(),
                        loc_spot_count: (__flight_source.loc_spot_count).clone(),
                        loc_spot_lights: (__flight_source.loc_spot_lights).clone(),
                    }
                },
                &lights,
            );
            bind_gl_pbr_standard_block(&state, &mut program, ((standard).clone()).clone());
            crate::host_value::<()>("host.uniform1f");
        },
    )
        as Box<
            dyn FnMut(GlRenderState, Option<Material>, SceneLightBlock, Camera) -> ()
                + Send
                + 'static,
        >)),
    draw: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
        move |mut state: GlRenderState,
              proxy: SceneRenderProxy,
              mut geometry: MeshGeometry|
              -> () {
            let mut program = (get_gl_scene_runtime(&mut state).active_mesh_program).clone();
            if (program).is_none() {
                return;
            }
            draw_gl_mesh_subset(
                &mut state,
                &mut program.as_mut().unwrap(),
                &proxy,
                &mut geometry,
            );
        },
    )
        as Box<
            dyn FnMut(GlRenderState, SceneRenderProxy, MeshGeometry) -> () + Send + 'static,
        >)),
});

// Source: upstream/packages/scene-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:79 (sha256:b572cb72caedddcb8b853c40b894cc09dc9a7b23516314f5acf5ca60efe0bb83)
pub fn register_specular_glossiness_pbr_gl_material(state: &mut GlRenderState) -> () {
    register_gl_mesh_material_renderer(
        state,
        (specular_glossiness_pbr_material_kind_constant).to_owned(),
        &SPECULAR_GLOSSINESS_PBR_GL_MESH_MATERIAL_RENDERER,
    );
}

// Source: upstream/packages/scene-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:88 (sha256:2a88920601a90f154bfab8997d8f9ad0a183345b81bc1684e47a62c210867f61)
fn convert_specular_glossiness_to_standard(
    material: &SpecularGlossinessPbrMaterial,
) -> StandardPbrMaterialProperties {
    unpack_color_to_linear(&mut SCRATCH_DIFFUSE, material.diffuse);
    unpack_color_to_linear(&mut SCRATCH_SPECULAR, material.specular);
    let specular_brightness = ((SCRATCH_SPECULAR[0.0_f64 as usize].clone())
        .max(SCRATCH_SPECULAR[1.0_f64 as usize].clone()))
    .max(SCRATCH_SPECULAR[2.0_f64 as usize].clone());
    let one_minus_specular_strength = (1.0_f64 - specular_brightness);
    let diffuse_brightness = ((SCRATCH_DIFFUSE[0.0_f64 as usize].clone())
        .max(SCRATCH_DIFFUSE[1.0_f64 as usize].clone()))
    .max(SCRATCH_DIFFUSE[2.0_f64 as usize].clone());
    let metallic = solve_metallic(
        diffuse_brightness,
        specular_brightness,
        one_minus_specular_strength,
    );
    let denom = (1.0_f64 - DIELECTRIC_SPECULAR).max(0.0001_f64);
    let r = lerp(
        ((SCRATCH_DIFFUSE[0.0_f64 as usize].clone() * one_minus_specular_strength) / denom),
        SCRATCH_SPECULAR[0.0_f64 as usize].clone(),
        metallic,
    );
    let g = lerp(
        ((SCRATCH_DIFFUSE[1.0_f64 as usize].clone() * one_minus_specular_strength) / denom),
        SCRATCH_SPECULAR[1.0_f64 as usize].clone(),
        metallic,
    );
    let b = lerp(
        ((SCRATCH_DIFFUSE[2.0_f64 as usize].clone() * one_minus_specular_strength) / denom),
        SCRATCH_SPECULAR[2.0_f64 as usize].clone(),
        metallic,
    );
    return StandardPbrMaterialProperties {
        __flight_identity: std::sync::Arc::new(()),
        base_color: pack_linear_rgba(r, g, b, SCRATCH_DIFFUSE[3.0_f64 as usize].clone()),
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

// Source: upstream/packages/scene-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:122 (sha256:cd0b0c7f12d62073921747b45ae6e65432836c014bfddce7b6618c8baafc1450)
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    return (a + ((b - a) * t));
}

// Source: upstream/packages/scene-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:128 (sha256:626a62e8ce825f13d669ae542bb99d554ef9e1c4c6ea66eb51587029fa9494eb)
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

// Source: upstream/packages/scene-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:141 (sha256:a1c15f5161cfc62eb7be16df30b9216d1c76369868335a877e1e596db421e329)
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

// Source: upstream/packages/scene-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:150 (sha256:6eafeff718d709fcec713ab73bf33641a15f4332172b7fc86e7d4648e9f4863c)
const DIELECTRIC_SPECULAR: f64 = 0.04_f64;

// Source: upstream/packages/scene-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:151 (sha256:584fea5993f7928db33b03e244cc367cf635b860292ffc9d4823806de8e1ce72)
static SCRATCH_DIFFUSE: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/scene-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:152 (sha256:4386710dd492cfc412409f758b2713609d4d20b0babac253dcb67a2987dc9561)
static SCRATCH_SPECULAR: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));
