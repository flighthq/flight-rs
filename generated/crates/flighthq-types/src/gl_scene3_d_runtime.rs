// @generated from upstream/packages/types/src/GlScene3DRuntime.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GlMeshMaterialRenderer, GlMeshProgram, GlPbrExtensionRegistration, GlPbrTransmissionSceneColor,
    GlRenderState, GlRenderTarget, GlSkinPaletteTexture, Kind, Matrix4, Mesh, MeshGeometry,
    ModifierRegistry, PbrExtension, Scene3DLightBlock, Scene3DLightsLike, TextureColorSpace,
};

// Source: upstream/packages/types/src/GlScene3DRuntime.ts:21 (sha256:90dc2896eefb221192ca20bc54bd58a95732b1f3cae3374b147c11b71718f0e7)
#[derive(Clone, Default)]
pub struct GlScene3DShadow {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub enabled: bool,
    pub matrix: Matrix4,
    pub normal_bias_world: f64,
    pub pcf_radius: f64,
    pub shadow_bias: f64,
    pub texture: crate::OpaqueHostValue,
}
impl PartialEq for GlScene3DShadow {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlScene3DRuntime.ts:35 (sha256:60a7d244805c8bf2b3b72e2fcf4777fe83b902d5676a58e0975baa9b8cf7d52c)
#[derive(Clone, Default)]
pub struct GlScene3DIbl {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub brdf_lut: crate::OpaqueHostValue,
    pub intensity: f64,
    pub irradiance_cube: crate::OpaqueHostValue,
    pub prefiltered_cube: crate::OpaqueHostValue,
    pub prefiltered_mip_count: f64,
}
impl PartialEq for GlScene3DIbl {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlScene3DRuntime.ts:46 (sha256:4e3a499b44071f2a83f997c36a83d95dd82c343b59af35480c3a88fa1d605add)
#[derive(Clone, Default)]
pub struct GlScene3DDrawEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: f64,
    pub clip_w: f64,
    pub color_matrix: Option<crate::OpaqueHostValue>,
    pub color_scale_bias: Option<crate::OpaqueHostValue>,
    pub light_block: Scene3DLightBlock,
    pub material: crate::OpaqueHostValue,
    pub mesh: crate::OpaqueHostValue,
    pub renderer: crate::OpaqueHostValue,
    pub subset: crate::OpaqueHostValue,
    pub world_matrix: crate::OpaqueHostValue,
}
impl PartialEq for GlScene3DDrawEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlScene3DRuntime.ts:74 (sha256:4699244536c3feef6f3f739f35112b90e8382c1ec3ae5327e196af03c3d85b16)
#[derive(Clone, Default)]
pub struct GlScene3DRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub active_color_adjustment_run: bool,
    pub active_color_matrix_run: bool,
    pub active_mesh_program: Option<GlMeshProgram>,
    pub active_skinned_run: bool,
    pub blended_draw_list: Vec<GlScene3DDrawEntry>,
    pub blended_pool: Vec<GlScene3DDrawEntry>,
    pub color_space_guard:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub custom_shader_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(GlRenderState, crate::OpaqueHostValue, String) -> () + Send + 'static,
                >,
            >,
        >,
    >,
    pub deform_guard:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Mesh) -> () + Send + 'static>>>>,
    pub environment_source_cube: Option<crate::OpaqueHostValue>,
    pub environment_source_cube_color_space: TextureColorSpace,
    pub ibl: Option<GlScene3DIbl>,
    pub ibl_bake_framebuffer: Option<crate::OpaqueHostValue>,
    pub material_registry: Vec<(Kind, GlMeshMaterialRenderer)>,
    pub modifier_snippet_registry: Option<ModifierRegistry>,
    pub forward_light_selection_guard: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Scene3DLightsLike) -> () + Send + 'static>>>,
    >,
    pub opaque_draw_list: Vec<GlScene3DDrawEntry>,
    pub opaque_pool: Vec<GlScene3DDrawEntry>,
    pub pbr_extension_guard: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Vec<PbrExtension>) -> () + Send + 'static>>>,
    >,
    pub pbr_extension_registry: Vec<(Kind, GlPbrExtensionRegistration)>,
    pub pbr_extension_registry_version: f64,
    pub pbr_transmission_scene_color: Option<GlPbrTransmissionSceneColor>,
    pub program_cache: Vec<(String, GlMeshProgram)>,
    pub shadow: Option<GlScene3DShadow>,
    pub shadow_target: Option<GlRenderTarget>,
    pub skin_palette: Option<GlSkinPaletteTexture>,
    pub time: f64,
    pub upload_cache: Vec<(MeshGeometry, GlMeshUpload)>,
}
impl PartialEq for GlScene3DRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlScene3DRuntime.ts:134 (sha256:ea701c770e76279c2c1ed247f4e08cca4953589f33791d7e9964c4acbb38c508)
#[derive(Clone, Default)]
pub struct GlMeshUpload {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub index_buffer: Option<crate::OpaqueHostValue>,
    pub index_count: f64,
    pub index_type: f64,
    pub primitive_mode: f64,
    pub skin_bind_uploaded: Option<bool>,
    pub vao: crate::OpaqueHostValue,
    pub version: f64,
    pub vertex_buffer: crate::OpaqueHostValue,
}
impl PartialEq for GlMeshUpload {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
