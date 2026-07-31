// @generated from upstream/packages/scene-gl/src/depthGlMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GlDebugDefineKey, GlMeshProgram, begin_gl_mesh_draw, bind_gl_debug_range, draw_gl_mesh_subset,
    ensure_gl_debug_program, get_gl_scene_runtime, register_gl_mesh_material_renderer,
    set_gl_mesh_view_projection,
};
use flighthq_types::{
    BlendMode, Camera, DEPTH_MATERIAL_KIND as depth_material_kind_constant, DisplayObjectClipHooks,
    GlMeshMaterialRenderer, GlRenderState, ImageResource, Material, Matrix, MeshGeometry, Sampler,
    SceneGraphSyncPolicy, SceneLightBlock, SceneRenderProxy, SceneResourceRef, TextureColorSpace,
    TextureFilter, TextureWrap, Vector2,
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

// Source: upstream/packages/scene-gl/src/depthGlMeshMaterialRenderer.ts:23 (sha256:e9f8e799fdd5175a22796adbc483f0af9772e05283c29df5fa22db644dafad47)
pub static DEPTH_GL_MESH_MATERIAL_RENDERER: std::sync::LazyLock<GlMeshMaterialRenderer> =
    std::sync::LazyLock::new(|| GlMeshMaterialRenderer {
        __flight_identity: std::sync::Arc::new(()),
        bind: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut state: GlRenderState,
                  material: Option<Material>,
                  _lights: SceneLightBlock,
                  camera: Camera|
                  -> () {
                let gl = (state.gl).clone();
                let depth = material;
                let program = ensure_gl_debug_program(
                    &mut state,
                    GlDebugDefineKey {
                        __flight_identity: std::sync::Arc::new(()),
                        has_normal_map: false,
                        mode: "depth".to_owned(),
                    },
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
                    ((depth).is_some()) && (depth.as_ref().unwrap().double_sided),
                );
                set_gl_mesh_view_projection(
                    (gl).clone(),
                    ((program.loc_view_projection).clone()).clone(),
                    &camera,
                );
                if (depth).is_none() {
                    bind_gl_debug_range(&state, &program, 0.0_f64, 1.0_f64);
                    return;
                }
                bind_gl_debug_range(
                    &state,
                    &program,
                    depth.as_ref().unwrap().near,
                    depth.as_ref().unwrap().far,
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

// Source: upstream/packages/scene-gl/src/depthGlMeshMaterialRenderer.ts:52 (sha256:313ca7707ce2dee2568de2dc1cdeb301734efad804de2663d6116a8a151959b0)
pub fn register_depth_gl_material(state: &mut GlRenderState) -> () {
    register_gl_mesh_material_renderer(
        state,
        (depth_material_kind_constant).to_owned(),
        &DEPTH_GL_MESH_MATERIAL_RENDERER,
    );
}
