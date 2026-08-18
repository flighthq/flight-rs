// @generated from upstream/packages/types/src/WgpuScene3DRuntime.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, CustomShaderMaterial, Kind, Matrix4, ModifierRegistry, Scene3DLightsLike,
    WgpuCustomMaterialShaderSource, WgpuMeshMaterialRenderer, WgpuMeshPipeline, WgpuRenderState,
};

// Source: upstream/packages/types/src/WgpuScene3DRuntime.ts:19 (sha256:51198c8940045753f24c81e72c79afa768610dc0b4ad580609876711fd13e77f)
#[derive(Clone, Default)]
pub struct WgpuScene3DShadow {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub depth_texture: crate::OpaqueHostValue,
    pub depth_view: crate::OpaqueHostValue,
    pub enabled: bool,
    pub map_height: f64,
    pub map_width: f64,
    pub matrix: Matrix4,
    pub normal_bias_world: f64,
    pub pcf_radius: f64,
    pub shadow_bias: f64,
}
impl PartialEq for WgpuScene3DShadow {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuScene3DRuntime.ts:40 (sha256:2c9de49060c0caec1db063676aaff00b42a147d1453a572eebc9698cad804d96)
#[derive(Clone, Default)]
pub struct WgpuScene3DIbl {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub brdf_lut: crate::OpaqueHostValue,
    pub brdf_lut_view: crate::OpaqueHostValue,
    pub intensity: f64,
    pub irradiance_cube: crate::OpaqueHostValue,
    pub irradiance_cube_view: crate::OpaqueHostValue,
    pub prefiltered_cube: crate::OpaqueHostValue,
    pub prefiltered_cube_view: crate::OpaqueHostValue,
    pub prefiltered_mip_count: f64,
}
impl PartialEq for WgpuScene3DIbl {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuScene3DRuntime.ts:51 (sha256:7ae1f3611cb970dc9a5bba4681dabc3f580fff5642b9b52a0f5f557b6f864e01)
#[derive(Clone, Default)]
pub struct WgpuScene3DFrameBinding {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bind_group: crate::OpaqueHostValue,
    pub buffer: crate::OpaqueHostValue,
}
impl PartialEq for WgpuScene3DFrameBinding {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuScene3DRuntime.ts:58 (sha256:b4d8b046bbaf156d910380ce47f8e5eb9bdcdfac78735a793394fc189153168d)
#[derive(Clone, Default)]
pub struct WgpuScene3DDrawEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: f64,
    pub color_matrix: Option<crate::OpaqueHostValue>,
    pub color_scale_bias: Option<crate::OpaqueHostValue>,
    pub depth: f64,
    pub light_block: crate::OpaqueHostValue,
    pub material: crate::OpaqueHostValue,
    pub mesh: crate::OpaqueHostValue,
    pub renderer: crate::OpaqueHostValue,
    pub subset: crate::OpaqueHostValue,
    pub world_matrix: crate::OpaqueHostValue,
}
impl PartialEq for WgpuScene3DDrawEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuScene3DRuntime.ts:82 (sha256:4dab30bcdbb8075f68a1edc7500087cb2a72c3202eb16b2ffb6f143591215923)
#[derive(Clone, Default)]
pub struct WgpuScene3DRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub active_blend_mode: Option<BlendMode>,
    pub active_blended_run: bool,
    pub active_color_adjustment_run: bool,
    pub active_color_matrix_run: bool,
    pub active_skinned_run: bool,
    pub active_mesh_pipeline: Option<WgpuMeshPipeline>,
    pub blended_draw_list: Vec<WgpuScene3DDrawEntry>,
    pub blended_pool: Vec<WgpuScene3DDrawEntry>,
    pub draw_bind_group: Option<crate::OpaqueHostValue>,
    pub draw_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub frame_bind_group: Option<crate::OpaqueHostValue>,
    pub frame_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub frame_buffer: Option<crate::OpaqueHostValue>,
    pub frame_bindings: Vec<(crate::OpaqueHostValue, WgpuScene3DFrameBinding)>,
    pub custom_shader_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            WgpuRenderState,
                            String,
                            WgpuCustomMaterialShaderSource,
                            CustomShaderMaterial,
                        ) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub forward_light_selection_guard: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Scene3DLightsLike) -> () + Send + 'static>>>,
    >,
    pub environment_source_cube: Option<crate::OpaqueHostValue>,
    pub environment_source_cube_view: Option<crate::OpaqueHostValue>,
    pub ibl: Option<WgpuScene3DIbl>,
    pub ibl_dummy_cube_texture: Option<crate::OpaqueHostValue>,
    pub ibl_dummy_cube_view: Option<crate::OpaqueHostValue>,
    pub ibl_dummy_lut_texture: Option<crate::OpaqueHostValue>,
    pub ibl_dummy_lut_view: Option<crate::OpaqueHostValue>,
    pub ibl_sample_bind_group: Option<crate::OpaqueHostValue>,
    pub ibl_sample_cube_view: Option<crate::OpaqueHostValue>,
    pub ibl_sample_layout: Option<crate::OpaqueHostValue>,
    pub ibl_sampler: Option<crate::OpaqueHostValue>,
    pub ibl_uniform_buffer: Option<crate::OpaqueHostValue>,
    pub material_bind_groups: Vec<(crate::OpaqueHostValue, WgpuMaterialBinding)>,
    pub pbr_sample_bind_group: Option<crate::OpaqueHostValue>,
    pub pbr_sample_ibl_cube_view: Option<crate::OpaqueHostValue>,
    pub pbr_sample_layout: Option<crate::OpaqueHostValue>,
    pub pbr_sample_shadow_view: Option<crate::OpaqueHostValue>,
    pub material_registry: Vec<(Kind, WgpuMeshMaterialRenderer)>,
    pub modifier_snippet_registry: Option<ModifierRegistry>,
    pub modifier_snippet_revision: f64,
    pub opaque_draw_list: Vec<WgpuScene3DDrawEntry>,
    pub opaque_pool: Vec<WgpuScene3DDrawEntry>,
    pub pending_draw_offset: f64,
    pub pending_uv_transform: Vec<f32>,
    pub pipeline_cache: Vec<(String, WgpuMeshPipeline)>,
    pub placeholder_view: Option<crate::OpaqueHostValue>,
    pub shadow: Option<WgpuScene3DShadow>,
    pub shadow_comparison_sampler: Option<crate::OpaqueHostValue>,
    pub shadow_depth_pipeline: Option<crate::OpaqueHostValue>,
    pub shadow_depth_skinned_pipeline: Option<crate::OpaqueHostValue>,
    pub shadow_dummy_texture: Option<crate::OpaqueHostValue>,
    pub shadow_dummy_view: Option<crate::OpaqueHostValue>,
    pub shadow_sample_bind_group: Option<crate::OpaqueHostValue>,
    pub shadow_sample_layout: Option<crate::OpaqueHostValue>,
    pub shadow_sample_view: Option<crate::OpaqueHostValue>,
    pub shadow_uniform_buffer: Option<crate::OpaqueHostValue>,
    pub shaded_material_binding_cache: Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>,
    pub shaded_material_plan_cache: Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>,
    pub skin_draw_bind_group: Option<crate::OpaqueHostValue>,
    pub skin_draw_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub skin_palette_capacity: f64,
    pub skin_palette_texture: Option<crate::OpaqueHostValue>,
    pub skin_palette_view: Option<crate::OpaqueHostValue>,
    pub skinning_adapter: Option<crate::OpaqueHostValue>,
    pub upload_cache: Vec<(crate::OpaqueHostValue, WgpuMeshUpload)>,
}
impl PartialEq for WgpuScene3DRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuScene3DRuntime.ts:184 (sha256:5ded44df647ee2b8cd70a1c2fac7b6afdecf21d4ad1b059efa421212360c4c4d)
#[derive(Clone, Default)]
pub struct WgpuMeshUpload {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub index_buffer: Option<crate::OpaqueHostValue>,
    pub index_count: f64,
    pub index_format: crate::OpaqueHostValue,
    pub skin_bind_uploaded: Option<bool>,
    pub version: f64,
    pub vertex_buffer: crate::OpaqueHostValue,
}
impl PartialEq for WgpuMeshUpload {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuScene3DRuntime.ts:204 (sha256:71124ab66d3db75c82780773391a5291b531082454785c3c0049492c1e9d793a)
#[derive(Clone, Default)]
pub struct WgpuMaterialBinding {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bind_group: crate::OpaqueHostValue,
    pub buffer: crate::OpaqueHostValue,
    pub samplers: Option<Vec<crate::OpaqueHostValue>>,
    pub sampler: Option<crate::OpaqueHostValue>,
    pub views: Option<Vec<crate::OpaqueHostValue>>,
}
impl PartialEq for WgpuMaterialBinding {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
