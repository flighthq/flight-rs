// @generated from upstream/packages/types/src/WgpuScene3DRuntime.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, CustomShaderMaterial, Matrix4, Scene3DLightsLike, WgpuCustomMaterialShaderSource,
    WgpuMeshPipeline, WgpuRenderState,
};

// Source: upstream/packages/types/src/WgpuScene3DRuntime.ts:16 (sha256:51198c8940045753f24c81e72c79afa768610dc0b4ad580609876711fd13e77f)
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

// Source: upstream/packages/types/src/WgpuScene3DRuntime.ts:37 (sha256:2c9de49060c0caec1db063676aaff00b42a147d1453a572eebc9698cad804d96)
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

// Source: upstream/packages/types/src/WgpuScene3DRuntime.ts:48 (sha256:7ae1f3611cb970dc9a5bba4681dabc3f580fff5642b9b52a0f5f557b6f864e01)
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

// Source: upstream/packages/types/src/WgpuScene3DRuntime.ts:55 (sha256:b4d8b046bbaf156d910380ce47f8e5eb9bdcdfac78735a793394fc189153168d)
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

// Source: upstream/packages/types/src/WgpuScene3DRuntime.ts:79 (sha256:d73b5ce1b57506125a02a6af3df57a93e786f126e2f9cc4a43a4ca12cc6647fe)
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
    pub shaded_material_binding_cache: Vec<(crate::OpaqueHostValue, crate::FlightValue)>,
    pub shaded_material_plan_cache: Vec<(crate::OpaqueHostValue, crate::FlightValue)>,
    pub skin_draw_bind_group: Option<crate::OpaqueHostValue>,
    pub skin_draw_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub skin_mesh_draw_bind_group: Option<crate::OpaqueHostValue>,
    pub skin_mesh_draw_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub skin_arena_frame: Option<crate::OpaqueHostValue>,
    pub skin_normal_palette_arena_bases: Option<Vec<(Vec<f32>, f64)>>,
    pub skin_normal_palette_arena_cursor: f64,
    pub skin_normal_palette_arena_rows: f64,
    pub skin_normal_palette_texture: Option<crate::OpaqueHostValue>,
    pub skin_normal_palette_view: Option<crate::OpaqueHostValue>,
    pub skin_palette_arena_bases: Option<Vec<(Vec<f32>, f64)>>,
    pub skin_palette_arena_cursor: f64,
    pub skin_palette_arena_rows: f64,
    pub skin_palette_texture: Option<crate::OpaqueHostValue>,
    pub skin_palette_view: Option<crate::OpaqueHostValue>,
    pub pending_skin_normal_palette_base: f64,
    pub pending_skin_palette_base: f64,
    pub skinning_adapter: Option<crate::FlightValue>,
    pub upload_cache: Vec<(crate::OpaqueHostValue, WgpuMeshUpload)>,
}
impl PartialEq for WgpuScene3DRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuScene3DRuntime.ts:220 (sha256:31d73dbaa19b2ef6cf67f3fadd4a1b5319ee6881fb53b0fa1213c11a3b34115d)
#[derive(Clone, Default)]
pub struct WgpuMeshUpload {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub index_buffer: Option<crate::OpaqueHostValue>,
    pub index_count: f64,
    pub index_format: Option<crate::OpaqueHostValue>,
    pub skin_bind_uploaded: Option<bool>,
    pub version: f64,
    pub vertex_buffer: crate::OpaqueHostValue,
}
impl PartialEq for WgpuMeshUpload {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuScene3DRuntime.ts:240 (sha256:71124ab66d3db75c82780773391a5291b531082454785c3c0049492c1e9d793a)
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
