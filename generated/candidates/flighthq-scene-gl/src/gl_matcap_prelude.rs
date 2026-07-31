// @generated from upstream/packages/scene-gl/src/glMatcapPrelude.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{compile_gl_program, ensure_gl_scene_program};
use flighthq_color::LinearColor;
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

// Source: upstream/packages/scene-gl/src/glMatcapPrelude.ts:24 (sha256:b6cfbc67234ee6691197d4fe73fa24e393307aa6e693bd47b74d22c9679f4c51)
#[derive(Clone, Default)]
pub struct GlMatcapDefineKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_mask_enabled: bool,
    pub has_matcap: bool,
}
impl PartialEq for GlMatcapDefineKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glMatcapPrelude.ts:34 (sha256:97bd638a633ce260e33310e914fb6e0a453ebc2627245c8c8bf2e0e9838137c6)
#[derive(Clone, Default)]
pub struct GlMatcapProgram {
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
    pub loc_matcap: Option<crate::OpaqueHostValue>,
    pub loc_tint: Option<crate::OpaqueHostValue>,
    pub loc_view: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlMatcapProgram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glMatcapPrelude.ts:44 (sha256:819bdd7e1af08710a56a115f06ee4668e92ad63e53e26eb6d072ba14a0174bc9)
pub fn bind_gl_matcap_surface(
    state: &GlRenderState,
    program: &GlMatcapProgram,
    tint: LinearColor,
    matcap: Option<Texture>,
    alpha_cutoff: f64,
) -> () {
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.uniform4f");
    crate::host_value::<()>("host.uniform1f");
    if (((matcap).is_some()) && (((matcap.as_ref().unwrap().image).clone()).is_some()))
        && (has_image_resource_pixels(matcap.as_ref().unwrap().image.as_ref().unwrap()))
    {
        crate::host_value::<()>("host.activeTexture");
        bind_gl_image_resource_texture(
            state,
            matcap.as_ref().unwrap().image.as_ref().unwrap(),
            Some(((matcap.as_ref().unwrap().sampler).clone()).clone()),
        );
        crate::host_value::<()>("host.uniform1i");
    }
}

// Source: upstream/packages/scene-gl/src/glMatcapPrelude.ts:64 (sha256:f9efdd5d40d48e51f0e0bed3ea6d2d207642b7b596d4463ba1d01ee1b638dd58)
pub fn build_gl_matcap_define_key(key: &GlMatcapDefineKey) -> String {
    return format!(
        "{}{}",
        if key.alpha_mask_enabled {
            "m".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_matcap {
            "t".to_owned()
        } else {
            "-".to_owned()
        }
    );
}

// Source: upstream/packages/scene-gl/src/glMatcapPrelude.ts:70 (sha256:79ee0ed950e8903915a9fa6bfa860d5e5f158191085a2c0946ffde7f6f1c6718)
pub fn compile_gl_matcap_program(
    gl: crate::OpaqueHostValue,
    key: &GlMatcapDefineKey,
) -> GlMatcapProgram {
    let program = compile_gl_program(
        (gl).clone(),
        get_gl_matcap_vertex_source_for_key(key),
        get_gl_matcap_fragment_source_for_key(key),
    );
    return GlMatcapProgram {
        __flight_identity: std::sync::Arc::new(()),
        loc_alpha_cutoff: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_matcap: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_model: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_normal_matrix: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_tint: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_view: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_view_projection: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        program: (program).clone(),
    };
}

// Source: upstream/packages/scene-gl/src/glMatcapPrelude.ts:86 (sha256:bbb5207b0471b8407beae1d2f7437eb78e17f514693f90fc7fb24d20669f1713)
pub fn ensure_gl_matcap_program(
    state: &mut GlRenderState,
    key: GlMatcapDefineKey,
) -> GlMatcapProgram {
    return ensure_gl_scene_program(
        state,
        format!("matcap:{}", build_gl_matcap_define_key(&key)),
        &mut |gl: crate::OpaqueHostValue| -> GlMatcapProgram {
            compile_gl_matcap_program((gl).clone(), &key)
        },
    );
}

// Source: upstream/packages/scene-gl/src/glMatcapPrelude.ts:91 (sha256:5bd55499e3e048359be2422c17849d9e6eb4acf58198d5c8f1f650ac77736434)
pub fn get_gl_matcap_fragment_source_for_key(key: &GlMatcapDefineKey) -> String {
    return (build_define_source(key) + MATCAP_FRAGMENT_BODY);
}

// Source: upstream/packages/scene-gl/src/glMatcapPrelude.ts:96 (sha256:365fe62c9e0241d4dda32c79fe2362dc60c1779fb665d74c23d641377a31c4e5)
pub fn get_gl_matcap_vertex_source_for_key(key: &GlMatcapDefineKey) -> String {
    return (build_define_source(key) + MATCAP_VERTEX_BODY);
}

// Source: upstream/packages/scene-gl/src/glMatcapPrelude.ts:100 (sha256:c4405eea9e4d999e38862011c1d731eccfbfab994abaac50f999a899eecd9905)
fn build_define_source(key: &GlMatcapDefineKey) -> String {
    let mut defines = "#version 300 es\n";
    if key.alpha_mask_enabled {
        defines += "#define ALPHA_MASK\n".to_owned();
    }
    if key.has_matcap {
        defines += "#define HAS_MATCAP\n".to_owned();
    }
    return defines;
}

// Source: upstream/packages/scene-gl/src/glMatcapPrelude.ts:107 (sha256:1d7ed58c9d9b3b3d058e5bfe9f7525311195f0f02db0adca87c8638a7e40190e)
const MATCAP_VERTEX_BODY: &'static str = "\nlayout(location = 0) in vec3 a_position;\nlayout(location = 1) in vec3 a_normal;\n\nuniform mat4 u_viewProjection;\nuniform mat4 u_model;\nuniform mat4 u_view;\nuniform mat3 u_normalMatrix;\n\nout vec3 v_viewNormal;\n\nvoid main() {\n  // u_normalMatrix takes the object normal into world space (handles model rotation/scale);\n  // mat3(u_view) rotates it into view space. Normalized in the fragment stage.\n  v_viewNormal = mat3(u_view) * (u_normalMatrix * a_normal);\n  gl_Position = u_viewProjection * u_model * vec4(a_position, 1.0);\n}\n";

// Source: upstream/packages/scene-gl/src/glMatcapPrelude.ts:126 (sha256:7abdb1a5dfb7e48f87ad557a516f6cc6c65c4098304a141196b67d6df927f79a)
const MATCAP_FRAGMENT_BODY: &'static str = "\nprecision highp float;\n\nin vec3 v_viewNormal;\n\nuniform vec4 u_tint;\n#ifdef HAS_MATCAP\nuniform sampler2D u_matcap;\n#endif\n#ifdef ALPHA_MASK\nuniform float u_alphaCutoff;\n#endif\n\nuniform float u_objectAlpha;\n\nout vec4 fragColor;\n\n// sRgb texels are gamma-encoded; decode to linear before use. u_tint is already linear (decoded on\n// the CPU at bind), so only the sampled matcap needs decoding.\nvec3 srgbToLinear(vec3 c) {\n  vec3 lo = c / 12.92;\n  vec3 hi = pow((c + 0.055) / 1.055, vec3(2.4));\n  return mix(lo, hi, step(0.04045, c));\n}\n\nvoid main() {\n  vec4 color = u_tint;\n#ifdef HAS_MATCAP\n  // The view-space normal projected to 2D indexes the prebaked-lit sphere: uv = n.xy * 0.5 + 0.5.\n  vec3 viewNormal = normalize(v_viewNormal);\n  vec2 matcapUv = viewNormal.xy * 0.5 + 0.5;\n  vec4 sampled = texture(u_matcap, matcapUv);\n  color.rgb *= srgbToLinear(sampled.rgb);\n  color.a *= sampled.a;\n#endif\n#ifdef ALPHA_MASK\n  if (color.a < u_alphaCutoff) discard;\n#endif\n  fragColor = color;\n  fragColor.a *= u_objectAlpha;\n}\n";
