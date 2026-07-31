// @generated from upstream/packages/scene-gl/src/emissiveGlMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GlMeshProgram, GlUnlitDefineKey, begin_gl_mesh_draw, bind_gl_unlit_surface,
    bind_gl_uv_transform, draw_gl_mesh_subset, ensure_gl_unlit_program, get_gl_scene_runtime,
    has_gl_uv_transform, register_gl_mesh_material_renderer, set_gl_mesh_view_projection,
};
use flighthq_color::{LinearColor, unpack_color_to_linear};
use flighthq_types::{
    BlendMode, Camera, DisplayObjectClipHooks,
    EMISSIVE_MATERIAL_KIND as emissive_material_kind_constant, EmissiveMaterial,
    GlMeshMaterialRenderer, GlRenderState, ImageResource, LinearColor, Material, Matrix,
    MeshGeometry, Sampler, SceneGraphSyncPolicy, SceneLightBlock, SceneRenderProxy,
    SceneResourceRef, TextureColorSpace, TextureFilter, TextureWrap, Vector2,
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

// Source: upstream/packages/scene-gl/src/emissiveGlMeshMaterialRenderer.ts:31 (sha256:0d2d468b9dbc058130c22abfb89e089557d003f49f873defa6490dcd25dbc522)
pub static EMISSIVE_GL_MESH_MATERIAL_RENDERER: std::sync::LazyLock<GlMeshMaterialRenderer> =
    std::sync::LazyLock::new(|| GlMeshMaterialRenderer {
        __flight_identity: std::sync::Arc::new(()),
        bind: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut state: GlRenderState,
                  material: Option<Material>,
                  _lights: SceneLightBlock,
                  camera: Camera|
                  -> () {
                let gl = (state.gl).clone();
                let emissive = material;
                let mut program = ensure_gl_unlit_program(
                    &mut state,
                    &define_key_for_material(((emissive).clone()).clone()),
                );
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
                    ((emissive).is_some()) && (emissive.as_ref().unwrap().double_sided),
                );
                set_gl_mesh_view_projection(
                    (gl).clone(),
                    ((program.loc_view_projection).clone()).clone(),
                    &camera,
                );
                if (emissive).is_none() {
                    bind_gl_unlit_surface(&state, &program, &WHITE, 1.0_f64, None, 0.5_f64);
                    return;
                }
                unpack_color_to_linear(&mut SCRATCH_RGBA, emissive.as_ref().unwrap().emissive);
                bind_gl_unlit_surface(
                    &state,
                    &program,
                    &SCRATCH_RGBA,
                    emissive.as_ref().unwrap().emissive_strength,
                    ((emissive.as_ref().unwrap().emissive_map).clone()).clone(),
                    emissive.as_ref().unwrap().alpha_cutoff,
                );
                bind_gl_uv_transform(
                    (gl).clone(),
                    &mut program,
                    ((emissive.as_ref().unwrap().emissive_map).clone()).clone(),
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

// Source: upstream/packages/scene-gl/src/emissiveGlMeshMaterialRenderer.ts:70 (sha256:68b66fb7cc8b1f0a2f3f24d3f339cad7e797d5b28f96fd20c9a65ae367559103)
pub fn register_emissive_gl_material(state: &mut GlRenderState) -> () {
    register_gl_mesh_material_renderer(
        state,
        (emissive_material_kind_constant).to_owned(),
        &EMISSIVE_GL_MESH_MATERIAL_RENDERER,
    );
}

// Source: upstream/packages/scene-gl/src/emissiveGlMeshMaterialRenderer.ts:74 (sha256:4ce5f318b107f9ffcbe28eda9f06b3affcbeb9eac246ce945128456a3807019c)
fn define_key_for_material(material: Option<EmissiveMaterial>) -> GlUnlitDefineKey {
    return GlUnlitDefineKey {
        __flight_identity: std::sync::Arc::new(()),
        alpha_mask_enabled: ((material).is_some())
            && ((material.as_ref().unwrap().alpha_mode).clone() == "mask"),
        has_color_map: (((material).is_some())
            && (((material.as_ref().unwrap().emissive_map).clone()).is_some()))
            && (((material
                .as_ref()
                .unwrap()
                .emissive_map
                .as_ref()
                .unwrap()
                .image)
                .clone())
            .is_some()),
        has_uv_transform: has_gl_uv_transform(
            (if (material).is_some() {
                (material.as_ref().unwrap().emissive_map).clone()
            } else {
                None
            })
            .clone(),
        ),
        vertex_color: false,
        has_skin: None,
    };
}

// Source: upstream/packages/scene-gl/src/emissiveGlMeshMaterialRenderer.ts:83 (sha256:33cbdb0c15208a5943cbbf7c6e6dbffb13fa7a74ea8e21fc4e9ff194027c4ad2)
static SCRATCH_RGBA: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/scene-gl/src/emissiveGlMeshMaterialRenderer.ts:84 (sha256:aff9b994c4ec1ff829784b9cbcec2e44480de30e79309b95921d6d455229dd18)
static WHITE: std::sync::LazyLock<LinearColor> =
    std::sync::LazyLock::new(|| vec![1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64]);
