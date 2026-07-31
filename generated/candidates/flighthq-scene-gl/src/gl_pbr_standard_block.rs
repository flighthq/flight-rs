// @generated from upstream/packages/scene-gl/src/glPbrStandardBlock.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlPbrDefineKey, GlPbrProgram, bind_gl_uv_transform};
use flighthq_color::{LinearColor, unpack_color_to_linear};
use flighthq_image::has_image_resource_pixels;
use flighthq_render_gl::bind_gl_image_resource_texture;
use flighthq_texture::has_texture_uv_transform;
use flighthq_types::{
    BlendMode, DisplayObjectClipHooks, GlRenderState, ImageResource, LinearColor, Matrix, Sampler,
    SceneGraphSyncPolicy, SceneResourceRef, StandardPbrMaterialProperties, Texture,
    TextureColorSpace, TextureFilter, TextureWrap, Vector2,
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

// Source: upstream/packages/scene-gl/src/glPbrStandardBlock.ts:15 (sha256:a6ada2a09ddedb9065e5b10c81d3fbf8aeb558b7fd0017c5055c41dfbcaa3daf)
pub const GL_PBR_BASE_COLOR_TEXTURE_UNIT: f64 = 0.0_f64;

// Source: upstream/packages/scene-gl/src/glPbrStandardBlock.ts:16 (sha256:9d44d3d24c1e0f90bef3b7b6fea0c52f4549ee1862262e8e3442eb3caee3de0f)
pub const GL_PBR_NORMAL_TEXTURE_UNIT: f64 = 1.0_f64;

// Source: upstream/packages/scene-gl/src/glPbrStandardBlock.ts:17 (sha256:e83984d78630266d83d2d0d870597252caf1a6f39e54a5ce93b492dc7d830e6a)
pub const GL_PBR_METALLIC_ROUGHNESS_TEXTURE_UNIT: f64 = 2.0_f64;

// Source: upstream/packages/scene-gl/src/glPbrStandardBlock.ts:18 (sha256:68018b7e986f3a40b02d65c773a7bf124b2c8268df0d1ae2b685a8cbbdc18877)
pub const GL_PBR_OCCLUSION_TEXTURE_UNIT: f64 = 3.0_f64;

// Source: upstream/packages/scene-gl/src/glPbrStandardBlock.ts:19 (sha256:59d640560193ac9a7a67138610e4ba916156b219ca3a2305b44e025afeeae42c)
pub const GL_PBR_EMISSIVE_TEXTURE_UNIT: f64 = 4.0_f64;

// Source: upstream/packages/scene-gl/src/glPbrStandardBlock.ts:23 (sha256:7d7dbe83c5d38fc376b9db3b1ffcca2a27a5034d0c6a3c6cbf8012fc35850613)
pub const GL_PBR_EXTENSION_TEXTURE_UNIT: f64 = 5.0_f64;

// Source: upstream/packages/scene-gl/src/glPbrStandardBlock.ts:34 (sha256:a8c34ffbfe63a45ba76da857695929b4c07f66ed1b0a21b8a98d0ba1baf1dac3)
pub fn bind_gl_pbr_standard_block(
    state: &GlRenderState,
    program: &mut GlPbrProgram,
    standard: Option<StandardPbrMaterialProperties>,
) -> () {
    let gl = (state.gl).clone();
    if (standard).is_none() {
        crate::host_value::<()>("host.uniform4f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform3f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform1f");
        return;
    }
    unpack_color_to_linear(&mut SCRATCH_RGBA, standard.as_ref().unwrap().base_color);
    crate::host_value::<()>("host.uniform4f");
    crate::host_value::<()>("host.uniform1f");
    crate::host_value::<()>("host.uniform1f");
    crate::host_value::<()>("host.uniform1f");
    crate::host_value::<()>("host.uniform1f");
    unpack_color_to_linear(&mut SCRATCH_RGBA, standard.as_ref().unwrap().emissive);
    crate::host_value::<()>("host.uniform3f");
    crate::host_value::<()>("host.uniform1f");
    bind_gl_pbr_standard_texture(
        state,
        ((standard.as_ref().unwrap().base_color_map).clone()).clone(),
        ((program.loc_base_color_map).clone()).clone(),
        GL_PBR_BASE_COLOR_TEXTURE_UNIT,
    );
    bind_gl_pbr_standard_texture(
        state,
        ((standard.as_ref().unwrap().normal_map).clone()).clone(),
        ((program.loc_normal_map).clone()).clone(),
        GL_PBR_NORMAL_TEXTURE_UNIT,
    );
    bind_gl_pbr_standard_texture(
        state,
        ((standard.as_ref().unwrap().metallic_roughness_map).clone()).clone(),
        ((program.loc_metallic_roughness_map).clone()).clone(),
        GL_PBR_METALLIC_ROUGHNESS_TEXTURE_UNIT,
    );
    bind_gl_pbr_standard_texture(
        state,
        ((standard.as_ref().unwrap().occlusion_map).clone()).clone(),
        ((program.loc_occlusion_map).clone()).clone(),
        GL_PBR_OCCLUSION_TEXTURE_UNIT,
    );
    bind_gl_pbr_standard_texture(
        state,
        ((standard.as_ref().unwrap().emissive_map).clone()).clone(),
        ((program.loc_emissive_map).clone()).clone(),
        GL_PBR_EMISSIVE_TEXTURE_UNIT,
    );
    bind_gl_uv_transform(
        (gl).clone(),
        program,
        ((standard.as_ref().unwrap().base_color_map).clone()).clone(),
    );
}

// Source: upstream/packages/scene-gl/src/glPbrStandardBlock.ts:80 (sha256:d2ef3481be54eedec0484cc40df385e075284d690008ffab2b796739f53f6b19)
pub fn bind_gl_pbr_standard_texture(
    state: &GlRenderState,
    texture: Option<Texture>,
    location: Option<crate::OpaqueHostValue>,
    unit: f64,
) -> () {
    if (!is_gl_texture_ready(((texture).clone()).clone())) {
        return;
    }
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.activeTexture");
    bind_gl_image_resource_texture(
        state,
        texture.as_ref().unwrap().image.as_ref().unwrap(),
        Some(((texture.as_ref().unwrap().sampler).clone()).clone()),
    );
    crate::host_value::<()>("host.uniform1i");
}

// Source: upstream/packages/scene-gl/src/glPbrStandardBlock.ts:98 (sha256:1f0643233477b4fe2373ab6e3374908f2ba4fb4224d03dd2e40363c7cdb45830)
pub fn build_gl_pbr_standard_define_key(
    standard: Option<StandardPbrMaterialProperties>,
    alpha_mask_enabled: bool,
) -> GlPbrDefineKey {
    let base_color_map = standard
        .as_ref()
        .and_then(|value| (value.base_color_map).clone());
    return GlPbrDefineKey {
        __flight_identity: std::sync::Arc::new(()),
        alpha_mask_enabled: alpha_mask_enabled,
        anisotropy_enabled: false,
        clearcoat_enabled: false,
        has_base_color_map: is_gl_texture_ready(((base_color_map).clone()).clone()),
        has_emissive_map: is_gl_texture_ready(
            (standard
                .as_ref()
                .and_then(|value| (value.emissive_map).clone()))
            .clone(),
        ),
        has_metallic_roughness_map: is_gl_texture_ready(
            (standard
                .as_ref()
                .and_then(|value| (value.metallic_roughness_map).clone()))
            .clone(),
        ),
        has_normal_map: is_gl_texture_ready(
            (standard
                .as_ref()
                .and_then(|value| (value.normal_map).clone()))
            .clone(),
        ),
        has_occlusion_map: is_gl_texture_ready(
            (standard
                .as_ref()
                .and_then(|value| (value.occlusion_map).clone()))
            .clone(),
        ),
        has_uv_transform: (((base_color_map).is_some())
            && (is_gl_texture_ready(((base_color_map).clone()).clone())))
            && (has_texture_uv_transform(&base_color_map)),
        iridescence_enabled: false,
        sheen_enabled: false,
        specular_enabled: false,
        subsurface_enabled: false,
        transmission_enabled: false,
        has_skin: None,
    };
}

// Source: upstream/packages/scene-gl/src/glPbrStandardBlock.ts:126 (sha256:cd3647beb4b73808b93028defc1ab8e1757cd6071dd39f5a05314e5fd4b41675)
pub fn is_gl_texture_ready(texture: Option<Texture>) -> bool {
    return (((texture).is_some()) && (((texture.as_ref().unwrap().image).clone()).is_some()))
        && (has_image_resource_pixels(texture.as_ref().unwrap().image.as_ref().unwrap()));
}

// Source: upstream/packages/scene-gl/src/glPbrStandardBlock.ts:130 (sha256:33cbdb0c15208a5943cbbf7c6e6dbffb13fa7a74ea8e21fc4e9ff194027c4ad2)
static SCRATCH_RGBA: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));
