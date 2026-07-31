// @generated from upstream/packages/scene-gl/src/glPbrProgramCache.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GlPbrDefineKey, build_gl_pbr_define_key, compile_gl_program, ensure_gl_scene_program,
    get_gl_pbr_fragment_source_for_key, get_gl_pbr_vertex_source_for_key, get_gl_scene_runtime,
    resolve_gl_lit_locations,
};
use flighthq_types::{
    BlendMode, DisplayObjectClipHooks, GlRenderState, ImageResource, Matrix, Sampler,
    SceneGraphSyncPolicy, SceneResourceRef, TextureColorSpace, TextureFilter, TextureWrap, Vector2,
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

// Source: upstream/packages/scene-gl/src/glPbrProgramCache.ts:18 (sha256:be15c7f939aa65ac7978372bfc6a38c748bd924581b8e92951dc898088eb1b82)
#[derive(Clone, Default)]
pub struct GlPbrProgram {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loc_object_alpha: Option<crate::OpaqueHostValue>,
    pub loc_joint_texture: Option<crate::OpaqueHostValue>,
    pub loc_model: Option<crate::OpaqueHostValue>,
    pub loc_normal_matrix: Option<crate::OpaqueHostValue>,
    pub loc_uv_transform: Option<crate::OpaqueHostValue>,
    pub loc_view_projection: Option<crate::OpaqueHostValue>,
    pub program: crate::OpaqueHostValue,
    pub loc_ambient_count: Option<crate::OpaqueHostValue>,
    pub loc_ambient_radiance: Option<crate::OpaqueHostValue>,
    pub loc_camera_position: Option<crate::OpaqueHostValue>,
    pub loc_directional: Option<crate::OpaqueHostValue>,
    pub loc_directional_count: Option<crate::OpaqueHostValue>,
    pub loc_directional_radiance: Option<crate::OpaqueHostValue>,
    pub loc_hemisphere_count: Option<crate::OpaqueHostValue>,
    pub loc_hemisphere_lights: Option<crate::OpaqueHostValue>,
    pub loc_ibl_brdf: Option<crate::OpaqueHostValue>,
    pub loc_ibl_enabled: Option<crate::OpaqueHostValue>,
    pub loc_ibl_intensity: Option<crate::OpaqueHostValue>,
    pub loc_ibl_irradiance: Option<crate::OpaqueHostValue>,
    pub loc_ibl_max_mip: Option<crate::OpaqueHostValue>,
    pub loc_ibl_prefiltered: Option<crate::OpaqueHostValue>,
    pub loc_point_count: Option<crate::OpaqueHostValue>,
    pub loc_point_lights: Option<crate::OpaqueHostValue>,
    pub loc_shadow_enabled: Option<crate::OpaqueHostValue>,
    pub loc_shadow_map: Option<crate::OpaqueHostValue>,
    pub loc_shadow_matrix: Option<crate::OpaqueHostValue>,
    pub loc_spot_count: Option<crate::OpaqueHostValue>,
    pub loc_spot_lights: Option<crate::OpaqueHostValue>,
    pub loc_alpha_cutoff: Option<crate::OpaqueHostValue>,
    pub loc_anisotropy_rotation: Option<crate::OpaqueHostValue>,
    pub loc_anisotropy_strength: Option<crate::OpaqueHostValue>,
    pub loc_attenuation_color: Option<crate::OpaqueHostValue>,
    pub loc_base_color: Option<crate::OpaqueHostValue>,
    pub loc_base_color_map: Option<crate::OpaqueHostValue>,
    pub loc_clearcoat: Option<crate::OpaqueHostValue>,
    pub loc_clearcoat_roughness: Option<crate::OpaqueHostValue>,
    pub loc_emissive: Option<crate::OpaqueHostValue>,
    pub loc_emissive_map: Option<crate::OpaqueHostValue>,
    pub loc_emissive_strength: Option<crate::OpaqueHostValue>,
    pub loc_iridescence: Option<crate::OpaqueHostValue>,
    pub loc_iridescence_ior: Option<crate::OpaqueHostValue>,
    pub loc_iridescence_thickness: Option<crate::OpaqueHostValue>,
    pub loc_metallic: Option<crate::OpaqueHostValue>,
    pub loc_metallic_roughness_map: Option<crate::OpaqueHostValue>,
    pub loc_normal_map: Option<crate::OpaqueHostValue>,
    pub loc_normal_scale: Option<crate::OpaqueHostValue>,
    pub loc_occlusion_map: Option<crate::OpaqueHostValue>,
    pub loc_occlusion_strength: Option<crate::OpaqueHostValue>,
    pub loc_roughness: Option<crate::OpaqueHostValue>,
    pub loc_sheen_color: Option<crate::OpaqueHostValue>,
    pub loc_sheen_roughness: Option<crate::OpaqueHostValue>,
    pub loc_specular: Option<crate::OpaqueHostValue>,
    pub loc_specular_color: Option<crate::OpaqueHostValue>,
    pub loc_subsurface: Option<crate::OpaqueHostValue>,
    pub loc_subsurface_color: Option<crate::OpaqueHostValue>,
    pub loc_thickness: Option<crate::OpaqueHostValue>,
    pub loc_transmission: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlPbrProgram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glPbrProgramCache.ts:53 (sha256:84ea4c4f9a8aa5f3d0b7bd9062e870299942e85f73a9751bb7b9521f39c669a9)
pub fn compile_gl_pbr_program(gl: crate::OpaqueHostValue, key: &GlPbrDefineKey) -> GlPbrProgram {
    let vertex_source = get_gl_pbr_vertex_source_for_key(key);
    let fragment_source = get_gl_pbr_fragment_source_for_key(key);
    let program = compile_gl_program(
        (gl).clone(),
        (vertex_source).clone(),
        (fragment_source).clone(),
    );
    return {
        let __flight_spread_0 = resolve_gl_lit_locations((gl).clone(), (program).clone());
        GlPbrProgram {
            __flight_identity: std::sync::Arc::new(()),
            loc_object_alpha: (__flight_spread_0.loc_object_alpha).clone(),
            loc_joint_texture: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_model: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_normal_matrix: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_uv_transform: (__flight_spread_0.loc_uv_transform).clone(),
            loc_view_projection: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            program: (program).clone(),
            loc_ambient_count: (__flight_spread_0.loc_ambient_count).clone(),
            loc_ambient_radiance: (__flight_spread_0.loc_ambient_radiance).clone(),
            loc_camera_position: (__flight_spread_0.loc_camera_position).clone(),
            loc_directional: (__flight_spread_0.loc_directional).clone(),
            loc_directional_count: (__flight_spread_0.loc_directional_count).clone(),
            loc_directional_radiance: (__flight_spread_0.loc_directional_radiance).clone(),
            loc_hemisphere_count: (__flight_spread_0.loc_hemisphere_count).clone(),
            loc_hemisphere_lights: (__flight_spread_0.loc_hemisphere_lights).clone(),
            loc_ibl_brdf: (__flight_spread_0.loc_ibl_brdf).clone(),
            loc_ibl_enabled: (__flight_spread_0.loc_ibl_enabled).clone(),
            loc_ibl_intensity: (__flight_spread_0.loc_ibl_intensity).clone(),
            loc_ibl_irradiance: (__flight_spread_0.loc_ibl_irradiance).clone(),
            loc_ibl_max_mip: (__flight_spread_0.loc_ibl_max_mip).clone(),
            loc_ibl_prefiltered: (__flight_spread_0.loc_ibl_prefiltered).clone(),
            loc_point_count: (__flight_spread_0.loc_point_count).clone(),
            loc_point_lights: (__flight_spread_0.loc_point_lights).clone(),
            loc_shadow_enabled: (__flight_spread_0.loc_shadow_enabled).clone(),
            loc_shadow_map: (__flight_spread_0.loc_shadow_map).clone(),
            loc_shadow_matrix: (__flight_spread_0.loc_shadow_matrix).clone(),
            loc_spot_count: (__flight_spread_0.loc_spot_count).clone(),
            loc_spot_lights: (__flight_spread_0.loc_spot_lights).clone(),
            loc_alpha_cutoff: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_anisotropy_rotation: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_anisotropy_strength: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_attenuation_color: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_base_color: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_base_color_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_clearcoat: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_clearcoat_roughness: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_emissive: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_emissive_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_emissive_strength: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_iridescence: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_iridescence_ior: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_iridescence_thickness: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_metallic: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_metallic_roughness_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_normal_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_normal_scale: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_occlusion_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_occlusion_strength: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_roughness: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_sheen_color: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_sheen_roughness: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_specular: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_specular_color: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_subsurface: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_subsurface_color: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_thickness: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_transmission: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
        }
    };
}

// Source: upstream/packages/scene-gl/src/glPbrProgramCache.ts:99 (sha256:941ae9f6be1f44fb597a00e50e224c3623a88eef14e279e12456e4f3640e49e5)
pub fn ensure_gl_pbr_program(state: &mut GlRenderState, key: &GlPbrDefineKey) -> GlPbrProgram {
    let full_key: GlPbrDefineKey = GlPbrDefineKey {
        has_skin: Some(get_gl_scene_runtime(state).active_skinned_run),
        ..((*key).clone()).clone()
    };
    return ensure_gl_scene_program(
        state,
        format!("pbr:{}", build_gl_pbr_define_key(&full_key)),
        &mut |gl: crate::OpaqueHostValue| -> GlPbrProgram {
            compile_gl_pbr_program((gl).clone(), &full_key)
        },
    );
}
