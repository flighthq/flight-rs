// @generated from upstream/packages/scene-wgpu/src/wgpuEnvironmentSkybox.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ensure_wgpu_environment_source_cube, get_wgpu_scene_runtime};
use flighthq_camera::get_camera_inverse_view_projection_matrix4;
use flighthq_geometry::create_matrix4;
use flighthq_render_wgpu::get_wgpu_render_state_runtime;
use flighthq_types::{
    BlendMode, Camera, ColorTransform, DisplayObjectClipHooks, Environment, ImageResource, Matrix,
    Matrix4, Sampler, SceneGraphSyncPolicy, SceneResourceRef, TextureColorSpace, TextureFilter,
    TextureWrap, Vector2, WgpuRenderState,
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

#[derive(Clone, Default)]
pub struct FlightPartialRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentSkybox.ts:17 (sha256:717d86c6a4fea6611c3e385393d7813880dd3c66a4ed0da19a7e973a205d8ff2)
pub fn draw_wgpu_environment_skybox(
    state: &mut WgpuRenderState,
    environment: &Environment,
    camera: &Camera,
    aspect: f64,
) -> () {
    let cube_view = ensure_wgpu_environment_source_cube(state, environment);
    if (cube_view).is_none() {
        return;
    }
    let state_runtime = get_wgpu_render_state_runtime(state);
    let pass = (state_runtime.inner.lock().unwrap().render_pass).clone();
    if (pass).is_none() {
        return;
    }
    let scene = get_wgpu_scene_runtime(state);
    let format = ((state_runtime.inner.lock().unwrap().current_color_format).clone())
        .unwrap_or((state.format).clone());
    let mut sky = ensure_wgpu_skybox_pipeline(state, (format).clone());
    get_camera_inverse_view_projection_matrix4(
        &mut (*_INVERSE_VIEW_PROJECTION.lock().unwrap()),
        camera,
        aspect,
    );
    {
        let mut i = 0.0_f64;
        while (i < 16.0_f64) {
            (*_SKY_SCRATCH.lock().unwrap())[i as usize] =
                ((*_INVERSE_VIEW_PROJECTION.lock().unwrap()).m[i as usize] as f64) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    (*_SKY_SCRATCH.lock().unwrap())[16.0_f64 as usize] = (environment.intensity) as f32;
    (*_SKY_SCRATCH.lock().unwrap())[17.0_f64 as usize] = (0.0_f64) as f32;
    (*_SKY_SCRATCH.lock().unwrap())[18.0_f64 as usize] = (0.0_f64) as f32;
    (*_SKY_SCRATCH.lock().unwrap())[19.0_f64 as usize] = (0.0_f64) as f32;
    crate::host_value::<()>("host.writeBuffer");
    if (((sky.cube_bind_group).clone()).is_none())
        || (!(((sky.cube_view).clone()) == Some((cube_view.as_ref().unwrap()).clone())))
    {
        sky.cube_bind_group = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createBindGroup",
        ));
        sky.cube_view = Some((cube_view.as_ref().unwrap()).clone());
    }
    crate::host_value::<()>("host.setPipeline");
    crate::host_value::<()>("host.setBindGroup");
    crate::host_value::<()>("host.setBindGroup");
    crate::host_value::<()>("host.draw");
    {
        scene;
        ()
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentSkybox.ts:64 (sha256:cf7396d5036c7f13ae949c917ed7496e70288e145962364b81d727aa132bf7ea)
#[derive(Clone, Default)]
struct WgpuSkybox {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cube_bind_group: Option<crate::OpaqueHostValue>,
    pub cube_bind_group_layout: crate::OpaqueHostValue,
    pub cube_view: Option<crate::OpaqueHostValue>,
    pub pipeline: crate::OpaqueHostValue,
    pub uniform_bind_group: crate::OpaqueHostValue,
    pub uniform_buffer: crate::OpaqueHostValue,
}
impl PartialEq for WgpuSkybox {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentSkybox.ts:78 (sha256:6c85a9feeeae91880811f48a4c35ab82919f4f342d382f36d7aac6a449383a60)
#[derive(Clone, Default)]
struct EnsureWgpuSkyboxPipelineRecord5 {
    __flight_identity: std::sync::Arc<()>,
    type_: String,
}
impl PartialEq for EnsureWgpuSkyboxPipelineRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct EnsureWgpuSkyboxPipelineRecord6 {
    __flight_identity: std::sync::Arc<()>,
    sample_type: String,
    view_dimension: String,
}
impl PartialEq for EnsureWgpuSkyboxPipelineRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct EnsureWgpuSkyboxPipelineRecord7 {
    __flight_identity: std::sync::Arc<()>,
    topology: String,
}
impl PartialEq for EnsureWgpuSkyboxPipelineRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn ensure_wgpu_skybox_pipeline(
    state: &WgpuRenderState,
    format: crate::OpaqueHostValue,
) -> WgpuSkybox {
    let mut by_state = (*_SKYBOXES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (by_state).is_none() {
        by_state = Some(Vec::new());
        {
            let __flight_key = (*state).clone();
            let __flight_value = (by_state).clone().unwrap();
            if let Some((_, value)) = (*_SKYBOXES.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*_SKYBOXES.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
    }
    let mut sky = by_state
        .as_mut()
        .unwrap()
        .iter()
        .find(|(key, _)| key == &(format).clone())
        .map(|(_, value)| value.clone());
    if (sky).is_some() {
        return ((sky.as_mut().unwrap()).clone()).clone();
    }
    let device = (state.device).clone();
    let module = crate::host_value::<()>("host.createShaderModule");
    let uniform_bind_group_layout = crate::host_value::<()>("host.createBindGroupLayout");
    let cube_bind_group_layout = crate::host_value::<()>("host.createBindGroupLayout");
    let layout = crate::host_value::<()>("host.createPipelineLayout");
    let pipeline = crate::host_value::<()>("host.createRenderPipeline");
    let uniform_buffer = crate::host_value::<()>("host.createBuffer");
    let uniform_bind_group = crate::host_value::<()>("host.createBindGroup");
    sky = Some(WgpuSkybox {
        __flight_identity: std::sync::Arc::new(()),
        cube_bind_group: None,
        cube_bind_group_layout: (cube_bind_group_layout).clone(),
        cube_view: None,
        pipeline: (pipeline).clone(),
        uniform_bind_group: (uniform_bind_group).clone(),
        uniform_buffer: (uniform_buffer).clone(),
    });
    {
        let __flight_key = (format).clone();
        let __flight_value = (sky).clone().unwrap();
        if let Some((_, value)) = by_state
            .as_mut()
            .unwrap()
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            by_state
                .as_mut()
                .unwrap()
                .push((__flight_key, __flight_value));
        }
    };
    return ((sky).clone().unwrap()).clone();
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentSkybox.ts:120 (sha256:e181f6b509653e3981567b63604abce49481e44dccf89a2113ebbf6925610287)
#[derive(Clone, Default)]
struct GetWgpuSkyboxSamplerRecord5 {
    __flight_identity: std::sync::Arc<()>,
    mag_filter: String,
    min_filter: String,
}
impl PartialEq for GetWgpuSkyboxSamplerRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn get_wgpu_skybox_sampler(state: &WgpuRenderState) -> crate::OpaqueHostValue {
    let mut sampler = (*_SKYBOX_SAMPLERS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (sampler).is_none() {
        sampler = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createSampler",
        ));
        {
            let __flight_key = (*state).clone();
            let __flight_value = (sampler).clone().unwrap();
            if let Some((_, value)) = (*_SKYBOX_SAMPLERS.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*_SKYBOX_SAMPLERS.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
    }
    return ((sampler).clone().unwrap()).clone();
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentSkybox.ts:131 (sha256:a8d4b2b53f0f92507533b103d1516a7f8f2a8375756c594ab4ef10223d3c71d5)
const SKYBOX_DEPTH_STENCIL_FORMAT: &'static str = "depth24plus-stencil8";

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentSkybox.ts:134 (sha256:9561a0dfdfe76f7a7287f9ef677cc5b835125bbfa0fffa297d81441d27513fd4)
const SKYBOX_UNIFORM_BYTES: f64 = 80.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentSkybox.ts:136 (sha256:140c4ea9d3bbec70ea90f21b1af017737b7598c34c56eee26a85379ab0c997af)
static _INVERSE_VIEW_PROJECTION: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentSkybox.ts:137 (sha256:0ff96ec9223d6c4ba70906ab5d17e6cdcfd11c636926b75a5e1635479af93346)
static _SKY_SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<f32>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![0.0_f32; (SKYBOX_UNIFORM_BYTES / 4.0_f64) as usize])
    });

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentSkybox.ts:138 (sha256:7d7c21cde6f80080cb696d8838774af83ad9549bb58f125a41ce0561dbd2ca00)
static _SKYBOXES: std::sync::LazyLock<
    std::sync::Mutex<Vec<(WgpuRenderState, Vec<(crate::OpaqueHostValue, WgpuSkybox)>)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentSkybox.ts:139 (sha256:edf6f8cfbee3f23f49380a5a470e42b1ebb1dc382948b02a475478fe7d27776a)
static _SKYBOX_SAMPLERS: std::sync::LazyLock<
    std::sync::Mutex<Vec<(WgpuRenderState, crate::OpaqueHostValue)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentSkybox.ts:141 (sha256:922e89f1cc8480643ccd8407f4a58fbeac970814eda75854c712838a6f13732a)
const SKYBOX_WGSL: &'static str = "\nstruct SkyUniform {\n  inverseViewProjection : mat4x4f,\n  params : vec4f,   // x = intensity\n};\n\n@group(0) @binding(0) var<uniform> sky : SkyUniform;\n@group(1) @binding(0) var envCube : texture_cube<f32>;\n@group(1) @binding(1) var envSampler : sampler;\n\nstruct VertexOutput {\n  @builtin(position) clipPosition : vec4f,\n  @location(0) ndc : vec2f,\n};\n\n@vertex fn vs_main(@builtin(vertex_index) vi : u32) -> VertexOutput {\n  var out : VertexOutput;\n  // Full-screen triangle from the vertex index alone (no vertex buffer).\n  let x = f32((vi & 1u) << 2u) - 1.0;\n  let y = f32((vi & 2u) << 1u) - 1.0;\n  out.ndc = vec2f(x, y);\n  // Emit at the far plane (WebGPU clip z in 0..1) so the backdrop sits at maximum depth.\n  out.clipPosition = vec4f(x, y, 1.0, 1.0);\n  return out;\n}\n\nfn srgbToLinear(c : vec3f) -> vec3f {\n  let lo = c / 12.92;\n  let hi = pow((c + vec3f(0.055)) / 1.055, vec3f(2.4));\n  return select(lo, hi, c > vec3f(0.04045));\n}\n\n@fragment fn fs_main(in : VertexOutput) -> @location(0) vec4f {\n  // Reconstruct the world-space ray through this pixel from the near- and far-plane unprojections. The\n  // projection is GL-convention (clip z in -1..1), so unproject at z = -1 (near) and z = +1 (far),\n  // matching scene-gl's skybox exactly.\n  let nearW = sky.inverseViewProjection * vec4f(in.ndc, -1.0, 1.0);\n  let farW = sky.inverseViewProjection * vec4f(in.ndc, 1.0, 1.0);\n  let dir = normalize(farW.xyz / farW.w - nearW.xyz / nearW.w);\n  let color = srgbToLinear(textureSampleLevel(envCube, envSampler, dir, 0.0).rgb) * sky.params.x;\n  return vec4f(color, 1.0);\n}\n";
