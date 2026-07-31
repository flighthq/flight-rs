// @generated from upstream/packages/scene-gl/src/shadedGlMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GlLitProgram, GlMeshProgram, GlModifierBindContext, GlShadedDefineKey, GlShadedProgram,
    begin_gl_mesh_draw, bind_gl_mesh_light_block, bind_gl_uv_transform, draw_gl_mesh_subset,
    ensure_gl_shaded_program, get_gl_scene_runtime, has_gl_uv_transform,
    register_gl_mesh_material_renderer, set_gl_mesh_camera_position, set_gl_mesh_view_projection,
};
use flighthq_color::{LinearColor, unpack_color_to_linear};
use flighthq_image::has_image_resource_pixels;
use flighthq_render_gl::bind_gl_image_resource_texture;
use flighthq_shading::{ModifierRegistry, order_modifier_stack, resolve_modifier};
use flighthq_types::{
    BlendMode, Camera, DisplayObjectClipHooks, GlMeshMaterialRenderer, GlRenderState,
    ImageResource, LinearColor, Material, Matrix, MeshGeometry, Modifier,
    SHADED_MATERIAL_KIND as shaded_material_kind_constant, Sampler, SceneGraphSyncPolicy,
    SceneLightBlock, SceneRenderProxy, SceneResourceRef, ShadedMaterial, TextureColorSpace,
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

// Source: upstream/packages/scene-gl/src/shadedGlMeshMaterialRenderer.ts:44 (sha256:2e479b24610cc18babb22b26aa1652d0bb25720c853b36a9a0ff862604d2bb00)
pub static SHADED_GL_MESH_MATERIAL_RENDERER: std::sync::LazyLock<GlMeshMaterialRenderer> =
    std::sync::LazyLock::new(|| GlMeshMaterialRenderer {
        __flight_identity: std::sync::Arc::new(()),
        bind: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut state: GlRenderState,
                  material: Option<Material>,
                  lights: SceneLightBlock,
                  camera: Camera|
                  -> () {
                let gl = (state.gl).clone();
                let shaded = material;
                let mut modifiers = if (shaded).is_some() {
                    (shaded.as_ref().unwrap().modifiers).clone()
                } else {
                    ((*EMPTY_MODIFIERS).clone()).clone()
                };
                let mut program = ensure_gl_shaded_program(
                    &mut state,
                    &define_key_for_material(((shaded).clone()).clone()),
                    &mut modifiers,
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
                    ((shaded).is_some()) && (shaded.as_ref().unwrap().double_sided),
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
                bind_gl_shaded_material_uniforms(&state, &mut program, ((shaded).clone()).clone());
                crate::host_value::<()>("host.uniform1f");
                if ((modifiers.len() as f64) > 0.0_f64) {
                    bind_gl_shaded_modifiers(&mut state, &program, &mut modifiers);
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

// Source: upstream/packages/scene-gl/src/shadedGlMeshMaterialRenderer.ts:76 (sha256:da30c193b447b86ccf1f5c1f211c9fc63414f2c0ab612108aebc7ee96ddf926d)
pub fn register_shaded_gl_material(state: &mut GlRenderState) -> () {
    register_gl_mesh_material_renderer(
        state,
        (shaded_material_kind_constant).to_owned(),
        &SHADED_GL_MESH_MATERIAL_RENDERER,
    );
}

// Source: upstream/packages/scene-gl/src/shadedGlMeshMaterialRenderer.ts:85 (sha256:0c9789f1b6662c1f5ba562da798661b5f72313ff326d5a6eae990926287566e5)
fn bind_gl_shaded_modifiers(
    state: &mut GlRenderState,
    program: &GlShadedProgram,
    modifiers: &mut Vec<Modifier>,
) -> () {
    let registry: Option<ModifierRegistry> =
        (get_gl_scene_runtime(state).modifier_snippet_registry).clone();
    if (registry).is_none() {
        return;
    }
    let ordered = order_modifier_stack(modifiers);
    let next_texture_unit: std::sync::Arc<std::sync::Mutex<f64>> = std::sync::Arc::new(
        std::sync::Mutex::new((*MODIFIER_TEXTURE_UNIT_BASE.lock().unwrap()).clone()),
    );
    let mut context: GlModifierBindContext = GlModifierBindContext {
        __flight_identity: std::sync::Arc::new(()),
        acquire_modifier_texture_unit: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut next_texture_unit = next_texture_unit.clone();
            move || -> f64 {
                if ((*next_texture_unit.lock().unwrap()).clone() < MODIFIER_TEXTURE_UNIT_LIMIT) {
                    {
                        (*next_texture_unit.lock().unwrap()) += 1.0;
                        (*next_texture_unit.lock().unwrap())
                    }
                } else {
                    (-1.0_f64)
                }
            }
        })
            as Box<dyn FnMut() -> f64 + Send + 'static>)),
        index: 0.0_f64,
        program: (program.program).clone(),
        state: (*state).clone(),
    };
    {
        let mut index = 0.0_f64;
        while (index < (ordered.len() as f64)) {
            let modifier = ordered[index as usize].clone();
            let snippet = resolve_modifier(&registry.as_ref().unwrap(), (modifier.kind).clone());
            if ((snippet).is_none()) || (((snippet.as_ref().unwrap().bind).clone()).is_none()) {
                {
                    index += 1.0;
                    index
                };
                continue;
            }
            context.index = index;
            {
                let __flight_callback = snippet.as_ref().unwrap().bind.as_ref().unwrap().clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()((modifier).clone(), (context).clone());
                __flight_result
            };
            {
                index += 1.0;
                index
            };
        }
    }
}

// Source: upstream/packages/scene-gl/src/shadedGlMeshMaterialRenderer.ts:112 (sha256:475ff9592eb5b3d53405c082bcec2d2d6745398a9be5476e32986571967f4cc0)
fn bind_gl_shaded_material_uniforms(
    state: &GlRenderState,
    program: &mut GlShadedProgram,
    material: Option<ShadedMaterial>,
) -> () {
    let gl = (state.gl).clone();
    if (material).is_none() {
        crate::host_value::<()>("host.uniform4f");
        crate::host_value::<()>("host.uniform4f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform1f");
        return;
    }
    unpack_color_to_linear(&mut SCRATCH_RGBA, material.as_ref().unwrap().diffuse);
    crate::host_value::<()>("host.uniform4f");
    unpack_color_to_linear(&mut SCRATCH_RGBA, material.as_ref().unwrap().specular);
    crate::host_value::<()>("host.uniform4f");
    crate::host_value::<()>("host.uniform1f");
    crate::host_value::<()>("host.uniform1f");
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
    let specular_map = (material.as_ref().unwrap().specular_map).clone();
    if (((specular_map).is_some()) && (((specular_map.as_ref().unwrap().image).clone()).is_some()))
        && (has_image_resource_pixels(specular_map.as_ref().unwrap().image.as_ref().unwrap()))
    {
        crate::host_value::<()>("host.activeTexture");
        bind_gl_image_resource_texture(
            state,
            specular_map.as_ref().unwrap().image.as_ref().unwrap(),
            Some(((specular_map.as_ref().unwrap().sampler).clone()).clone()),
        );
        crate::host_value::<()>("host.uniform1i");
    }
    let normal_map = (material.as_ref().unwrap().normal_map).clone();
    if (((normal_map).is_some()) && (((normal_map.as_ref().unwrap().image).clone()).is_some()))
        && (has_image_resource_pixels(normal_map.as_ref().unwrap().image.as_ref().unwrap()))
    {
        crate::host_value::<()>("host.activeTexture");
        bind_gl_image_resource_texture(
            state,
            normal_map.as_ref().unwrap().image.as_ref().unwrap(),
            Some(((normal_map.as_ref().unwrap().sampler).clone()).clone()),
        );
        crate::host_value::<()>("host.uniform1i");
    }
    bind_gl_uv_transform((gl).clone(), program, (diffuse_map).clone());
}

// Source: upstream/packages/scene-gl/src/shadedGlMeshMaterialRenderer.ts:162 (sha256:b1f98b41ee24f215c1138664598e264fc6c61610743819d2a98cb98f067ad612)
fn define_key_for_material(material: Option<ShadedMaterial>) -> GlShadedDefineKey {
    return GlShadedDefineKey {
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
        has_normal_map: (((material).is_some())
            && (((material.as_ref().unwrap().normal_map).clone()).is_some()))
            && (((material
                .as_ref()
                .unwrap()
                .normal_map
                .as_ref()
                .unwrap()
                .image)
                .clone())
            .is_some()),
        has_specular_map: (((material).is_some())
            && (((material.as_ref().unwrap().specular_map).clone()).is_some()))
            && (((material
                .as_ref()
                .unwrap()
                .specular_map
                .as_ref()
                .unwrap()
                .image)
                .clone())
            .is_some()),
        has_uv_transform: has_gl_uv_transform(
            (if (material).is_some() {
                (material.as_ref().unwrap().diffuse_map).clone()
            } else {
                None
            })
            .clone(),
        ),
        has_skin: None,
    };
}

// Source: upstream/packages/scene-gl/src/shadedGlMeshMaterialRenderer.ts:176 (sha256:c67fb8c99b92f92630a7167c68bc25e4f93dfafc822db45f7f756f01631bba74)
static MODIFIER_TEXTURE_UNIT_BASE: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(3.0_f64));

// Source: upstream/packages/scene-gl/src/shadedGlMeshMaterialRenderer.ts:177 (sha256:5f00dbec01e8cb0b4e49f119899e610eba30cfbea868ac2ce9fe9c6a7a70c793)
const MODIFIER_TEXTURE_UNIT_LIMIT: f64 = 8.0_f64;

// Source: upstream/packages/scene-gl/src/shadedGlMeshMaterialRenderer.ts:178 (sha256:7647cc45234b174ba58d4f965dd197db33aa93ad75655b6213b790294d0b6303)
static EMPTY_MODIFIERS: std::sync::LazyLock<Vec<Modifier>> = std::sync::LazyLock::new(|| vec![]);

// Source: upstream/packages/scene-gl/src/shadedGlMeshMaterialRenderer.ts:179 (sha256:33cbdb0c15208a5943cbbf7c6e6dbffb13fa7a74ea8e21fc4e9ff194027c4ad2)
static SCRATCH_RGBA: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));
