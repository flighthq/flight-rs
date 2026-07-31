// @generated from upstream/packages/scene-gl/src/customShaderGlMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GlMeshProgram, begin_gl_mesh_draw, compile_gl_program, draw_gl_mesh_subset,
    ensure_gl_scene_program, get_gl_scene_runtime, register_gl_mesh_material_renderer,
    set_gl_mesh_camera_position, set_gl_mesh_view_projection,
};
use flighthq_image::has_image_resource_pixels;
use flighthq_render_gl::bind_gl_image_resource_texture;
use flighthq_types::{
    BlendMode, CUSTOM_SHADER_MATERIAL_KIND as custom_shader_material_kind_constant, Camera,
    CustomShaderMaterial, DisplayObjectClipHooks, GlMeshMaterialRenderer, GlRenderState,
    ImageResource, Material, Matrix, MeshGeometry, Sampler, SceneGraphSyncPolicy, SceneLightBlock,
    SceneRenderProxy, SceneResourceRef, Texture, TextureColorSpace, TextureFilter, TextureWrap,
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

// Source: upstream/packages/scene-gl/src/customShaderGlMeshMaterialRenderer.ts:31 (sha256:a42f5cccd06ccfde1e126b27d20be607852f9b63029078a8938b6187bf1c0ee8)
#[derive(Clone, Default)]
struct GlCustomShaderProgram {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loc_object_alpha: Option<crate::OpaqueHostValue>,
    pub loc_joint_texture: Option<crate::OpaqueHostValue>,
    pub loc_model: Option<crate::OpaqueHostValue>,
    pub loc_normal_matrix: Option<crate::OpaqueHostValue>,
    pub loc_uv_transform: Option<crate::OpaqueHostValue>,
    pub loc_view_projection: Option<crate::OpaqueHostValue>,
    pub program: crate::OpaqueHostValue,
    pub loc_camera_position: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlCustomShaderProgram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/customShaderGlMeshMaterialRenderer.ts:41 (sha256:dd63d97ff9e5c7c53acff66d255c6d386df932be540b060e0f3ef7febf7c9509)
pub static CUSTOM_SHADER_GL_MESH_MATERIAL_RENDERER: std::sync::LazyLock<GlMeshMaterialRenderer> =
    std::sync::LazyLock::new(|| GlMeshMaterialRenderer {
        __flight_identity: std::sync::Arc::new(()),
        bind: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut state: GlRenderState,
                  material: Option<Material>,
                  _lights: SceneLightBlock,
                  camera: Camera|
                  -> () {
                let custom = material;
                if ((custom).is_none()) || ((custom.as_ref().unwrap().shader_key).clone() == "") {
                    get_gl_scene_runtime(&mut state).active_mesh_program = None;
                    return;
                }
                let source = get_gl_custom_material_shader_source(
                    &state,
                    (custom.as_ref().unwrap().shader_key).clone(),
                );
                if (source).is_none() {
                    get_gl_scene_runtime(&mut state).active_mesh_program = None;
                    return;
                }
                let program = ensure_gl_custom_shader_program(
                    &mut state,
                    (custom.as_ref().unwrap().shader_key).clone(),
                    (source.as_ref().unwrap()).clone(),
                );
                {
                    let __flight_callback =
                        (get_gl_scene_runtime(&mut state).custom_shader_guard).clone();
                    __flight_callback.as_ref().map(|callback| {
                        callback.lock().unwrap()(
                            (state).clone(),
                            (program.program).clone(),
                            (custom.as_ref().unwrap().shader_key).clone(),
                        )
                    })
                };
                begin_gl_mesh_draw(
                    &mut state,
                    &{
                        let __flight_source = &(program);
                        GlMeshProgram {
                            __flight_identity: std::sync::Arc::clone(
                                &__flight_source.__flight_identity,
                            ),
                            loc_object_alpha: (__flight_source.loc_object_alpha).clone(),
                            loc_joint_texture: (__flight_source.loc_joint_texture).clone(),
                            loc_model: (__flight_source.loc_model).clone(),
                            loc_normal_matrix: (__flight_source.loc_normal_matrix).clone(),
                            loc_uv_transform: (__flight_source.loc_uv_transform).clone(),
                            loc_view_projection: (__flight_source.loc_view_projection).clone(),
                            program: (__flight_source.program).clone(),
                        }
                    },
                    custom.as_ref().unwrap().double_sided,
                );
                set_gl_mesh_view_projection(
                    (state.gl).clone(),
                    ((program.loc_view_projection).clone()).clone(),
                    &camera,
                );
                set_gl_mesh_camera_position(
                    (state.gl).clone(),
                    ((program.loc_camera_position).clone()).clone(),
                    &camera,
                );
                upload_custom_shader_material_uniforms(
                    (state.gl).clone(),
                    (program.program).clone(),
                    custom.as_ref().unwrap(),
                );
                upload_custom_shader_material_textures(
                    &state,
                    (program.program).clone(),
                    custom.as_ref().unwrap(),
                );
            },
        )
            as Box<
                dyn FnMut(GlRenderState, Option<Material>, SceneLightBlock, Camera) -> ()
                    + Send
                    + 'static,
            >)),
        draw: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut state: GlRenderState,
                  proxy: SceneRenderProxy,
                  mut geometry: MeshGeometry|
                  -> () {
                let mut program = (get_gl_scene_runtime(&mut state).active_mesh_program).clone();
                if (program).is_none() {
                    return;
                }
                draw_gl_mesh_subset(
                    &mut state,
                    &mut program.as_mut().unwrap(),
                    &proxy,
                    &mut geometry,
                );
            },
        )
            as Box<
                dyn FnMut(GlRenderState, SceneRenderProxy, MeshGeometry) -> () + Send + 'static,
            >)),
    });

// Source: upstream/packages/scene-gl/src/customShaderGlMeshMaterialRenderer.ts:79 (sha256:92654ea017db7f0f469c932c791d74eff9e7b600cc208a77d1a056479cac9c8f)
pub fn get_gl_custom_material_shader_source(
    state: &GlRenderState,
    shader_key: String,
) -> Option<GlCustomMaterialShaderSource> {
    return (*_CUSTOM_MATERIAL_SHADERS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone())
        .as_mut()
        .unwrap()
        .iter()
        .find(|(key, _)| key == &(shader_key).clone())
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/scene-gl/src/customShaderGlMeshMaterialRenderer.ts:89 (sha256:c50abd5020cbbbb3a6f6b3b917fe13395ddb276160442df78d78f046b0d42d8a)
pub fn register_custom_shader_gl_material(state: &mut GlRenderState) -> () {
    register_gl_mesh_material_renderer(
        state,
        (custom_shader_material_kind_constant).to_owned(),
        &CUSTOM_SHADER_GL_MESH_MATERIAL_RENDERER,
    );
}

// Source: upstream/packages/scene-gl/src/customShaderGlMeshMaterialRenderer.ts:105 (sha256:557cfec69c4335c4c0e0398a97f33f8232116e60417b359d4802e71590a8ceae)
pub fn register_gl_custom_material_shader(
    state: &GlRenderState,
    shader_key: String,
    source: &GlCustomMaterialShaderSource,
) -> () {
    let mut registry = (*_CUSTOM_MATERIAL_SHADERS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (registry).is_none() {
        registry = Some(Vec::new());
        {
            let __flight_key = (*state).clone();
            let __flight_value = (registry).clone().unwrap();
            if let Some((_, value)) = (*_CUSTOM_MATERIAL_SHADERS.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*_CUSTOM_MATERIAL_SHADERS.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
    }
    {
        let __flight_key = (shader_key).clone();
        let __flight_value = (*source).clone();
        if let Some((_, value)) = registry
            .as_mut()
            .unwrap()
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            registry
                .as_mut()
                .unwrap()
                .push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/scene-gl/src/customShaderGlMeshMaterialRenderer.ts:119 (sha256:3b5b0c8ea32666202f6736c34615afdb80979165d23c93c59e22144094966556)
#[derive(Clone, Default)]
pub struct GlCustomMaterialShaderSource {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fragment: String,
    pub vertex: String,
}
impl PartialEq for GlCustomMaterialShaderSource {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/customShaderGlMeshMaterialRenderer.ts:124 (sha256:79a0d5878ac6311ce4f5be2b71cb5525a18133b04070c772a9df853139f2f267)
fn ensure_gl_custom_shader_program(
    state: &mut GlRenderState,
    shader_key: String,
    source: GlCustomMaterialShaderSource,
) -> GlCustomShaderProgram {
    return ensure_gl_scene_program(
        state,
        format!("custom:{}", shader_key),
        &mut |gl: crate::OpaqueHostValue| -> GlCustomShaderProgram {
            compile_gl_custom_shader_program((gl).clone(), &source)
        },
    );
}

// Source: upstream/packages/scene-gl/src/customShaderGlMeshMaterialRenderer.ts:132 (sha256:86091581dd6a83835f1eef7a6b70ce466eee0fadae5675792f8138f4d497f03d)
fn compile_gl_custom_shader_program(
    gl: crate::OpaqueHostValue,
    source: &GlCustomMaterialShaderSource,
) -> GlCustomShaderProgram {
    let linked = compile_gl_program(
        (gl).clone(),
        (source.vertex).clone(),
        (source.fragment).clone(),
    );
    return GlCustomShaderProgram {
        __flight_identity: std::sync::Arc::new(()),
        loc_camera_position: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_model: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_normal_matrix: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_view_projection: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        program: (linked).clone(),
    };
}

// Source: upstream/packages/scene-gl/src/customShaderGlMeshMaterialRenderer.ts:146 (sha256:61fbcbfd9410ee112d275197e6ea5fb0d534850e2d6d24dd3315c9968eea58f7)
fn upload_custom_shader_material_uniforms(
    gl: crate::OpaqueHostValue,
    program: crate::OpaqueHostValue,
    material: &CustomShaderMaterial,
) -> () {
    let uniforms = (material.uniforms).clone();
    if (uniforms).is_none() {
        return;
    }
    for name in (crate::host_value::<()>("host.keys")).iter().cloned() {
        let location = crate::host_value::<()>("host.getUniformLocation");
        if (location).is_none() {
            continue;
        }
        let value = uniforms
            .as_ref()
            .unwrap()
            .iter()
            .find(|(key, _)| key == &name)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone();
        if (match &(value) {
            crate::FlightUnion2::A(_) => "number",
            crate::FlightUnion2::B(value) => "object",
        } == "number")
        {
            crate::host_value::<()>("host.uniform1f");
            continue;
        }
        {
            let __switch_value = value.length;
            let __flight_case = if __switch_value == 1.0_f64 {
                0_usize
            } else if __switch_value == 2.0_f64 {
                1_usize
            } else if __switch_value == 3.0_f64 {
                2_usize
            } else if __switch_value == 4.0_f64 {
                3_usize
            } else {
                4_usize
            };
            '__flight_switch: {
                if __flight_case <= 0_usize {
                    crate::host_value::<()>("host.uniform1f");
                    break '__flight_switch;
                }
                if __flight_case <= 1_usize {
                    crate::host_value::<()>("host.uniform2fv");
                    break '__flight_switch;
                }
                if __flight_case <= 2_usize {
                    crate::host_value::<()>("host.uniform3fv");
                    break '__flight_switch;
                }
                if __flight_case <= 3_usize {
                    crate::host_value::<()>("host.uniform4fv");
                    break '__flight_switch;
                }
                if __flight_case <= 4_usize {
                    crate::host_value::<()>("host.uniform1fv");
                    break '__flight_switch;
                }
            }
        }
    }
}

// Source: upstream/packages/scene-gl/src/customShaderGlMeshMaterialRenderer.ts:181 (sha256:e861121a912ca4d27909868fd14e05821165f702d8eb501c2533c257b8624b6c)
fn upload_custom_shader_material_textures(
    state: &GlRenderState,
    program: crate::OpaqueHostValue,
    material: &CustomShaderMaterial,
) -> () {
    let textures = (material.textures).clone();
    if (textures).is_none() {
        return;
    }
    let gl = (state.gl).clone();
    let mut unit = 0.0_f64;
    for name in (crate::host_value::<()>("host.keys")).iter().cloned() {
        let texture: Texture = textures
            .as_ref()
            .unwrap()
            .iter()
            .find(|(key, _)| key == &name)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone();
        if (((texture.image).clone()).is_none())
            || (!has_image_resource_pixels(texture.image.as_ref().unwrap()))
        {
            continue;
        }
        let location = crate::host_value::<()>("host.getUniformLocation");
        if (location).is_none() {
            continue;
        }
        crate::host_value::<()>("host.activeTexture");
        bind_gl_image_resource_texture(
            state,
            texture.image.as_ref().unwrap(),
            Some(((texture.sampler).clone()).clone()),
        );
        crate::host_value::<()>("host.uniform1i");
        {
            unit += 1.0;
            unit
        };
    }
}

// Source: upstream/packages/scene-gl/src/customShaderGlMeshMaterialRenderer.ts:202 (sha256:fa0e78d9eae404e484a5c9f186db8993d9c3d32d44d8e376297f418533c6ca71)
static _CUSTOM_MATERIAL_SHADERS: std::sync::LazyLock<
    std::sync::Mutex<Vec<(GlRenderState, Vec<(String, GlCustomMaterialShaderSource)>)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
