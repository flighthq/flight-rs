// @generated from upstream/packages/scene-gl/src/glToonPrelude.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GL_MESH_LIGHT_BLOCK_GLSL as gl_mesh_light_block_glsl_constant,
    GL_SKIN_VERTEX_DECLARATIONS_GLSL as gl_skin_vertex_declarations_glsl_constant,
    GL_UV_TRANSFORM_VERTEX_GLSL as gl_uv_transform_vertex_glsl_constant, compile_gl_program,
    ensure_gl_scene_program, get_gl_scene_runtime, resolve_gl_lit_locations,
};
use flighthq_types::{
    BlendMode, DisplayObjectClipHooks, GlRenderState, ImageResource,
    MAX_FORWARD_LIGHTS as max_forward_lights_constant, Matrix, Sampler, SceneGraphSyncPolicy,
    SceneResourceRef, TextureColorSpace, TextureFilter, TextureWrap, Vector2,
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

// Source: upstream/packages/scene-gl/src/glToonPrelude.ts:38 (sha256:c6b315718ecec2b181e33f0eac91428424a4b592837007264c7287f6786786f9)
#[derive(Clone, Default)]
pub struct GlToonDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mask_enabled: bool,
    pub has_base_color_map: bool,
    pub has_ramp: bool,
    pub has_skin: Option<bool>,
    pub has_uv_transform: bool,
}
impl PartialEq for GlToonDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glToonPrelude.ts:54 (sha256:246dc6c529144431c519bf28ad1b7b38a1b7d40a7d85e35ea777ffeaebb1748c)
#[derive(Clone, Default)]
pub struct GlToonProgram {
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
    pub loc_base_color: Option<crate::OpaqueHostValue>,
    pub loc_base_color_map: Option<crate::OpaqueHostValue>,
    pub loc_ramp: Option<crate::OpaqueHostValue>,
    pub loc_steps: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlToonProgram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glToonPrelude.ts:64 (sha256:d305dc15a30eab68e72bdff66b21449834455270214064f94cced37437243c7d)
pub fn build_gl_toon_define_key(key: &GlToonDefineKey) -> String {
    return format!(
        "{}{}{}{}{}",
        if key.alpha_mask_enabled {
            "m".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_base_color_map {
            "b".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_ramp {
            "r".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_uv_transform {
            "u".to_owned()
        } else {
            "-".to_owned()
        },
        if (key.has_skin).unwrap_or(false) {
            "k".to_owned()
        } else {
            "-".to_owned()
        }
    );
}

// Source: upstream/packages/scene-gl/src/glToonPrelude.ts:75 (sha256:4cc7bfe24b231946b8d5333060070ac358aba7cabe96ad41b3ee5d4a876a2147)
pub fn compile_gl_toon_program(gl: crate::OpaqueHostValue, key: &GlToonDefineKey) -> GlToonProgram {
    let program = compile_gl_program(
        (gl).clone(),
        get_gl_toon_vertex_source_for_key(key),
        get_gl_toon_fragment_source_for_key(key),
    );
    return {
        let __flight_spread_0 = resolve_gl_lit_locations((gl).clone(), (program).clone());
        GlToonProgram {
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
            loc_base_color: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_base_color_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_ramp: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_steps: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
        }
    };
}

// Source: upstream/packages/scene-gl/src/glToonPrelude.ts:95 (sha256:fb1ce3582c38c95587f5b1a744cc4ceae736b6dd033b70595648093556f74b68)
pub fn ensure_gl_toon_program(state: &mut GlRenderState, key: &GlToonDefineKey) -> GlToonProgram {
    let full_key: GlToonDefineKey = GlToonDefineKey {
        has_skin: Some(get_gl_scene_runtime(state).active_skinned_run),
        ..((*key).clone()).clone()
    };
    return ensure_gl_scene_program(
        state,
        format!("toon:{}", build_gl_toon_define_key(&full_key)),
        &mut |gl: crate::OpaqueHostValue| -> GlToonProgram {
            compile_gl_toon_program((gl).clone(), &full_key)
        },
    );
}

// Source: upstream/packages/scene-gl/src/glToonPrelude.ts:108 (sha256:ce91b3ebe827490240dc6998416a7b9229ad5b883a597670c1a5cb80f5567a41)
pub fn get_gl_toon_fragment_source_for_key(key: &GlToonDefineKey) -> String {
    return (build_gl_toon_define_source(key) + TOON_FRAGMENT_BODY);
}

// Source: upstream/packages/scene-gl/src/glToonPrelude.ts:113 (sha256:ce2a7f381aec3b1a5e7a5d6065747449d8f87bae208a1ae60153c727c7d5e4de)
pub fn get_gl_toon_vertex_source_for_key(key: &GlToonDefineKey) -> String {
    let skin = if (key.has_skin).unwrap_or(false) {
        (gl_skin_vertex_declarations_glsl_constant).to_owned()
    } else {
        "".to_owned()
    };
    return ((build_gl_toon_define_source(key) + skin) + TOON_VERTEX_BODY);
}

// Source: upstream/packages/scene-gl/src/glToonPrelude.ts:121 (sha256:1edb1f65a9dbfd27c9e064f7fe0a1bf7ff03fc202d3c15eaf1ca565b812c6385)
fn build_gl_toon_define_source(key: &GlToonDefineKey) -> String {
    let mut defines = format!(
        "#version 300 es\n#define MAX_FORWARD_LIGHTS {}\n",
        max_forward_lights_constant
    );
    if key.alpha_mask_enabled {
        defines += "#define ALPHA_MASK\n".to_owned();
    }
    if key.has_base_color_map {
        defines += "#define HAS_BASE_COLOR_MAP\n".to_owned();
    }
    if key.has_ramp {
        defines += "#define HAS_RAMP\n".to_owned();
    }
    if key.has_uv_transform {
        defines += "#define HAS_UV_TRANSFORM\n".to_owned();
    }
    if (key.has_skin).unwrap_or(false) {
        defines += "#define HAS_SKIN\n".to_owned();
    }
    return defines;
}

// Source: upstream/packages/scene-gl/src/glToonPrelude.ts:131 (sha256:2fe768a5cbaef81baf387e2deb1dcead63b2969543f9362d1587fd248d437bfa)
static TOON_VERTEX_BODY: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "\nlayout(location = 0) in vec3 a_position;\nlayout(location = 1) in vec3 a_normal;\nlayout(location = 3) in vec2 a_uv0;\n\nuniform mat4 u_viewProjection;\nuniform mat4 u_model;\nuniform mat3 u_normalMatrix;\n{}\nout vec3 v_worldPosition;\nout vec3 v_normal;\nout vec2 v_uv0;\n\nvoid main() {{\n#ifdef HAS_SKIN\n  mat4 skin = skinMatrix();\n  vec4 localPosition = skin * vec4(a_position, 1.0);\n  vec3 localNormal = mat3(skin) * a_normal;\n#else\n  vec4 localPosition = vec4(a_position, 1.0);\n  vec3 localNormal = a_normal;\n#endif\n  vec4 worldPosition = u_model * localPosition;\n  v_worldPosition = worldPosition.xyz;\n  v_normal = u_normalMatrix * localNormal;\n  v_uv0 = applyUvTransform(a_uv0);\n  gl_Position = u_viewProjection * worldPosition;\n}}\n",
        gl_uv_transform_vertex_glsl_constant
    )
});

// Source: upstream/packages/scene-gl/src/glToonPrelude.ts:161 (sha256:a308b502c47cf7164c112d08f4abdc06dd5cf167f5966553bef07930f9c5f89b)
static TOON_FRAGMENT_BODY: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "\nprecision highp float;\n\nin vec3 v_worldPosition;\nin vec3 v_normal;\nin vec2 v_uv0;\n\nuniform vec4 u_baseColor;   // already linear (decoded on the CPU at bind)\nuniform float u_steps;      // band count for the stepped floor quantizer (no ramp)\nuniform float u_alphaCutoff;\n{}\n#ifdef HAS_BASE_COLOR_MAP\nuniform sampler2D u_baseColorMap;\n#endif\n#ifdef HAS_RAMP\nuniform sampler2D u_ramp;\n#endif\n\nuniform float u_objectAlpha;\n\nout vec4 fragColor;\n\n// sRgb albedo texels are gamma-encoded; decode to linear before lighting. u_baseColor is already\n// linear (decoded on the CPU at bind), so only sampled textures need decoding.\nvec3 srgbToLinear(vec3 c) {{\n  vec3 lo = c / 12.92;\n  vec3 hi = pow((c + 0.055) / 1.055, vec3(2.4));\n  return mix(lo, hi, step(0.04045, c));\n}}\n\nvoid main() {{\n  vec4 baseColor = u_baseColor;\n#ifdef HAS_BASE_COLOR_MAP\n  vec4 sampled = texture(u_baseColorMap, v_uv0);\n  baseColor.rgb *= srgbToLinear(sampled.rgb);\n  baseColor.a *= sampled.a;\n#endif\n\n#ifdef ALPHA_MASK\n  if (baseColor.a < u_alphaCutoff) discard;\n#endif\n\n  vec3 normal = normalize(v_normal);\n  if (!gl_FrontFacing) normal = -normal;\n\n  vec3 radiance = vec3(0.0);\n\n  // Directional light: -direction is the surface-to-light vector (light travels along direction).\n  // The raw N·L is quantized into cel bands — via a 1D ramp lookup or a stepped floor — then scales\n  // the base color and the directional radiance. The banded contribution is shadow-mapped like the\n  // classic/PBR directional term; sampleDirectionalShadow is 1.0 when no shadow map is bound, so a toon\n  // scene that never calls drawGlSceneShadowMap is unchanged.\n  if (u_directionalCount > 0.5) {{\n    vec3 lightDir = normalize(-u_directional.xyz);\n    float nDotL = clamp(dot(normal, lightDir), 0.0, 1.0);\n#ifdef HAS_RAMP\n    vec3 band = texture(u_ramp, vec2(nDotL, 0.5)).rgb;\n    vec3 direct = baseColor.rgb * band * u_directionalRadiance.rgb;\n#else\n    float band = floor(nDotL * u_steps) / max(u_steps, 1.0);\n    vec3 direct = baseColor.rgb * band * u_directionalRadiance.rgb;\n#endif\n    radiance += direct * sampleDirectionalShadow(v_worldPosition);\n  }}\n\n  // Ambient term: flat irradiance over the base color (unbanded).\n  if (u_ambientCount > 0.5) {{\n    radiance += baseColor.rgb * u_ambientRadiance;\n  }}\n\n  fragColor = vec4(radiance, baseColor.a);\n  fragColor.a *= u_objectAlpha;\n}}\n",
        gl_mesh_light_block_glsl_constant
    )
});
