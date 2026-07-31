// @generated from upstream/packages/scene-gl/src/glWireframePrelude.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{compile_gl_program, ensure_gl_scene_program};
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

// Source: upstream/packages/scene-gl/src/glWireframePrelude.ts:14 (sha256:723f5fe4bfc922ec53fa07bd5d76eeba13cb0f311dcb212d97826acb02c08447)
#[derive(Clone, Default)]
pub struct GlWireframeProgram {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loc_object_alpha: Option<crate::OpaqueHostValue>,
    pub loc_joint_texture: Option<crate::OpaqueHostValue>,
    pub loc_model: Option<crate::OpaqueHostValue>,
    pub loc_normal_matrix: Option<crate::OpaqueHostValue>,
    pub loc_uv_transform: Option<crate::OpaqueHostValue>,
    pub loc_view_projection: Option<crate::OpaqueHostValue>,
    pub program: crate::OpaqueHostValue,
    pub loc_color: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlWireframeProgram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glWireframePrelude.ts:20 (sha256:c12d012b8a4c3572589857960ecd6bb58defcc24ffffddd2d794a59baba56b58)
pub fn compile_gl_wireframe_program(gl: crate::OpaqueHostValue) -> GlWireframeProgram {
    let program = compile_gl_program(
        (gl).clone(),
        get_gl_wireframe_vertex_source(),
        get_gl_wireframe_fragment_source(),
    );
    return GlWireframeProgram {
        __flight_identity: std::sync::Arc::new(()),
        loc_color: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_model: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_normal_matrix: None,
        loc_view_projection: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        program: (program).clone(),
    };
}

// Source: upstream/packages/scene-gl/src/glWireframePrelude.ts:33 (sha256:35e6064bf4ffc0fb60adbad1650e427151201f8933526b99d9e8f7829bb16aba)
pub fn ensure_gl_wireframe_program(state: &mut GlRenderState) -> GlWireframeProgram {
    return ensure_gl_scene_program(
        state,
        "wireframe:".to_owned(),
        &mut |gl: crate::OpaqueHostValue| -> GlWireframeProgram {
            compile_gl_wireframe_program((gl).clone())
        },
    );
}

// Source: upstream/packages/scene-gl/src/glWireframePrelude.ts:38 (sha256:5ed5305196cf97659638147890d9ce216ef81df5477b54382adf437ed92db0a2)
pub fn get_gl_wireframe_fragment_source() -> String {
    return ((WIREFRAME_FRAGMENT).clone()).to_owned();
}

// Source: upstream/packages/scene-gl/src/glWireframePrelude.ts:43 (sha256:b393f1a5bc2efafcfbf87f4e807ece4035f518f3de36a90775e6ab80756ab688)
pub fn get_gl_wireframe_vertex_source() -> String {
    return ((WIREFRAME_VERTEX).clone()).to_owned();
}

// Source: upstream/packages/scene-gl/src/glWireframePrelude.ts:47 (sha256:cb7ba1f739452a029661f882fd2e904f498740b1d2fa623db9673c5183d527d3)
const WIREFRAME_VERTEX: &'static str = "#version 300 es\nlayout(location = 0) in vec3 a_position;\n\nuniform mat4 u_viewProjection;\nuniform mat4 u_model;\n\nvoid main() {\n  gl_Position = u_viewProjection * u_model * vec4(a_position, 1.0);\n}\n";

// Source: upstream/packages/scene-gl/src/glWireframePrelude.ts:58 (sha256:24220d1c1f601afb63bbd97647631e56c53989e6608fe2c7634a3b673b5a4df2)
const WIREFRAME_FRAGMENT: &'static str = "#version 300 es\nprecision highp float;\n\nuniform vec4 u_color;\n\nuniform float u_objectAlpha;\n\nout vec4 fragColor;\n\nvoid main() {\n  fragColor = u_color;\n  fragColor.a *= u_objectAlpha;\n}\n";
