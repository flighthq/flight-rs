// @generated from upstream/packages/scene-gl/src/specularPbrGlMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GlLitProgram, GlMeshProgram, begin_gl_mesh_draw, bind_gl_mesh_light_block,
    bind_gl_pbr_standard_block, build_gl_pbr_standard_define_key, draw_gl_mesh_subset,
    ensure_gl_pbr_program, get_gl_scene_runtime, register_gl_mesh_material_renderer,
    set_gl_mesh_camera_position, set_gl_mesh_view_projection,
};
use flighthq_color::{LinearColor, unpack_color_to_linear};
use flighthq_types::{
    BlendMode, Camera, DisplayObjectClipHooks, GlMeshMaterialRenderer, GlRenderState,
    ImageResource, LinearColor, Material, Matrix, MeshGeometry,
    SPECULAR_PBR_MATERIAL_KIND as specular_pbr_material_kind_constant, Sampler,
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

// Source: upstream/packages/scene-gl/src/specularPbrGlMeshMaterialRenderer.ts:31 (sha256:25b718b6673bd4f854828d7f8e41b61f94797ff678f7189c6860baf660cbd936)
pub static SPECULAR_PBR_GL_MESH_MATERIAL_RENDERER: std::sync::LazyLock<GlMeshMaterialRenderer> =
    std::sync::LazyLock::new(|| GlMeshMaterialRenderer {
        __flight_identity: std::sync::Arc::new(()),
        bind: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut state: GlRenderState,
                  material: Option<Material>,
                  lights: SceneLightBlock,
                  camera: Camera|
                  -> () {
                let gl = (state.gl).clone();
                let specular = material;
                let standard = if (specular).is_some() {
                    Some((specular.as_ref().unwrap().standard).clone())
                } else {
                    None
                };
                let mut key = build_gl_pbr_standard_define_key(
                    ((standard).clone()).clone(),
                    ((specular).is_some())
                        && ((specular.as_ref().unwrap().alpha_mode).clone() == "mask"),
                );
                key.specular_enabled = true;
                let mut program = ensure_gl_pbr_program(&mut state, &key);
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
                    ((specular).is_some()) && (specular.as_ref().unwrap().double_sided),
                );
                set_gl_mesh_view_projection(
                    (gl).clone(),
                    ((program.loc_view_projection).clone()).clone(),
                    &camera,
                );
                set_gl_mesh_camera_position(
                    (gl).clone(),
                    ((program.loc_camera_position).clone()).clone(),
                    &camera,
                );
                bind_gl_mesh_light_block(
                    &mut state,
                    &{
                        let __flight_source = &(program);
                        GlLitProgram {
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
                            loc_ambient_count: (__flight_source.loc_ambient_count).clone(),
                            loc_ambient_radiance: (__flight_source.loc_ambient_radiance).clone(),
                            loc_camera_position: (__flight_source.loc_camera_position).clone(),
                            loc_directional: (__flight_source.loc_directional).clone(),
                            loc_directional_count: (__flight_source.loc_directional_count).clone(),
                            loc_directional_radiance: (__flight_source.loc_directional_radiance)
                                .clone(),
                            loc_hemisphere_count: (__flight_source.loc_hemisphere_count).clone(),
                            loc_hemisphere_lights: (__flight_source.loc_hemisphere_lights).clone(),
                            loc_ibl_brdf: (__flight_source.loc_ibl_brdf).clone(),
                            loc_ibl_enabled: (__flight_source.loc_ibl_enabled).clone(),
                            loc_ibl_intensity: (__flight_source.loc_ibl_intensity).clone(),
                            loc_ibl_irradiance: (__flight_source.loc_ibl_irradiance).clone(),
                            loc_ibl_max_mip: (__flight_source.loc_ibl_max_mip).clone(),
                            loc_ibl_prefiltered: (__flight_source.loc_ibl_prefiltered).clone(),
                            loc_point_count: (__flight_source.loc_point_count).clone(),
                            loc_point_lights: (__flight_source.loc_point_lights).clone(),
                            loc_shadow_enabled: (__flight_source.loc_shadow_enabled).clone(),
                            loc_shadow_map: (__flight_source.loc_shadow_map).clone(),
                            loc_shadow_matrix: (__flight_source.loc_shadow_matrix).clone(),
                            loc_spot_count: (__flight_source.loc_spot_count).clone(),
                            loc_spot_lights: (__flight_source.loc_spot_lights).clone(),
                        }
                    },
                    &lights,
                );
                bind_gl_pbr_standard_block(&state, &mut program, ((standard).clone()).clone());
                crate::host_value::<()>("host.uniform1f");
                if (specular).is_some() {
                    unpack_color_to_linear(
                        &mut SCRATCH_RGBA,
                        specular.as_ref().unwrap().specular_color,
                    );
                    crate::host_value::<()>("host.uniform1f");
                    crate::host_value::<()>("host.uniform3f");
                } else {
                    crate::host_value::<()>("host.uniform1f");
                    crate::host_value::<()>("host.uniform3f");
                }
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

// Source: upstream/packages/scene-gl/src/specularPbrGlMeshMaterialRenderer.ts:71 (sha256:602b2a23bd1b489f8ec52bb2a3eaba1232df54fede82ebb9319e782ece0e5e80)
pub fn register_specular_pbr_gl_material(state: &mut GlRenderState) -> () {
    register_gl_mesh_material_renderer(
        state,
        (specular_pbr_material_kind_constant).to_owned(),
        &SPECULAR_PBR_GL_MESH_MATERIAL_RENDERER,
    );
}

// Source: upstream/packages/scene-gl/src/specularPbrGlMeshMaterialRenderer.ts:75 (sha256:33cbdb0c15208a5943cbbf7c6e6dbffb13fa7a74ea8e21fc4e9ff194027c4ad2)
static SCRATCH_RGBA: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));
