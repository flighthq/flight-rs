// @generated from upstream/packages/scene-gl/src/lambertGlMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GlClassicDefineKey, GlClassicProgram, GlLitProgram, GlMeshProgram, begin_gl_mesh_draw,
    bind_gl_mesh_light_block, bind_gl_uv_transform, draw_gl_mesh_subset, ensure_gl_classic_program,
    get_gl_scene_runtime, has_gl_uv_transform, register_gl_mesh_material_renderer,
    set_gl_mesh_view_projection,
};
use flighthq_color::{LinearColor, unpack_color_to_linear};
use flighthq_image::has_image_resource_pixels;
use flighthq_render_gl::bind_gl_image_resource_texture;
use flighthq_types::{
    BlendMode, Camera, DisplayObjectClipHooks, GlMeshMaterialRenderer, GlRenderState,
    ImageResource, LAMBERT_MATERIAL_KIND as lambert_material_kind_constant, LambertMaterial,
    LinearColor, Material, Matrix, MeshGeometry, Sampler, SceneGraphSyncPolicy, SceneLightBlock,
    SceneRenderProxy, SceneResourceRef, TextureColorSpace, TextureFilter, TextureWrap, Vector2,
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

// Source: upstream/packages/scene-gl/src/lambertGlMeshMaterialRenderer.ts:37 (sha256:22bb6c4b3bade3b98e0f4cf1fc5ddb29330f8fefeba6a8303aafeec78ce16b39)
pub static LAMBERT_GL_MESH_MATERIAL_RENDERER: std::sync::LazyLock<GlMeshMaterialRenderer> =
    std::sync::LazyLock::new(|| GlMeshMaterialRenderer {
        __flight_identity: std::sync::Arc::new(()),
        bind: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut state: GlRenderState,
                  material: Option<Material>,
                  lights: SceneLightBlock,
                  camera: Camera|
                  -> () {
                let gl = (state.gl).clone();
                let lambert = material;
                let mut program = ensure_gl_classic_program(
                    &mut state,
                    &define_key_for_material(((lambert).clone()).clone()),
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
                    ((lambert).is_some()) && (lambert.as_ref().unwrap().double_sided),
                );
                set_gl_mesh_view_projection(
                    (gl).clone(),
                    ((program.loc_view_projection).clone()).clone(),
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
                bind_gl_lambert_material_uniforms(
                    &state,
                    &mut program,
                    ((lambert).clone()).clone(),
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

// Source: upstream/packages/scene-gl/src/lambertGlMeshMaterialRenderer.ts:63 (sha256:877aac0806557a6585f30eec5fa5f89a61b580c143bc5f98ff14eeaaa21f4c7a)
pub fn register_lambert_gl_material(state: &mut GlRenderState) -> () {
    register_gl_mesh_material_renderer(
        state,
        (lambert_material_kind_constant).to_owned(),
        &LAMBERT_GL_MESH_MATERIAL_RENDERER,
    );
}

// Source: upstream/packages/scene-gl/src/lambertGlMeshMaterialRenderer.ts:67 (sha256:6eb6b48429befd300b338f66dcd60b1303cd7ef62112ee27020d8e50e30cef98)
fn bind_gl_lambert_material_uniforms(
    state: &GlRenderState,
    program: &mut GlClassicProgram,
    material: Option<LambertMaterial>,
) -> () {
    let gl = (state.gl).clone();
    if (material).is_none() {
        crate::host_value::<()>("host.uniform4f");
        crate::host_value::<()>("host.uniform1f");
        return;
    }
    unpack_color_to_linear(&mut SCRATCH_RGBA, material.as_ref().unwrap().diffuse);
    crate::host_value::<()>("host.uniform4f");
    crate::host_value::<()>("host.uniform1f");
    let diffuse_map = (material.as_ref().unwrap().diffuse_map).clone();
    if (((diffuse_map).is_some()) && (((diffuse_map.as_ref().unwrap().image).clone()).is_some()))
        && (has_image_resource_pixels(diffuse_map.as_ref().unwrap().image.as_ref().unwrap()))
    {
        crate::host_value::<()>("host.activeTexture");
        bind_gl_image_resource_texture(
            state,
            diffuse_map.as_ref().unwrap().image.as_ref().unwrap(),
            Some(((diffuse_map.as_ref().unwrap().sampler).clone()).clone()),
        );
        crate::host_value::<()>("host.uniform1i");
    }
    bind_gl_uv_transform((gl).clone(), program, (diffuse_map).clone());
}

// Source: upstream/packages/scene-gl/src/lambertGlMeshMaterialRenderer.ts:94 (sha256:168d3c66d798c93dbd77ae08814181d4aedcc5128af3cdd8b1625ff811c47f82)
fn define_key_for_material(material: Option<LambertMaterial>) -> GlClassicDefineKey {
    return GlClassicDefineKey {
        __flight_identity: std::sync::Arc::new(()),
        alpha_mask_enabled: ((material).is_some())
            && ((material.as_ref().unwrap().alpha_mode).clone() == "mask"),
        has_diffuse_map: (((material).is_some())
            && (((material.as_ref().unwrap().diffuse_map).clone()).is_some()))
            && (((material
                .as_ref()
                .unwrap()
                .diffuse_map
                .as_ref()
                .unwrap()
                .image)
                .clone())
            .is_some()),
        has_normal_map: false,
        has_specular_map: false,
        has_uv_transform: has_gl_uv_transform(
            (if (material).is_some() {
                (material.as_ref().unwrap().diffuse_map).clone()
            } else {
                None
            })
            .clone(),
        ),
        lighting_model: "lambert".to_owned(),
        has_skin: None,
    };
}

// Source: upstream/packages/scene-gl/src/lambertGlMeshMaterialRenderer.ts:105 (sha256:33cbdb0c15208a5943cbbf7c6e6dbffb13fa7a74ea8e21fc4e9ff194027c4ad2)
static SCRATCH_RGBA: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));
