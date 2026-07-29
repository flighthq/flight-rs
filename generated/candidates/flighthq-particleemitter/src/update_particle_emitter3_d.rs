// @generated from upstream/packages/particleemitter/src/updateParticleEmitter3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::reserve_particle_emitter3_d;
use flighthq_node::{get_node_local_matrix4, get_node_world_matrix4};
use flighthq_particles::{
    PARTICLE_VELOCITY_STRIDE as particle_velocity_stride_constant,
    ensure_particle_emitter_state_capacity, get_particle_emitter_signals,
    sample_particle_color_curve, sample_particle_curve,
};
use flighthq_types::{
    Adjustment, AdjustmentKind, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform,
    ImageResource, InteractionSignals, Kind, Material, MaterialData, Matrix, Matrix4, Node,
    NodeData, NodeInteractionState, NodeSignals, NodeTraitsKey, ParticleEmitter3D,
    ParticleEmitterCallbacks, ParticleEmitterConfig, ParticleEmitterState, Quaternion, Rectangle,
    Stage, StageSignals, TextureAtlas, Transform3DNode, Vector3,
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
    pub alphas: Option<Vec<f32>>,
    pub atlas: Option<TextureAtlas>,
    pub colors: Option<Vec<f32>>,
    pub ids: Option<Vec<u16>>,
    pub particle_count: Option<f64>,
    pub positions_z: Option<Vec<f32>>,
    pub transforms: Option<Vec<f32>>,
    pub velocities: Option<Vec<f32>>,
    pub world_space: Option<bool>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub data: Option<NodeData>,
    pub enabled: Option<bool>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
    pub position: Option<Vector3>,
    pub rotation: Option<Quaternion>,
    pub scale: Option<Vector3>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub color_matrix: Option<Vec<f64>>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord7 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord8 {
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for FlightPartialRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord9 {
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
impl PartialEq for FlightPartialRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord10 {
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for FlightPartialRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord11 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: Option<ClipRegion>,
}
impl PartialEq for FlightPartialRecord11 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord12 {
    pub __flight_identity: std::sync::Arc<()>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
}
impl PartialEq for FlightPartialRecord12 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord13 {
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
impl PartialEq for FlightPartialRecord13 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord14 {
    pub __flight_identity: std::sync::Arc<()>,
    pub position: Option<Vector3>,
    pub rotation: Option<Quaternion>,
    pub scale: Option<Vector3>,
}
impl PartialEq for FlightPartialRecord14 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/particleemitter/src/updateParticleEmitter3D.ts:19 (sha256:c3dc807b578ac94141dd73c6a0532f43a0d50d26a1aa7884792f64f943d23ca6)
const PARTICLE_TRANSFORM_STRIDE: f64 = 4.0_f64;

// Source: upstream/packages/particleemitter/src/updateParticleEmitter3D.ts:20 (sha256:eed66b9413dd1f3589bacb1e13051ec02a5deec60c911cd17bad429c276ac74b)
const TWO_PI: f64 = 6.283185307179586_f64;

// Source: upstream/packages/particleemitter/src/updateParticleEmitter3D.ts:22 (sha256:6d9b35516c9b0a99a03f8312e896f872ec54601828e69c660d6ef7959bdcef0e)
pub fn is_particle_emitter3_d_complete(
    emitter: &ParticleEmitter3D,
    state: &ParticleEmitterState,
    config: &ParticleEmitterConfig,
) -> bool {
    if (config.duration <= 0.0_f64) || (config.loop_) {
        return false;
    }
    return (state.emitter_age >= config.duration) && (emitter.data.particle_count == 0.0_f64);
}

// Source: upstream/packages/particleemitter/src/updateParticleEmitter3D.ts:31 (sha256:e0c225ef7b89615e590a9ca4c09617b0a80f425bf3c179ad015ea08eb69f83c0)
fn is_emitting(config: &ParticleEmitterConfig, emitter_age: f64) -> bool {
    return ((config.duration <= 0.0_f64) || (config.loop_)) || (emitter_age < config.duration);
}

// Source: upstream/packages/particleemitter/src/updateParticleEmitter3D.ts:35 (sha256:531c37e4bde2d468f624ad2cadf8bb9ede30a8c0e92c049219c6f96bab8bf70e)
pub fn update_particle_emitter3_d(
    emitter: &mut ParticleEmitter3D,
    state: &mut ParticleEmitterState,
    config: &ParticleEmitterConfig,
    delta_time: f64,
    callbacks: Option<ParticleEmitterCallbacks>,
) -> () {
    let world_m = if config.world_space {
        Some(
            (get_node_world_matrix4(&{
                let __flight_source = &(emitter);
                Transform3DNode {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    __flight_entity_runtime: std::sync::Arc::clone(
                        &__flight_source.__flight_entity_runtime,
                    ),
                    data: (__flight_source.data).clone(),
                    enabled: __flight_source.enabled,
                    kind: (__flight_source.kind).clone(),
                    name: (__flight_source.name).clone(),
                    position: (__flight_source.position).clone(),
                    rotation: (__flight_source.rotation).clone(),
                    scale: (__flight_source.scale).clone(),
                }
            })
            .m)
                .clone(),
        )
    } else {
        None
    };
    emitter.data.world_space = (world_m).is_some();
    if (delta_time <= 0.0_f64) {
        return;
    }
    let origin_m = if (world_m).is_some() {
        (world_m.as_ref().unwrap()).clone()
    } else {
        (get_node_local_matrix4(&{
            let __flight_source = &(emitter);
            Transform3DNode {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                data: (__flight_source.data).clone(),
                enabled: __flight_source.enabled,
                kind: (__flight_source.kind).clone(),
                name: (__flight_source.name).clone(),
                position: (__flight_source.position).clone(),
                rotation: (__flight_source.rotation).clone(),
                scale: (__flight_source.scale).clone(),
            }
        })
        .m)
            .clone()
    };
    let track_x = (origin_m[12.0_f64 as usize] as f64);
    let track_y = (origin_m[13.0_f64 as usize] as f64);
    let track_z = (origin_m[14.0_f64 as usize] as f64);
    let has_vel_inherit = (config.velocity_inheritance != 0.0_f64);
    let mut emitter_vel_x = 0.0_f64;
    let mut emitter_vel_y = 0.0_f64;
    let mut emitter_vel_z = 0.0_f64;
    if (!(state.prev_x).is_nan()) {
        emitter_vel_x = ((track_x - state.prev_x) / delta_time);
        emitter_vel_y = ((track_y - state.prev_y) / delta_time);
        emitter_vel_z = ((track_z - state.prev_z) / delta_time);
    }
    let gx = (config.gravity_x * delta_time);
    let gy = (config.gravity_y * delta_time);
    let gz = (config.gravity_z * delta_time);
    let color_start_r = config.color_start_r;
    let color_start_g = config.color_start_g;
    let color_start_b = config.color_start_b;
    let color_end_r = config.color_end_r;
    let color_end_g = config.color_end_g;
    let color_end_b = config.color_end_b;
    let has_color_variance = (((((config.color_start_variance_r != 0.0_f64)
        || (config.color_start_variance_g != 0.0_f64))
        || (config.color_start_variance_b != 0.0_f64))
        || (config.color_end_variance_r != 0.0_f64))
        || (config.color_end_variance_g != 0.0_f64))
        || (config.color_end_variance_b != 0.0_f64);
    let has_color_gradient = (((has_color_variance) || (color_start_r != color_end_r))
        || (color_start_g != color_end_g))
        || (color_start_b != color_end_b);
    let alpha_curve = (config.alpha_curve).clone();
    let color_curve = (config.color_curve).clone();
    let scale_curve = (config.scale_curve).clone();
    let has_alpha_curve =
        ((alpha_curve).is_some()) && (alpha_curve.as_ref().unwrap().length > 0.0_f64);
    let has_color_curve =
        ((color_curve).is_some()) && (color_curve.as_ref().unwrap().length >= 3.0_f64);
    let has_scale_curve =
        ((scale_curve).is_some()) && (scale_curve.as_ref().unwrap().length > 0.0_f64);
    let has_scale_anim = (config.scale_end != 1.0_f64) || (has_scale_curve);
    let has_color_work = (has_color_curve) || (has_color_gradient);
    let has_rotation_speed =
        (config.rotation_speed_min != 0.0_f64) || (config.rotation_speed_max != 0.0_f64);
    let has_flipbook = (config.frame_count > 1.0_f64);
    let signals = get_particle_emitter_signals((state).clone());
    let on_death = callbacks
        .as_ref()
        .and_then(|value| (value.on_death).clone());
    let on_spawn = callbacks
        .as_ref()
        .and_then(|value| (value.on_spawn).clone());
    let mut live_count = emitter.data.particle_count;
    let mut i = 0.0_f64;
    while (i < live_count) {
        let lt = (i * 2.0_f64);
        state.lifetimes[lt as usize] += (delta_time) as f32;
        if ((state.lifetimes[lt as usize] as f64)
            >= (state.lifetimes[(lt + 1.0_f64) as usize] as f64))
        {
            if ((on_death).is_some()) || ((signals).is_some()) {
                let tt = (i * PARTICLE_TRANSFORM_STRIDE);
                let dx = (emitter.data.transforms[tt as usize] as f64);
                let dy = (emitter.data.transforms[(tt + 1.0_f64) as usize] as f64);
                let dz = (emitter.data.positions_z[i as usize] as f64);
                {
                    let __flight_callback = on_death;
                    __flight_callback
                        .as_ref()
                        .map(|callback| callback.lock().unwrap()(dx, dy, dz))
                };
                {
                    let __flight_callback = signals
                        .as_ref()
                        .unwrap()
                        .on_particle_death
                        .as_ref()
                        .unwrap()
                        .emit
                        .as_ref()
                        .unwrap()
                        .clone();
                    let __flight_result = __flight_callback.lock().unwrap()(dx, dy, dz);
                    __flight_result
                };
            }
            {
                live_count -= 1.0;
                live_count
            };
            if (i < live_count) {
                let lt2 = (live_count * 2.0_f64);
                state.lifetimes[lt as usize] = (state.lifetimes[lt2 as usize] as f64) as f32;
                state.lifetimes[(lt + 1.0_f64) as usize] =
                    (state.lifetimes[(lt2 + 1.0_f64) as usize] as f64) as f32;
                let vt = (i * particle_velocity_stride_constant);
                let vt2 = (live_count * particle_velocity_stride_constant);
                state.velocities[vt as usize] = (state.velocities[vt2 as usize] as f64) as f32;
                state.velocities[(vt + 1.0_f64) as usize] =
                    (state.velocities[(vt2 + 1.0_f64) as usize] as f64) as f32;
                state.velocities[(vt + 2.0_f64) as usize] =
                    (state.velocities[(vt2 + 2.0_f64) as usize] as f64) as f32;
                let tt = (i * PARTICLE_TRANSFORM_STRIDE);
                let tt2 = (live_count * PARTICLE_TRANSFORM_STRIDE);
                emitter.data.transforms[tt as usize] =
                    (emitter.data.transforms[tt2 as usize] as f64) as f32;
                emitter.data.transforms[(tt + 1.0_f64) as usize] =
                    (emitter.data.transforms[(tt2 + 1.0_f64) as usize] as f64) as f32;
                emitter.data.transforms[(tt + 2.0_f64) as usize] =
                    (emitter.data.transforms[(tt2 + 2.0_f64) as usize] as f64) as f32;
                emitter.data.transforms[(tt + 3.0_f64) as usize] =
                    (emitter.data.transforms[(tt2 + 3.0_f64) as usize] as f64) as f32;
                emitter.data.positions_z[i as usize] =
                    (emitter.data.positions_z[live_count as usize] as f64) as f32;
                emitter.data.alphas[i as usize] =
                    (emitter.data.alphas[live_count as usize] as f64) as f32;
                emitter.data.ids[i as usize] =
                    (emitter.data.ids[live_count as usize] as f64) as u16;
                let ct = (i * 3.0_f64);
                let ct2 = (live_count * 3.0_f64);
                emitter.data.colors[ct as usize] =
                    (emitter.data.colors[ct2 as usize] as f64) as f32;
                emitter.data.colors[(ct + 1.0_f64) as usize] =
                    (emitter.data.colors[(ct2 + 1.0_f64) as usize] as f64) as f32;
                emitter.data.colors[(ct + 2.0_f64) as usize] =
                    (emitter.data.colors[(ct2 + 2.0_f64) as usize] as f64) as f32;
                state.scales[i as usize] = (state.scales[live_count as usize] as f64) as f32;
                state.rotation_speeds[i as usize] =
                    (state.rotation_speeds[live_count as usize] as f64) as f32;
                if has_color_variance {
                    state.color_birth[ct as usize] =
                        (state.color_birth[ct2 as usize] as f64) as f32;
                    state.color_birth[(ct + 1.0_f64) as usize] =
                        (state.color_birth[(ct2 + 1.0_f64) as usize] as f64) as f32;
                    state.color_birth[(ct + 2.0_f64) as usize] =
                        (state.color_birth[(ct2 + 2.0_f64) as usize] as f64) as f32;
                    state.color_death[ct as usize] =
                        (state.color_death[ct2 as usize] as f64) as f32;
                    state.color_death[(ct + 1.0_f64) as usize] =
                        (state.color_death[(ct2 + 1.0_f64) as usize] as f64) as f32;
                    state.color_death[(ct + 2.0_f64) as usize] =
                        (state.color_death[(ct2 + 2.0_f64) as usize] as f64) as f32;
                }
            }
            continue;
        }
        let vt = (i * particle_velocity_stride_constant);
        state.velocities[vt as usize] += (gx) as f32;
        state.velocities[(vt + 1.0_f64) as usize] += (gy) as f32;
        state.velocities[(vt + 2.0_f64) as usize] += (gz) as f32;
        let tt = (i * PARTICLE_TRANSFORM_STRIDE);
        emitter.data.transforms[tt as usize] +=
            ((state.velocities[vt as usize] as f64) * delta_time) as f32;
        emitter.data.transforms[(tt + 1.0_f64) as usize] +=
            ((state.velocities[(vt + 1.0_f64) as usize] as f64) * delta_time) as f32;
        emitter.data.positions_z[i as usize] +=
            ((state.velocities[(vt + 2.0_f64) as usize] as f64) * delta_time) as f32;
        let life_fraction = ((state.lifetimes[lt as usize] as f64)
            / (state.lifetimes[(lt + 1.0_f64) as usize] as f64));
        emitter.data.alphas[i as usize] = if has_alpha_curve {
            (sample_particle_curve(alpha_curve.as_ref().unwrap(), life_fraction)) as f32
        } else {
            (config.alpha_start + ((config.alpha_end - config.alpha_start) * life_fraction)) as f32
        };
        if has_color_work {
            let ct = (i * 3.0_f64);
            if has_color_curve {
                sample_particle_color_curve(
                    &mut emitter.data.colors,
                    ct,
                    color_curve.as_ref().unwrap(),
                    life_fraction,
                );
            } else {
                if has_color_variance {
                    emitter.data.colors[ct as usize] = ((state.color_birth[ct as usize] as f64)
                        + (((state.color_death[ct as usize] as f64)
                            - (state.color_birth[ct as usize] as f64))
                            * life_fraction))
                        as f32;
                    emitter.data.colors[(ct + 1.0_f64) as usize] =
                        ((state.color_birth[(ct + 1.0_f64) as usize] as f64)
                            + (((state.color_death[(ct + 1.0_f64) as usize] as f64)
                                - (state.color_birth[(ct + 1.0_f64) as usize] as f64))
                                * life_fraction)) as f32;
                    emitter.data.colors[(ct + 2.0_f64) as usize] =
                        ((state.color_birth[(ct + 2.0_f64) as usize] as f64)
                            + (((state.color_death[(ct + 2.0_f64) as usize] as f64)
                                - (state.color_birth[(ct + 2.0_f64) as usize] as f64))
                                * life_fraction)) as f32;
                } else {
                    emitter.data.colors[ct as usize] =
                        (color_start_r + ((color_end_r - color_start_r) * life_fraction)) as f32;
                    emitter.data.colors[(ct + 1.0_f64) as usize] =
                        (color_start_g + ((color_end_g - color_start_g) * life_fraction)) as f32;
                    emitter.data.colors[(ct + 2.0_f64) as usize] =
                        (color_start_b + ((color_end_b - color_start_b) * life_fraction)) as f32;
                }
            }
        }
        if has_scale_anim {
            let scale_factor = if has_scale_curve {
                sample_particle_curve(scale_curve.as_ref().unwrap(), life_fraction)
            } else {
                (1.0_f64 + ((config.scale_end - 1.0_f64) * life_fraction))
            };
            emitter.data.transforms[(tt + 3.0_f64) as usize] =
                ((state.scales[i as usize] as f64) * scale_factor) as f32;
        }
        if has_rotation_speed {
            emitter.data.transforms[(tt + 2.0_f64) as usize] +=
                ((state.rotation_speeds[i as usize] as f64) * delta_time) as f32;
        }
        if has_flipbook {
            let frame = (((state.lifetimes[lt as usize] as f64) * config.frame_rate).floor()
                % config.frame_count);
            emitter.data.ids[i as usize] = (config.region_id_min + frame) as u16;
        }
        {
            i += 1.0;
            i
        };
    }
    emitter.data.particle_count = live_count;
    let emitting = is_emitting(config, state.emitter_age);
    if (config.duration > 0.0_f64) && (!config.loop_) {
        state.emitter_age += delta_time;
    }
    state.spawn_accumulator += if emitting {
        (config.spawn_rate * delta_time)
    } else {
        0.0_f64
    };
    let mut to_spawn = (state.spawn_accumulator).floor();
    state.spawn_accumulator -= to_spawn;
    if (emitting) && (config.burst_count > 0.0_f64) {
        state.burst_timer -= delta_time;
        if (state.burst_timer <= 0.0_f64) {
            to_spawn += config.burst_count;
            state.burst_timer = if (config.burst_interval > 0.0_f64) {
                config.burst_interval
            } else {
                f64::INFINITY
            };
        }
    }
    let max_new = (config.max_particles - live_count);
    if (to_spawn > max_new) {
        to_spawn = max_new;
    }
    if (to_spawn > 0.0_f64) {
        let new_count = (live_count + to_spawn);
        reserve_particle_emitter3_d(emitter, new_count);
        ensure_particle_emitter_state_capacity(state, new_count, has_color_variance);
        let base_angle = (config.direction_y).atan2(config.direction_x);
        let region_range = (config.region_id_max - config.region_id_min);
        let region_id_min = config.region_id_min;
        let rot_speed_range = (config.rotation_speed_max - config.rotation_speed_min);
        let has_rot_speed =
            (config.rotation_speed_min != 0.0_f64) || (config.rotation_speed_max != 0.0_f64);
        let dir_len = (((config.direction_x * config.direction_x)
            + (config.direction_y * config.direction_y))
            + (config.direction_z * config.direction_z))
            .sqrt();
        let dir_nx = if (dir_len > 0.000001_f64) {
            (config.direction_x / dir_len)
        } else {
            0.0_f64
        };
        let dir_ny = if (dir_len > 0.000001_f64) {
            (config.direction_y / dir_len)
        } else {
            (-1.0_f64)
        };
        let dir_nz = if (dir_len > 0.000001_f64) {
            (config.direction_z / dir_len)
        } else {
            0.0_f64
        };
        let do_trail = ((world_m).is_some()) && (!(state.prev_x).is_nan());
        let prev_path_x = if do_trail { state.prev_x } else { track_x };
        let prev_path_y = if do_trail { state.prev_y } else { track_y };
        let prev_path_z = if do_trail { state.prev_z } else { track_z };
        {
            let mut s_idx = 0.0_f64;
            while (s_idx < to_spawn) {
                let idx = (live_count + s_idx);
                let lifetime = (config.lifetime_min
                    + ({
                        let __flight_callback = (state.random).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    } * (config.lifetime_max - config.lifetime_min)));
                let lt = (idx * 2.0_f64);
                state.lifetimes[lt as usize] = (0.0_f64) as f32;
                state.lifetimes[(lt + 1.0_f64) as usize] = (lifetime) as f32;
                let speed = (config.speed_min
                    + ({
                        let __flight_callback = (state.random).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    } * (config.speed_max - config.speed_min)));
                let mut vx: f64;
                let mut vy: f64;
                let mut vz: f64;
                let mut spawn_x = 0.0_f64;
                let mut spawn_y = 0.0_f64;
                let mut spawn_z = 0.0_f64;
                let shape = (config.emitter_shape).clone();
                if (shape == "sphere") || (shape == "cone3d") {
                    let mut sx: f64;
                    let mut sy: f64;
                    let mut sz: f64;
                    if (shape == "cone3d") && (config.emitter_cone_angle > 0.0_f64) {
                        let cone_half = (config.emitter_cone_angle / 2.0_f64);
                        let cos_theta = (1.0_f64
                            - ({
                                let __flight_callback = (state.random).clone();
                                let __flight_result = __flight_callback.lock().unwrap()();
                                __flight_result
                            } * (1.0_f64 - (cone_half).cos())));
                        let sin_theta = (1.0_f64 - (cos_theta * cos_theta)).sqrt();
                        let phi = ({
                            let __flight_callback = (state.random).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        } * TWO_PI);
                        let lx = (sin_theta * (phi).cos());
                        let ly = (sin_theta * (phi).sin());
                        let lz = cos_theta;
                        let r_dir = rotate_to_direction(lx, ly, lz, dir_nx, dir_ny, dir_nz);
                        sx = r_dir[0.0_f64 as usize].clone();
                        sy = r_dir[1.0_f64 as usize].clone();
                        sz = r_dir[2.0_f64 as usize].clone();
                    } else {
                        let mut u: f64;
                        let mut v: f64;
                        let mut s2: f64;
                        loop {
                            {
                                u = (({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } * 2.0_f64)
                                    - 1.0_f64);
                                v = (({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } * 2.0_f64)
                                    - 1.0_f64);
                                s2 = ((u * u) + (v * v));
                            }
                            if !((s2 >= 1.0_f64) || (s2 == 0.0_f64)) {
                                break;
                            }
                        }
                        let f = (2.0_f64 * (1.0_f64 - s2).sqrt());
                        sx = (u * f);
                        sy = (v * f);
                        sz = (1.0_f64 - (2.0_f64 * s2));
                    }
                    vx = (sx * speed);
                    vy = (sy * speed);
                    vz = (sz * speed);
                    if (config.emitter_radius > 0.0_f64) {
                        let r = (({
                            let __flight_callback = (state.random).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        })
                        .cbrt()
                            * config.emitter_radius);
                        let mut pu: f64;
                        let mut pv: f64;
                        let mut ps2: f64;
                        loop {
                            {
                                pu = (({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } * 2.0_f64)
                                    - 1.0_f64);
                                pv = (({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } * 2.0_f64)
                                    - 1.0_f64);
                                ps2 = ((pu * pu) + (pv * pv));
                            }
                            if !((ps2 >= 1.0_f64) || (ps2 == 0.0_f64)) {
                                break;
                            }
                        }
                        let pf = (2.0_f64 * (1.0_f64 - ps2).sqrt());
                        spawn_x = ((pu * pf) * r);
                        spawn_y = ((pv * pf) * r);
                        spawn_z = ((1.0_f64 - (2.0_f64 * ps2)) * r);
                    }
                } else {
                    if (shape == "box") {
                        let angle = (base_angle
                            + ((({
                                let __flight_callback = (state.random).clone();
                                let __flight_result = __flight_callback.lock().unwrap()();
                                __flight_result
                            } - 0.5_f64)
                                * 2.0_f64)
                                * config.spread));
                        vx = ((angle).cos() * speed);
                        vy = ((angle).sin() * speed);
                        vz = ((config.direction_z * speed)
                            / if (dir_len > 0.000001_f64) {
                                dir_len
                            } else {
                                1.0_f64
                            });
                        spawn_x = (({
                            let __flight_callback = (state.random).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        } - 0.5_f64)
                            * config.emitter_width);
                        spawn_y = (({
                            let __flight_callback = (state.random).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        } - 0.5_f64)
                            * config.emitter_height);
                        spawn_z = (({
                            let __flight_callback = (state.random).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        } - 0.5_f64)
                            * config.emitter_depth);
                    } else {
                        let angle = (base_angle
                            + ((({
                                let __flight_callback = (state.random).clone();
                                let __flight_result = __flight_callback.lock().unwrap()();
                                __flight_result
                            } - 0.5_f64)
                                * 2.0_f64)
                                * config.spread));
                        vx = ((angle).cos() * speed);
                        vy = ((angle).sin() * speed);
                        vz = 0.0_f64;
                        if (shape == "circle") && (config.emitter_radius > 0.0_f64) {
                            let r = (({
                                let __flight_callback = (state.random).clone();
                                let __flight_result = __flight_callback.lock().unwrap()();
                                __flight_result
                            })
                            .sqrt()
                                * config.emitter_radius);
                            let a = ({
                                let __flight_callback = (state.random).clone();
                                let __flight_result = __flight_callback.lock().unwrap()();
                                __flight_result
                            } * TWO_PI);
                            spawn_x = ((a).cos() * r);
                            spawn_y = ((a).sin() * r);
                        } else {
                            if (shape == "rect")
                                && ((config.emitter_width > 0.0_f64)
                                    || (config.emitter_height > 0.0_f64))
                            {
                                spawn_x = (({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } - 0.5_f64)
                                    * config.emitter_width);
                                spawn_y = (({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } - 0.5_f64)
                                    * config.emitter_height);
                            }
                        }
                    }
                }
                if (world_m).is_some() {
                    let t = if (to_spawn > 1.0_f64) {
                        (s_idx / (to_spawn - 1.0_f64))
                    } else {
                        1.0_f64
                    };
                    let origin_x = (prev_path_x + ((track_x - prev_path_x) * t));
                    let origin_y = (prev_path_y + ((track_y - prev_path_y) * t));
                    let origin_z = (prev_path_z + ((track_z - prev_path_z) * t));
                    let px = (((((world_m.as_ref().unwrap()[0.0_f64 as usize] as f64) * spawn_x)
                        + ((world_m.as_ref().unwrap()[4.0_f64 as usize] as f64) * spawn_y))
                        + ((world_m.as_ref().unwrap()[8.0_f64 as usize] as f64) * spawn_z))
                        + origin_x);
                    let py = (((((world_m.as_ref().unwrap()[1.0_f64 as usize] as f64) * spawn_x)
                        + ((world_m.as_ref().unwrap()[5.0_f64 as usize] as f64) * spawn_y))
                        + ((world_m.as_ref().unwrap()[9.0_f64 as usize] as f64) * spawn_z))
                        + origin_y);
                    let pz = (((((world_m.as_ref().unwrap()[2.0_f64 as usize] as f64) * spawn_x)
                        + ((world_m.as_ref().unwrap()[6.0_f64 as usize] as f64) * spawn_y))
                        + ((world_m.as_ref().unwrap()[10.0_f64 as usize] as f64) * spawn_z))
                        + origin_z);
                    let wvx = ((((world_m.as_ref().unwrap()[0.0_f64 as usize] as f64) * vx)
                        + ((world_m.as_ref().unwrap()[4.0_f64 as usize] as f64) * vy))
                        + ((world_m.as_ref().unwrap()[8.0_f64 as usize] as f64) * vz));
                    let wvy = ((((world_m.as_ref().unwrap()[1.0_f64 as usize] as f64) * vx)
                        + ((world_m.as_ref().unwrap()[5.0_f64 as usize] as f64) * vy))
                        + ((world_m.as_ref().unwrap()[9.0_f64 as usize] as f64) * vz));
                    let wvz = ((((world_m.as_ref().unwrap()[2.0_f64 as usize] as f64) * vx)
                        + ((world_m.as_ref().unwrap()[6.0_f64 as usize] as f64) * vy))
                        + ((world_m.as_ref().unwrap()[10.0_f64 as usize] as f64) * vz));
                    spawn_x = px;
                    spawn_y = py;
                    spawn_z = pz;
                    vx = wvx;
                    vy = wvy;
                    vz = wvz;
                }
                if (has_vel_inherit) && (!(state.prev_x).is_nan()) {
                    vx += (emitter_vel_x * config.velocity_inheritance);
                    vy += (emitter_vel_y * config.velocity_inheritance);
                    vz += (emitter_vel_z * config.velocity_inheritance);
                }
                let vt = (idx * particle_velocity_stride_constant);
                state.velocities[vt as usize] = (vx) as f32;
                state.velocities[(vt + 1.0_f64) as usize] = (vy) as f32;
                state.velocities[(vt + 2.0_f64) as usize] = (vz) as f32;
                let spawn_scale = (config.scale_min
                    + ({
                        let __flight_callback = (state.random).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    } * (config.scale_max - config.scale_min)));
                state.scales[idx as usize] = (spawn_scale) as f32;
                let tt = (idx * PARTICLE_TRANSFORM_STRIDE);
                emitter.data.transforms[tt as usize] = (spawn_x) as f32;
                emitter.data.transforms[(tt + 1.0_f64) as usize] = (spawn_y) as f32;
                emitter.data.transforms[(tt + 2.0_f64) as usize] =
                    if ((shape == "sphere") || (shape == "cone3d")) || (shape == "box") {
                        ((vy).atan2(vx)) as f32
                    } else {
                        (base_angle
                            + ((({
                                let __flight_callback = (state.random).clone();
                                let __flight_result = __flight_callback.lock().unwrap()();
                                __flight_result
                            } - 0.5_f64)
                                * 2.0_f64)
                                * config.spread)) as f32
                    };
                emitter.data.transforms[(tt + 3.0_f64) as usize] = if has_scale_curve {
                    (spawn_scale * sample_particle_curve(scale_curve.as_ref().unwrap(), 0.0_f64))
                        as f32
                } else {
                    (spawn_scale) as f32
                };
                emitter.data.positions_z[idx as usize] = (spawn_z) as f32;
                emitter.data.alphas[idx as usize] = if has_alpha_curve {
                    (sample_particle_curve(alpha_curve.as_ref().unwrap(), 0.0_f64)) as f32
                } else {
                    (config.alpha_start) as f32
                };
                let ct = (idx * 3.0_f64);
                if has_color_curve {
                    sample_particle_color_curve(
                        &mut emitter.data.colors,
                        ct,
                        color_curve.as_ref().unwrap(),
                        0.0_f64,
                    );
                } else {
                    if has_color_variance {
                        let r0 = clamp01(
                            (color_start_r
                                + ((({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } - 0.5_f64)
                                    * 2.0_f64)
                                    * config.color_start_variance_r)),
                        );
                        let g0 = clamp01(
                            (color_start_g
                                + ((({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } - 0.5_f64)
                                    * 2.0_f64)
                                    * config.color_start_variance_g)),
                        );
                        let b0 = clamp01(
                            (color_start_b
                                + ((({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } - 0.5_f64)
                                    * 2.0_f64)
                                    * config.color_start_variance_b)),
                        );
                        let r1 = clamp01(
                            (color_end_r
                                + ((({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } - 0.5_f64)
                                    * 2.0_f64)
                                    * config.color_end_variance_r)),
                        );
                        let g1 = clamp01(
                            (color_end_g
                                + ((({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } - 0.5_f64)
                                    * 2.0_f64)
                                    * config.color_end_variance_g)),
                        );
                        let b1 = clamp01(
                            (color_end_b
                                + ((({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } - 0.5_f64)
                                    * 2.0_f64)
                                    * config.color_end_variance_b)),
                        );
                        state.color_birth[ct as usize] = (r0) as f32;
                        state.color_birth[(ct + 1.0_f64) as usize] = (g0) as f32;
                        state.color_birth[(ct + 2.0_f64) as usize] = (b0) as f32;
                        state.color_death[ct as usize] = (r1) as f32;
                        state.color_death[(ct + 1.0_f64) as usize] = (g1) as f32;
                        state.color_death[(ct + 2.0_f64) as usize] = (b1) as f32;
                        emitter.data.colors[ct as usize] = (r0) as f32;
                        emitter.data.colors[(ct + 1.0_f64) as usize] = (g0) as f32;
                        emitter.data.colors[(ct + 2.0_f64) as usize] = (b0) as f32;
                    } else {
                        emitter.data.colors[ct as usize] = (color_start_r) as f32;
                        emitter.data.colors[(ct + 1.0_f64) as usize] = (color_start_g) as f32;
                        emitter.data.colors[(ct + 2.0_f64) as usize] = (color_start_b) as f32;
                    }
                }
                emitter.data.ids[idx as usize] = (region_id_min
                    + if (config.frame_count > 1.0_f64) {
                        0.0_f64
                    } else {
                        if (region_range > 0.0_f64) {
                            (__flight_js_to_i32(
                                ({
                                    let __flight_callback = (state.random).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                } * region_range),
                            ) | __flight_js_to_i32(0.0_f64)) as f64
                        } else {
                            0.0_f64
                        }
                    }) as u16;
                state.rotation_speeds[idx as usize] = if has_rot_speed {
                    (config.rotation_speed_min
                        + ({
                            let __flight_callback = (state.random).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        } * rot_speed_range)) as f32
                } else {
                    (0.0_f64) as f32
                };
                {
                    let __flight_callback = on_spawn;
                    __flight_callback
                        .as_ref()
                        .map(|callback| callback.lock().unwrap()(spawn_x, spawn_y, spawn_z))
                };
                if (signals).is_some() {
                    {
                        let __flight_callback =
                            (signals.as_ref().unwrap().on_particle_spawn.emit).clone();
                        let __flight_result = __flight_callback.lock().unwrap()(
                            spawn_x, spawn_y, spawn_z, vx, vy, vz,
                        );
                        __flight_result
                    };
                }
                {
                    s_idx += 1.0;
                    s_idx
                };
            }
        }
        emitter.data.particle_count = new_count;
    }
    state.prev_x = track_x;
    state.prev_y = track_y;
    state.prev_z = track_z;
    let live_velocity_count = (emitter.data.particle_count * particle_velocity_stride_constant);
    if ((emitter.data.velocities.len() as f64) >= live_velocity_count) {
        {
            let mut vi = 0.0_f64;
            while (vi < live_velocity_count) {
                emitter.data.velocities[vi as usize] =
                    (state.velocities[vi as usize] as f64) as f32;
                {
                    vi += 1.0;
                    vi
                };
            }
        }
    }
    if ((signals).is_some()) && (is_particle_emitter3_d_complete(emitter, state, config)) {
        {
            let __flight_callback = (signals.as_ref().unwrap().on_emitter_complete.emit).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
    }
}

// Source: upstream/packages/particleemitter/src/updateParticleEmitter3D.ts:450 (sha256:92c4452839ded0362c28adef5c15154deeaad9b404aff5129f0596af7fea21ad)
fn clamp01(v: f64) -> f64 {
    return if (v < 0.0_f64) {
        0.0_f64
    } else {
        if (v > 1.0_f64) { 1.0_f64 } else { v }
    };
}

// Source: upstream/packages/particleemitter/src/updateParticleEmitter3D.ts:454 (sha256:50a57123f6b45907eaef3ae1022afb661b7f38f7a0e7fcbd096e0aa2999baa2f)
static _ROT: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/particleemitter/src/updateParticleEmitter3D.ts:455 (sha256:e2d7d2c9f9dd807fc1910a68b896fa20837e220fd3c55befee041eb5ee51c534)
fn rotate_to_direction(lx: f64, ly: f64, lz: f64, dx: f64, dy: f64, dz: f64) -> Vec<f64> {
    let kx = (-dy);
    let ky = dx;
    let sin_angle = ((kx * kx) + (ky * ky)).sqrt();
    let cos_angle = dz;
    if (sin_angle < 0.000001_f64) {
        if (cos_angle > 0.0_f64) {
            {
                let __flight_index = (0.0_f64) as usize;
                let __flight_value = lx;
                if __flight_index == _ROT.lock().unwrap().len() {
                    _ROT.lock().unwrap().push(__flight_value);
                } else {
                    _ROT.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (1.0_f64) as usize;
                let __flight_value = ly;
                if __flight_index == _ROT.lock().unwrap().len() {
                    _ROT.lock().unwrap().push(__flight_value);
                } else {
                    _ROT.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (2.0_f64) as usize;
                let __flight_value = lz;
                if __flight_index == _ROT.lock().unwrap().len() {
                    _ROT.lock().unwrap().push(__flight_value);
                } else {
                    _ROT.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
        } else {
            {
                let __flight_index = (0.0_f64) as usize;
                let __flight_value = lx;
                if __flight_index == _ROT.lock().unwrap().len() {
                    _ROT.lock().unwrap().push(__flight_value);
                } else {
                    _ROT.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (1.0_f64) as usize;
                let __flight_value = (-ly);
                if __flight_index == _ROT.lock().unwrap().len() {
                    _ROT.lock().unwrap().push(__flight_value);
                } else {
                    _ROT.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (2.0_f64) as usize;
                let __flight_value = (-lz);
                if __flight_index == _ROT.lock().unwrap().len() {
                    _ROT.lock().unwrap().push(__flight_value);
                } else {
                    _ROT.lock().unwrap()[__flight_index] = __flight_value;
                }
            };
        }
        return (_ROT).clone();
    }
    let inv_sin = (1.0_f64 / sin_angle);
    let ax = (kx * inv_sin);
    let ay = (ky * inv_sin);
    let kdotv = ((ax * lx) + (ay * ly));
    let cross_x = (ay * lz);
    let cross_y = ((-ax) * lz);
    let cross_z = ((ax * ly) - (ay * lx));
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value =
            (((lx * cos_angle) + (cross_x * sin_angle)) + ((ax * kdotv) * (1.0_f64 - cos_angle)));
        if __flight_index == _ROT.lock().unwrap().len() {
            _ROT.lock().unwrap().push(__flight_value);
        } else {
            _ROT.lock().unwrap()[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value =
            (((ly * cos_angle) + (cross_y * sin_angle)) + ((ay * kdotv) * (1.0_f64 - cos_angle)));
        if __flight_index == _ROT.lock().unwrap().len() {
            _ROT.lock().unwrap().push(__flight_value);
        } else {
            _ROT.lock().unwrap()[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = ((lz * cos_angle) + (cross_z * sin_angle));
        if __flight_index == _ROT.lock().unwrap().len() {
            _ROT.lock().unwrap().push(__flight_value);
        } else {
            _ROT.lock().unwrap()[__flight_index] = __flight_value;
        }
    };
    return (_ROT).clone();
}
