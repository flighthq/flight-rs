// @generated from upstream/packages/scene-gl/src/glDebugPrelude.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{compile_gl_program, ensure_gl_scene_program};
use flighthq_image::has_image_resource_pixels;
use flighthq_render_gl::bind_gl_image_resource_texture;
use flighthq_types::{
    BlendMode, DisplayObjectClipHooks, GlRenderState, ImageResource, Matrix, Sampler,
    SceneGraphSyncPolicy, SceneResourceRef, Texture, TextureColorSpace, TextureFilter, TextureWrap,
    Vector2,
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

// Source: upstream/packages/scene-gl/src/glDebugPrelude.ts:28 (sha256:c1b8a63115d3c8446824e2c965a250c8d4a15932b7c28a51cc685b7457e1f203)
#[derive(Clone, Default)]
pub struct GlDebugProgram {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loc_object_alpha: Option<crate::OpaqueHostValue>,
    pub loc_joint_texture: Option<crate::OpaqueHostValue>,
    pub loc_model: Option<crate::OpaqueHostValue>,
    pub loc_normal_matrix: Option<crate::OpaqueHostValue>,
    pub loc_uv_transform: Option<crate::OpaqueHostValue>,
    pub loc_view_projection: Option<crate::OpaqueHostValue>,
    pub program: crate::OpaqueHostValue,
    pub loc_far: Option<crate::OpaqueHostValue>,
    pub loc_near: Option<crate::OpaqueHostValue>,
    pub loc_normal_map: Option<crate::OpaqueHostValue>,
    pub loc_normal_scale: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlDebugProgram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glDebugPrelude.ts:38 (sha256:5e25e2c86a4b18eaba180cd053638b4bc2f4cea008df3c8c76baeef06665984b)
#[derive(Clone, Default)]
pub struct GlDebugDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub has_normal_map: bool,
    pub mode: String,
}
impl PartialEq for GlDebugDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glDebugPrelude.ts:46 (sha256:d3ece0a30e8fa73012c854186de0e41388c0ea644961514926adc4bd9d6a3541)
pub fn bind_gl_debug_normal_map(
    state: &GlRenderState,
    program: &GlDebugProgram,
    normal_map: Option<Texture>,
    normal_scale: f64,
) -> () {
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.uniform1f");
    if (((normal_map).is_some()) && (((normal_map.as_ref().unwrap().image).clone()).is_some()))
        && (has_image_resource_pixels(normal_map.as_ref().unwrap().image.as_ref().unwrap()))
    {
        crate::host_value::<()>("host.activeTexture");
        bind_gl_image_resource_texture(
            state,
            normal_map.as_ref().unwrap().image.as_ref().unwrap(),
            Some(((normal_map.as_ref().unwrap().sampler).clone()).clone()),
        );
        crate::host_value::<()>("host.uniform1i");
    }
}

// Source: upstream/packages/scene-gl/src/glDebugPrelude.ts:65 (sha256:cca10a750a9c52420d73fc66a8a63f4ae930cae48fc69c67f368b4e5d8f62754)
pub fn bind_gl_debug_range(
    state: &GlRenderState,
    program: &GlDebugProgram,
    near: f64,
    far: f64,
) -> () {
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.uniform1f");
    crate::host_value::<()>("host.uniform1f");
}

// Source: upstream/packages/scene-gl/src/glDebugPrelude.ts:78 (sha256:19b71fef9692ea32a7d91fa76a93441f91eac6dc11a4d6f76a717cce9a4b331a)
pub fn build_gl_debug_define_key(key: &GlDebugDefineKey) -> String {
    return format!(
        "{}{}",
        if ((key.mode).clone() == "depth") {
            "d".to_owned()
        } else {
            "n".to_owned()
        },
        if key.has_normal_map {
            "m".to_owned()
        } else {
            "-".to_owned()
        }
    );
}

// Source: upstream/packages/scene-gl/src/glDebugPrelude.ts:84 (sha256:06ecf1287e67e8d9ca1e39d778e16459c7326e2de93cf81f8c5f970f0ba051c9)
pub fn compile_gl_debug_program(
    gl: crate::OpaqueHostValue,
    key: &GlDebugDefineKey,
) -> GlDebugProgram {
    let program = compile_gl_program(
        (gl).clone(),
        get_gl_debug_vertex_source_for_key(key),
        get_gl_debug_fragment_source_for_key(key),
    );
    return GlDebugProgram {
        __flight_identity: std::sync::Arc::new(()),
        loc_far: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_model: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_near: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_normal_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_normal_matrix: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_normal_scale: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_view_projection: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        program: (program).clone(),
    };
}

// Source: upstream/packages/scene-gl/src/glDebugPrelude.ts:100 (sha256:44110012dda2beabdd188bcb8db0cc4cc7f2f31993c80b725a0164bcfd994015)
pub fn ensure_gl_debug_program(state: &mut GlRenderState, key: GlDebugDefineKey) -> GlDebugProgram {
    return ensure_gl_scene_program(
        state,
        format!("debug:{}", build_gl_debug_define_key(&key)),
        &mut |gl: crate::OpaqueHostValue| -> GlDebugProgram {
            compile_gl_debug_program((gl).clone(), &key)
        },
    );
}

// Source: upstream/packages/scene-gl/src/glDebugPrelude.ts:105 (sha256:831b73d5309a3e69ba02462d2e756ae2c170dddaba591134b65f815414e91e52)
pub fn get_gl_debug_fragment_source_for_key(key: &GlDebugDefineKey) -> String {
    return (build_define_source(key) + DEBUG_FRAGMENT_BODY);
}

// Source: upstream/packages/scene-gl/src/glDebugPrelude.ts:110 (sha256:481a96657085a3d07faa4b24e25e2d98b2518fdd247fe887f1e6a6a0e91d1c71)
pub fn get_gl_debug_vertex_source_for_key(key: &GlDebugDefineKey) -> String {
    return (build_define_source(key) + DEBUG_VERTEX_BODY);
}

// Source: upstream/packages/scene-gl/src/glDebugPrelude.ts:114 (sha256:29bb5d56e377a60c582365846354a6d0d10fa826ad78bc05b40bf72b9bc5f503)
fn build_define_source(key: &GlDebugDefineKey) -> String {
    let mut defines = "#version 300 es\n";
    if ((key.mode).clone() == "depth") {
        defines += "#define DEPTH_MODE\n".to_owned();
    } else {
        defines += "#define NORMAL_MODE\n".to_owned();
    }
    if key.has_normal_map {
        defines += "#define HAS_NORMAL_MAP\n".to_owned();
    }
    return defines;
}

// Source: upstream/packages/scene-gl/src/glDebugPrelude.ts:122 (sha256:3d3058793e98c9e2e6b260565c1d0b3a9d7b569c71f843c8e553ad3fb9b4f0b9)
const DEBUG_VERTEX_BODY: &'static str = "\nlayout(location = 0) in vec3 a_position;\nlayout(location = 1) in vec3 a_normal;\nlayout(location = 2) in vec4 a_tangent;\nlayout(location = 3) in vec2 a_uv0;\n\nuniform mat4 u_viewProjection;\nuniform mat4 u_model;\nuniform mat3 u_normalMatrix;\n\nout vec3 v_worldPosition;\nout vec3 v_normal;\nout vec4 v_tangent;\nout vec2 v_uv0;\n\nvoid main() {\n  vec4 worldPosition = u_model * vec4(a_position, 1.0);\n  v_worldPosition = worldPosition.xyz;\n  v_normal = u_normalMatrix * a_normal;\n  v_tangent = vec4(u_normalMatrix * a_tangent.xyz, a_tangent.w);\n  v_uv0 = a_uv0;\n  gl_Position = u_viewProjection * worldPosition;\n}\n";

// Source: upstream/packages/scene-gl/src/glDebugPrelude.ts:147 (sha256:2e56f3b64d7ed2d6c03bdb3c53eac9ebdfbd617b52d4033af49d4baf7ae2f7cb)
const DEBUG_FRAGMENT_BODY: &'static str = "\nprecision highp float;\n\nin vec3 v_worldPosition;\nin vec3 v_normal;\nin vec4 v_tangent;\nin vec2 v_uv0;\n\n#ifdef DEPTH_MODE\nuniform float u_near;\nuniform float u_far;\n#endif\n#ifdef NORMAL_MODE\nuniform float u_normalScale;\n#ifdef HAS_NORMAL_MAP\nuniform sampler2D u_normalMap;\n#endif\n#endif\n\nuniform float u_objectAlpha;\n\nout vec4 fragColor;\n\nvoid main() {\n#ifdef DEPTH_MODE\n  // Linear view-space distance is the perspective w: 1.0 / gl_FragCoord.w == w_clip == eye distance.\n  // This is camera-agnostic (no camera near/far needed); map it across the material's [u_near, u_far]\n  // visualization window to grayscale [0, 1].\n  float eyeDepth = 1.0 / gl_FragCoord.w;\n  float d = clamp((eyeDepth - u_near) / max(u_far - u_near, 1e-6), 0.0, 1.0);\n  fragColor = vec4(vec3(d), 1.0);\n#endif\n#ifdef NORMAL_MODE\n  // Visualize the WORLD-space surface normal (the geometric normal carried through u_normalMatrix).\n  vec3 geometricNormal = normalize(v_normal);\n  if (!gl_FrontFacing) geometricNormal = -geometricNormal;\n\n  vec3 normal = geometricNormal;\n#ifdef HAS_NORMAL_MAP\n  vec3 tangent = normalize(v_tangent.xyz);\n  vec3 bitangent = cross(geometricNormal, tangent) * v_tangent.w;\n  vec3 tangentNormal = texture(u_normalMap, v_uv0).xyz * 2.0 - 1.0;\n  tangentNormal.xy *= u_normalScale;\n  mat3 tbn = mat3(tangent, bitangent, geometricNormal);\n  normal = normalize(tbn * tangentNormal);\n#endif\n\n  fragColor = vec4(normal * 0.5 + 0.5, 1.0);\n#endif\n  fragColor.a *= u_objectAlpha;\n}\n";
