// @generated from upstream/packages/scene-gl/src/glEnvironmentSkybox.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ensure_gl_environment_source_cube;
use flighthq_camera::get_camera_inverse_view_projection_matrix4;
use flighthq_geometry::create_matrix4;
use flighthq_render_gl::create_gl_program;
use flighthq_types::{
    BlendMode, Camera, DisplayObjectClipHooks, Environment, GlRenderState, ImageResource, Matrix,
    Matrix4, Sampler, SceneGraphSyncPolicy, SceneResourceRef, TextureColorSpace, TextureFilter,
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

// Source: upstream/packages/scene-gl/src/glEnvironmentSkybox.ts:14 (sha256:8ac997255366e1f60c0a9cbe9dd6444990fab0e88d751a9798eb942922642573)
pub fn draw_gl_environment_skybox(
    state: &mut GlRenderState,
    environment: &Environment,
    camera: &Camera,
    aspect: f64,
) -> () {
    let cube = ensure_gl_environment_source_cube(state, environment);
    if (cube).is_none() {
        return;
    }
    let gl = (state.gl).clone();
    let sky = ensure_gl_skybox(state);
    get_camera_inverse_view_projection_matrix4(
        &mut (*_INVERSE_VIEW_PROJECTION.lock().unwrap()),
        camera,
        aspect,
    );
    let prev_depth_test = crate::host_value::<bool>("host.getParameter");
    crate::host_value::<()>("host.depthMask");
    crate::host_value::<()>("host.disable");
    crate::host_value::<()>("host.disable");
    crate::host_value::<()>("host.useProgram");
    crate::host_value::<()>("host.uniformMatrix4fv");
    crate::host_value::<()>("host.uniform1f");
    crate::host_value::<()>("host.activeTexture");
    crate::host_value::<()>("host.bindTexture");
    crate::host_value::<()>("host.uniform1i");
    crate::host_value::<()>("host.bindVertexArray");
    crate::host_value::<()>("host.drawArrays");
    crate::host_value::<()>("host.bindVertexArray");
    crate::host_value::<()>("host.depthMask");
    if prev_depth_test {
        crate::host_value::<()>("host.enable");
    }
}

// Source: upstream/packages/scene-gl/src/glEnvironmentSkybox.ts:48 (sha256:48052e135b8be00282865fd63ae09cb69ca392d7e60918cef6a2d358db67c93f)
#[derive(Clone, Default)]
struct GlSkybox {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loc_env_cube: Option<crate::OpaqueHostValue>,
    pub loc_inverse_view_projection: Option<crate::OpaqueHostValue>,
    pub loc_intensity: Option<crate::OpaqueHostValue>,
    pub program: crate::OpaqueHostValue,
    pub vao: crate::OpaqueHostValue,
}
impl PartialEq for GlSkybox {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/glEnvironmentSkybox.ts:56 (sha256:075fb07a9026489d14c105019dfb6f4807174ec89ebca5e6b4ed9241bde55d4f)
fn ensure_gl_skybox(state: &GlRenderState) -> GlSkybox {
    let gl = (state.gl).clone();
    let mut sky = (*_SKYBOXES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (sky).is_some() {
        return ((sky.as_mut().unwrap()).clone()).clone();
    }
    let program = link_gl_skybox_program((gl).clone());
    let vao = crate::host_value::<()>("host.createVertexArray");
    crate::host_value::<()>("host.bindVertexArray");
    let buffer = crate::host_value::<()>("host.createBuffer");
    crate::host_value::<()>("host.bindBuffer");
    crate::host_value::<()>("host.bufferData");
    crate::host_value::<()>("host.enableVertexAttribArray");
    crate::host_value::<()>("host.vertexAttribPointer");
    crate::host_value::<()>("host.bindVertexArray");
    sky = Some(GlSkybox {
        __flight_identity: std::sync::Arc::new(()),
        loc_env_cube: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_inverse_view_projection: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_intensity: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        program: (program).clone(),
        vao: (vao).clone(),
    });
    {
        let __flight_key = (*state).clone();
        let __flight_value = (sky).clone().unwrap();
        if let Some((_, value)) = (*_SKYBOXES.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_SKYBOXES.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    return ((sky).clone().unwrap()).clone();
}

// Source: upstream/packages/scene-gl/src/glEnvironmentSkybox.ts:82 (sha256:fa72ea95621d0ca1337b58b003d8f82e39d3c03cc408c52753eaf4ae78c7bf32)
fn link_gl_skybox_program(gl: crate::OpaqueHostValue) -> crate::OpaqueHostValue {
    return create_gl_program(
        (gl).clone(),
        (SKYBOX_VERTEX).clone(),
        (SKYBOX_FRAGMENT).clone(),
        Some(("Skybox".to_owned()).clone()),
    );
}

// Source: upstream/packages/scene-gl/src/glEnvironmentSkybox.ts:86 (sha256:140c4ea9d3bbec70ea90f21b1af017737b7598c34c56eee26a85379ab0c997af)
static _INVERSE_VIEW_PROJECTION: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/scene-gl/src/glEnvironmentSkybox.ts:87 (sha256:ab8af434cb511536b9b65d28a287d306880acdf6b11f4e5ed6adaad64ee8d496)
static _QUAD: std::sync::LazyLock<Vec<f32>> = std::sync::LazyLock::new(|| {
    (vec![
        (-1.0_f64),
        (-1.0_f64),
        1.0_f64,
        (-1.0_f64),
        (-1.0_f64),
        1.0_f64,
        1.0_f64,
        1.0_f64,
    ])
    .iter()
    .map(|value| (*value) as f32)
    .collect()
});

// Source: upstream/packages/scene-gl/src/glEnvironmentSkybox.ts:88 (sha256:2d5d4b4614823ce6e54abd9a1f0b9b4d3e2f695503cb93b36fcf1af8cf64620c)
static _SKYBOXES: std::sync::LazyLock<std::sync::Mutex<Vec<(GlRenderState, GlSkybox)>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/scene-gl/src/glEnvironmentSkybox.ts:90 (sha256:6b95d7b8b2aabcb06bb16ba4f9042e515241f1d3d70caf7b75cb6ee3688d89be)
const SKYBOX_VERTEX: &'static str = "#version 300 es\nlayout(location = 0) in vec2 a_position;\nout vec2 v_ndc;\nvoid main() {\n  v_ndc = a_position;\n  // Emit at the far plane (z = w) so the backdrop sits behind every drawn fragment.\n  gl_Position = vec4(a_position, 1.0, 1.0);\n}\n";

// Source: upstream/packages/scene-gl/src/glEnvironmentSkybox.ts:100 (sha256:fc85758d202e14dbcaed304e4d10c97406f38124178cd67d927367fb8799f825)
const SKYBOX_FRAGMENT: &'static str = "#version 300 es\nprecision highp float;\nin vec2 v_ndc;\nuniform samplerCube u_envCube;\nuniform mat4 u_inverseViewProjection;\nuniform float u_intensity;\nout vec4 fragColor;\n\nvec3 srgbToLinear(vec3 c) {\n  return mix(c / 12.92, pow((c + 0.055) / 1.055, vec3(2.4)), step(0.04045, c));\n}\n\nvoid main() {\n  // Reconstruct the world-space ray through this pixel from the near- and far-plane unprojections.\n  vec4 nearW = u_inverseViewProjection * vec4(v_ndc, -1.0, 1.0);\n  vec4 farW = u_inverseViewProjection * vec4(v_ndc, 1.0, 1.0);\n  vec3 dir = normalize(farW.xyz / farW.w - nearW.xyz / nearW.w);\n  vec3 color = srgbToLinear(texture(u_envCube, dir).rgb) * u_intensity;\n  fragColor = vec4(color, 1.0);\n}\n";
