// @generated from upstream/packages/types/src/GlScene3DRuntime.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Camera3D, GlMeshProgram, GlPbrTransmissionSceneColor, GlRenderState, GlSkinPaletteTexture,
    GlTextureRenderTarget, Matrix4, Mesh, MeshGeometry, Node3D, PbrExtension, Scene3DLightBlock,
    Scene3DLightsLike, Scene3DRenderProxy, Texture, TextureColorSpace,
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

// Source: upstream/packages/types/src/GlScene3DRuntime.ts:35 (sha256:46a6bf29b28caec7e116ac00ce989ad811eb9c0bd25cf5583c7c5a0d97517fef)
#[derive(Clone, Default)]
pub struct GlScene3DIbl {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub brdf_lut: crate::OpaqueHostValue,
    pub environment_source_revision: f64,
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

// Source: upstream/packages/types/src/GlScene3DRuntime.ts:49 (sha256:30dbf16e6756a05ad333cf73cde94cf0b87d7498ae71458132ab4782c6468158)
#[derive(Clone, Default)]
pub struct GlScene3DDrawEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: f64,
    pub color_matrix: Option<crate::OpaqueHostValue>,
    pub color_scale_bias: Option<crate::OpaqueHostValue>,
    pub depth: f64,
    pub light_block: Scene3DLightBlock,
    pub material: crate::OpaqueHostValue,
    pub mesh: crate::OpaqueHostValue,
    pub renderer: crate::OpaqueHostValue,
    pub sort_key: f64,
    pub subset: crate::OpaqueHostValue,
    pub world_matrix: crate::OpaqueHostValue,
}
impl PartialEq for GlScene3DDrawEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlScene3DRuntime.ts:83 (sha256:92170083a6dab3fa6ce93ae5e76f029ad9b6a6bcf2485b272da1b07e5eb98205)
#[derive(Clone)]
pub struct GlMeshSkinFeature {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bind_mesh_skin_palette: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(GlRenderState, GlMeshProgram, Scene3DRenderProxy) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub bind_shadow_skin_palette: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(GlRenderState, Vec<f32>) -> () + Send + 'static>>,
    >,
    pub vertex_declarations_glsl: String,
}
impl PartialEq for GlMeshSkinFeature {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlScene3DRuntime.ts:103 (sha256:4f966902a8c16cffca1810333366096914c37f44e70b5dfce2420a0777899752)
pub type GlScene3DPass = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(GlRenderState, Node3D, Camera3D, Scene3DLightsLike) -> () + Send + 'static>,
    >,
>;

// Source: upstream/packages/types/src/GlScene3DRuntime.ts:110 (sha256:345c44bebf3aa3013e9a1cbee2d386f42f23cc2d4f4b769a23415b194001aa6b)
#[derive(Clone, Default)]
pub struct GlScene3DRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub active_blended_run: bool,
    pub active_color_adjustment_run: bool,
    pub active_color_matrix_run: bool,
    pub active_instanced_run: bool,
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
    pub environment_source_cube_face_versions: Vec<f64>,
    pub environment_source_revision: f64,
    pub environment_source_texture: Option<Texture>,
    pub environment_source_texture_version: f64,
    pub ibl: Option<GlScene3DIbl>,
    pub ibl_bake_framebuffer: Option<crate::OpaqueHostValue>,
    pub mesh_skin_feature: Option<GlMeshSkinFeature>,
    pub forward_light_selection_guard: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Scene3DLightsLike) -> () + Send + 'static>>>,
    >,
    pub opaque_draw_list: Vec<GlScene3DDrawEntry>,
    pub opaque_pool: Vec<GlScene3DDrawEntry>,
    pub pbr_extension_guard: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Vec<PbrExtension>) -> () + Send + 'static>>>,
    >,
    pub pbr_transmission_scene_color: Option<GlPbrTransmissionSceneColor>,
    pub program_cache: Vec<(String, GlMeshProgram)>,
    pub resource_cleanups: Option<
        Vec<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(GlRenderState) -> () + Send + 'static>>>>,
    >,
    pub shadow: Option<GlScene3DShadow>,
    pub shadow_target: Option<GlTextureRenderTarget>,
    pub instance_palette: Option<GlSkinPaletteTexture>,
    pub instance_color_palette: Option<GlSkinPaletteTexture>,
    pub skin_palette: Option<GlSkinPaletteTexture>,
    pub skin_normal_palette: Option<GlSkinPaletteTexture>,
    pub time: f64,
    pub upload_cache: Vec<(MeshGeometry, GlMeshUpload)>,
}
impl PartialEq for GlScene3DRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlScene3DRuntime.ts:197 (sha256:ea701c770e76279c2c1ed247f4e08cca4953589f33791d7e9964c4acbb38c508)
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
