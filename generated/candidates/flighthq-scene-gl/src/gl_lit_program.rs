// @generated from upstream/packages/scene-gl/src/glLitProgram.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_gl_scene_runtime;
use flighthq_types::{
    BlendMode, DisplayObjectClipHooks, GlRenderState, ImageResource, Matrix, Sampler,
    SceneGraphSyncPolicy, SceneLightBlock, SceneResourceRef, TextureColorSpace, TextureFilter,
    TextureWrap, Vector2,
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

// Source: upstream/packages/scene-gl/src/glLitProgram.ts:21 (sha256:8a65129a9984b1fe82b28e2a061874bdf64c8d0d5ecf2377a9ebb16f10208e02)
#[derive(Clone, Default)]
pub struct GlLitProgram {
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
}
impl PartialEq for GlLitProgram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glLitProgram.ts:48 (sha256:9a1b35287d8f162c495fdafbb86162e91a5f9c6f8dad57c692641a3faa442739)
const SHADOW_MAP_TEXTURE_UNIT: f64 = 8.0_f64;

// Source: upstream/packages/scene-gl/src/glLitProgram.ts:49 (sha256:097e6357aabcbe0e4bb771579b520f780372d29ef1551a14f95e2dfede08d87f)
const IBL_IRRADIANCE_TEXTURE_UNIT: f64 = 9.0_f64;

// Source: upstream/packages/scene-gl/src/glLitProgram.ts:50 (sha256:3d95bb469c80c852641a8872c263bec28b7a3b888d9cea8eefda2e8c9d4a0390)
const IBL_PREFILTERED_TEXTURE_UNIT: f64 = 10.0_f64;

// Source: upstream/packages/scene-gl/src/glLitProgram.ts:51 (sha256:1d1b914fc8e9f173701b4dd5a9b1fab93ca79debadf7344e673878863be66fe2)
const IBL_BRDF_TEXTURE_UNIT: f64 = 11.0_f64;

// Source: upstream/packages/scene-gl/src/glLitProgram.ts:53 (sha256:24b69273437c7506f6535403f4a6d4289d7eb7cd63aef98d51875284f3293376)
#[derive(Clone, Default)]
struct GlIblPlaceholders {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cube: crate::OpaqueHostValue,
    pub lut: crate::OpaqueHostValue,
}
impl PartialEq for GlIblPlaceholders {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glLitProgram.ts:58 (sha256:08d104b5a9cafd11b4fee9a2cf09130aa50054db39990bd5d31202138b6edea0)
static _IBL_PLACEHOLDERS: std::sync::LazyLock<
    std::sync::Mutex<Vec<(GlRenderState, GlIblPlaceholders)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/scene-gl/src/glLitProgram.ts:62 (sha256:460538ea4a5cf78a0d196ce4acfcf0bb157b4e3d415365caf24c60abfce17741)
static _UPLOADED_LIGHT_VERSION: std::sync::LazyLock<std::sync::Mutex<Vec<(GlLitProgram, f64)>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/scene-gl/src/glLitProgram.ts:70 (sha256:445666018abd038e37f41ed71b7fbf6c9adad13b731daaa17e904ed5ccfefbd2)
pub fn bind_gl_mesh_light_block(
    state: &mut GlRenderState,
    program: &GlLitProgram,
    lights: &SceneLightBlock,
) -> () {
    let gl = (state.gl).clone();
    if !(((*_UPLOADED_LIGHT_VERSION.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*program).clone())
        .map(|(_, value)| value.clone()))
        == Some(lights.version))
    {
        crate::host_value::<()>("host.uniform4f");
        crate::host_value::<()>("host.uniform4f");
        crate::host_value::<()>("host.uniform3f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform4fv");
        crate::host_value::<()>("host.uniform4fv");
        crate::host_value::<()>("host.uniform4fv");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.uniform1i");
        {
            let __flight_key = (*program).clone();
            let __flight_value = lights.version;
            if let Some((_, value)) = (*_UPLOADED_LIGHT_VERSION.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*_UPLOADED_LIGHT_VERSION.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
    }
    let runtime = get_gl_scene_runtime(state);
    let shadow = (runtime.shadow).clone();
    if (shadow).is_some() {
        crate::host_value::<()>("host.activeTexture");
        crate::host_value::<()>("host.bindTexture");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.uniformMatrix4fv");
        crate::host_value::<()>("host.uniform1f");
    } else {
        crate::host_value::<()>("host.uniform1f");
    }
    let ibl = (runtime.ibl).clone();
    if (ibl).is_some() {
        crate::host_value::<()>("host.activeTexture");
        crate::host_value::<()>("host.bindTexture");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.activeTexture");
        crate::host_value::<()>("host.bindTexture");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.activeTexture");
        crate::host_value::<()>("host.bindTexture");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.activeTexture");
    } else {
        let placeholders = ensure_gl_ibl_placeholders(state);
        crate::host_value::<()>("host.activeTexture");
        crate::host_value::<()>("host.bindTexture");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.activeTexture");
        crate::host_value::<()>("host.bindTexture");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.activeTexture");
        crate::host_value::<()>("host.bindTexture");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.activeTexture");
    }
}

// Source: upstream/packages/scene-gl/src/glLitProgram.ts:169 (sha256:1c012768e63de7ba17f613a0e1fe7351a021c8e85bd2bbbc1a4265962e5a1815)
fn ensure_gl_ibl_placeholders(state: &GlRenderState) -> GlIblPlaceholders {
    let mut placeholders = (*_IBL_PLACEHOLDERS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (placeholders).is_some() {
        return ((placeholders.as_mut().unwrap()).clone()).clone();
    }
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.activeTexture");
    let black: Vec<u8> = (vec![0.0_f64, 0.0_f64, 0.0_f64, 255.0_f64])
        .iter()
        .map(|value| (*value) as u8)
        .collect();
    let cube = crate::host_value::<()>("host.createTexture");
    crate::host_value::<()>("host.bindTexture");
    {
        let mut face = 0.0_f64;
        while (face < 6.0_f64) {
            crate::host_value::<()>("host.texImage2D");
            {
                face += 1.0;
                face
            };
        }
    }
    crate::host_value::<()>("host.texParameteri");
    crate::host_value::<()>("host.texParameteri");
    let lut = crate::host_value::<()>("host.createTexture");
    crate::host_value::<()>("host.bindTexture");
    crate::host_value::<()>("host.texImage2D");
    crate::host_value::<()>("host.texParameteri");
    crate::host_value::<()>("host.texParameteri");
    placeholders = Some(GlIblPlaceholders {
        __flight_identity: std::sync::Arc::new(()),
        cube: (cube).clone(),
        lut: (lut).clone(),
    });
    {
        let __flight_key = (*state).clone();
        let __flight_value = (placeholders).clone().unwrap();
        if let Some((_, value)) = (*_IBL_PLACEHOLDERS.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_IBL_PLACEHOLDERS.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    return ((placeholders).clone().unwrap()).clone();
}

// Source: upstream/packages/scene-gl/src/glLitProgram.ts:202 (sha256:f7e07d3e0f5dd73a6aab1c8225dd242bdee70d794d71e7f048702b4c4bf0befb)
pub fn resolve_gl_lit_locations(
    gl: crate::OpaqueHostValue,
    program: crate::OpaqueHostValue,
) -> GlLitProgram {
    return GlLitProgram {
        __flight_identity: std::sync::Arc::new(()),
        loc_ambient_count: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_ambient_radiance: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_camera_position: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_directional: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_directional_count: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_directional_radiance: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_hemisphere_count: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_hemisphere_lights: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_ibl_brdf: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_ibl_enabled: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_ibl_intensity: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_ibl_irradiance: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_ibl_max_mip: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_ibl_prefiltered: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_point_count: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_point_lights: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_shadow_enabled: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_shadow_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_shadow_matrix: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_spot_count: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_spot_lights: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
    };
}

// Source: upstream/packages/scene-gl/src/glLitProgram.ts:235 (sha256:1d09182294426d0acfd33c25bb969b70661dc751513a4ffcd2237beef23bbe7d)
pub const GL_MESH_LIGHT_BLOCK_GLSL: &'static str = "\nuniform vec4 u_directional;          // xyz = light travel direction (surface->light is -xyz)\nuniform vec4 u_directionalRadiance;  // rgb = linear radiance, premultiplied by intensity\nuniform vec3 u_ambientRadiance;      // linear ambient irradiance\nuniform float u_directionalCount;    // 0 or 1 — gates the directional term\nuniform float u_ambientCount;        // 0 or 1 — gates the ambient term\nuniform vec3 u_cameraPosition;       // world-space camera position for view-dependent terms\nuniform sampler2D u_shadowMap;       // directional shadow depth map\nuniform mat4 u_shadowMatrix;         // world -> shadow light-clip\nuniform float u_shadowEnabled;       // 0 or 1 — gates shadow sampling\n\n// Punctual (point/spot/hemisphere) forward-light arrays. Fixed MAX_FORWARD_LIGHTS-wide; each count\n// uniform bounds its loop. Layout matches SceneLightBlock.data (packSceneLightBlock) byte-for-byte:\n//   point[i]      = u_pointLights[i*2+0]={pos.xyz,range}, [i*2+1]={radiance.rgb,invSqrRange}\n//   spot[i]       = u_spotLights[i*4+0..1] as point, [i*4+2]={dir.xyz,_}, [i*4+3]={cosInner,cosOuter,_,_}\n//   hemisphere[i] = u_hemisphereLights[i*3+0]={sky.rgb,_}, [i*3+1]={ground.rgb,_}, [i*3+2]={up.xyz,_}\nuniform vec4 u_pointLights[MAX_FORWARD_LIGHTS * 2];\nuniform vec4 u_spotLights[MAX_FORWARD_LIGHTS * 4];\nuniform vec4 u_hemisphereLights[MAX_FORWARD_LIGHTS * 3];\nuniform int u_pointCount;\nuniform int u_spotCount;\nuniform int u_hemisphereCount;\n\n// Smooth inverse-square range window (glTF/UE4): 1 near the light, eased to 0 at the range. invSqrRange\n// is 1/range^2 (0 = infinite range, no cutoff). dist2 is the squared surface->light distance.\nfloat rangeWindow(float dist2, float invSqrRange) {\n  float factor = dist2 * invSqrRange;\n  float windowed = clamp(1.0 - factor * factor, 0.0, 1.0);\n  return windowed * windowed;\n}\n\n// Directional shadow factor at a world position: 1.0 fully lit, 0.0 fully shadowed, with 3x3 PCF.\n// Fragments outside the shadow frustum are treated as lit.\nfloat sampleDirectionalShadow(vec3 worldPos) {\n  if (u_shadowEnabled < 0.5) return 1.0;\n  vec4 clip = u_shadowMatrix * vec4(worldPos, 1.0);\n  vec3 ndc = clip.xyz / clip.w;\n  vec3 uvz = ndc * 0.5 + 0.5;\n  if (uvz.x < 0.0 || uvz.x > 1.0 || uvz.y < 0.0 || uvz.y > 1.0 || uvz.z > 1.0) return 1.0;\n  float current = uvz.z - 0.0025;\n  vec2 texel = 1.0 / vec2(textureSize(u_shadowMap, 0));\n  float sum = 0.0;\n  for (int x = -1; x <= 1; ++x) {\n    for (int y = -1; y <= 1; ++y) {\n      float closest = texture(u_shadowMap, uvz.xy + vec2(float(x), float(y)) * texel).r;\n      sum += current <= closest ? 1.0 : 0.0;\n    }\n  }\n  return sum / 9.0;\n}\n";
