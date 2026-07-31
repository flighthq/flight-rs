// @generated from upstream/packages/scene-gl/src/glClassicPrelude.ts; do not edit.
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

// Source: upstream/packages/scene-gl/src/glClassicPrelude.ts:37 (sha256:da8c86a67466de62e68689a19c47b1ae7903c06c2335c6065e75d25da995a406)
pub type GlClassicLightingModel = String;

// Source: upstream/packages/scene-gl/src/glClassicPrelude.ts:43 (sha256:cc1c36349b3c951029903530183281aca7cf55cdd1097e2a0bd86a5d9a346c6d)
#[derive(Clone, Default)]
pub struct GlClassicDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mask_enabled: bool,
    pub has_diffuse_map: bool,
    pub has_normal_map: bool,
    pub has_skin: Option<bool>,
    pub has_specular_map: bool,
    pub has_uv_transform: bool,
    pub lighting_model: GlClassicLightingModel,
}
impl PartialEq for GlClassicDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glClassicPrelude.ts:63 (sha256:3480ef045fbd827bee969bd874202c450e667aad075d6e0f62a450311b93a3c3)
#[derive(Clone, Default)]
pub struct GlClassicProgram {
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
    pub loc_diffuse: Option<crate::OpaqueHostValue>,
    pub loc_diffuse_map: Option<crate::OpaqueHostValue>,
    pub loc_normal_map: Option<crate::OpaqueHostValue>,
    pub loc_normal_scale: Option<crate::OpaqueHostValue>,
    pub loc_shininess: Option<crate::OpaqueHostValue>,
    pub loc_specular: Option<crate::OpaqueHostValue>,
    pub loc_specular_map: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlClassicProgram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glClassicPrelude.ts:78 (sha256:40e1e705f50e4c8ce06c7923ec72619c0f7d4ac00c36aa7cc281bae5b10cad79)
pub fn build_gl_classic_define_key(key: &GlClassicDefineKey) -> String {
    let model = if ((key.lighting_model).clone() == "phong") {
        "p".to_owned()
    } else {
        if ((key.lighting_model).clone() == "blinnphong") {
            "b".to_owned()
        } else {
            "l".to_owned()
        }
    };
    return format!(
        "{}{}{}{}{}{}{}",
        model,
        if key.alpha_mask_enabled {
            "m".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_diffuse_map {
            "d".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_specular_map {
            "s".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_normal_map {
            "n".to_owned()
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

// Source: upstream/packages/scene-gl/src/glClassicPrelude.ts:88 (sha256:0a4a366dcf5c5fc6828681c271cf13c139e728b474916a7c706d27cd40b3cf26)
pub fn compile_gl_classic_program(
    gl: crate::OpaqueHostValue,
    key: &GlClassicDefineKey,
) -> GlClassicProgram {
    let vertex_source = get_gl_classic_vertex_source_for_key(key);
    let fragment_source = get_gl_classic_fragment_source_for_key(key);
    let program = compile_gl_program(
        (gl).clone(),
        (vertex_source).clone(),
        (fragment_source).clone(),
    );
    return {
        let __flight_spread_0 = resolve_gl_lit_locations((gl).clone(), (program).clone());
        GlClassicProgram {
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
            loc_diffuse: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_diffuse_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_normal_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_normal_scale: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_shininess: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_specular: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_specular_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
        }
    };
}

// Source: upstream/packages/scene-gl/src/glClassicPrelude.ts:116 (sha256:b66d4ac9331b01a1f71e315160f8d2e45d529eb53baaf75c73084c0ff52e9307)
pub fn ensure_gl_classic_program(
    state: &mut GlRenderState,
    key: &GlClassicDefineKey,
) -> GlClassicProgram {
    let full_key: GlClassicDefineKey = GlClassicDefineKey {
        has_skin: Some(get_gl_scene_runtime(state).active_skinned_run),
        ..((*key).clone()).clone()
    };
    return ensure_gl_scene_program(
        state,
        format!("classic:{}", build_gl_classic_define_key(&full_key)),
        &mut |gl: crate::OpaqueHostValue| -> GlClassicProgram {
            compile_gl_classic_program((gl).clone(), &full_key)
        },
    );
}

// Source: upstream/packages/scene-gl/src/glClassicPrelude.ts:131 (sha256:0b8f0d33c0c624e0e8540f9db7f3e0ef32f5b7e4073d5072ef0488d05c8b6ba1)
pub fn get_gl_classic_fragment_source() -> String {
    return ((CLASSIC_FRAGMENT_BODY).clone()).to_owned();
}

// Source: upstream/packages/scene-gl/src/glClassicPrelude.ts:136 (sha256:238e6226752cf9815690a9fff7bef00ec86cee0d9b87b341345792f8858575e2)
pub fn get_gl_classic_fragment_source_for_key(key: &GlClassicDefineKey) -> String {
    return (build_gl_classic_define_source(key) + CLASSIC_FRAGMENT_BODY);
}

// Source: upstream/packages/scene-gl/src/glClassicPrelude.ts:144 (sha256:6b3b13d4ecfab4910c8c3d90895166f9fef3cd613a5447cf78e4f806f0a4e837)
pub fn get_gl_classic_vertex_source() -> String {
    return ((CLASSIC_VERTEX_BODY).clone()).to_owned();
}

// Source: upstream/packages/scene-gl/src/glClassicPrelude.ts:151 (sha256:8670e09de8e7a231b3868ed3dd1faa83ba22273e519c7fb7001d59ce15f7b246)
pub fn get_gl_classic_vertex_source_for_key(key: &GlClassicDefineKey) -> String {
    let skin = if (key.has_skin).unwrap_or(false) {
        (gl_skin_vertex_declarations_glsl_constant).to_owned()
    } else {
        "".to_owned()
    };
    return ((build_gl_classic_define_source(key) + skin) + CLASSIC_VERTEX_BODY);
}

// Source: upstream/packages/scene-gl/src/glClassicPrelude.ts:159 (sha256:dfb0f401695d0d26339e70a224976e0ae48df61fa176f243e97efaa7615ccc75)
fn build_gl_classic_define_source(key: &GlClassicDefineKey) -> String {
    let mut defines = format!(
        "#version 300 es\n#define MAX_FORWARD_LIGHTS {}\n",
        max_forward_lights_constant
    );
    if ((key.lighting_model).clone() == "phong") {
        defines += "#define LIGHTING_PHONG\n".to_owned();
    }
    if ((key.lighting_model).clone() == "blinnphong") {
        defines += "#define LIGHTING_BLINNPHONG\n".to_owned();
    }
    if key.alpha_mask_enabled {
        defines += "#define ALPHA_MASK\n".to_owned();
    }
    if key.has_diffuse_map {
        defines += "#define HAS_DIFFUSE_MAP\n".to_owned();
    }
    if key.has_specular_map {
        defines += "#define HAS_SPECULAR_MAP\n".to_owned();
    }
    if key.has_normal_map {
        defines += "#define HAS_NORMAL_MAP\n".to_owned();
    }
    if key.has_uv_transform {
        defines += "#define HAS_UV_TRANSFORM\n".to_owned();
    }
    if (key.has_skin).unwrap_or(false) {
        defines += "#define HAS_SKIN\n".to_owned();
    }
    return defines;
}

// Source: upstream/packages/scene-gl/src/glClassicPrelude.ts:172 (sha256:e76e4dc4c78c049504712f6c0947f019c7df8a771fa86e45e82ef929d02a0272)
static CLASSIC_VERTEX_BODY: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "\nlayout(location = 0) in vec3 a_position;\nlayout(location = 1) in vec3 a_normal;\nlayout(location = 2) in vec4 a_tangent;\nlayout(location = 3) in vec2 a_uv0;\n\nuniform mat4 u_viewProjection;\nuniform mat4 u_model;\nuniform mat3 u_normalMatrix;\n{}\nout vec3 v_worldPosition;\nout vec3 v_normal;\nout vec4 v_tangent;\nout vec2 v_uv0;\n\nvoid main() {{\n#ifdef HAS_SKIN\n  mat4 skin = skinMatrix();\n  vec4 localPosition = skin * vec4(a_position, 1.0);\n  vec3 localNormal = mat3(skin) * a_normal;\n  vec3 localTangent = mat3(skin) * a_tangent.xyz;\n#else\n  vec4 localPosition = vec4(a_position, 1.0);\n  vec3 localNormal = a_normal;\n  vec3 localTangent = a_tangent.xyz;\n#endif\n  vec4 worldPosition = u_model * localPosition;\n  v_worldPosition = worldPosition.xyz;\n  v_normal = u_normalMatrix * localNormal;\n  v_tangent = vec4(u_normalMatrix * localTangent, a_tangent.w);\n  v_uv0 = applyUvTransform(a_uv0);\n  gl_Position = u_viewProjection * worldPosition;\n}}\n",
        gl_uv_transform_vertex_glsl_constant
    )
});

// Source: upstream/packages/scene-gl/src/glClassicPrelude.ts:210 (sha256:3054420c77a1a5456088040facb0da5feb7c11b25776b1833c93510001a43ec5)
static CLASSIC_FRAGMENT_BODY: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "\nprecision highp float;\n\nin vec3 v_worldPosition;\nin vec3 v_normal;\nin vec4 v_tangent;\nin vec2 v_uv0;\n\nuniform vec4 u_diffuse;\nuniform float u_alphaCutoff;\n{}\n#if defined(LIGHTING_PHONG) || defined(LIGHTING_BLINNPHONG)\nuniform vec4 u_specular;\nuniform float u_shininess;\nuniform float u_normalScale;\n#endif\n\n#ifdef HAS_DIFFUSE_MAP\nuniform sampler2D u_diffuseMap;\n#endif\n#ifdef HAS_SPECULAR_MAP\nuniform sampler2D u_specularMap;\n#endif\n#ifdef HAS_NORMAL_MAP\nuniform sampler2D u_normalMap;\n#endif\n\nuniform float u_objectAlpha;\n\nout vec4 fragColor;\n\n// sRgb albedo texels are gamma-encoded; decode to linear before lighting.\nvec3 srgbToLinear(vec3 c) {{\n  vec3 lo = c / 12.92;\n  vec3 hi = pow((c + 0.055) / 1.055, vec3(2.4));\n  return mix(lo, hi, step(0.04045, c));\n}}\n\n// The classic shading for ONE light: Lambert diffuse plus the optional Phong/BlinnPhong specular\n// lobe. Every light type (directional, point, spot) routes through this one BRDF so they never fork\n// the shading model — the caller supplies the surface->light direction and the light's (attenuated,\n// cone-scaled) radiance. Specular reads the view vector and material specular/shininess from globals.\nvec3 shadeClassicLight(vec3 normal, vec3 lightDir, vec3 lightColor, vec3 diffuseRgb) {{\n  float nDotL = max(dot(normal, lightDir), 0.0);\n  vec3 result = diffuseRgb * nDotL * lightColor;\n#if defined(LIGHTING_PHONG) || defined(LIGHTING_BLINNPHONG)\n  if (nDotL > 0.0) {{\n    vec3 viewDir = normalize(u_cameraPosition - v_worldPosition);\n    vec3 specularColor = u_specular.rgb;\n  #ifdef HAS_SPECULAR_MAP\n    specularColor *= srgbToLinear(texture(u_specularMap, v_uv0).rgb);\n  #endif\n  #ifdef LIGHTING_PHONG\n    // Phong: reflection-vector specular.\n    vec3 reflectDir = reflect(-lightDir, normal);\n    float specAngle = max(dot(reflectDir, viewDir), 0.0);\n  #else\n    // BlinnPhong: half-vector specular.\n    vec3 halfVec = normalize(lightDir + viewDir);\n    float specAngle = max(dot(normal, halfVec), 0.0);\n  #endif\n    float specular = pow(specAngle, max(u_shininess, 1.0));\n    result += specular * specularColor * lightColor;\n  }}\n#endif\n  return result;\n}}\n\nvoid main() {{\n  vec4 diffuse = u_diffuse;\n#ifdef HAS_DIFFUSE_MAP\n  vec4 sampledDiffuse = texture(u_diffuseMap, v_uv0);\n  diffuse.rgb *= srgbToLinear(sampledDiffuse.rgb);\n  diffuse.a *= sampledDiffuse.a;\n#endif\n\n#ifdef ALPHA_MASK\n  if (diffuse.a < u_alphaCutoff) discard;\n#endif\n\n  vec3 geometricNormal = normalize(v_normal);\n  if (!gl_FrontFacing) geometricNormal = -geometricNormal;\n\n  vec3 normal = geometricNormal;\n#ifdef HAS_NORMAL_MAP\n  vec3 tangent = normalize(v_tangent.xyz);\n  vec3 bitangent = cross(geometricNormal, tangent) * v_tangent.w;\n  vec3 tangentNormal = texture(u_normalMap, v_uv0).xyz * 2.0 - 1.0;\n  tangentNormal.xy *= u_normalScale;\n  mat3 tbn = mat3(tangent, bitangent, geometricNormal);\n  normal = normalize(tbn * tangentNormal);\n#endif\n\n  vec3 radiance = vec3(0.0);\n\n  // Directional light: -direction is the surface-to-light vector (light travels along direction).\n  // Only the directional term is shadow-mapped (mirrors the PBR path); point/spot/ambient stay unshadowed.\n  // sampleDirectionalShadow returns 1.0 when no shadow map is bound (u_shadowEnabled == 0), so a classic\n  // scene that never calls drawGlSceneShadowMap is unchanged.\n  if (u_directionalCount > 0.5) {{\n    vec3 lightDir = normalize(-u_directional.xyz);\n    radiance += shadeClassicLight(normal, lightDir, u_directionalRadiance.rgb, diffuse.rgb)\n                * sampleDirectionalShadow(v_worldPosition);\n  }}\n\n  // Point lights: surface->light direction with a smooth inverse-square range falloff.\n  for (int i = 0; i < MAX_FORWARD_LIGHTS; i++) {{\n    if (i >= u_pointCount) break;\n    vec3 toLight = u_pointLights[i * 2 + 0].xyz - v_worldPosition;\n    float dist2 = dot(toLight, toLight);\n    vec3 lightDir = toLight * inversesqrt(max(dist2, 1e-8));\n    float atten = rangeWindow(dist2, u_pointLights[i * 2 + 1].w) / max(dist2, 1e-4);\n    radiance += shadeClassicLight(normal, lightDir, u_pointLights[i * 2 + 1].rgb * atten, diffuse.rgb);\n  }}\n\n  // Spot lights: point attenuation times a smooth cone falloff between the inner/outer cosines.\n  for (int i = 0; i < MAX_FORWARD_LIGHTS; i++) {{\n    if (i >= u_spotCount) break;\n    vec3 toLight = u_spotLights[i * 4 + 0].xyz - v_worldPosition;\n    float dist2 = dot(toLight, toLight);\n    vec3 lightDir = toLight * inversesqrt(max(dist2, 1e-8));\n    float atten = rangeWindow(dist2, u_spotLights[i * 4 + 1].w) / max(dist2, 1e-4);\n    float cone = smoothstep(u_spotLights[i * 4 + 3].y, u_spotLights[i * 4 + 3].x,\n                            dot(normalize(u_spotLights[i * 4 + 2].xyz), -lightDir));\n    radiance += shadeClassicLight(normal, lightDir, u_spotLights[i * 4 + 1].rgb * atten * cone, diffuse.rgb);\n  }}\n\n  // Ambient term: flat irradiance over the diffuse albedo.\n  if (u_ambientCount > 0.5) {{\n    radiance += diffuse.rgb * u_ambientRadiance;\n  }}\n\n  // Hemisphere fill: sky/ground gradient blended by the normal's vertical component.\n  for (int i = 0; i < MAX_FORWARD_LIGHTS; i++) {{\n    if (i >= u_hemisphereCount) break;\n    float f = 0.5 + 0.5 * dot(normal, u_hemisphereLights[i * 3 + 2].xyz);\n    radiance += mix(u_hemisphereLights[i * 3 + 1].rgb, u_hemisphereLights[i * 3 + 0].rgb, f) * diffuse.rgb;\n  }}\n\n  fragColor = vec4(radiance, diffuse.a);\n  fragColor.a *= u_objectAlpha;\n}}\n",
        gl_mesh_light_block_glsl_constant
    )
});
