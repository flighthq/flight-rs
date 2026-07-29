// @generated from upstream/packages/bitmaptext/src/updateBitmapText.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_adjustments::create_color_transform_adjustment;
use flighthq_displayobject::{get_display_object_runtime, set_display_object_color_adjustments};
use flighthq_geometry::create_rectangle;
use flighthq_materials::create_color_transform;
use flighthq_node::{add_node_child, invalidate_node_local_bounds};
use flighthq_sprite::{append_quad_batch_instance, clear_quad_batch, create_quad_batch};
use flighthq_textureatlas::{add_texture_atlas_region, create_texture_atlas};
use flighthq_types::{
    Adjustment, AdjustmentKind, BitmapText, BitmapTextData, BitmapTextRuntime, BlendMode,
    BoundsNodeAny, ClipRegion, ColorTransform, GlyphEntry, GlyphSource, ImageResource,
    InteractionSignals, Kind, Material, MaterialData, Matrix, Matrix4, Node, NodeInteractionState,
    NodeSignals, NodeTraitsKey, QuadBatch, QuadBatchData, QuadTransformType, Rectangle, Stage,
    StageSignals, TextureAtlas, TextureAtlasRegion,
};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_multiplier: Option<f64>,
    pub alpha_offset: Option<f64>,
    pub blue_multiplier: Option<f64>,
    pub blue_offset: Option<f64>,
    pub green_multiplier: Option<f64>,
    pub green_offset: Option<f64>,
    pub red_multiplier: Option<f64>,
    pub red_offset: Option<f64>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub anisotropy_ext: Option<crate::OpaqueHostValue>,
    pub appearance_id: Option<f64>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub bounds_rectangle: Option<Rectangle>,
    pub bounds_using_local_bounds_id: Option<f64>,
    pub bounds_using_local_transform_id: Option<f64>,
    pub build_text_layout_params: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(TextLabel, TextMeasureFunction) -> TextLayoutParams + Send + 'static>,
            >,
        >,
    >,
    pub canvas_texture_view: Option<crate::OpaqueHostValue>,
    pub canvas_view_cleared: Option<bool>,
    pub clip_contour_pipelines: Option<Vec<(crate::OpaqueHostValue, WgpuClipContourPipelines)>>,
    pub clip_contour_stack: Option<Vec<WgpuClipContourEntry>>,
    pub clip_forms: Option<Vec<String>>,
    pub color_adjustment_channel_mixing_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RenderState, Renderable) -> () + Send + 'static>>,
        >,
    >,
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub color_adjustments_channel_mixing: Option<bool>,
    pub color_transform_instanced_shader: Option<GlColorTransformInstancedShader>,
    pub command_encoder: Option<crate::OpaqueHostValue>,
    pub compressed_texture_decoder: Option<GlCompressedTextureDecoder>,
    pub compressed_texture_upload: Option<GlCompressedTextureUploader>,
    pub compute_local_bounds_rectangle: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Rectangle, BoundsNodeAny) -> () + Send + 'static>>,
        >,
    >,
    pub current_blend_mode: Option<BlendMode>,
    pub current_color_format: Option<crate::OpaqueHostValue>,
    pub current_framebuffer: Option<crate::OpaqueHostValue>,
    pub current_frame_id: Option<f64>,
    pub current_mask_depth: Option<f64>,
    pub current_program: Option<crate::OpaqueHostValue>,
    pub current_render_target: Option<GlRenderTarget>,
    pub current_texture: Option<crate::OpaqueHostValue>,
    pub depth_stencil_height: Option<f64>,
    pub depth_stencil_texture: Option<crate::OpaqueHostValue>,
    pub depth_stencil_view: Option<crate::OpaqueHostValue>,
    pub depth_stencil_width: Option<f64>,
    pub dom_clip_hooks: Option<DomClipHooks>,
    pub dom_clip_stack: Option<Vec<DomClipEntry>>,
    pub dom_current_element: Option<crate::OpaqueHostValue>,
    pub dom_element_map: Option<Vec<(RenderProxy2D, crate::OpaqueHostValue)>>,
    pub dom_next_order_list: Option<Vec<RenderProxy2D>>,
    pub dom_order_length: Option<f64>,
    pub dom_order_list: Option<Vec<RenderProxy2D>>,
    pub element: Option<crate::OpaqueHostValue>,
    pub frame_capture_buffer: Option<crate::OpaqueHostValue>,
    pub frame_capture_bytes_per_row: Option<f64>,
    pub frame_capture_enabled: Option<bool>,
    pub frame_capture_height: Option<f64>,
    pub frame_capture_texture: Option<crate::OpaqueHostValue>,
    pub frame_capture_width: Option<f64>,
    pub gl_blend_mode_registry: Option<Vec<(BlendMode, GlBlendRealization)>>,
    pub gl_color_adjustment_fold: Option<GlColorAdjustmentFold>,
    pub gl_color_adjustment_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(GlRenderState, ColorTransform) -> () + Send + 'static>>,
        >,
    >,
    pub image_smoothing_enabled: Option<bool>,
    pub image_smoothing_quality: Option<crate::OpaqueHostValue>,
    pub input: Option<TextInputState>,
    pub instance_velocities: Option<Vec<f32>>,
    pub interaction_signals: Option<InteractionSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub linear_sampler: Option<crate::OpaqueHostValue>,
    pub local_bounds_id: Option<f64>,
    pub local_bounds_rectangle: Option<Rectangle>,
    pub local_bounds_using_local_bounds_id: Option<f64>,
    pub local_content_id: Option<f64>,
    pub local_matrix: Option<Matrix>,
    pub local_matrix4: Option<Matrix4>,
    pub local_matrix4_detached: Option<bool>,
    pub local_transform_id: Option<f64>,
    pub local_transform_using_local_transform_id: Option<f64>,
    pub mask_write_mode: Option<bool>,
    pub material_bitmap_shader_map: Option<Vec<(Kind, GlBitmapShader)>>,
    pub matrix_array: Option<Vec<f32>>,
    pub max_anisotropy: Option<f64>,
    pub measured_height: Option<f64>,
    pub measured_width: Option<f64>,
    pub mipmap_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub mipmapped_textures: Option<Vec<crate::OpaqueHostValue>>,
    pub mipmap_pipeline: Option<crate::OpaqueHostValue>,
    pub morph_bind_pose: Option<MeshMorphBindPose>,
    pub movie_clip_signals: Option<MovieClipSignals>,
    pub nearest_sampler: Option<crate::OpaqueHostValue>,
    pub node_signals: Option<NodeSignals>,
    pub particle_corner_buffer: Option<crate::OpaqueHostValue>,
    pub particle_instance_capacity: Option<f64>,
    pub particle_shader: Option<GlParticleShader>,
    pub pipeline_cache: Option<Vec<(String, crate::OpaqueHostValue)>>,
    pub quad_batch_corner_buffer: Option<crate::OpaqueHostValue>,
    pub quad_batches: Option<Vec<QuadBatch>>,
    pub quad_batch_shader: Option<GlQuadBatchShader>,
    pub quad_index_buffer: Option<crate::OpaqueHostValue>,
    pub quad_vertex_buffer: Option<crate::OpaqueHostValue>,
    pub quad_vertex_data: Option<Vec<f32>>,
    pub render_adapt_hook: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(RenderState, Renderable, RenderProxy2D) -> () + Send + 'static>,
            >,
        >,
    >,
    pub renderer_map: Option<Vec<(Kind, Renderer)>>,
    pub renderer_map_id: Option<f64>,
    pub render_pass: Option<crate::OpaqueHostValue>,
    pub render_proxy_adapter_map: Option<Vec<(Renderable, RenderProxyAdapter)>>,
    pub render_proxy_map: Option<Vec<(Renderable, RenderProxy)>>,
    pub render_target_stack: Option<Vec<WgpuSavedPassState>>,
    pub resolved_color_transform: Option<ColorTransform>,
    pub retired_buffers: Option<Vec<crate::OpaqueHostValue>>,
    pub rich_text_content: Option<RichTextContent>,
    pub rotation_angle: Option<f64>,
    pub rotation_cosine: Option<f64>,
    pub rotation_sine: Option<f64>,
    pub sampler_cache: Option<Vec<(String, crate::OpaqueHostValue)>>,
    pub scene_mesh_upload_cache: Option<Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>>,
    pub selection_begin_index: Option<f64>,
    pub selection_end_index: Option<f64>,
    pub shader_loc: Option<GlShaderLocations>,
    pub shape_mesh_color_transform_shader: Option<GlShapeMeshColorTransformShader>,
    pub shape_mesh_pipelines: Option<Vec<(crate::OpaqueHostValue, WgpuShapeMeshPipeline)>>,
    pub skin_bind_pose: Option<MeshSkinBindPose>,
    pub sprite_batch_blend_mode: Option<BlendMode>,
    pub sprite_batch_buffer_cursor: Option<f64>,
    pub sprite_batch_buffer_pool: Option<Vec<WgpuSpriteBatchBufferSlot>>,
    pub sprite_batch_color_transform_buffer: Option<crate::OpaqueHostValue>,
    pub sprite_batch_color_transform_data: Option<Vec<f32>>,
    pub sprite_batch_color_transform_mode: Option<f64>,
    pub sprite_batch_count: Option<f64>,
    pub sprite_batch_instance_buffer: Option<crate::OpaqueHostValue>,
    pub sprite_batch_instance_data: Option<Vec<f32>>,
    pub sprite_batch_material: Option<Material>,
    pub sprite_batch_material_buffer: Option<crate::OpaqueHostValue>,
    pub sprite_batch_material_data: Option<Vec<f32>>,
    pub sprite_batch_material_floats: Option<f64>,
    pub sprite_batch_texture: Option<ImageResource>,
    pub sprite_batch_uniform_color_transform: Option<ColorTransform>,
    pub stage: Option<Stage>,
    pub stage_signals: Option<StageSignals>,
    pub temp_stack: Option<Vec<Renderable>>,
    pub text_field_signals: Option<TextFieldSignals>,
    pub text_layout: Option<TextLayoutResult>,
    pub text_layout_using_content_id: Option<f64>,
    pub texture_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub uniform_bind_group: Option<crate::OpaqueHostValue>,
    pub uniform_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub uniform_buffer: Option<crate::OpaqueHostValue>,
    pub uniform_color_transform_shader: Option<GlUniformColorTransformShader>,
    pub uniform_data: Option<Vec<f32>>,
    pub uniform_data_u32: Option<Vec<u32>>,
    pub uniform_offset: Option<f64>,
    pub uniform_stride: Option<f64>,
    pub webgl_data: Option<MeshGeometryGlData>,
    pub webgl_shader_binding_resolver: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(RenderProxy2D) -> Option<GlBitmapShader> + Send + 'static>,
            >,
        >,
    >,
    pub webgpu_data: Option<MeshGeometryWgpuData>,
    pub webgpu_shader_binding_resolver: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(RenderProxy2D) -> Option<WgpuBitmapShader> + Send + 'static>,
            >,
        >,
    >,
    pub wgpu_color_adjustment_fold: Option<WgpuColorAdjustmentFold>,
    pub wgpu_color_adjustment_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(WgpuRenderState, ColorTransform) -> () + Send + 'static>,
            >,
        >,
    >,
    pub world_alpha: Option<f64>,
    pub world_alpha_using_appearance_id: Option<f64>,
    pub world_alpha_using_parent_appearance_id: Option<f64>,
    pub world_appearance_id: Option<f64>,
    pub world_bounds_rectangle: Option<Rectangle>,
    pub world_bounds_using_local_bounds_id: Option<f64>,
    pub world_bounds_using_world_transform_id: Option<f64>,
    pub world_matrix: Option<Matrix>,
    pub world_matrix4: Option<Matrix4>,
    pub world_transform_id: Option<f64>,
    pub world_transform_using_local_transform_id: Option<f64>,
    pub world_transform_using_parent_transform_id: Option<f64>,
    pub can_add_child: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    >,
    pub children: Option<Vec<Node>>,
    pub traits: Option<NodeTraitsKey>,
    pub parent: Option<Node>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub anisotropy_ext: Option<crate::OpaqueHostValue>,
    pub appearance_id: Option<f64>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub bounds_rectangle: Option<Rectangle>,
    pub bounds_using_local_bounds_id: Option<f64>,
    pub bounds_using_local_transform_id: Option<f64>,
    pub build_text_layout_params: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(TextLabel, TextMeasureFunction) -> TextLayoutParams + Send + 'static>,
            >,
        >,
    >,
    pub canvas_texture_view: Option<crate::OpaqueHostValue>,
    pub canvas_view_cleared: Option<bool>,
    pub clip_contour_pipelines: Option<Vec<(crate::OpaqueHostValue, WgpuClipContourPipelines)>>,
    pub clip_contour_stack: Option<Vec<WgpuClipContourEntry>>,
    pub clip_forms: Option<Vec<String>>,
    pub color_adjustment_channel_mixing_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RenderState, Renderable) -> () + Send + 'static>>,
        >,
    >,
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub color_adjustments_channel_mixing: Option<bool>,
    pub color_transform_instanced_shader: Option<GlColorTransformInstancedShader>,
    pub command_encoder: Option<crate::OpaqueHostValue>,
    pub compressed_texture_decoder: Option<GlCompressedTextureDecoder>,
    pub compressed_texture_upload: Option<GlCompressedTextureUploader>,
    pub compute_local_bounds_rectangle: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Rectangle, BoundsNodeAny) -> () + Send + 'static>>,
        >,
    >,
    pub current_blend_mode: Option<BlendMode>,
    pub current_color_format: Option<crate::OpaqueHostValue>,
    pub current_framebuffer: Option<crate::OpaqueHostValue>,
    pub current_frame_id: Option<f64>,
    pub current_mask_depth: Option<f64>,
    pub current_program: Option<crate::OpaqueHostValue>,
    pub current_render_target: Option<GlRenderTarget>,
    pub current_texture: Option<crate::OpaqueHostValue>,
    pub depth_stencil_height: Option<f64>,
    pub depth_stencil_texture: Option<crate::OpaqueHostValue>,
    pub depth_stencil_view: Option<crate::OpaqueHostValue>,
    pub depth_stencil_width: Option<f64>,
    pub dom_clip_hooks: Option<DomClipHooks>,
    pub dom_clip_stack: Option<Vec<DomClipEntry>>,
    pub dom_current_element: Option<crate::OpaqueHostValue>,
    pub dom_element_map: Option<Vec<(RenderProxy2D, crate::OpaqueHostValue)>>,
    pub dom_next_order_list: Option<Vec<RenderProxy2D>>,
    pub dom_order_length: Option<f64>,
    pub dom_order_list: Option<Vec<RenderProxy2D>>,
    pub element: Option<crate::OpaqueHostValue>,
    pub frame_capture_buffer: Option<crate::OpaqueHostValue>,
    pub frame_capture_bytes_per_row: Option<f64>,
    pub frame_capture_enabled: Option<bool>,
    pub frame_capture_height: Option<f64>,
    pub frame_capture_texture: Option<crate::OpaqueHostValue>,
    pub frame_capture_width: Option<f64>,
    pub gl_blend_mode_registry: Option<Vec<(BlendMode, GlBlendRealization)>>,
    pub gl_color_adjustment_fold: Option<GlColorAdjustmentFold>,
    pub gl_color_adjustment_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(GlRenderState, ColorTransform) -> () + Send + 'static>>,
        >,
    >,
    pub image_smoothing_enabled: Option<bool>,
    pub image_smoothing_quality: Option<crate::OpaqueHostValue>,
    pub input: Option<TextInputState>,
    pub instance_velocities: Option<Vec<f32>>,
    pub interaction_signals: Option<InteractionSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub linear_sampler: Option<crate::OpaqueHostValue>,
    pub local_bounds_id: Option<f64>,
    pub local_bounds_rectangle: Option<Rectangle>,
    pub local_bounds_using_local_bounds_id: Option<f64>,
    pub local_content_id: Option<f64>,
    pub local_matrix: Option<Matrix>,
    pub local_matrix4: Option<Matrix4>,
    pub local_matrix4_detached: Option<bool>,
    pub local_transform_id: Option<f64>,
    pub local_transform_using_local_transform_id: Option<f64>,
    pub mask_write_mode: Option<bool>,
    pub material_bitmap_shader_map: Option<Vec<(Kind, GlBitmapShader)>>,
    pub matrix_array: Option<Vec<f32>>,
    pub max_anisotropy: Option<f64>,
    pub measured_height: Option<f64>,
    pub measured_width: Option<f64>,
    pub mipmap_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub mipmapped_textures: Option<Vec<crate::OpaqueHostValue>>,
    pub mipmap_pipeline: Option<crate::OpaqueHostValue>,
    pub morph_bind_pose: Option<MeshMorphBindPose>,
    pub movie_clip_signals: Option<MovieClipSignals>,
    pub nearest_sampler: Option<crate::OpaqueHostValue>,
    pub node_signals: Option<NodeSignals>,
    pub particle_corner_buffer: Option<crate::OpaqueHostValue>,
    pub particle_instance_capacity: Option<f64>,
    pub particle_shader: Option<GlParticleShader>,
    pub pipeline_cache: Option<Vec<(String, crate::OpaqueHostValue)>>,
    pub quad_batch_corner_buffer: Option<crate::OpaqueHostValue>,
    pub quad_batches: Option<Vec<QuadBatch>>,
    pub quad_batch_shader: Option<GlQuadBatchShader>,
    pub quad_index_buffer: Option<crate::OpaqueHostValue>,
    pub quad_vertex_buffer: Option<crate::OpaqueHostValue>,
    pub quad_vertex_data: Option<Vec<f32>>,
    pub render_adapt_hook: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(RenderState, Renderable, RenderProxy2D) -> () + Send + 'static>,
            >,
        >,
    >,
    pub renderer_map: Option<Vec<(Kind, Renderer)>>,
    pub renderer_map_id: Option<f64>,
    pub render_pass: Option<crate::OpaqueHostValue>,
    pub render_proxy_adapter_map: Option<Vec<(Renderable, RenderProxyAdapter)>>,
    pub render_proxy_map: Option<Vec<(Renderable, RenderProxy)>>,
    pub render_target_stack: Option<Vec<WgpuSavedPassState>>,
    pub resolved_color_transform: Option<ColorTransform>,
    pub retired_buffers: Option<Vec<crate::OpaqueHostValue>>,
    pub rich_text_content: Option<RichTextContent>,
    pub rotation_angle: Option<f64>,
    pub rotation_cosine: Option<f64>,
    pub rotation_sine: Option<f64>,
    pub sampler_cache: Option<Vec<(String, crate::OpaqueHostValue)>>,
    pub scene_mesh_upload_cache: Option<Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>>,
    pub selection_begin_index: Option<f64>,
    pub selection_end_index: Option<f64>,
    pub shader_loc: Option<GlShaderLocations>,
    pub shape_mesh_color_transform_shader: Option<GlShapeMeshColorTransformShader>,
    pub shape_mesh_pipelines: Option<Vec<(crate::OpaqueHostValue, WgpuShapeMeshPipeline)>>,
    pub skin_bind_pose: Option<MeshSkinBindPose>,
    pub sprite_batch_blend_mode: Option<BlendMode>,
    pub sprite_batch_buffer_cursor: Option<f64>,
    pub sprite_batch_buffer_pool: Option<Vec<WgpuSpriteBatchBufferSlot>>,
    pub sprite_batch_color_transform_buffer: Option<crate::OpaqueHostValue>,
    pub sprite_batch_color_transform_data: Option<Vec<f32>>,
    pub sprite_batch_color_transform_mode: Option<f64>,
    pub sprite_batch_count: Option<f64>,
    pub sprite_batch_instance_buffer: Option<crate::OpaqueHostValue>,
    pub sprite_batch_instance_data: Option<Vec<f32>>,
    pub sprite_batch_material: Option<Material>,
    pub sprite_batch_material_buffer: Option<crate::OpaqueHostValue>,
    pub sprite_batch_material_data: Option<Vec<f32>>,
    pub sprite_batch_material_floats: Option<f64>,
    pub sprite_batch_texture: Option<ImageResource>,
    pub sprite_batch_uniform_color_transform: Option<ColorTransform>,
    pub stage: Option<Stage>,
    pub stage_signals: Option<StageSignals>,
    pub temp_stack: Option<Vec<Renderable>>,
    pub text_field_signals: Option<TextFieldSignals>,
    pub text_layout: Option<TextLayoutResult>,
    pub text_layout_using_content_id: Option<f64>,
    pub texture_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub uniform_bind_group: Option<crate::OpaqueHostValue>,
    pub uniform_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub uniform_buffer: Option<crate::OpaqueHostValue>,
    pub uniform_color_transform_shader: Option<GlUniformColorTransformShader>,
    pub uniform_data: Option<Vec<f32>>,
    pub uniform_data_u32: Option<Vec<u32>>,
    pub uniform_offset: Option<f64>,
    pub uniform_stride: Option<f64>,
    pub webgl_data: Option<MeshGeometryGlData>,
    pub webgl_shader_binding_resolver: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(RenderProxy2D) -> Option<GlBitmapShader> + Send + 'static>,
            >,
        >,
    >,
    pub webgpu_data: Option<MeshGeometryWgpuData>,
    pub webgpu_shader_binding_resolver: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(RenderProxy2D) -> Option<WgpuBitmapShader> + Send + 'static>,
            >,
        >,
    >,
    pub wgpu_color_adjustment_fold: Option<WgpuColorAdjustmentFold>,
    pub wgpu_color_adjustment_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(WgpuRenderState, ColorTransform) -> () + Send + 'static>,
            >,
        >,
    >,
    pub world_alpha: Option<f64>,
    pub world_alpha_using_appearance_id: Option<f64>,
    pub world_alpha_using_parent_appearance_id: Option<f64>,
    pub world_appearance_id: Option<f64>,
    pub world_bounds_rectangle: Option<Rectangle>,
    pub world_bounds_using_local_bounds_id: Option<f64>,
    pub world_bounds_using_world_transform_id: Option<f64>,
    pub world_matrix: Option<Matrix>,
    pub world_matrix4: Option<Matrix4>,
    pub world_transform_id: Option<f64>,
    pub world_transform_using_local_transform_id: Option<f64>,
    pub world_transform_using_parent_transform_id: Option<f64>,
    pub can_add_child: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    >,
    pub children: Option<Vec<Node>>,
    pub traits: Option<NodeTraitsKey>,
    pub parent: Option<Node>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub atlas: Option<TextureAtlas>,
    pub ids: Option<Vec<u16>>,
    pub instance_count: Option<f64>,
    pub material_data: Option<Vec<Option<MaterialData>>>,
    pub transforms: Option<Vec<f32>>,
    pub transform_type: Option<QuadTransformType>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub image: Option<ImageResource>,
    pub regions: Option<Vec<TextureAtlasRegion>>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub height: Option<f64>,
    pub id: Option<f64>,
    pub name: Option<String>,
    pub original_height: Option<f64>,
    pub original_width: Option<f64>,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotated: Option<bool>,
    pub source_x: Option<f64>,
    pub source_y: Option<f64>,
    pub trimmed: Option<bool>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord7 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub color_matrix: Option<Vec<f64>>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord8 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord9 {
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for FlightPartialRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord10 {
    pub __flight_identity: std::sync::Arc<()>,
    pub anisotropy_ext: Option<crate::OpaqueHostValue>,
    pub appearance_id: Option<f64>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub bounds_rectangle: Option<Rectangle>,
    pub bounds_using_local_bounds_id: Option<f64>,
    pub bounds_using_local_transform_id: Option<f64>,
    pub build_text_layout_params: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(TextLabel, TextMeasureFunction) -> TextLayoutParams + Send + 'static>,
            >,
        >,
    >,
    pub canvas_texture_view: Option<crate::OpaqueHostValue>,
    pub canvas_view_cleared: Option<bool>,
    pub clip_contour_pipelines: Option<Vec<(crate::OpaqueHostValue, WgpuClipContourPipelines)>>,
    pub clip_contour_stack: Option<Vec<WgpuClipContourEntry>>,
    pub clip_forms: Option<Vec<String>>,
    pub color_adjustment_channel_mixing_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RenderState, Renderable) -> () + Send + 'static>>,
        >,
    >,
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub color_adjustments_channel_mixing: Option<bool>,
    pub color_transform_instanced_shader: Option<GlColorTransformInstancedShader>,
    pub command_encoder: Option<crate::OpaqueHostValue>,
    pub compressed_texture_decoder: Option<GlCompressedTextureDecoder>,
    pub compressed_texture_upload: Option<GlCompressedTextureUploader>,
    pub compute_local_bounds_rectangle: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Rectangle, BoundsNodeAny) -> () + Send + 'static>>,
        >,
    >,
    pub current_blend_mode: Option<BlendMode>,
    pub current_color_format: Option<crate::OpaqueHostValue>,
    pub current_framebuffer: Option<crate::OpaqueHostValue>,
    pub current_frame_id: Option<f64>,
    pub current_mask_depth: Option<f64>,
    pub current_program: Option<crate::OpaqueHostValue>,
    pub current_render_target: Option<GlRenderTarget>,
    pub current_texture: Option<crate::OpaqueHostValue>,
    pub depth_stencil_height: Option<f64>,
    pub depth_stencil_texture: Option<crate::OpaqueHostValue>,
    pub depth_stencil_view: Option<crate::OpaqueHostValue>,
    pub depth_stencil_width: Option<f64>,
    pub dom_clip_hooks: Option<DomClipHooks>,
    pub dom_clip_stack: Option<Vec<DomClipEntry>>,
    pub dom_current_element: Option<crate::OpaqueHostValue>,
    pub dom_element_map: Option<Vec<(RenderProxy2D, crate::OpaqueHostValue)>>,
    pub dom_next_order_list: Option<Vec<RenderProxy2D>>,
    pub dom_order_length: Option<f64>,
    pub dom_order_list: Option<Vec<RenderProxy2D>>,
    pub element: Option<crate::OpaqueHostValue>,
    pub frame_capture_buffer: Option<crate::OpaqueHostValue>,
    pub frame_capture_bytes_per_row: Option<f64>,
    pub frame_capture_enabled: Option<bool>,
    pub frame_capture_height: Option<f64>,
    pub frame_capture_texture: Option<crate::OpaqueHostValue>,
    pub frame_capture_width: Option<f64>,
    pub gl_blend_mode_registry: Option<Vec<(BlendMode, GlBlendRealization)>>,
    pub gl_color_adjustment_fold: Option<GlColorAdjustmentFold>,
    pub gl_color_adjustment_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(GlRenderState, ColorTransform) -> () + Send + 'static>>,
        >,
    >,
    pub image_smoothing_enabled: Option<bool>,
    pub image_smoothing_quality: Option<crate::OpaqueHostValue>,
    pub input: Option<TextInputState>,
    pub instance_velocities: Option<Vec<f32>>,
    pub interaction_signals: Option<InteractionSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub linear_sampler: Option<crate::OpaqueHostValue>,
    pub local_bounds_id: Option<f64>,
    pub local_bounds_rectangle: Option<Rectangle>,
    pub local_bounds_using_local_bounds_id: Option<f64>,
    pub local_content_id: Option<f64>,
    pub local_matrix: Option<Matrix>,
    pub local_matrix4: Option<Matrix4>,
    pub local_matrix4_detached: Option<bool>,
    pub local_transform_id: Option<f64>,
    pub local_transform_using_local_transform_id: Option<f64>,
    pub mask_write_mode: Option<bool>,
    pub material_bitmap_shader_map: Option<Vec<(Kind, GlBitmapShader)>>,
    pub matrix_array: Option<Vec<f32>>,
    pub max_anisotropy: Option<f64>,
    pub measured_height: Option<f64>,
    pub measured_width: Option<f64>,
    pub mipmap_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub mipmapped_textures: Option<Vec<crate::OpaqueHostValue>>,
    pub mipmap_pipeline: Option<crate::OpaqueHostValue>,
    pub morph_bind_pose: Option<MeshMorphBindPose>,
    pub movie_clip_signals: Option<MovieClipSignals>,
    pub nearest_sampler: Option<crate::OpaqueHostValue>,
    pub node_signals: Option<NodeSignals>,
    pub particle_corner_buffer: Option<crate::OpaqueHostValue>,
    pub particle_instance_capacity: Option<f64>,
    pub particle_shader: Option<GlParticleShader>,
    pub pipeline_cache: Option<Vec<(String, crate::OpaqueHostValue)>>,
    pub quad_batch_corner_buffer: Option<crate::OpaqueHostValue>,
    pub quad_batches: Option<Vec<QuadBatch>>,
    pub quad_batch_shader: Option<GlQuadBatchShader>,
    pub quad_index_buffer: Option<crate::OpaqueHostValue>,
    pub quad_vertex_buffer: Option<crate::OpaqueHostValue>,
    pub quad_vertex_data: Option<Vec<f32>>,
    pub render_adapt_hook: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(RenderState, Renderable, RenderProxy2D) -> () + Send + 'static>,
            >,
        >,
    >,
    pub renderer_map: Option<Vec<(Kind, Renderer)>>,
    pub renderer_map_id: Option<f64>,
    pub render_pass: Option<crate::OpaqueHostValue>,
    pub render_proxy_adapter_map: Option<Vec<(Renderable, RenderProxyAdapter)>>,
    pub render_proxy_map: Option<Vec<(Renderable, RenderProxy)>>,
    pub render_target_stack: Option<Vec<WgpuSavedPassState>>,
    pub resolved_color_transform: Option<ColorTransform>,
    pub retired_buffers: Option<Vec<crate::OpaqueHostValue>>,
    pub rich_text_content: Option<RichTextContent>,
    pub rotation_angle: Option<f64>,
    pub rotation_cosine: Option<f64>,
    pub rotation_sine: Option<f64>,
    pub sampler_cache: Option<Vec<(String, crate::OpaqueHostValue)>>,
    pub scene_mesh_upload_cache: Option<Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>>,
    pub selection_begin_index: Option<f64>,
    pub selection_end_index: Option<f64>,
    pub shader_loc: Option<GlShaderLocations>,
    pub shape_mesh_color_transform_shader: Option<GlShapeMeshColorTransformShader>,
    pub shape_mesh_pipelines: Option<Vec<(crate::OpaqueHostValue, WgpuShapeMeshPipeline)>>,
    pub skin_bind_pose: Option<MeshSkinBindPose>,
    pub sprite_batch_blend_mode: Option<BlendMode>,
    pub sprite_batch_buffer_cursor: Option<f64>,
    pub sprite_batch_buffer_pool: Option<Vec<WgpuSpriteBatchBufferSlot>>,
    pub sprite_batch_color_transform_buffer: Option<crate::OpaqueHostValue>,
    pub sprite_batch_color_transform_data: Option<Vec<f32>>,
    pub sprite_batch_color_transform_mode: Option<f64>,
    pub sprite_batch_count: Option<f64>,
    pub sprite_batch_instance_buffer: Option<crate::OpaqueHostValue>,
    pub sprite_batch_instance_data: Option<Vec<f32>>,
    pub sprite_batch_material: Option<Material>,
    pub sprite_batch_material_buffer: Option<crate::OpaqueHostValue>,
    pub sprite_batch_material_data: Option<Vec<f32>>,
    pub sprite_batch_material_floats: Option<f64>,
    pub sprite_batch_texture: Option<ImageResource>,
    pub sprite_batch_uniform_color_transform: Option<ColorTransform>,
    pub stage: Option<Stage>,
    pub stage_signals: Option<StageSignals>,
    pub temp_stack: Option<Vec<Renderable>>,
    pub text_field_signals: Option<TextFieldSignals>,
    pub text_layout: Option<TextLayoutResult>,
    pub text_layout_using_content_id: Option<f64>,
    pub texture_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub uniform_bind_group: Option<crate::OpaqueHostValue>,
    pub uniform_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub uniform_buffer: Option<crate::OpaqueHostValue>,
    pub uniform_color_transform_shader: Option<GlUniformColorTransformShader>,
    pub uniform_data: Option<Vec<f32>>,
    pub uniform_data_u32: Option<Vec<u32>>,
    pub uniform_offset: Option<f64>,
    pub uniform_stride: Option<f64>,
    pub webgl_data: Option<MeshGeometryGlData>,
    pub webgl_shader_binding_resolver: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(RenderProxy2D) -> Option<GlBitmapShader> + Send + 'static>,
            >,
        >,
    >,
    pub webgpu_data: Option<MeshGeometryWgpuData>,
    pub webgpu_shader_binding_resolver: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(RenderProxy2D) -> Option<WgpuBitmapShader> + Send + 'static>,
            >,
        >,
    >,
    pub wgpu_color_adjustment_fold: Option<WgpuColorAdjustmentFold>,
    pub wgpu_color_adjustment_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(WgpuRenderState, ColorTransform) -> () + Send + 'static>,
            >,
        >,
    >,
    pub world_alpha: Option<f64>,
    pub world_alpha_using_appearance_id: Option<f64>,
    pub world_alpha_using_parent_appearance_id: Option<f64>,
    pub world_appearance_id: Option<f64>,
    pub world_bounds_rectangle: Option<Rectangle>,
    pub world_bounds_using_local_bounds_id: Option<f64>,
    pub world_bounds_using_world_transform_id: Option<f64>,
    pub world_matrix: Option<Matrix>,
    pub world_matrix4: Option<Matrix4>,
    pub world_transform_id: Option<f64>,
    pub world_transform_using_local_transform_id: Option<f64>,
    pub world_transform_using_parent_transform_id: Option<f64>,
}
impl PartialEq for FlightPartialRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord11 {
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for FlightPartialRecord11 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord12 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: Option<ClipRegion>,
}
impl PartialEq for FlightPartialRecord12 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord13 {
    pub __flight_identity: std::sync::Arc<()>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
}
impl PartialEq for FlightPartialRecord13 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord14 {
    pub __flight_identity: std::sync::Arc<()>,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotation: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub skew_x: Option<f64>,
    pub skew_y: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}
impl PartialEq for FlightPartialRecord14 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:20 (sha256:6092d0ea3715e74338136af9aad3fb871bc32e15dea783aa072633373219d2fe)
const BITMAP_TEXT_DEFAULT_COLOR: f64 = 4294967295.0_f64;

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:21 (sha256:945bc25bcb4a4dbbb1555bb1828c3a1d08003f223c5b9e79cdc1cce606986637)
const CARRIAGE_RETURN: f64 = 13.0_f64;

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:22 (sha256:111c1172e5ab8d948ad84285fdac7880dc972715f0b0a9c7da683887c231e300)
const SPACE: f64 = 32.0_f64;

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:34 (sha256:50064fa2c76e353881909c6c12cf4b2525bf2e44ac6204da17ec036199a6aa95)
pub fn update_bitmap_text(bitmap_text: &BitmapText) -> () {
    let mut runtime = {
        let __flight_source = &(get_display_object_runtime(bitmap_text));
        BitmapTextRuntime {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            anisotropy_ext: (__flight_source.anisotropy_ext).clone(),
            appearance_id: __flight_source.appearance_id,
            binding: (__flight_source.binding).clone(),
            bounds_rectangle: (__flight_source.bounds_rectangle).clone(),
            bounds_using_local_bounds_id: __flight_source.bounds_using_local_bounds_id,
            bounds_using_local_transform_id: __flight_source.bounds_using_local_transform_id,
            build_text_layout_params: (__flight_source.build_text_layout_params).clone(),
            canvas_texture_view: (__flight_source.canvas_texture_view).clone(),
            canvas_view_cleared: __flight_source.canvas_view_cleared,
            clip_contour_pipelines: (__flight_source.clip_contour_pipelines).clone(),
            clip_contour_stack: (__flight_source.clip_contour_stack).clone(),
            clip_forms: (__flight_source.clip_forms).clone(),
            color_adjustment_channel_mixing_guard: (__flight_source
                .color_adjustment_channel_mixing_guard)
                .clone(),
            color_adjustments: (__flight_source.color_adjustments).clone(),
            color_adjustments_channel_mixing: __flight_source.color_adjustments_channel_mixing,
            color_transform_instanced_shader: (__flight_source.color_transform_instanced_shader)
                .clone(),
            command_encoder: (__flight_source.command_encoder).clone(),
            compressed_texture_decoder: (__flight_source.compressed_texture_decoder).clone(),
            compressed_texture_upload: (__flight_source.compressed_texture_upload).clone(),
            compute_local_bounds_rectangle: (__flight_source.compute_local_bounds_rectangle)
                .clone(),
            current_blend_mode: (__flight_source.current_blend_mode).clone(),
            current_color_format: (__flight_source.current_color_format).clone(),
            current_framebuffer: (__flight_source.current_framebuffer).clone(),
            current_frame_id: __flight_source.current_frame_id,
            current_mask_depth: __flight_source.current_mask_depth,
            current_program: (__flight_source.current_program).clone(),
            current_render_target: (__flight_source.current_render_target).clone(),
            current_texture: (__flight_source.current_texture).clone(),
            depth_stencil_height: __flight_source.depth_stencil_height,
            depth_stencil_texture: (__flight_source.depth_stencil_texture).clone(),
            depth_stencil_view: (__flight_source.depth_stencil_view).clone(),
            depth_stencil_width: __flight_source.depth_stencil_width,
            dom_clip_hooks: (__flight_source.dom_clip_hooks).clone(),
            dom_clip_stack: (__flight_source.dom_clip_stack).clone(),
            dom_current_element: (__flight_source.dom_current_element).clone(),
            dom_element_map: (__flight_source.dom_element_map).clone(),
            dom_next_order_list: (__flight_source.dom_next_order_list).clone(),
            dom_order_length: __flight_source.dom_order_length,
            dom_order_list: (__flight_source.dom_order_list).clone(),
            element: (__flight_source.element).clone(),
            frame_capture_buffer: (__flight_source.frame_capture_buffer).clone(),
            frame_capture_bytes_per_row: __flight_source.frame_capture_bytes_per_row,
            frame_capture_enabled: __flight_source.frame_capture_enabled,
            frame_capture_height: __flight_source.frame_capture_height,
            frame_capture_texture: (__flight_source.frame_capture_texture).clone(),
            frame_capture_width: __flight_source.frame_capture_width,
            gl_blend_mode_registry: (__flight_source.gl_blend_mode_registry).clone(),
            gl_color_adjustment_fold: (__flight_source.gl_color_adjustment_fold).clone(),
            gl_color_adjustment_guard: (__flight_source.gl_color_adjustment_guard).clone(),
            image_smoothing_enabled: __flight_source.image_smoothing_enabled,
            image_smoothing_quality: (__flight_source.image_smoothing_quality).clone(),
            input: (__flight_source.input).clone(),
            instance_velocities: (__flight_source.instance_velocities).clone(),
            interaction_signals: (__flight_source.interaction_signals).clone(),
            interaction_state: (__flight_source.interaction_state).clone(),
            linear_sampler: (__flight_source.linear_sampler).clone(),
            local_bounds_id: __flight_source.local_bounds_id,
            local_bounds_rectangle: (__flight_source.local_bounds_rectangle).clone(),
            local_bounds_using_local_bounds_id: __flight_source.local_bounds_using_local_bounds_id,
            local_content_id: __flight_source.local_content_id,
            local_matrix: (__flight_source.local_matrix).clone(),
            local_matrix4: (__flight_source.local_matrix4).clone(),
            local_matrix4_detached: __flight_source.local_matrix4_detached,
            local_transform_id: __flight_source.local_transform_id,
            local_transform_using_local_transform_id: __flight_source
                .local_transform_using_local_transform_id,
            mask_write_mode: __flight_source.mask_write_mode,
            material_bitmap_shader_map: (__flight_source.material_bitmap_shader_map).clone(),
            matrix_array: (__flight_source.matrix_array).clone(),
            max_anisotropy: __flight_source.max_anisotropy,
            measured_height: __flight_source.measured_height,
            measured_width: __flight_source.measured_width,
            mipmap_bind_group_layout: (__flight_source.mipmap_bind_group_layout).clone(),
            mipmapped_textures: (__flight_source.mipmapped_textures).clone(),
            mipmap_pipeline: (__flight_source.mipmap_pipeline).clone(),
            morph_bind_pose: (__flight_source.morph_bind_pose).clone(),
            movie_clip_signals: (__flight_source.movie_clip_signals).clone(),
            nearest_sampler: (__flight_source.nearest_sampler).clone(),
            node_signals: (__flight_source.node_signals).clone(),
            particle_corner_buffer: (__flight_source.particle_corner_buffer).clone(),
            particle_instance_capacity: __flight_source.particle_instance_capacity,
            particle_shader: (__flight_source.particle_shader).clone(),
            pipeline_cache: (__flight_source.pipeline_cache).clone(),
            quad_batch_corner_buffer: (__flight_source.quad_batch_corner_buffer).clone(),
            quad_batches: (__flight_source.quad_batches).clone(),
            quad_batch_shader: (__flight_source.quad_batch_shader).clone(),
            quad_index_buffer: (__flight_source.quad_index_buffer).clone(),
            quad_vertex_buffer: (__flight_source.quad_vertex_buffer).clone(),
            quad_vertex_data: (__flight_source.quad_vertex_data).clone(),
            render_adapt_hook: (__flight_source.render_adapt_hook).clone(),
            renderer_map: (__flight_source.renderer_map).clone(),
            renderer_map_id: __flight_source.renderer_map_id,
            render_pass: (__flight_source.render_pass).clone(),
            render_proxy_adapter_map: (__flight_source.render_proxy_adapter_map).clone(),
            render_proxy_map: (__flight_source.render_proxy_map).clone(),
            render_target_stack: (__flight_source.render_target_stack).clone(),
            resolved_color_transform: (__flight_source.resolved_color_transform).clone(),
            retired_buffers: (__flight_source.retired_buffers).clone(),
            rich_text_content: (__flight_source.rich_text_content).clone(),
            rotation_angle: __flight_source.rotation_angle,
            rotation_cosine: __flight_source.rotation_cosine,
            rotation_sine: __flight_source.rotation_sine,
            sampler_cache: (__flight_source.sampler_cache).clone(),
            scene_mesh_upload_cache: (__flight_source.scene_mesh_upload_cache).clone(),
            selection_begin_index: __flight_source.selection_begin_index,
            selection_end_index: __flight_source.selection_end_index,
            shader_loc: (__flight_source.shader_loc).clone(),
            shape_mesh_color_transform_shader: (__flight_source.shape_mesh_color_transform_shader)
                .clone(),
            shape_mesh_pipelines: (__flight_source.shape_mesh_pipelines).clone(),
            skin_bind_pose: (__flight_source.skin_bind_pose).clone(),
            sprite_batch_blend_mode: (__flight_source.sprite_batch_blend_mode).clone(),
            sprite_batch_buffer_cursor: __flight_source.sprite_batch_buffer_cursor,
            sprite_batch_buffer_pool: (__flight_source.sprite_batch_buffer_pool).clone(),
            sprite_batch_color_transform_buffer: (__flight_source
                .sprite_batch_color_transform_buffer)
                .clone(),
            sprite_batch_color_transform_data: (__flight_source.sprite_batch_color_transform_data)
                .clone(),
            sprite_batch_color_transform_mode: __flight_source.sprite_batch_color_transform_mode,
            sprite_batch_count: __flight_source.sprite_batch_count,
            sprite_batch_instance_buffer: (__flight_source.sprite_batch_instance_buffer).clone(),
            sprite_batch_instance_data: (__flight_source.sprite_batch_instance_data).clone(),
            sprite_batch_material: (__flight_source.sprite_batch_material).clone(),
            sprite_batch_material_buffer: (__flight_source.sprite_batch_material_buffer).clone(),
            sprite_batch_material_data: (__flight_source.sprite_batch_material_data).clone(),
            sprite_batch_material_floats: __flight_source.sprite_batch_material_floats,
            sprite_batch_texture: (__flight_source.sprite_batch_texture).clone(),
            sprite_batch_uniform_color_transform: (__flight_source
                .sprite_batch_uniform_color_transform)
                .clone(),
            stage: (__flight_source.stage).clone(),
            stage_signals: (__flight_source.stage_signals).clone(),
            temp_stack: (__flight_source.temp_stack).clone(),
            text_field_signals: (__flight_source.text_field_signals).clone(),
            text_layout: (__flight_source.text_layout).clone(),
            text_layout_using_content_id: __flight_source.text_layout_using_content_id,
            texture_bind_group_layout: (__flight_source.texture_bind_group_layout).clone(),
            uniform_bind_group: (__flight_source.uniform_bind_group).clone(),
            uniform_bind_group_layout: (__flight_source.uniform_bind_group_layout).clone(),
            uniform_buffer: (__flight_source.uniform_buffer).clone(),
            uniform_color_transform_shader: (__flight_source.uniform_color_transform_shader)
                .clone(),
            uniform_data: (__flight_source.uniform_data).clone(),
            uniform_data_u32: (__flight_source.uniform_data_u32).clone(),
            uniform_offset: __flight_source.uniform_offset,
            uniform_stride: __flight_source.uniform_stride,
            webgl_data: (__flight_source.webgl_data).clone(),
            webgl_shader_binding_resolver: (__flight_source.webgl_shader_binding_resolver).clone(),
            webgpu_data: (__flight_source.webgpu_data).clone(),
            webgpu_shader_binding_resolver: (__flight_source.webgpu_shader_binding_resolver)
                .clone(),
            wgpu_color_adjustment_fold: (__flight_source.wgpu_color_adjustment_fold).clone(),
            wgpu_color_adjustment_guard: (__flight_source.wgpu_color_adjustment_guard).clone(),
            world_alpha: __flight_source.world_alpha,
            world_alpha_using_appearance_id: __flight_source.world_alpha_using_appearance_id,
            world_alpha_using_parent_appearance_id: __flight_source
                .world_alpha_using_parent_appearance_id,
            world_appearance_id: __flight_source.world_appearance_id,
            world_bounds_rectangle: (__flight_source.world_bounds_rectangle).clone(),
            world_bounds_using_local_bounds_id: __flight_source.world_bounds_using_local_bounds_id,
            world_bounds_using_world_transform_id: __flight_source
                .world_bounds_using_world_transform_id,
            world_matrix: (__flight_source.world_matrix).clone(),
            world_matrix4: (__flight_source.world_matrix4).clone(),
            world_transform_id: __flight_source.world_transform_id,
            world_transform_using_local_transform_id: __flight_source
                .world_transform_using_local_transform_id,
            world_transform_using_parent_transform_id: __flight_source
                .world_transform_using_parent_transform_id,
            can_add_child: (__flight_source.can_add_child).clone(),
            children: (__flight_source.children).clone(),
            traits: (__flight_source.traits).clone(),
            parent: (__flight_source.parent).clone(),
        }
    };
    let mut bounds = ensure_bounds_rectangle((runtime).clone());
    for quad_batch in ((runtime.inner.lock().unwrap().quad_batches).clone())
        .iter()
        .cloned()
    {
        clear_quad_batch(&mut quad_batch);
        if ((quad_batch.data.atlas).clone()).is_some() {
            quad_batch.data.atlas.as_mut().unwrap().regions.clear();
        }
        apply_bitmap_text_color(&quad_batch, bitmap_text.data.color);
    }
    let glyph_source = (bitmap_text.data.glyph_source).clone();
    if ((glyph_source).is_none())
        || ((bitmap_text.data.text.encode_utf16().count() as f64) == 0.0_f64)
    {
        set_empty_rectangle(&mut bounds);
        invalidate_node_local_bounds(bitmap_text);
        return;
    }
    let metrics = {
        let __flight_callback = (glyph_source.as_ref().unwrap().get_glyph_metrics).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
    let line_advance =
        (((metrics.ascent + metrics.descent) + metrics.line_gap) * bitmap_text.data.line_height);
    let lines = layout_bitmap_text_lines(glyph_source.as_ref().unwrap(), &bitmap_text.data);
    let ref_width = (bitmap_text.data.wrap_width).unwrap_or(max_line_width(&lines));
    let mut pages: Vec<(f64, BitmapTextPageBatch)> = Vec::new();
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = (-f64::INFINITY);
    let mut max_y = (-f64::INFINITY);
    {
        let mut li = 0.0_f64;
        while (li < (lines.len() as f64)) {
            let line = lines[li as usize].clone();
            let baseline_y = (metrics.ascent + (li * line_advance));
            let mut start_x = 0.0_f64;
            let mut gap_extra = 0.0_f64;
            if ((bitmap_text.data.align).clone() == "center") {
                start_x = ((ref_width - line.width) / 2.0_f64);
            } else {
                if ((bitmap_text.data.align).clone() == "right") {
                    start_x = (ref_width - line.width);
                } else {
                    if ((((bitmap_text.data.align).clone() == "justify")
                        && ((bitmap_text.data.wrap_width).is_some()))
                        && (!line.paragraph_end))
                        && ((line.gaps.len() as f64) > 0.0_f64)
                    {
                        gap_extra =
                            ((bitmap_text.data.wrap_width - line.width) / (line.gaps.len() as f64));
                    }
                }
            }
            let mut pen_x = start_x;
            {
                let mut wi = 0.0_f64;
                while (wi < (line.words.len() as f64)) {
                    if (wi > 0.0_f64) {
                        pen_x += (line.gaps[(wi - 1.0_f64) as usize].clone() + gap_extra);
                    }
                    let word = line.words[wi as usize].clone();
                    for glyph in ((word.glyphs).clone()).iter().cloned() {
                        let mut page = ensure_bitmap_text_page_batch(
                            bitmap_text,
                            (runtime).clone(),
                            glyph_source.as_ref().unwrap(),
                            bitmap_text.data.color,
                            &mut pages,
                            glyph.entry.page,
                        );
                        if (page).is_none() {
                            continue;
                        }
                        let quad_x = ((pen_x + glyph.pen_within_word) + glyph.entry.bearing_x);
                        let quad_y = (baseline_y - glyph.entry.bearing_y);
                        let mut region_id = page
                            .as_mut()
                            .unwrap()
                            .region_by_codepoint
                            .iter()
                            .find(|(key, _)| key == &glyph.codepoint)
                            .map(|(_, value)| value.clone());
                        if (region_id).is_none() {
                            add_texture_atlas_region(
                                &mut page.as_mut().unwrap().atlas,
                                glyph.entry.x,
                                glyph.entry.y,
                                glyph.entry.width,
                                glyph.entry.height,
                                None,
                                None,
                                None,
                            );
                            region_id = Some(
                                ((page.as_mut().unwrap().atlas.regions.len() as f64) - 1.0_f64),
                            );
                            {
                                let __flight_key = glyph.codepoint;
                                let __flight_value = (region_id).clone().unwrap();
                                if let Some((_, value)) = page
                                    .as_mut()
                                    .unwrap()
                                    .region_by_codepoint
                                    .iter_mut()
                                    .find(|(key, _)| key == &__flight_key)
                                {
                                    *value = __flight_value;
                                } else {
                                    page.as_mut()
                                        .unwrap()
                                        .region_by_codepoint
                                        .push((__flight_key, __flight_value));
                                }
                            };
                        }
                        append_quad_batch_instance(
                            &mut page.as_mut().unwrap().quad_batch,
                            (region_id).clone().unwrap(),
                            quad_x,
                            quad_y,
                        );
                        if (quad_x < min_x) {
                            min_x = quad_x;
                        }
                        if (quad_y < min_y) {
                            min_y = quad_y;
                        }
                        if ((quad_x + glyph.entry.width) > max_x) {
                            max_x = (quad_x + glyph.entry.width);
                        }
                        if ((quad_y + glyph.entry.height) > max_y) {
                            max_y = (quad_y + glyph.entry.height);
                        }
                    }
                    pen_x += word.width;
                    {
                        wi += 1.0;
                        wi
                    };
                }
            }
            {
                li += 1.0;
                li
            };
        }
    }
    if (min_x == f64::INFINITY) {
        set_empty_rectangle(&mut bounds);
    } else {
        bounds.x = min_x;
        bounds.y = min_y;
        bounds.width = (max_x - min_x);
        bounds.height = (max_y - min_y);
    }
    invalidate_node_local_bounds(bitmap_text);
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:116 (sha256:3ae1df5b1514f0d988011929cfbab2cccae11bc3e085653db3e58a62480dc212)
fn apply_bitmap_text_color(quad_batch: &QuadBatch, color: f64) -> () {
    if (color == BITMAP_TEXT_DEFAULT_COLOR) {
        set_display_object_color_adjustments(quad_batch, None);
        return;
    }
    let color_transform = create_color_transform(Some(FlightPartialRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        red_multiplier: Some(
            ((__flight_js_to_i32(
                (__flight_js_to_u32(color) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
            ) & __flight_js_to_i32(255.0_f64)) as f64
                / 255.0_f64),
        ),
        green_multiplier: Some(
            ((__flight_js_to_i32(
                (__flight_js_to_u32(color) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
            ) & __flight_js_to_i32(255.0_f64)) as f64
                / 255.0_f64),
        ),
        blue_multiplier: Some(
            ((__flight_js_to_i32(
                (__flight_js_to_u32(color) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
            ) & __flight_js_to_i32(255.0_f64)) as f64
                / 255.0_f64),
        ),
        alpha_multiplier: Some(
            ((__flight_js_to_i32(color) & __flight_js_to_i32(255.0_f64)) as f64 / 255.0_f64),
        ),
        alpha_offset: None,
        blue_offset: None,
        green_offset: None,
        red_offset: None,
    }));
    set_display_object_color_adjustments(
        quad_batch,
        Some((vec![create_color_transform_adjustment(&color_transform)]).clone()),
    );
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:133 (sha256:4f3667584f9d957b98c2e77b16859c5a22f92185621203b3ac759aa1ee2e5efb)
fn build_bitmap_text_words(
    glyph_source: &GlyphSource,
    paragraph: String,
    letter_spacing: f64,
) -> Vec<BitmapTextToken> {
    let tokens: std::sync::Arc<std::sync::Mutex<Vec<BitmapTextToken>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let pending_gap: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let glyphs: std::sync::Arc<std::sync::Mutex<Vec<BitmapTextGlyph>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let pen: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let previous_codepoint: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new((-1.0_f64)));
    let in_word: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(false));
    let mut flush: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut glyphs = glyphs.clone();
            let mut in_word = in_word.clone();
            let mut pen = pen.clone();
            let mut pending_gap = pending_gap.clone();
            let mut previous_codepoint = previous_codepoint.clone();
            let mut tokens = tokens.clone();
            move || -> () {
                if (!(*in_word.lock().unwrap()).clone()) {
                    return;
                }
                (*tokens.lock().unwrap()).push(BitmapTextToken {
                    __flight_identity: std::sync::Arc::new(()),
                    gap: (*pending_gap.lock().unwrap()).clone(),
                    word: BitmapTextWord {
                        __flight_identity: std::sync::Arc::new(()),
                        glyphs: (*glyphs.lock().unwrap()).clone(),
                        width: (*pen.lock().unwrap()).clone(),
                    },
                });
                (*pending_gap.lock().unwrap()) = 0.0_f64;
                (*glyphs.lock().unwrap()) = vec![];
                (*pen.lock().unwrap()) = 0.0_f64;
                (*previous_codepoint.lock().unwrap()) = (-1.0_f64);
                (*in_word.lock().unwrap()) = false;
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    for character in (paragraph).iter().cloned() {
        let codepoint = (character.code_point_at)(0.0_f64);
        if ((codepoint).is_none()) || (codepoint == CARRIAGE_RETURN) {
            continue;
        }
        if (codepoint == SPACE) {
            {
                let __flight_callback = (flush).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            };
            let space_entry = {
                let __flight_callback = (glyph_source.get_glyph_entry).clone();
                let __flight_result = __flight_callback.lock().unwrap()(SPACE);
                __flight_result
            };
            (*pending_gap.lock().unwrap()) += (if (space_entry).is_some() {
                space_entry.as_ref().unwrap().advance
            } else {
                0.0_f64
            } + letter_spacing);
            continue;
        }
        let entry = {
            let __flight_callback = (glyph_source.get_glyph_entry).clone();
            let __flight_result = __flight_callback.lock().unwrap()(codepoint);
            __flight_result
        };
        if (entry).is_none() {
            continue;
        }
        if ((*previous_codepoint.lock().unwrap()).clone() >= 0.0_f64) {
            (*pen.lock().unwrap()) += {
                let __flight_callback = (glyph_source.get_glyph_kerning).clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    (*previous_codepoint.lock().unwrap()).clone(),
                    codepoint,
                );
                __flight_result
            };
        }
        if (entry.as_ref().unwrap().width > 0.0_f64) && (entry.as_ref().unwrap().height > 0.0_f64) {
            (*glyphs.lock().unwrap()).push(BitmapTextGlyph {
                __flight_identity: std::sync::Arc::new(()),
                codepoint: codepoint,
                entry: (entry).clone().unwrap(),
                pen_within_word: (*pen.lock().unwrap()).clone(),
            });
        }
        (*pen.lock().unwrap()) += (entry.as_ref().unwrap().advance + letter_spacing);
        (*previous_codepoint.lock().unwrap()) = codepoint;
        (*in_word.lock().unwrap()) = true;
    }
    {
        let __flight_callback = (flush).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
    return (*tokens.lock().unwrap()).clone();
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:177 (sha256:6204caf477405079ed4f0f65be3ee86fe97d7be8e1103f06325b4fe3027cd8b5)
fn ensure_bitmap_text_page_batch(
    bitmap_text: &BitmapText,
    mut runtime: BitmapTextRuntime,
    glyph_source: &GlyphSource,
    color: f64,
    pages: &mut Vec<(f64, BitmapTextPageBatch)>,
    page: f64,
) -> Option<BitmapTextPageBatch> {
    let cached = pages
        .iter()
        .find(|(key, _)| key == &page)
        .map(|(_, value)| value.clone());
    if (cached).is_some() {
        return Some((cached.as_ref().unwrap()).clone());
    }
    let image = {
        let __flight_callback = (glyph_source.get_glyph_atlas_image).clone();
        let __flight_result = __flight_callback.lock().unwrap()(Some(page));
        __flight_result
    };
    if (image).is_none() {
        return None;
    }
    while ((runtime.inner.lock().unwrap().quad_batches.len() as f64) <= page) {
        let created = create_quad_batch(Some(QuadBatch {
            __flight_identity: std::sync::Arc::new(()),
            __flight_entity_runtime: Default::default(),
            data: QuadBatchData {
                __flight_identity: std::sync::Arc::new(()),
                atlas: Some(create_texture_atlas(None)),
            },
        }));
        apply_bitmap_text_color(&created, color);
        runtime
            .inner
            .lock()
            .unwrap()
            .quad_batches
            .push(((created).clone()).clone());
        add_node_child(bitmap_text, &created);
    }
    let mut quad_batch = runtime.inner.lock().unwrap().quad_batches[page as usize].clone();
    let mut atlas = (quad_batch.data.atlas).clone();
    atlas.as_mut().unwrap().image = Some((image.as_ref().unwrap()).clone());
    let mut page_batch: BitmapTextPageBatch = BitmapTextPageBatch {
        __flight_identity: std::sync::Arc::new(()),
        atlas: (atlas).clone().unwrap(),
        quad_batch: (quad_batch).clone(),
        region_by_codepoint: Vec::new(),
    };
    {
        let __flight_key = page;
        let __flight_value = (page_batch).clone();
        if let Some((_, value)) = pages.iter_mut().find(|(key, _)| key == &__flight_key) {
            *value = __flight_value;
        } else {
            pages.push((__flight_key, __flight_value));
        }
    };
    return Some((page_batch).clone());
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:205 (sha256:3a3b63bd630db2bd5b2173e069198940411e4fa8618ab60eed8210d1ef0855cb)
fn ensure_bounds_rectangle(mut runtime: BitmapTextRuntime) -> Rectangle {
    if ((runtime.inner.lock().unwrap().local_bounds_rectangle).clone()).is_none() {
        {
            let __flight_runtime = runtime;
            let __flight_value = Some(create_rectangle(None, None, None, None));
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.local_bounds_rectangle = __flight_value;
        };
    }
    return ((runtime.inner.lock().unwrap().local_bounds_rectangle).clone()).unwrap();
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:213 (sha256:37a28d4828e8076d45bbf780947328d6faebfad02cf933c4140e5b2eed58534d)
fn layout_bitmap_text_lines(
    glyph_source: &GlyphSource,
    data: &BitmapTextData,
) -> Vec<BitmapTextLine> {
    let mut lines: Vec<BitmapTextLine> = vec![];
    let paragraphs = ((data.text).clone())
        .split("\n".to_owned().as_str())
        .map(|part| part.to_owned())
        .collect::<Vec<_>>();
    {
        let mut pi = 0.0_f64;
        while (pi < (paragraphs.len() as f64)) {
            let tokens = build_bitmap_text_words(
                glyph_source,
                paragraphs[pi as usize].clone(),
                data.letter_spacing,
            );
            let mut current: BitmapTextLine = BitmapTextLine {
                __flight_identity: std::sync::Arc::new(()),
                words: vec![],
                gaps: vec![],
                width: 0.0_f64,
                paragraph_end: false,
            };
            for token in (tokens).iter().cloned() {
                let wraps = (((data.wrap_width).is_some())
                    && ((current.words.len() as f64) > 0.0_f64))
                    && (((current.width + token.gap) + token.word.width) > data.wrap_width);
                if wraps {
                    lines.push(((current).clone()).clone());
                    current = BitmapTextLine {
                        __flight_identity: std::sync::Arc::new(()),
                        words: vec![(token.word).clone()],
                        gaps: vec![],
                        width: token.word.width,
                        paragraph_end: false,
                    };
                } else {
                    if ((current.words.len() as f64) > 0.0_f64) {
                        current.gaps.push(token.gap);
                        current.width += token.gap;
                    }
                    current.words.push((token.word).clone());
                    current.width += token.word.width;
                }
            }
            current.paragraph_end = true;
            lines.push(((current).clone()).clone());
            {
                pi += 1.0;
                pi
            };
        }
    }
    return lines;
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:242 (sha256:d2a73c20d95a1a1512ff1f0278c6bd97e0108f4d220b7013c3ed2c4ffbff3066)
fn max_line_width(lines: &Vec<BitmapTextLine>) -> f64 {
    let mut max = 0.0_f64;
    for line in (lines).iter().cloned() {
        if (line.width > max) {
            max = line.width;
        }
    }
    return max;
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:248 (sha256:a394c69d44085bed7f8938b68dae56d8b1073fef074f7cca0b7872f173cda766)
fn set_empty_rectangle(out: &mut Rectangle) -> () {
    out.x = 0.0_f64;
    out.y = 0.0_f64;
    out.width = 0.0_f64;
    out.height = 0.0_f64;
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:257 (sha256:addde9091e717fe715e2337f95e4c64b535673b79412c3fccda76136eb05f90a)
#[derive(Clone, Default)]
struct BitmapTextGlyph {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub codepoint: f64,
    pub entry: GlyphEntry,
    pub pen_within_word: f64,
}
impl PartialEq for BitmapTextGlyph {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:265 (sha256:f6f1145192267bf5628b287995e2a28d9e834b5c7c96a7c9d16b90e38ccf38eb)
#[derive(Clone, Default)]
struct BitmapTextPageBatch {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub atlas: TextureAtlas,
    pub quad_batch: QuadBatch,
    pub region_by_codepoint: Vec<(f64, f64)>,
}
impl PartialEq for BitmapTextPageBatch {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:273 (sha256:e16d053098a238a17b8aff009c25ee0710c6f3819a3a825092cf06b4ed9783c0)
#[derive(Clone, Default)]
struct BitmapTextLine {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub gaps: Vec<f64>,
    pub paragraph_end: bool,
    pub width: f64,
    pub words: Vec<BitmapTextWord>,
}
impl PartialEq for BitmapTextLine {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:281 (sha256:2dafecf5ba6b88df3930aa9c085040985a8191db73398e385aacc144cc6698c4)
#[derive(Clone, Default)]
struct BitmapTextToken {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub gap: f64,
    pub word: BitmapTextWord,
}
impl PartialEq for BitmapTextToken {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:287 (sha256:3401bd7270b995d95bd0883e9b0104675fac441bbb5d1b368e2ff25dc110b1b9)
#[derive(Clone, Default)]
struct BitmapTextWord {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub glyphs: Vec<BitmapTextGlyph>,
    pub width: f64,
}
impl PartialEq for BitmapTextWord {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
