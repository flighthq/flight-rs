// @generated from upstream/packages/scene-wgpu/src/wgpuEnvironmentCube.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_wgpu_scene_runtime;
use flighthq_render_wgpu::upload_wgpu_texture_image_resource;
use flighthq_types::{
    BlendMode, ColorTransform, CubeTexture, DisplayObjectClipHooks, Environment, ImageResource,
    Matrix, Sampler, SceneGraphSyncPolicy, SceneResourceRef, TextureColorSpace, TextureFilter,
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

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentCube.ts:18 (sha256:0f9e8c559a24cd49de97a9d6d8b941ef1c110a71cd54966f6d46f7da7936f2b5)
#[derive(Clone, Default)]
struct EnsureWgpuEnvironmentSourceCubeRecord5 {
    __flight_identity: std::sync::Arc<()>,
    dimension: String,
}
impl PartialEq for EnsureWgpuEnvironmentSourceCubeRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_wgpu_environment_source_cube(
    state: &mut WgpuRenderState,
    environment: &Environment,
) -> Option<crate::OpaqueHostValue> {
    let mut scene = get_wgpu_scene_runtime(state);
    if ((scene.environment_source_cube_view).clone()).is_some() {
        return (scene.environment_source_cube_view).clone();
    }
    let cube = (environment.environment).clone();
    if ((cube).is_none()) || (!has_wgpu_cube_face_pixels(cube.as_ref().unwrap())) {
        return None;
    }
    let size = cube.as_ref().unwrap().faces[0.0_f64 as usize]
        .as_ref()
        .unwrap()
        .width;
    let device = (state.device).clone();
    let texture = crate::host_value::<()>("host.createTexture");
    {
        let mut face = 0.0_f64;
        while (face < 6.0_f64) {
            upload_wgpu_texture_image_resource(
                (device).clone(),
                (texture).clone(),
                vec![0.0_f64, 0.0_f64, face],
                cube.as_ref().unwrap().faces[face as usize]
                    .as_ref()
                    .unwrap(),
            );
            {
                face += 1.0;
                face
            };
        }
    }
    let view = crate::host_value::<()>("host.createView");
    scene.environment_source_cube = Some((texture).clone());
    scene.environment_source_cube_view = Some((view).clone());
    return Some((view).clone());
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentCube.ts:54 (sha256:0b94893299da9f0161a431dbdaef46dd795de695003b613f89ce8e78b36393a0)
pub fn update_wgpu_environment_cube_face(
    state: &mut WgpuRenderState,
    face: f64,
    image: &ImageResource,
) -> bool {
    let texture = (get_wgpu_scene_runtime(state).environment_source_cube).clone();
    if (texture).is_none() {
        return false;
    }
    upload_wgpu_texture_image_resource(
        (state.device).clone(),
        (texture.as_ref().unwrap()).clone(),
        vec![0.0_f64, 0.0_f64, face],
        image,
    );
    return true;
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentCube.ts:67 (sha256:3078d4baad184b339dea4a1013beff2fc0f4a9e5bdb1c0222bb13903e03ac6ea)
fn has_wgpu_cube_face_pixels(cube: &CubeTexture) -> bool {
    {
        let mut face = 0.0_f64;
        while (face < 6.0_f64) {
            let image = cube.faces[face as usize].clone();
            if ((image).is_none())
                || ((((image.as_ref().unwrap().source).clone()).is_none())
                    && (((image.as_ref().unwrap().data).clone()).is_none()))
            {
                return false;
            }
            {
                face += 1.0;
                face
            };
        }
    }
    return true;
}

// Source: upstream/packages/scene-wgpu/src/wgpuEnvironmentCube.ts:77 (sha256:dc91f6b149a587cfe58c92ef64cbfb5b57a25bdf73bdb778ffa9727c40204fe9)
const ENVIRONMENT_CUBE_FORMAT: &'static str = "rgba8unorm";
