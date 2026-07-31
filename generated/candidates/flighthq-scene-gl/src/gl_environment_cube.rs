// @generated from upstream/packages/scene-gl/src/glEnvironmentCube.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_gl_scene_runtime;
use flighthq_render_gl::upload_gl_texture_image_resource;
use flighthq_types::{
    BlendMode, CubeTexture, DisplayObjectClipHooks, Environment, GlRenderState, ImageResource,
    Matrix, Sampler, SceneGraphSyncPolicy, SceneResourceRef, TextureColorSpace, TextureFilter,
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

// Source: upstream/packages/scene-gl/src/glEnvironmentCube.ts:15 (sha256:54c2adb5fd2a0c260eecaf1365c6f02273fc541f49f86f22085fd161c1e64df9)
pub fn ensure_gl_environment_source_cube(
    state: &mut GlRenderState,
    environment: &Environment,
) -> Option<crate::OpaqueHostValue> {
    let mut runtime = get_gl_scene_runtime(state);
    if ((runtime.environment_source_cube).clone()).is_some() {
        return (runtime.environment_source_cube).clone();
    }
    let cube = (environment.environment).clone();
    if ((cube).is_none()) || (!has_gl_cube_face_pixels(cube.as_ref().unwrap())) {
        return None;
    }
    let gl = (state.gl).clone();
    let texture = crate::host_value::<()>("host.createTexture");
    crate::host_value::<()>("host.bindTexture");
    {
        let mut face = 0.0_f64;
        while (face < 6.0_f64) {
            upload_gl_texture_image_resource(
                (gl).clone(),
                get_gl_cube_face_target((gl).clone(), face),
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
    crate::host_value::<()>("host.texParameteri");
    crate::host_value::<()>("host.texParameteri");
    crate::host_value::<()>("host.texParameteri");
    crate::host_value::<()>("host.texParameteri");
    crate::host_value::<()>("host.texParameteri");
    crate::host_value::<()>("host.bindTexture");
    runtime.environment_source_cube = Some((texture).clone());
    return Some((texture).clone());
}

// Source: upstream/packages/scene-gl/src/glEnvironmentCube.ts:44 (sha256:a483d1014f3a5a594121479abda922615c55f6c64e098d6f415a3c3af14a0f70)
pub fn get_gl_cube_face_target(gl: crate::OpaqueHostValue, face: f64) -> f64 {
    return (crate::host_value::<crate::OpaqueHostValue>("host.TEXTURE_CUBE_MAP_POSITIVE_X")
        + face);
}

// Source: upstream/packages/scene-gl/src/glEnvironmentCube.ts:54 (sha256:2af29a4f551b9033be154ea0778416722ee303587ed5083cefe63f92a22a3e0a)
pub fn update_gl_environment_cube_face(
    state: &mut GlRenderState,
    face: f64,
    image: &ImageResource,
) -> bool {
    let texture = (get_gl_scene_runtime(state).environment_source_cube).clone();
    if (texture).is_none() {
        return false;
    }
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.bindTexture");
    upload_gl_texture_image_resource(
        (gl).clone(),
        get_gl_cube_face_target((gl).clone(), face),
        image,
    );
    crate::host_value::<()>("host.bindTexture");
    return true;
}

// Source: upstream/packages/scene-gl/src/glEnvironmentCube.ts:70 (sha256:4f84a1487a8e920727ae56ba0f814754f3eb398a21b961b97713ffb899547af2)
fn has_gl_cube_face_pixels(cube: &CubeTexture) -> bool {
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
