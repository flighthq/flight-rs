// @generated from upstream/packages/scene-gl/src/glUnlitPrelude.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GL_SKIN_VERTEX_DECLARATIONS_GLSL as gl_skin_vertex_declarations_glsl_constant,
    GL_UV_TRANSFORM_VERTEX_GLSL as gl_uv_transform_vertex_glsl_constant, compile_gl_program,
    ensure_gl_scene_program, get_gl_scene_runtime,
};
use flighthq_color::LinearColor;
use flighthq_image::has_image_resource_pixels;
use flighthq_render_gl::{bind_gl_image_resource_texture, bind_gl_video_texture};
use flighthq_types::{
    BlendMode, DisplayObjectClipHooks, GlRenderState, ImageResource, Matrix, Sampler,
    SceneGraphSyncPolicy, SceneResourceRef, Texture, TextureColorSpace, TextureFilter, TextureWrap,
    Vector2, VideoTexture,
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

// Source: upstream/packages/scene-gl/src/glUnlitPrelude.ts:27 (sha256:0f2616b769eaaf92696448aaa31de31a4000b4a57a45a911ae7f2835918b3b43)
#[derive(Clone, Default)]
pub struct GlUnlitDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mask_enabled: bool,
    pub has_color_map: bool,
    pub has_skin: Option<bool>,
    pub has_uv_transform: bool,
    pub vertex_color: bool,
}
impl PartialEq for GlUnlitDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glUnlitPrelude.ts:42 (sha256:b6dabb6a67295f3fde8a10576757919722c6c2769695c3ba5f9d2e71fb817f78)
#[derive(Clone, Default)]
pub struct GlUnlitProgram {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loc_object_alpha: Option<crate::OpaqueHostValue>,
    pub loc_joint_texture: Option<crate::OpaqueHostValue>,
    pub loc_model: Option<crate::OpaqueHostValue>,
    pub loc_normal_matrix: Option<crate::OpaqueHostValue>,
    pub loc_uv_transform: Option<crate::OpaqueHostValue>,
    pub loc_view_projection: Option<crate::OpaqueHostValue>,
    pub program: crate::OpaqueHostValue,
    pub loc_alpha_cutoff: Option<crate::OpaqueHostValue>,
    pub loc_color: Option<crate::OpaqueHostValue>,
    pub loc_color_map: Option<crate::OpaqueHostValue>,
    pub loc_intensity: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlUnlitProgram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glUnlitPrelude.ts:53 (sha256:ca32870d8a2529f24884fe4730072323f2333472a8e1bf7796d7701a34a46922)
pub fn bind_gl_unlit_surface(
    state: &GlRenderState,
    program: &GlUnlitProgram,
    color: LinearColor,
    intensity: f64,
    color_map: Option<Texture>,
    alpha_cutoff: f64,
) -> () {
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.uniform4f");
    crate::host_value::<()>("host.uniform1f");
    crate::host_value::<()>("host.uniform1f");
    if (((color_map).is_some()) && (((color_map.as_ref().unwrap().image).clone()).is_some()))
        && (has_image_resource_pixels(color_map.as_ref().unwrap().image.as_ref().unwrap()))
    {
        crate::host_value::<()>("host.activeTexture");
        bind_gl_image_resource_texture(
            state,
            color_map.as_ref().unwrap().image.as_ref().unwrap(),
            Some(((color_map.as_ref().unwrap().sampler).clone()).clone()),
        );
        crate::host_value::<()>("host.uniform1i");
    }
}

// Source: upstream/packages/scene-gl/src/glUnlitPrelude.ts:79 (sha256:e30c3ffb4c204f7afb367421c712b22345e8f237557a61e82e9ef997f2cfffc0)
pub fn bind_gl_unlit_video_surface(
    state: &GlRenderState,
    program: &GlUnlitProgram,
    color: LinearColor,
    intensity: f64,
    video_map: &VideoTexture,
    alpha_cutoff: f64,
) -> () {
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.uniform4f");
    crate::host_value::<()>("host.uniform1f");
    crate::host_value::<()>("host.uniform1f");
    crate::host_value::<()>("host.activeTexture");
    bind_gl_video_texture(state, video_map, None);
    crate::host_value::<()>("host.uniform1i");
}

// Source: upstream/packages/scene-gl/src/glUnlitPrelude.ts:98 (sha256:b0f222166941c10d6b98d0eab2e7326e6fb8c308dbd31ccf5750f29c8bc910e1)
pub fn build_gl_unlit_define_key(key: &GlUnlitDefineKey) -> String {
    return format!(
        "{}{}{}{}{}",
        if key.alpha_mask_enabled {
            "m".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_color_map {
            "c".to_owned()
        } else {
            "-".to_owned()
        },
        if key.vertex_color {
            "v".to_owned()
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

// Source: upstream/packages/scene-gl/src/glUnlitPrelude.ts:106 (sha256:3800522d0031b2c6d1168b1074fe5a14f18b753595e1f3e192e446b3566f5c98)
pub fn compile_gl_unlit_program(
    gl: crate::OpaqueHostValue,
    key: &GlUnlitDefineKey,
) -> GlUnlitProgram {
    let program = compile_gl_program(
        (gl).clone(),
        get_gl_unlit_vertex_source_for_key(key),
        get_gl_unlit_fragment_source_for_key(key),
    );
    return GlUnlitProgram {
        __flight_identity: std::sync::Arc::new(()),
        loc_alpha_cutoff: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_color: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_color_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_intensity: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_joint_texture: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_model: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_normal_matrix: None,
        loc_view_projection: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        program: (program).clone(),
    };
}

// Source: upstream/packages/scene-gl/src/glUnlitPrelude.ts:123 (sha256:991a6ed9713a0d625182841ea1fede45251f8de8ed66d681df2234da71f64c95)
pub fn ensure_gl_unlit_program(
    state: &mut GlRenderState,
    key: &GlUnlitDefineKey,
) -> GlUnlitProgram {
    let full_key: GlUnlitDefineKey = GlUnlitDefineKey {
        has_skin: Some(get_gl_scene_runtime(state).active_skinned_run),
        ..((*key).clone()).clone()
    };
    return ensure_gl_scene_program(
        state,
        format!("unlit:{}", build_gl_unlit_define_key(&full_key)),
        &mut |gl: crate::OpaqueHostValue| -> GlUnlitProgram {
            compile_gl_unlit_program((gl).clone(), &full_key)
        },
    );
}

// Source: upstream/packages/scene-gl/src/glUnlitPrelude.ts:136 (sha256:b4486273a9149807b50ab979aa6a13d5e9d7fca25e880da4dcbc3503e475e8aa)
pub fn get_gl_unlit_fragment_source_for_key(key: &GlUnlitDefineKey) -> String {
    return (build_define_source(key) + UNLIT_FRAGMENT_BODY);
}

// Source: upstream/packages/scene-gl/src/glUnlitPrelude.ts:141 (sha256:2e877f631aa438dedd5d02d50b9c61a78e6cdea7460e6126960b4ae0c21939cc)
pub fn get_gl_unlit_vertex_source_for_key(key: &GlUnlitDefineKey) -> String {
    return ((build_define_source(key)
        + if (key.has_skin).unwrap_or(false) {
            (gl_skin_vertex_declarations_glsl_constant).to_owned()
        } else {
            "".to_owned()
        })
        + UNLIT_VERTEX_BODY);
}

// Source: upstream/packages/scene-gl/src/glUnlitPrelude.ts:145 (sha256:798eee80c3bd04b30a0b4d55056d752d4859b74ee9520ec6ed9fd3b528c54931)
fn build_define_source(key: &GlUnlitDefineKey) -> String {
    let mut defines = "#version 300 es\n";
    if key.alpha_mask_enabled {
        defines += "#define ALPHA_MASK\n".to_owned();
    }
    if key.has_color_map {
        defines += "#define HAS_COLOR_MAP\n".to_owned();
    }
    if key.has_uv_transform {
        defines += "#define HAS_UV_TRANSFORM\n".to_owned();
    }
    if key.vertex_color {
        defines += "#define VERTEX_COLOR\n".to_owned();
    }
    if (key.has_skin).unwrap_or(false) {
        defines += "#define HAS_SKIN\n".to_owned();
    }
    return defines;
}

// Source: upstream/packages/scene-gl/src/glUnlitPrelude.ts:155 (sha256:59decd435c72f3028dca14b7ab7c621cada8d1ab59c05f06286028c94efa5e0b)
static UNLIT_VERTEX_BODY: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "\nlayout(location = 0) in vec3 a_position;\nlayout(location = 3) in vec2 a_uv0;\n#ifdef VERTEX_COLOR\nlayout(location = 4) in vec4 a_color0;\nout vec4 v_color0;\n#endif\n\nuniform mat4 u_viewProjection;\nuniform mat4 u_model;\n{}\nout vec2 v_uv0;\n\nvoid main() {{\n  v_uv0 = applyUvTransform(a_uv0);\n#ifdef VERTEX_COLOR\n  v_color0 = a_color0;\n#endif\n#ifdef HAS_SKIN\n  gl_Position = u_viewProjection * u_model * skinMatrix() * vec4(a_position, 1.0);\n#else\n  gl_Position = u_viewProjection * u_model * vec4(a_position, 1.0);\n#endif\n}}\n",
        gl_uv_transform_vertex_glsl_constant
    )
});

// Source: upstream/packages/scene-gl/src/glUnlitPrelude.ts:181 (sha256:6de7e40aebf06c825bae9efd8be332533ab57cef0ec6199f58dcac05e69b16cf)
const UNLIT_FRAGMENT_BODY: &'static str = "\nprecision highp float;\n\nin vec2 v_uv0;\n#ifdef VERTEX_COLOR\nin vec4 v_color0;\n#endif\n\nuniform vec4 u_color;\nuniform float u_intensity;\n#ifdef HAS_COLOR_MAP\nuniform sampler2D u_colorMap;\n#endif\n#ifdef ALPHA_MASK\nuniform float u_alphaCutoff;\n#endif\n\nuniform float u_objectAlpha;\n\nout vec4 fragColor;\n\n// sRgb texels are gamma-encoded; decode to linear before use. u_color is already linear (decoded on\n// the CPU at bind), so only the sampled color-map needs decoding.\nvec3 srgbToLinear(vec3 c) {\n  vec3 lo = c / 12.92;\n  vec3 hi = pow((c + 0.055) / 1.055, vec3(2.4));\n  return mix(lo, hi, step(0.04045, c));\n}\n\nvoid main() {\n  vec4 color = u_color;\n#ifdef VERTEX_COLOR\n  color *= v_color0;\n#endif\n#ifdef HAS_COLOR_MAP\n  vec4 sampled = texture(u_colorMap, v_uv0);\n  color.rgb *= srgbToLinear(sampled.rgb);\n  color.a *= sampled.a;\n#endif\n#ifdef ALPHA_MASK\n  if (color.a < u_alphaCutoff) discard;\n#endif\n  fragColor = vec4(color.rgb * u_intensity, color.a);\n  fragColor.a *= u_objectAlpha;\n}\n";
