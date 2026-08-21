// @generated from upstream/packages/particleemitter/src/particleEmitter.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::{
    copy_rectangle, create_rectangle, reserve_float32_array, reserve_uint16_array,
};
use flighthq_node::invalidate_node_local_bounds;
use flighthq_scene2d::{create_node2_d, create_node2_d_runtime, get_node2_d_runtime};
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, ClipRegion, ColorScaleBias, InteractionSignals, Kind,
    Material, MaterialData, Matrix, Matrix4, Node, Node2DData, NodeInteractionState, NodeSignals,
    NodeTraitsKey, PARTICLE_EMITTER2_D_KIND as particle_emitter2_d_kind_constant,
    ParticleEmitter2D, ParticleEmitter2DRuntime, ParticleEmitterData, Rectangle, RectangleLike,
    SamplerLike, Scene2D, Scene2DSignals, Texture, TextureAtlas, Vector2Like,
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
    pub apply_blend_mode_parent: Option<WgpuRenderState>,
    pub binding_cache_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(GlRenderState, crate::OpaqueHostValue) -> () + Send + 'static>,
            >,
        >,
    >,
    pub bounds_rectangle: Option<Rectangle>,
    pub bounds_using_local_bounds_id: Option<f64>,
    pub bounds_using_local_transform_id: Option<f64>,
    pub bounds_version: Option<f64>,
    pub build_text_layout_params: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(TextLabel, TextMeasureFunction) -> TextLayoutParams + Send + 'static>,
            >,
        >,
    >,
    pub canvas_blend_effect_backdrops: Option<Vec<(String, CanvasRenderTarget)>>,
    pub canvas_texture_resolvers: Option<CanvasTextureResolvers>,
    pub canvas_texture_view: Option<crate::OpaqueHostValue>,
    pub canvas_view_cleared: Option<bool>,
    pub children_id: Option<f64>,
    pub clip_contour_pipelines: Option<Vec<(crate::OpaqueHostValue, WgpuClipContourPipelines)>>,
    pub clip_contour_stack: Option<Vec<WgpuClipContourEntry>>,
    pub clip_forms: Option<Vec<String>>,
    pub color_matrix_instanced_shader: Option<GlColorScaleBiasInstancedShader>,
    pub color_scale_bias_instanced_shader: Option<GlColorScaleBiasInstancedShader>,
    pub color_tint_instanced_shader: Option<GlColorScaleBiasInstancedShader>,
    pub command_encoder: Option<crate::OpaqueHostValue>,
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
    pub current_texture: Option<crate::OpaqueHostValue>,
    pub current_texture_straight_alpha: Option<bool>,
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
    pub flush_pending_draws: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(GlRenderState) -> () + Send + 'static>>>,
    >,
    pub frame_capture_buffer: Option<crate::OpaqueHostValue>,
    pub frame_capture_bytes_per_row: Option<f64>,
    pub frame_capture_enabled: Option<bool>,
    pub frame_capture_height: Option<f64>,
    pub frame_capture_texture: Option<crate::OpaqueHostValue>,
    pub frame_capture_width: Option<f64>,
    pub gl_external_texture_cache: Option<Vec<(ExternalTexture, crate::OpaqueHostValue)>>,
    pub gl_render_texture_cache: Option<Vec<(RenderTexture, GlRenderTextureEntry)>>,
    pub gl_render_texture_guard: Option<GlRenderTextureGuard>,
    pub image_smoothing_enabled: Option<bool>,
    pub image_smoothing_quality: Option<crate::OpaqueHostValue>,
    pub input: Option<TextInputState>,
    pub instance_velocities: Option<Vec<f32>>,
    pub interaction_signals: Option<InteractionSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub is_local_bounds_rectangle_valid: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(BoundsNodeAny) -> bool + Send + 'static>>>,
    >,
    pub linear_sampler: Option<crate::OpaqueHostValue>,
    pub local_bounds_id: Option<f64>,
    pub local_bounds_rectangle: Option<Rectangle>,
    pub local_bounds_texture: Option<Texture>,
    pub local_bounds_texture_version: Option<f64>,
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
    pub media_stream: Option<crate::OpaqueHostValue>,
    pub mipmap_degraded_guard: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(WgpuRenderState) -> () + Send + 'static>>>,
    >,
    pub mipmap_generator: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            WgpuRenderState,
                            crate::OpaqueHostValue,
                            f64,
                            f64,
                            crate::OpaqueHostValue,
                        ) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub mipmapped_textures: Option<Vec<crate::OpaqueHostValue>>,
    pub morph_bind_pose: Option<MeshMorphBindPose>,
    pub morph_blended_weights: Option<Vec<f32>>,
    pub movie_clip_signals: Option<MovieClipSignals>,
    pub nearest_sampler: Option<crate::OpaqueHostValue>,
    pub node_signals: Option<NodeSignals>,
    pub pages: Option<Vec<BitmapTextPage>>,
    pub parent_reference_id: Option<f64>,
    pub particle_corner_buffer: Option<crate::OpaqueHostValue>,
    pub particle_instance_capacity: Option<f64>,
    pub particle_shader: Option<GlParticleShader>,
    pub pipeline_cache: Option<Vec<(String, crate::OpaqueHostValue)>>,
    pub quad_batch_corner_buffer: Option<crate::OpaqueHostValue>,
    pub quad_batch_shader: Option<GlQuadBatchShader>,
    pub quad_batch_writer_blend_mode: Option<BlendMode>,
    pub quad_batch_writer_buffer_cursor: Option<f64>,
    pub quad_batch_writer_buffer_pool: Option<Vec<WgpuQuadBatchWriterBufferSlot>>,
    pub quad_batch_writer_color_matrix_data: Option<Vec<f32>>,
    pub quad_batch_writer_color_scale_bias_buffer: Option<crate::OpaqueHostValue>,
    pub quad_batch_writer_color_scale_bias_data: Option<Vec<f32>>,
    pub quad_batch_writer_color_scale_bias_mode: Option<f64>,
    pub quad_batch_writer_color_tint_data: Option<Vec<u32>>,
    pub quad_batch_writer_count: Option<f64>,
    pub quad_batch_writer_instance_buffer: Option<crate::OpaqueHostValue>,
    pub quad_batch_writer_instance_data: Option<Vec<f32>>,
    pub quad_batch_writer_material: Option<Material>,
    pub quad_batch_writer_material_buffer: Option<crate::OpaqueHostValue>,
    pub quad_batch_writer_material_data: Option<Vec<f32>>,
    pub quad_batch_writer_material_floats: Option<f64>,
    pub quad_batch_writer_sampler: Option<SamplerLike>,
    pub quad_batch_writer_smoothing: Option<bool>,
    pub quad_batch_writer_straight_alpha: Option<bool>,
    pub quad_batch_writer_uniform_color_scale_bias: Option<
        crate::FlightUnion2<ColorScaleBias, crate::FlightUnion2<TintMaterialData, Vec<f64>>>,
    >,
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
    pub renderer_map_id: Option<f64>,
    pub render_pass: Option<crate::OpaqueHostValue>,
    pub render_proxy_adapter_map: Option<Vec<(Renderable, RenderProxyAdapter)>>,
    pub render_proxy_map: Option<Vec<(Renderable, RenderProxy)>>,
    pub render_proxy_sources: Option<Vec<Renderable>>,
    pub render_target_stack: Option<Vec<WgpuSavedPassState>>,
    pub retired_buffers: Option<Vec<crate::OpaqueHostValue>>,
    pub retired_textures: Option<Vec<crate::OpaqueHostValue>>,
    pub rich_text_content: Option<RichTextContent>,
    pub rotation_angle: Option<f64>,
    pub rotation_cosine: Option<f64>,
    pub rotation_sine: Option<f64>,
    pub sampler_cache: Option<Vec<(f64, crate::OpaqueHostValue)>>,
    pub scene2d: Option<Scene2D>,
    pub scene2d_signals: Option<Scene2DSignals>,
    pub scene_mesh_upload_cache: Option<Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>>,
    pub selection_begin_index: Option<f64>,
    pub selection_end_index: Option<f64>,
    pub shader_loc: Option<GlShaderLocations>,
    pub shape_bounds_command_registry_revision: Option<f64>,
    pub shape_mesh_color_matrix_shader: Option<GlShapeMeshColorScaleBiasShader>,
    pub shape_mesh_color_scale_bias_shader: Option<GlShapeMeshColorScaleBiasShader>,
    pub shape_mesh_pipelines: Option<Vec<(String, WgpuShapeMeshPipeline)>>,
    pub skin_bind_pose: Option<MeshSkinBindPose>,
    pub surface_antialias_enabled: Option<bool>,
    pub surface_antialias_height: Option<f64>,
    pub surface_antialias_resolve_bind_group: Option<crate::OpaqueHostValue>,
    pub surface_antialias_resolve_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub surface_antialias_resolve_pipeline: Option<crate::OpaqueHostValue>,
    pub surface_antialias_texture: Option<crate::OpaqueHostValue>,
    pub surface_antialias_view: Option<crate::OpaqueHostValue>,
    pub surface_antialias_width: Option<f64>,
    pub surface_presentation_view: Option<crate::OpaqueHostValue>,
    pub temp_stack: Option<Vec<Renderable>>,
    pub text_field_signals: Option<TextFieldSignals>,
    pub text_layout: Option<TextLayoutResult>,
    pub text_layout_using_content_id: Option<f64>,
    pub texture_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub uniform_bind_group: Option<crate::OpaqueHostValue>,
    pub uniform_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub uniform_buffer: Option<crate::OpaqueHostValue>,
    pub uniform_color_scale_bias_shader: Option<GlUniformColorScaleBiasShader>,
    pub uniform_data: Option<Vec<f32>>,
    pub uniform_data_u32: Option<Vec<u32>>,
    pub uniform_offset: Option<f64>,
    pub uniform_stride: Option<f64>,
    pub video_element: Option<crate::OpaqueHostValue>,
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
    pub wgpu_external_texture_cache: Option<Vec<(ExternalTexture, WgpuTextureEntry)>>,
    pub wgpu_render_texture_cache: Option<Vec<(RenderTexture, WgpuRenderTextureEntry)>>,
    pub wgpu_render_texture_guard: Option<WgpuRenderTextureGuard>,
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
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub color_adjustments_unsupported: Option<bool>,
    pub resolved_color_matrix: Option<Vec<f64>>,
    pub resolved_color_scale_bias: Option<ColorScaleBias>,
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
    pub apply_blend_mode_parent: Option<WgpuRenderState>,
    pub binding_cache_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(GlRenderState, crate::OpaqueHostValue) -> () + Send + 'static>,
            >,
        >,
    >,
    pub bounds_rectangle: Option<Rectangle>,
    pub bounds_using_local_bounds_id: Option<f64>,
    pub bounds_using_local_transform_id: Option<f64>,
    pub bounds_version: Option<f64>,
    pub build_text_layout_params: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(TextLabel, TextMeasureFunction) -> TextLayoutParams + Send + 'static>,
            >,
        >,
    >,
    pub canvas_blend_effect_backdrops: Option<Vec<(String, CanvasRenderTarget)>>,
    pub canvas_texture_resolvers: Option<CanvasTextureResolvers>,
    pub canvas_texture_view: Option<crate::OpaqueHostValue>,
    pub canvas_view_cleared: Option<bool>,
    pub children_id: Option<f64>,
    pub clip_contour_pipelines: Option<Vec<(crate::OpaqueHostValue, WgpuClipContourPipelines)>>,
    pub clip_contour_stack: Option<Vec<WgpuClipContourEntry>>,
    pub clip_forms: Option<Vec<String>>,
    pub color_matrix_instanced_shader: Option<GlColorScaleBiasInstancedShader>,
    pub color_scale_bias_instanced_shader: Option<GlColorScaleBiasInstancedShader>,
    pub color_tint_instanced_shader: Option<GlColorScaleBiasInstancedShader>,
    pub command_encoder: Option<crate::OpaqueHostValue>,
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
    pub current_texture: Option<crate::OpaqueHostValue>,
    pub current_texture_straight_alpha: Option<bool>,
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
    pub flush_pending_draws: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(GlRenderState) -> () + Send + 'static>>>,
    >,
    pub frame_capture_buffer: Option<crate::OpaqueHostValue>,
    pub frame_capture_bytes_per_row: Option<f64>,
    pub frame_capture_enabled: Option<bool>,
    pub frame_capture_height: Option<f64>,
    pub frame_capture_texture: Option<crate::OpaqueHostValue>,
    pub frame_capture_width: Option<f64>,
    pub gl_external_texture_cache: Option<Vec<(ExternalTexture, crate::OpaqueHostValue)>>,
    pub gl_render_texture_cache: Option<Vec<(RenderTexture, GlRenderTextureEntry)>>,
    pub gl_render_texture_guard: Option<GlRenderTextureGuard>,
    pub image_smoothing_enabled: Option<bool>,
    pub image_smoothing_quality: Option<crate::OpaqueHostValue>,
    pub input: Option<TextInputState>,
    pub instance_velocities: Option<Vec<f32>>,
    pub interaction_signals: Option<InteractionSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub is_local_bounds_rectangle_valid: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(BoundsNodeAny) -> bool + Send + 'static>>>,
    >,
    pub linear_sampler: Option<crate::OpaqueHostValue>,
    pub local_bounds_id: Option<f64>,
    pub local_bounds_rectangle: Option<Rectangle>,
    pub local_bounds_texture: Option<Texture>,
    pub local_bounds_texture_version: Option<f64>,
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
    pub media_stream: Option<crate::OpaqueHostValue>,
    pub mipmap_degraded_guard: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(WgpuRenderState) -> () + Send + 'static>>>,
    >,
    pub mipmap_generator: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            WgpuRenderState,
                            crate::OpaqueHostValue,
                            f64,
                            f64,
                            crate::OpaqueHostValue,
                        ) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub mipmapped_textures: Option<Vec<crate::OpaqueHostValue>>,
    pub morph_bind_pose: Option<MeshMorphBindPose>,
    pub morph_blended_weights: Option<Vec<f32>>,
    pub movie_clip_signals: Option<MovieClipSignals>,
    pub nearest_sampler: Option<crate::OpaqueHostValue>,
    pub node_signals: Option<NodeSignals>,
    pub pages: Option<Vec<BitmapTextPage>>,
    pub parent_reference_id: Option<f64>,
    pub particle_corner_buffer: Option<crate::OpaqueHostValue>,
    pub particle_instance_capacity: Option<f64>,
    pub particle_shader: Option<GlParticleShader>,
    pub pipeline_cache: Option<Vec<(String, crate::OpaqueHostValue)>>,
    pub quad_batch_corner_buffer: Option<crate::OpaqueHostValue>,
    pub quad_batch_shader: Option<GlQuadBatchShader>,
    pub quad_batch_writer_blend_mode: Option<BlendMode>,
    pub quad_batch_writer_buffer_cursor: Option<f64>,
    pub quad_batch_writer_buffer_pool: Option<Vec<WgpuQuadBatchWriterBufferSlot>>,
    pub quad_batch_writer_color_matrix_data: Option<Vec<f32>>,
    pub quad_batch_writer_color_scale_bias_buffer: Option<crate::OpaqueHostValue>,
    pub quad_batch_writer_color_scale_bias_data: Option<Vec<f32>>,
    pub quad_batch_writer_color_scale_bias_mode: Option<f64>,
    pub quad_batch_writer_color_tint_data: Option<Vec<u32>>,
    pub quad_batch_writer_count: Option<f64>,
    pub quad_batch_writer_instance_buffer: Option<crate::OpaqueHostValue>,
    pub quad_batch_writer_instance_data: Option<Vec<f32>>,
    pub quad_batch_writer_material: Option<Material>,
    pub quad_batch_writer_material_buffer: Option<crate::OpaqueHostValue>,
    pub quad_batch_writer_material_data: Option<Vec<f32>>,
    pub quad_batch_writer_material_floats: Option<f64>,
    pub quad_batch_writer_sampler: Option<SamplerLike>,
    pub quad_batch_writer_smoothing: Option<bool>,
    pub quad_batch_writer_straight_alpha: Option<bool>,
    pub quad_batch_writer_uniform_color_scale_bias: Option<
        crate::FlightUnion2<ColorScaleBias, crate::FlightUnion2<TintMaterialData, Vec<f64>>>,
    >,
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
    pub renderer_map_id: Option<f64>,
    pub render_pass: Option<crate::OpaqueHostValue>,
    pub render_proxy_adapter_map: Option<Vec<(Renderable, RenderProxyAdapter)>>,
    pub render_proxy_map: Option<Vec<(Renderable, RenderProxy)>>,
    pub render_proxy_sources: Option<Vec<Renderable>>,
    pub render_target_stack: Option<Vec<WgpuSavedPassState>>,
    pub retired_buffers: Option<Vec<crate::OpaqueHostValue>>,
    pub retired_textures: Option<Vec<crate::OpaqueHostValue>>,
    pub rich_text_content: Option<RichTextContent>,
    pub rotation_angle: Option<f64>,
    pub rotation_cosine: Option<f64>,
    pub rotation_sine: Option<f64>,
    pub sampler_cache: Option<Vec<(f64, crate::OpaqueHostValue)>>,
    pub scene2d: Option<Scene2D>,
    pub scene2d_signals: Option<Scene2DSignals>,
    pub scene_mesh_upload_cache: Option<Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>>,
    pub selection_begin_index: Option<f64>,
    pub selection_end_index: Option<f64>,
    pub shader_loc: Option<GlShaderLocations>,
    pub shape_bounds_command_registry_revision: Option<f64>,
    pub shape_mesh_color_matrix_shader: Option<GlShapeMeshColorScaleBiasShader>,
    pub shape_mesh_color_scale_bias_shader: Option<GlShapeMeshColorScaleBiasShader>,
    pub shape_mesh_pipelines: Option<Vec<(String, WgpuShapeMeshPipeline)>>,
    pub skin_bind_pose: Option<MeshSkinBindPose>,
    pub surface_antialias_enabled: Option<bool>,
    pub surface_antialias_height: Option<f64>,
    pub surface_antialias_resolve_bind_group: Option<crate::OpaqueHostValue>,
    pub surface_antialias_resolve_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub surface_antialias_resolve_pipeline: Option<crate::OpaqueHostValue>,
    pub surface_antialias_texture: Option<crate::OpaqueHostValue>,
    pub surface_antialias_view: Option<crate::OpaqueHostValue>,
    pub surface_antialias_width: Option<f64>,
    pub surface_presentation_view: Option<crate::OpaqueHostValue>,
    pub temp_stack: Option<Vec<Renderable>>,
    pub text_field_signals: Option<TextFieldSignals>,
    pub text_layout: Option<TextLayoutResult>,
    pub text_layout_using_content_id: Option<f64>,
    pub texture_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub uniform_bind_group: Option<crate::OpaqueHostValue>,
    pub uniform_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub uniform_buffer: Option<crate::OpaqueHostValue>,
    pub uniform_color_scale_bias_shader: Option<GlUniformColorScaleBiasShader>,
    pub uniform_data: Option<Vec<f32>>,
    pub uniform_data_u32: Option<Vec<u32>>,
    pub uniform_offset: Option<f64>,
    pub uniform_stride: Option<f64>,
    pub video_element: Option<crate::OpaqueHostValue>,
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
    pub wgpu_external_texture_cache: Option<Vec<(ExternalTexture, WgpuTextureEntry)>>,
    pub wgpu_render_texture_cache: Option<Vec<(RenderTexture, WgpuRenderTextureEntry)>>,
    pub wgpu_render_texture_guard: Option<WgpuRenderTextureGuard>,
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
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub color_adjustments_unsupported: Option<bool>,
    pub resolved_color_matrix: Option<Vec<f64>>,
    pub resolved_color_scale_bias: Option<ColorScaleBias>,
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
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub anisotropy_ext: Option<crate::OpaqueHostValue>,
    pub appearance_id: Option<f64>,
    pub apply_blend_mode_parent: Option<WgpuRenderState>,
    pub binding_cache_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(GlRenderState, crate::OpaqueHostValue) -> () + Send + 'static>,
            >,
        >,
    >,
    pub bounds_rectangle: Option<Rectangle>,
    pub bounds_using_local_bounds_id: Option<f64>,
    pub bounds_using_local_transform_id: Option<f64>,
    pub bounds_version: Option<f64>,
    pub build_text_layout_params: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(TextLabel, TextMeasureFunction) -> TextLayoutParams + Send + 'static>,
            >,
        >,
    >,
    pub canvas_blend_effect_backdrops: Option<Vec<(String, CanvasRenderTarget)>>,
    pub canvas_texture_resolvers: Option<CanvasTextureResolvers>,
    pub canvas_texture_view: Option<crate::OpaqueHostValue>,
    pub canvas_view_cleared: Option<bool>,
    pub children_id: Option<f64>,
    pub clip_contour_pipelines: Option<Vec<(crate::OpaqueHostValue, WgpuClipContourPipelines)>>,
    pub clip_contour_stack: Option<Vec<WgpuClipContourEntry>>,
    pub clip_forms: Option<Vec<String>>,
    pub color_matrix_instanced_shader: Option<GlColorScaleBiasInstancedShader>,
    pub color_scale_bias_instanced_shader: Option<GlColorScaleBiasInstancedShader>,
    pub color_tint_instanced_shader: Option<GlColorScaleBiasInstancedShader>,
    pub command_encoder: Option<crate::OpaqueHostValue>,
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
    pub current_texture: Option<crate::OpaqueHostValue>,
    pub current_texture_straight_alpha: Option<bool>,
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
    pub flush_pending_draws: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(GlRenderState) -> () + Send + 'static>>>,
    >,
    pub frame_capture_buffer: Option<crate::OpaqueHostValue>,
    pub frame_capture_bytes_per_row: Option<f64>,
    pub frame_capture_enabled: Option<bool>,
    pub frame_capture_height: Option<f64>,
    pub frame_capture_texture: Option<crate::OpaqueHostValue>,
    pub frame_capture_width: Option<f64>,
    pub gl_external_texture_cache: Option<Vec<(ExternalTexture, crate::OpaqueHostValue)>>,
    pub gl_render_texture_cache: Option<Vec<(RenderTexture, GlRenderTextureEntry)>>,
    pub gl_render_texture_guard: Option<GlRenderTextureGuard>,
    pub image_smoothing_enabled: Option<bool>,
    pub image_smoothing_quality: Option<crate::OpaqueHostValue>,
    pub input: Option<TextInputState>,
    pub instance_velocities: Option<Vec<f32>>,
    pub interaction_signals: Option<InteractionSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub is_local_bounds_rectangle_valid: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(BoundsNodeAny) -> bool + Send + 'static>>>,
    >,
    pub linear_sampler: Option<crate::OpaqueHostValue>,
    pub local_bounds_id: Option<f64>,
    pub local_bounds_rectangle: Option<Rectangle>,
    pub local_bounds_texture: Option<Texture>,
    pub local_bounds_texture_version: Option<f64>,
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
    pub media_stream: Option<crate::OpaqueHostValue>,
    pub mipmap_degraded_guard: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(WgpuRenderState) -> () + Send + 'static>>>,
    >,
    pub mipmap_generator: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            WgpuRenderState,
                            crate::OpaqueHostValue,
                            f64,
                            f64,
                            crate::OpaqueHostValue,
                        ) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub mipmapped_textures: Option<Vec<crate::OpaqueHostValue>>,
    pub morph_bind_pose: Option<MeshMorphBindPose>,
    pub morph_blended_weights: Option<Vec<f32>>,
    pub movie_clip_signals: Option<MovieClipSignals>,
    pub nearest_sampler: Option<crate::OpaqueHostValue>,
    pub node_signals: Option<NodeSignals>,
    pub pages: Option<Vec<BitmapTextPage>>,
    pub parent_reference_id: Option<f64>,
    pub particle_corner_buffer: Option<crate::OpaqueHostValue>,
    pub particle_instance_capacity: Option<f64>,
    pub particle_shader: Option<GlParticleShader>,
    pub pipeline_cache: Option<Vec<(String, crate::OpaqueHostValue)>>,
    pub quad_batch_corner_buffer: Option<crate::OpaqueHostValue>,
    pub quad_batch_shader: Option<GlQuadBatchShader>,
    pub quad_batch_writer_blend_mode: Option<BlendMode>,
    pub quad_batch_writer_buffer_cursor: Option<f64>,
    pub quad_batch_writer_buffer_pool: Option<Vec<WgpuQuadBatchWriterBufferSlot>>,
    pub quad_batch_writer_color_matrix_data: Option<Vec<f32>>,
    pub quad_batch_writer_color_scale_bias_buffer: Option<crate::OpaqueHostValue>,
    pub quad_batch_writer_color_scale_bias_data: Option<Vec<f32>>,
    pub quad_batch_writer_color_scale_bias_mode: Option<f64>,
    pub quad_batch_writer_color_tint_data: Option<Vec<u32>>,
    pub quad_batch_writer_count: Option<f64>,
    pub quad_batch_writer_instance_buffer: Option<crate::OpaqueHostValue>,
    pub quad_batch_writer_instance_data: Option<Vec<f32>>,
    pub quad_batch_writer_material: Option<Material>,
    pub quad_batch_writer_material_buffer: Option<crate::OpaqueHostValue>,
    pub quad_batch_writer_material_data: Option<Vec<f32>>,
    pub quad_batch_writer_material_floats: Option<f64>,
    pub quad_batch_writer_sampler: Option<SamplerLike>,
    pub quad_batch_writer_smoothing: Option<bool>,
    pub quad_batch_writer_straight_alpha: Option<bool>,
    pub quad_batch_writer_uniform_color_scale_bias: Option<
        crate::FlightUnion2<ColorScaleBias, crate::FlightUnion2<TintMaterialData, Vec<f64>>>,
    >,
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
    pub renderer_map_id: Option<f64>,
    pub render_pass: Option<crate::OpaqueHostValue>,
    pub render_proxy_adapter_map: Option<Vec<(Renderable, RenderProxyAdapter)>>,
    pub render_proxy_map: Option<Vec<(Renderable, RenderProxy)>>,
    pub render_proxy_sources: Option<Vec<Renderable>>,
    pub render_target_stack: Option<Vec<WgpuSavedPassState>>,
    pub retired_buffers: Option<Vec<crate::OpaqueHostValue>>,
    pub retired_textures: Option<Vec<crate::OpaqueHostValue>>,
    pub rich_text_content: Option<RichTextContent>,
    pub rotation_angle: Option<f64>,
    pub rotation_cosine: Option<f64>,
    pub rotation_sine: Option<f64>,
    pub sampler_cache: Option<Vec<(f64, crate::OpaqueHostValue)>>,
    pub scene2d: Option<Scene2D>,
    pub scene2d_signals: Option<Scene2DSignals>,
    pub scene_mesh_upload_cache: Option<Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>>,
    pub selection_begin_index: Option<f64>,
    pub selection_end_index: Option<f64>,
    pub shader_loc: Option<GlShaderLocations>,
    pub shape_bounds_command_registry_revision: Option<f64>,
    pub shape_mesh_color_matrix_shader: Option<GlShapeMeshColorScaleBiasShader>,
    pub shape_mesh_color_scale_bias_shader: Option<GlShapeMeshColorScaleBiasShader>,
    pub shape_mesh_pipelines: Option<Vec<(String, WgpuShapeMeshPipeline)>>,
    pub skin_bind_pose: Option<MeshSkinBindPose>,
    pub surface_antialias_enabled: Option<bool>,
    pub surface_antialias_height: Option<f64>,
    pub surface_antialias_resolve_bind_group: Option<crate::OpaqueHostValue>,
    pub surface_antialias_resolve_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub surface_antialias_resolve_pipeline: Option<crate::OpaqueHostValue>,
    pub surface_antialias_texture: Option<crate::OpaqueHostValue>,
    pub surface_antialias_view: Option<crate::OpaqueHostValue>,
    pub surface_antialias_width: Option<f64>,
    pub surface_presentation_view: Option<crate::OpaqueHostValue>,
    pub temp_stack: Option<Vec<Renderable>>,
    pub text_field_signals: Option<TextFieldSignals>,
    pub text_layout: Option<TextLayoutResult>,
    pub text_layout_using_content_id: Option<f64>,
    pub texture_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub uniform_bind_group: Option<crate::OpaqueHostValue>,
    pub uniform_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub uniform_buffer: Option<crate::OpaqueHostValue>,
    pub uniform_color_scale_bias_shader: Option<GlUniformColorScaleBiasShader>,
    pub uniform_data: Option<Vec<f32>>,
    pub uniform_data_u32: Option<Vec<u32>>,
    pub uniform_offset: Option<f64>,
    pub uniform_stride: Option<f64>,
    pub video_element: Option<crate::OpaqueHostValue>,
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
    pub wgpu_external_texture_cache: Option<Vec<(ExternalTexture, WgpuTextureEntry)>>,
    pub wgpu_render_texture_cache: Option<Vec<(RenderTexture, WgpuRenderTextureEntry)>>,
    pub wgpu_render_texture_guard: Option<WgpuRenderTextureGuard>,
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
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord7 {
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for FlightPartialRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord8 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: Option<ClipRegion>,
}
impl PartialEq for FlightPartialRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord9 {
    pub __flight_identity: std::sync::Arc<()>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
}
impl PartialEq for FlightPartialRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord10 {
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
impl PartialEq for FlightPartialRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:18 (sha256:c3dc807b578ac94141dd73c6a0532f43a0d50d26a1aa7884792f64f943d23ca6)
const PARTICLE_TRANSFORM_STRIDE: f64 = 4.0_f64;

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:19 (sha256:a68bb9efd032a89d93f67112fefca6fd2cfff5d832357b4e7a07ee6a442870a2)
const PARTICLE_COLOR_STRIDE: f64 = 3.0_f64;

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:20 (sha256:eb764aa733e7360185c6e3e6b75a3ea1c9dd0be629332a9f56e32be51bdeedb3)
const PARTICLE_VELOCITY_STRIDE: f64 = 2.0_f64;

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:23 (sha256:71d79f0d93e5989dbbf39feb93b7f7dcf8d21f62a250477d3e0611c854b652b7)
pub const PARTICLE_EMITTER_DELETED_ID: f64 = 65535.0_f64;

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:25 (sha256:0bcb30f485744bbdf3cde2436bbdfb071edbac3b13c9572d55417439ecebc41c)
fn copy_local_bounds_rectangle(out: &mut Rectangle, source: &Node) -> () {
    let runtime = {
        let __flight_source = &(get_node2_d_runtime(&source));
        ParticleEmitter2DRuntime {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            anisotropy_ext: (__flight_source.anisotropy_ext).clone(),
            appearance_id: __flight_source.appearance_id,
            apply_blend_mode_parent: (__flight_source.apply_blend_mode_parent).clone(),
            binding_cache_guard: (__flight_source.binding_cache_guard).clone(),
            bounds_rectangle: (__flight_source.bounds_rectangle).clone(),
            bounds_using_local_bounds_id: __flight_source.bounds_using_local_bounds_id,
            bounds_using_local_transform_id: __flight_source.bounds_using_local_transform_id,
            bounds_version: __flight_source.bounds_version,
            build_text_layout_params: (__flight_source.build_text_layout_params).clone(),
            canvas_blend_effect_backdrops: (__flight_source.canvas_blend_effect_backdrops).clone(),
            canvas_texture_resolvers: (__flight_source.canvas_texture_resolvers).clone(),
            canvas_texture_view: (__flight_source.canvas_texture_view).clone(),
            canvas_view_cleared: __flight_source.canvas_view_cleared,
            children_id: __flight_source.children_id,
            clip_contour_pipelines: (__flight_source.clip_contour_pipelines).clone(),
            clip_contour_stack: (__flight_source.clip_contour_stack).clone(),
            clip_forms: (__flight_source.clip_forms).clone(),
            color_matrix_instanced_shader: (__flight_source.color_matrix_instanced_shader).clone(),
            color_scale_bias_instanced_shader: (__flight_source.color_scale_bias_instanced_shader)
                .clone(),
            color_tint_instanced_shader: (__flight_source.color_tint_instanced_shader).clone(),
            command_encoder: (__flight_source.command_encoder).clone(),
            compute_local_bounds_rectangle: (__flight_source.compute_local_bounds_rectangle)
                .clone(),
            current_blend_mode: (__flight_source.current_blend_mode).clone(),
            current_color_format: (__flight_source.current_color_format).clone(),
            current_framebuffer: (__flight_source.current_framebuffer).clone(),
            current_frame_id: __flight_source.current_frame_id,
            current_mask_depth: __flight_source.current_mask_depth,
            current_program: (__flight_source.current_program).clone(),
            current_texture: (__flight_source.current_texture).clone(),
            current_texture_straight_alpha: __flight_source.current_texture_straight_alpha,
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
            flush_pending_draws: (__flight_source.flush_pending_draws).clone(),
            frame_capture_buffer: (__flight_source.frame_capture_buffer).clone(),
            frame_capture_bytes_per_row: __flight_source.frame_capture_bytes_per_row,
            frame_capture_enabled: __flight_source.frame_capture_enabled,
            frame_capture_height: __flight_source.frame_capture_height,
            frame_capture_texture: (__flight_source.frame_capture_texture).clone(),
            frame_capture_width: __flight_source.frame_capture_width,
            gl_external_texture_cache: (__flight_source.gl_external_texture_cache).clone(),
            gl_render_texture_cache: (__flight_source.gl_render_texture_cache).clone(),
            gl_render_texture_guard: (__flight_source.gl_render_texture_guard).clone(),
            image_smoothing_enabled: __flight_source.image_smoothing_enabled,
            image_smoothing_quality: (__flight_source.image_smoothing_quality).clone(),
            input: (__flight_source.input).clone(),
            instance_velocities: (__flight_source.instance_velocities).clone(),
            interaction_signals: (__flight_source.interaction_signals).clone(),
            interaction_state: (__flight_source.interaction_state).clone(),
            is_local_bounds_rectangle_valid: (__flight_source.is_local_bounds_rectangle_valid)
                .clone(),
            linear_sampler: (__flight_source.linear_sampler).clone(),
            local_bounds_id: __flight_source.local_bounds_id,
            local_bounds_rectangle: (__flight_source.local_bounds_rectangle).clone(),
            local_bounds_texture: (__flight_source.local_bounds_texture).clone(),
            local_bounds_texture_version: __flight_source.local_bounds_texture_version,
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
            media_stream: (__flight_source.media_stream).clone(),
            mipmap_degraded_guard: (__flight_source.mipmap_degraded_guard).clone(),
            mipmap_generator: (__flight_source.mipmap_generator).clone(),
            mipmapped_textures: (__flight_source.mipmapped_textures).clone(),
            morph_bind_pose: (__flight_source.morph_bind_pose).clone(),
            morph_blended_weights: (__flight_source.morph_blended_weights).clone(),
            movie_clip_signals: (__flight_source.movie_clip_signals).clone(),
            nearest_sampler: (__flight_source.nearest_sampler).clone(),
            node_signals: (__flight_source.node_signals).clone(),
            pages: (__flight_source.pages).clone(),
            parent_reference_id: __flight_source.parent_reference_id,
            particle_corner_buffer: (__flight_source.particle_corner_buffer).clone(),
            particle_instance_capacity: __flight_source.particle_instance_capacity,
            particle_shader: (__flight_source.particle_shader).clone(),
            pipeline_cache: (__flight_source.pipeline_cache).clone(),
            quad_batch_corner_buffer: (__flight_source.quad_batch_corner_buffer).clone(),
            quad_batch_shader: (__flight_source.quad_batch_shader).clone(),
            quad_batch_writer_blend_mode: (__flight_source.quad_batch_writer_blend_mode).clone(),
            quad_batch_writer_buffer_cursor: __flight_source.quad_batch_writer_buffer_cursor,
            quad_batch_writer_buffer_pool: (__flight_source.quad_batch_writer_buffer_pool).clone(),
            quad_batch_writer_color_matrix_data: (__flight_source
                .quad_batch_writer_color_matrix_data)
                .clone(),
            quad_batch_writer_color_scale_bias_buffer: (__flight_source
                .quad_batch_writer_color_scale_bias_buffer)
                .clone(),
            quad_batch_writer_color_scale_bias_data: (__flight_source
                .quad_batch_writer_color_scale_bias_data)
                .clone(),
            quad_batch_writer_color_scale_bias_mode: __flight_source
                .quad_batch_writer_color_scale_bias_mode,
            quad_batch_writer_color_tint_data: (__flight_source.quad_batch_writer_color_tint_data)
                .clone(),
            quad_batch_writer_count: __flight_source.quad_batch_writer_count,
            quad_batch_writer_instance_buffer: (__flight_source.quad_batch_writer_instance_buffer)
                .clone(),
            quad_batch_writer_instance_data: (__flight_source.quad_batch_writer_instance_data)
                .clone(),
            quad_batch_writer_material: (__flight_source.quad_batch_writer_material).clone(),
            quad_batch_writer_material_buffer: (__flight_source.quad_batch_writer_material_buffer)
                .clone(),
            quad_batch_writer_material_data: (__flight_source.quad_batch_writer_material_data)
                .clone(),
            quad_batch_writer_material_floats: __flight_source.quad_batch_writer_material_floats,
            quad_batch_writer_sampler: (__flight_source.quad_batch_writer_sampler).clone(),
            quad_batch_writer_smoothing: __flight_source.quad_batch_writer_smoothing,
            quad_batch_writer_straight_alpha: __flight_source.quad_batch_writer_straight_alpha,
            quad_batch_writer_uniform_color_scale_bias: (__flight_source
                .quad_batch_writer_uniform_color_scale_bias)
                .clone(),
            quad_index_buffer: (__flight_source.quad_index_buffer).clone(),
            quad_vertex_buffer: (__flight_source.quad_vertex_buffer).clone(),
            quad_vertex_data: (__flight_source.quad_vertex_data).clone(),
            render_adapt_hook: (__flight_source.render_adapt_hook).clone(),
            renderer_map_id: __flight_source.renderer_map_id,
            render_pass: (__flight_source.render_pass).clone(),
            render_proxy_adapter_map: (__flight_source.render_proxy_adapter_map).clone(),
            render_proxy_map: (__flight_source.render_proxy_map).clone(),
            render_proxy_sources: (__flight_source.render_proxy_sources).clone(),
            render_target_stack: (__flight_source.render_target_stack).clone(),
            retired_buffers: (__flight_source.retired_buffers).clone(),
            retired_textures: (__flight_source.retired_textures).clone(),
            rich_text_content: (__flight_source.rich_text_content).clone(),
            rotation_angle: __flight_source.rotation_angle,
            rotation_cosine: __flight_source.rotation_cosine,
            rotation_sine: __flight_source.rotation_sine,
            sampler_cache: (__flight_source.sampler_cache).clone(),
            scene2d: (__flight_source.scene2d).clone(),
            scene2d_signals: (__flight_source.scene2d_signals).clone(),
            scene_mesh_upload_cache: (__flight_source.scene_mesh_upload_cache).clone(),
            selection_begin_index: __flight_source.selection_begin_index,
            selection_end_index: __flight_source.selection_end_index,
            shader_loc: (__flight_source.shader_loc).clone(),
            shape_bounds_command_registry_revision: __flight_source
                .shape_bounds_command_registry_revision,
            shape_mesh_color_matrix_shader: (__flight_source.shape_mesh_color_matrix_shader)
                .clone(),
            shape_mesh_color_scale_bias_shader: (__flight_source
                .shape_mesh_color_scale_bias_shader)
                .clone(),
            shape_mesh_pipelines: (__flight_source.shape_mesh_pipelines).clone(),
            skin_bind_pose: (__flight_source.skin_bind_pose).clone(),
            surface_antialias_enabled: __flight_source.surface_antialias_enabled,
            surface_antialias_height: __flight_source.surface_antialias_height,
            surface_antialias_resolve_bind_group: (__flight_source
                .surface_antialias_resolve_bind_group)
                .clone(),
            surface_antialias_resolve_bind_group_layout: (__flight_source
                .surface_antialias_resolve_bind_group_layout)
                .clone(),
            surface_antialias_resolve_pipeline: (__flight_source
                .surface_antialias_resolve_pipeline)
                .clone(),
            surface_antialias_texture: (__flight_source.surface_antialias_texture).clone(),
            surface_antialias_view: (__flight_source.surface_antialias_view).clone(),
            surface_antialias_width: __flight_source.surface_antialias_width,
            surface_presentation_view: (__flight_source.surface_presentation_view).clone(),
            temp_stack: (__flight_source.temp_stack).clone(),
            text_field_signals: (__flight_source.text_field_signals).clone(),
            text_layout: (__flight_source.text_layout).clone(),
            text_layout_using_content_id: __flight_source.text_layout_using_content_id,
            texture_bind_group_layout: (__flight_source.texture_bind_group_layout).clone(),
            uniform_bind_group: (__flight_source.uniform_bind_group).clone(),
            uniform_bind_group_layout: (__flight_source.uniform_bind_group_layout).clone(),
            uniform_buffer: (__flight_source.uniform_buffer).clone(),
            uniform_color_scale_bias_shader: (__flight_source.uniform_color_scale_bias_shader)
                .clone(),
            uniform_data: (__flight_source.uniform_data).clone(),
            uniform_data_u32: (__flight_source.uniform_data_u32).clone(),
            uniform_offset: __flight_source.uniform_offset,
            uniform_stride: __flight_source.uniform_stride,
            video_element: (__flight_source.video_element).clone(),
            webgl_data: (__flight_source.webgl_data).clone(),
            webgl_shader_binding_resolver: (__flight_source.webgl_shader_binding_resolver).clone(),
            webgpu_data: (__flight_source.webgpu_data).clone(),
            webgpu_shader_binding_resolver: (__flight_source.webgpu_shader_binding_resolver)
                .clone(),
            wgpu_external_texture_cache: (__flight_source.wgpu_external_texture_cache).clone(),
            wgpu_render_texture_cache: (__flight_source.wgpu_render_texture_cache).clone(),
            wgpu_render_texture_guard: (__flight_source.wgpu_render_texture_guard).clone(),
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
            color_adjustments: (__flight_source.color_adjustments).clone(),
            color_adjustments_unsupported: __flight_source.color_adjustments_unsupported,
            resolved_color_matrix: (__flight_source.resolved_color_matrix).clone(),
            resolved_color_scale_bias: (__flight_source.resolved_color_scale_bias).clone(),
            can_add_child: (__flight_source.can_add_child).clone(),
            children: (__flight_source.children).clone(),
            traits: (__flight_source.traits).clone(),
            parent: (__flight_source.parent).clone(),
        }
    };
    if ((runtime.inner.lock().unwrap().local_bounds_rectangle).clone()).is_some() {
        copy_rectangle(
            out,
            runtime
                .inner
                .lock()
                .unwrap()
                .local_bounds_rectangle
                .as_ref()
                .unwrap(),
        );
    }
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:35 (sha256:2788ce505a542247808551d47f12ca2e78f18caeb15b11ab14cacd136629b61f)
pub fn append_particle_emitter2_d_particle(
    target: &mut ParticleEmitter2D,
    id: f64,
    x: f64,
    y: f64,
    rotation: f64,
    scale: f64,
) -> f64 {
    let index = target.data.particle_count;
    let needed = (index + 1.0_f64);
    if (get_particle_emitter2_d_capacity(target) < needed) {
        let new_capacity = (needed).max(if (target.data.particle_count * 2.0_f64) != 0.0_f64 {
            (target.data.particle_count * 2.0_f64)
        } else {
            8.0_f64
        });
        reserve_particle_emitter2_d(target, new_capacity);
    }
    target.data.particle_count = needed;
    target.data.ids[index as usize] = (id) as u16;
    let tt = (index * PARTICLE_TRANSFORM_STRIDE);
    target.data.transforms[tt as usize] = (x) as f32;
    target.data.transforms[(tt + 1.0_f64) as usize] = (y) as f32;
    target.data.transforms[(tt + 2.0_f64) as usize] = (rotation) as f32;
    target.data.transforms[(tt + 3.0_f64) as usize] = (scale) as f32;
    target.data.alphas[index as usize] = (1.0_f64) as f32;
    let ct = (index * PARTICLE_COLOR_STRIDE);
    target.data.colors[ct as usize] = (1.0_f64) as f32;
    target.data.colors[(ct + 1.0_f64) as usize] = (1.0_f64) as f32;
    target.data.colors[(ct + 2.0_f64) as usize] = (1.0_f64) as f32;
    let vt = (index * PARTICLE_VELOCITY_STRIDE);
    target.data.velocities[vt as usize] = (0.0_f64) as f32;
    target.data.velocities[(vt + 1.0_f64) as usize] = (0.0_f64) as f32;
    target.data.positions_z[index as usize] = (0.0_f64) as f32;
    return index;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:69 (sha256:11b7e83c0f02912ee4e83426d424880dce75ae4ab37dff169265ab5cc1e44fcc)
pub fn clear_particle_emitter2_d(target: &mut ParticleEmitter2D) -> () {
    target.data.particle_count = 0.0_f64;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:78 (sha256:8ed8e008f6acaf4c31555649e09ece614a8850a31d91dcd650a6436e0649ec11)
pub fn clone_particle_emitter2_d(source: &ParticleEmitter2D) -> ParticleEmitter2D {
    return create_particle_emitter2_d(Some(ParticleEmitter2D {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        data: ParticleEmitterData {
            __flight_identity: std::sync::Arc::new(()),
            alphas: ((source.data.alphas).clone()).clone(),
            atlas: (source.data.atlas).clone(),
            colors: ((source.data.colors).clone()).clone(),
            ids: ((source.data.ids).clone()).clone(),
            particle_count: source.data.particle_count,
            positions_z: ((source.data.positions_z).clone()).clone(),
            transforms: ((source.data.transforms).clone()).clone(),
            velocities: ((source.data.velocities).clone()).clone(),
            world_space: source.data.world_space,
        },
    }));
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:103 (sha256:a59bf4d29f9a5385ed82057f408cfbbfdece6e71b6943860b098ee8caffccc95)
pub fn compact_particle_emitter2_d(target: &mut ParticleEmitter2D) -> () {
    if (target.data.particle_count == 0.0_f64) {
        return;
    }
    let mut write = 0.0_f64;
    {
        let mut read = 0.0_f64;
        while (read < target.data.particle_count) {
            if ((target.data.ids[read as usize] as f64) == PARTICLE_EMITTER_DELETED_ID) {
                {
                    read += 1.0;
                    read
                };
                continue;
            }
            if (write != read) {
                target.data.ids[write as usize] = (target.data.ids[read as usize] as f64) as u16;
                let tt = (write * PARTICLE_TRANSFORM_STRIDE);
                let tts = (read * PARTICLE_TRANSFORM_STRIDE);
                target.data.transforms[tt as usize] =
                    (target.data.transforms[tts as usize] as f64) as f32;
                target.data.transforms[(tt + 1.0_f64) as usize] =
                    (target.data.transforms[(tts + 1.0_f64) as usize] as f64) as f32;
                target.data.transforms[(tt + 2.0_f64) as usize] =
                    (target.data.transforms[(tts + 2.0_f64) as usize] as f64) as f32;
                target.data.transforms[(tt + 3.0_f64) as usize] =
                    (target.data.transforms[(tts + 3.0_f64) as usize] as f64) as f32;
                target.data.alphas[write as usize] =
                    (target.data.alphas[read as usize] as f64) as f32;
                let ct = (write * PARTICLE_COLOR_STRIDE);
                let cts = (read * PARTICLE_COLOR_STRIDE);
                target.data.colors[ct as usize] = (target.data.colors[cts as usize] as f64) as f32;
                target.data.colors[(ct + 1.0_f64) as usize] =
                    (target.data.colors[(cts + 1.0_f64) as usize] as f64) as f32;
                target.data.colors[(ct + 2.0_f64) as usize] =
                    (target.data.colors[(cts + 2.0_f64) as usize] as f64) as f32;
                let vt = (write * PARTICLE_VELOCITY_STRIDE);
                let vts = (read * PARTICLE_VELOCITY_STRIDE);
                target.data.velocities[vt as usize] =
                    (target.data.velocities[vts as usize] as f64) as f32;
                target.data.velocities[(vt + 1.0_f64) as usize] =
                    (target.data.velocities[(vts + 1.0_f64) as usize] as f64) as f32;
                target.data.positions_z[write as usize] =
                    (target.data.positions_z[read as usize] as f64) as f32;
            }
            {
                write += 1.0;
                write
            };
            {
                read += 1.0;
                read
            };
        }
    }
    target.data.particle_count = write;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:134 (sha256:45bf6456c2160b69cf59562b9622e1e863a8e07b48c84c204fbd8d3966932de4)
pub fn compute_particle_emitter2_d_local_bounds_rectangle(
    out: &mut Rectangle,
    source: &ParticleEmitter2D,
) -> () {
    let atlas = (source.data.atlas).clone();
    let particle_count = source.data.particle_count;
    if ((atlas).is_none()) || (particle_count == 0.0_f64) {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        out.width = 0.0_f64;
        out.height = 0.0_f64;
        return;
    }
    let num_regions = (atlas.as_ref().unwrap().regions.len() as f64);
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = (-f64::INFINITY);
    let mut max_y = (-f64::INFINITY);
    {
        let mut i = 0.0_f64;
        while (i < particle_count) {
            let id = (source.data.ids[i as usize] as f64);
            if (id < 0.0_f64) || (id >= num_regions) {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let region = atlas.as_ref().unwrap().regions[id as usize].clone();
            if (region.width <= 0.0_f64) || (region.height <= 0.0_f64) {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let tt = (i * PARTICLE_TRANSFORM_STRIDE);
            let px = (source.data.transforms[tt as usize] as f64);
            let py = (source.data.transforms[(tt + 1.0_f64) as usize] as f64);
            let rotation = (source.data.transforms[(tt + 2.0_f64) as usize] as f64);
            let scale = (source.data.transforms[(tt + 3.0_f64) as usize] as f64);
            let cos_r = (((rotation).clone()).cos() * scale);
            let sin_r = (((rotation).clone()).sin() * scale);
            let w = region.width;
            let h = region.height;
            let x0 = (px).clone();
            let y0 = (py).clone();
            let x1 = ((cos_r * w) + (px).clone());
            let y1 = ((sin_r * w) + (py).clone());
            let x2 = (((cos_r * w) - (sin_r * h)) + (px).clone());
            let y2 = (((sin_r * w) + (cos_r * h)) + (py).clone());
            let x3 = (((-sin_r) * h) + (px).clone());
            let y3 = ((cos_r * h) + (py).clone());
            let q_min_x = ((((x0).clone()).min(x1)).min(x2)).min(x3);
            let q_min_y = ((((y0).clone()).min(y1)).min(y2)).min(y3);
            let q_max_x = ((((x0).clone()).max(x1)).max(x2)).max(x3);
            let q_max_y = ((((y0).clone()).max(y1)).max(y2)).max(y3);
            if (q_min_x < min_x) {
                min_x = q_min_x;
            }
            if (q_min_y < min_y) {
                min_y = q_min_y;
            }
            if (q_max_x > max_x) {
                max_x = q_max_x;
            }
            if (q_max_y > max_y) {
                max_y = q_max_y;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    if (min_x == f64::INFINITY) {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        out.width = 0.0_f64;
        out.height = 0.0_f64;
    } else {
        out.x = min_x;
        out.y = min_y;
        out.width = (max_x - min_x);
        out.height = (max_y - min_y);
    }
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:197 (sha256:1d12cd7665ca394b7e49e26b58cd774d511326a44f9b4d7ebc853318618c5032)
pub fn create_particle_emitter2_d(obj: Option<ParticleEmitter2D>) -> ParticleEmitter2D {
    return create_node2_d(
        (particle_emitter2_d_kind_constant).to_owned(),
        Some(((obj).clone().unwrap()).clone()),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<FlightPartialRecord7>| -> Node2DData {
                create_particle_emitter_data(Some(((__flight_argument_0).clone().unwrap()).clone()))
            },
        )
            as Box<
                dyn FnMut(Option<FlightPartialRecord7>) -> Node2DData + Send + 'static,
            >))),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<R>| -> R { create_particle_emitter2_d_runtime() },
        )
            as Box<dyn FnMut(Option<R>) -> R + Send + 'static>))),
    );
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:206 (sha256:de456ddf65f59b14ffc02923eb0ada237b0faa638bdd539c186a6079e1983999)
pub fn create_particle_emitter2_d_runtime() -> ParticleEmitter2DRuntime {
    let mut runtime = {
        let __flight_source = &(create_node2_d_runtime(Some(((*DEFAULT_METHODS).clone()).clone())));
        ParticleEmitter2DRuntime {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            anisotropy_ext: (__flight_source.anisotropy_ext).clone(),
            appearance_id: __flight_source.appearance_id,
            apply_blend_mode_parent: (__flight_source.apply_blend_mode_parent).clone(),
            binding_cache_guard: (__flight_source.binding_cache_guard).clone(),
            bounds_rectangle: (__flight_source.bounds_rectangle).clone(),
            bounds_using_local_bounds_id: __flight_source.bounds_using_local_bounds_id,
            bounds_using_local_transform_id: __flight_source.bounds_using_local_transform_id,
            bounds_version: __flight_source.bounds_version,
            build_text_layout_params: (__flight_source.build_text_layout_params).clone(),
            canvas_blend_effect_backdrops: (__flight_source.canvas_blend_effect_backdrops).clone(),
            canvas_texture_resolvers: (__flight_source.canvas_texture_resolvers).clone(),
            canvas_texture_view: (__flight_source.canvas_texture_view).clone(),
            canvas_view_cleared: __flight_source.canvas_view_cleared,
            children_id: __flight_source.children_id,
            clip_contour_pipelines: (__flight_source.clip_contour_pipelines).clone(),
            clip_contour_stack: (__flight_source.clip_contour_stack).clone(),
            clip_forms: (__flight_source.clip_forms).clone(),
            color_matrix_instanced_shader: (__flight_source.color_matrix_instanced_shader).clone(),
            color_scale_bias_instanced_shader: (__flight_source.color_scale_bias_instanced_shader)
                .clone(),
            color_tint_instanced_shader: (__flight_source.color_tint_instanced_shader).clone(),
            command_encoder: (__flight_source.command_encoder).clone(),
            compute_local_bounds_rectangle: (__flight_source.compute_local_bounds_rectangle)
                .clone(),
            current_blend_mode: (__flight_source.current_blend_mode).clone(),
            current_color_format: (__flight_source.current_color_format).clone(),
            current_framebuffer: (__flight_source.current_framebuffer).clone(),
            current_frame_id: __flight_source.current_frame_id,
            current_mask_depth: __flight_source.current_mask_depth,
            current_program: (__flight_source.current_program).clone(),
            current_texture: (__flight_source.current_texture).clone(),
            current_texture_straight_alpha: __flight_source.current_texture_straight_alpha,
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
            flush_pending_draws: (__flight_source.flush_pending_draws).clone(),
            frame_capture_buffer: (__flight_source.frame_capture_buffer).clone(),
            frame_capture_bytes_per_row: __flight_source.frame_capture_bytes_per_row,
            frame_capture_enabled: __flight_source.frame_capture_enabled,
            frame_capture_height: __flight_source.frame_capture_height,
            frame_capture_texture: (__flight_source.frame_capture_texture).clone(),
            frame_capture_width: __flight_source.frame_capture_width,
            gl_external_texture_cache: (__flight_source.gl_external_texture_cache).clone(),
            gl_render_texture_cache: (__flight_source.gl_render_texture_cache).clone(),
            gl_render_texture_guard: (__flight_source.gl_render_texture_guard).clone(),
            image_smoothing_enabled: __flight_source.image_smoothing_enabled,
            image_smoothing_quality: (__flight_source.image_smoothing_quality).clone(),
            input: (__flight_source.input).clone(),
            instance_velocities: (__flight_source.instance_velocities).clone(),
            interaction_signals: (__flight_source.interaction_signals).clone(),
            interaction_state: (__flight_source.interaction_state).clone(),
            is_local_bounds_rectangle_valid: (__flight_source.is_local_bounds_rectangle_valid)
                .clone(),
            linear_sampler: (__flight_source.linear_sampler).clone(),
            local_bounds_id: __flight_source.local_bounds_id,
            local_bounds_rectangle: (__flight_source.local_bounds_rectangle).clone(),
            local_bounds_texture: (__flight_source.local_bounds_texture).clone(),
            local_bounds_texture_version: __flight_source.local_bounds_texture_version,
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
            media_stream: (__flight_source.media_stream).clone(),
            mipmap_degraded_guard: (__flight_source.mipmap_degraded_guard).clone(),
            mipmap_generator: (__flight_source.mipmap_generator).clone(),
            mipmapped_textures: (__flight_source.mipmapped_textures).clone(),
            morph_bind_pose: (__flight_source.morph_bind_pose).clone(),
            morph_blended_weights: (__flight_source.morph_blended_weights).clone(),
            movie_clip_signals: (__flight_source.movie_clip_signals).clone(),
            nearest_sampler: (__flight_source.nearest_sampler).clone(),
            node_signals: (__flight_source.node_signals).clone(),
            pages: (__flight_source.pages).clone(),
            parent_reference_id: __flight_source.parent_reference_id,
            particle_corner_buffer: (__flight_source.particle_corner_buffer).clone(),
            particle_instance_capacity: __flight_source.particle_instance_capacity,
            particle_shader: (__flight_source.particle_shader).clone(),
            pipeline_cache: (__flight_source.pipeline_cache).clone(),
            quad_batch_corner_buffer: (__flight_source.quad_batch_corner_buffer).clone(),
            quad_batch_shader: (__flight_source.quad_batch_shader).clone(),
            quad_batch_writer_blend_mode: (__flight_source.quad_batch_writer_blend_mode).clone(),
            quad_batch_writer_buffer_cursor: __flight_source.quad_batch_writer_buffer_cursor,
            quad_batch_writer_buffer_pool: (__flight_source.quad_batch_writer_buffer_pool).clone(),
            quad_batch_writer_color_matrix_data: (__flight_source
                .quad_batch_writer_color_matrix_data)
                .clone(),
            quad_batch_writer_color_scale_bias_buffer: (__flight_source
                .quad_batch_writer_color_scale_bias_buffer)
                .clone(),
            quad_batch_writer_color_scale_bias_data: (__flight_source
                .quad_batch_writer_color_scale_bias_data)
                .clone(),
            quad_batch_writer_color_scale_bias_mode: __flight_source
                .quad_batch_writer_color_scale_bias_mode,
            quad_batch_writer_color_tint_data: (__flight_source.quad_batch_writer_color_tint_data)
                .clone(),
            quad_batch_writer_count: __flight_source.quad_batch_writer_count,
            quad_batch_writer_instance_buffer: (__flight_source.quad_batch_writer_instance_buffer)
                .clone(),
            quad_batch_writer_instance_data: (__flight_source.quad_batch_writer_instance_data)
                .clone(),
            quad_batch_writer_material: (__flight_source.quad_batch_writer_material).clone(),
            quad_batch_writer_material_buffer: (__flight_source.quad_batch_writer_material_buffer)
                .clone(),
            quad_batch_writer_material_data: (__flight_source.quad_batch_writer_material_data)
                .clone(),
            quad_batch_writer_material_floats: __flight_source.quad_batch_writer_material_floats,
            quad_batch_writer_sampler: (__flight_source.quad_batch_writer_sampler).clone(),
            quad_batch_writer_smoothing: __flight_source.quad_batch_writer_smoothing,
            quad_batch_writer_straight_alpha: __flight_source.quad_batch_writer_straight_alpha,
            quad_batch_writer_uniform_color_scale_bias: (__flight_source
                .quad_batch_writer_uniform_color_scale_bias)
                .clone(),
            quad_index_buffer: (__flight_source.quad_index_buffer).clone(),
            quad_vertex_buffer: (__flight_source.quad_vertex_buffer).clone(),
            quad_vertex_data: (__flight_source.quad_vertex_data).clone(),
            render_adapt_hook: (__flight_source.render_adapt_hook).clone(),
            renderer_map_id: __flight_source.renderer_map_id,
            render_pass: (__flight_source.render_pass).clone(),
            render_proxy_adapter_map: (__flight_source.render_proxy_adapter_map).clone(),
            render_proxy_map: (__flight_source.render_proxy_map).clone(),
            render_proxy_sources: (__flight_source.render_proxy_sources).clone(),
            render_target_stack: (__flight_source.render_target_stack).clone(),
            retired_buffers: (__flight_source.retired_buffers).clone(),
            retired_textures: (__flight_source.retired_textures).clone(),
            rich_text_content: (__flight_source.rich_text_content).clone(),
            rotation_angle: __flight_source.rotation_angle,
            rotation_cosine: __flight_source.rotation_cosine,
            rotation_sine: __flight_source.rotation_sine,
            sampler_cache: (__flight_source.sampler_cache).clone(),
            scene2d: (__flight_source.scene2d).clone(),
            scene2d_signals: (__flight_source.scene2d_signals).clone(),
            scene_mesh_upload_cache: (__flight_source.scene_mesh_upload_cache).clone(),
            selection_begin_index: __flight_source.selection_begin_index,
            selection_end_index: __flight_source.selection_end_index,
            shader_loc: (__flight_source.shader_loc).clone(),
            shape_bounds_command_registry_revision: __flight_source
                .shape_bounds_command_registry_revision,
            shape_mesh_color_matrix_shader: (__flight_source.shape_mesh_color_matrix_shader)
                .clone(),
            shape_mesh_color_scale_bias_shader: (__flight_source
                .shape_mesh_color_scale_bias_shader)
                .clone(),
            shape_mesh_pipelines: (__flight_source.shape_mesh_pipelines).clone(),
            skin_bind_pose: (__flight_source.skin_bind_pose).clone(),
            surface_antialias_enabled: __flight_source.surface_antialias_enabled,
            surface_antialias_height: __flight_source.surface_antialias_height,
            surface_antialias_resolve_bind_group: (__flight_source
                .surface_antialias_resolve_bind_group)
                .clone(),
            surface_antialias_resolve_bind_group_layout: (__flight_source
                .surface_antialias_resolve_bind_group_layout)
                .clone(),
            surface_antialias_resolve_pipeline: (__flight_source
                .surface_antialias_resolve_pipeline)
                .clone(),
            surface_antialias_texture: (__flight_source.surface_antialias_texture).clone(),
            surface_antialias_view: (__flight_source.surface_antialias_view).clone(),
            surface_antialias_width: __flight_source.surface_antialias_width,
            surface_presentation_view: (__flight_source.surface_presentation_view).clone(),
            temp_stack: (__flight_source.temp_stack).clone(),
            text_field_signals: (__flight_source.text_field_signals).clone(),
            text_layout: (__flight_source.text_layout).clone(),
            text_layout_using_content_id: __flight_source.text_layout_using_content_id,
            texture_bind_group_layout: (__flight_source.texture_bind_group_layout).clone(),
            uniform_bind_group: (__flight_source.uniform_bind_group).clone(),
            uniform_bind_group_layout: (__flight_source.uniform_bind_group_layout).clone(),
            uniform_buffer: (__flight_source.uniform_buffer).clone(),
            uniform_color_scale_bias_shader: (__flight_source.uniform_color_scale_bias_shader)
                .clone(),
            uniform_data: (__flight_source.uniform_data).clone(),
            uniform_data_u32: (__flight_source.uniform_data_u32).clone(),
            uniform_offset: __flight_source.uniform_offset,
            uniform_stride: __flight_source.uniform_stride,
            video_element: (__flight_source.video_element).clone(),
            webgl_data: (__flight_source.webgl_data).clone(),
            webgl_shader_binding_resolver: (__flight_source.webgl_shader_binding_resolver).clone(),
            webgpu_data: (__flight_source.webgpu_data).clone(),
            webgpu_shader_binding_resolver: (__flight_source.webgpu_shader_binding_resolver)
                .clone(),
            wgpu_external_texture_cache: (__flight_source.wgpu_external_texture_cache).clone(),
            wgpu_render_texture_cache: (__flight_source.wgpu_render_texture_cache).clone(),
            wgpu_render_texture_guard: (__flight_source.wgpu_render_texture_guard).clone(),
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
            color_adjustments: (__flight_source.color_adjustments).clone(),
            color_adjustments_unsupported: __flight_source.color_adjustments_unsupported,
            resolved_color_matrix: (__flight_source.resolved_color_matrix).clone(),
            resolved_color_scale_bias: (__flight_source.resolved_color_scale_bias).clone(),
            can_add_child: (__flight_source.can_add_child).clone(),
            children: (__flight_source.children).clone(),
            traits: (__flight_source.traits).clone(),
            parent: (__flight_source.parent).clone(),
        }
    };
    {
        let __flight_runtime = runtime;
        let __flight_value = None;
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.local_bounds_rectangle = __flight_value;
    };
    return runtime;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:212 (sha256:45f7f66c56c708593de9490b6aa1113bc1bdf2a0dbcc4e19357d6b7b403d3829)
pub fn create_particle_emitter_data(data: Option<FlightPartialRecord1>) -> ParticleEmitterData {
    return ParticleEmitterData {
        __flight_identity: std::sync::Arc::new(()),
        alphas: (data.as_ref().and_then(|value| (value.alphas).clone()))
            .clone()
            .unwrap_or(vec![0.0_f32; (0.0_f64) as usize]),
        atlas: data.as_ref().and_then(|value| (value.atlas).clone()),
        colors: (data.as_ref().and_then(|value| (value.colors).clone()))
            .clone()
            .unwrap_or(vec![0.0_f32; (0.0_f64) as usize]),
        ids: (data.as_ref().and_then(|value| (value.ids).clone()))
            .clone()
            .unwrap_or(vec![0_u16; (0.0_f64) as usize]),
        particle_count: (data.as_ref().and_then(|value| value.particle_count))
            .clone()
            .unwrap_or(0.0_f64),
        positions_z: (data.as_ref().and_then(|value| (value.positions_z).clone()))
            .clone()
            .unwrap_or(vec![0.0_f32; (0.0_f64) as usize]),
        transforms: (data.as_ref().and_then(|value| (value.transforms).clone()))
            .clone()
            .unwrap_or(vec![0.0_f32; (0.0_f64) as usize]),
        velocities: (data.as_ref().and_then(|value| (value.velocities).clone()))
            .clone()
            .unwrap_or(vec![0.0_f32; (0.0_f64) as usize]),
        world_space: (data.as_ref().and_then(|value| value.world_space))
            .clone()
            .unwrap_or(false),
    };
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:226 (sha256:2122bde041aaf676bc47b91f9ecd068397edf9664a5a32c87ae8213ca9032ae4)
pub fn get_particle_emitter2_d_capacity(source: &ParticleEmitter2D) -> f64 {
    let transform_capacity =
        (__flight_js_to_i32(((source.data.transforms.len() as f64) / PARTICLE_TRANSFORM_STRIDE))
            | __flight_js_to_i32(0.0_f64)) as f64;
    return ((source.data.ids.len() as f64).min((source.data.alphas.len() as f64)))
        .min(transform_capacity);
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:236 (sha256:7d546fd655588b8382824e4497bfd0c45d1e38dee80a9812030defe8e0e32940)
pub fn get_particle_emitter2_d_particle_alpha(source: &ParticleEmitter2D, index: f64) -> f64 {
    if (index < 0.0_f64) || (index >= source.data.particle_count) {
        return (-1.0_f64);
    }
    return (source.data.alphas[index as usize] as f64);
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:245 (sha256:e8962c58d485883de6971bd3bc89dcb9471dca9ea7f91f4576271bcc2a6a0b86)
pub fn get_particle_emitter2_d_particle_id(source: &ParticleEmitter2D, index: f64) -> f64 {
    if (index < 0.0_f64) || (index >= source.data.particle_count) {
        return (-1.0_f64);
    }
    return (source.data.ids[index as usize] as f64);
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:254 (sha256:2242f9402f423def48e50c87ff1b4cfa9c9747a3cbbe596023498e2d2411bda5)
pub fn get_particle_emitter2_d_particle_velocity(
    out: &mut Vector2Like,
    source: &ParticleEmitter2D,
    index: f64,
) -> bool {
    if (index < 0.0_f64) || (index >= source.data.particle_count) {
        return false;
    }
    let vt = (index * PARTICLE_VELOCITY_STRIDE);
    out.x = (source.data.velocities[vt as usize] as f64);
    out.y = (source.data.velocities[(vt + 1.0_f64) as usize] as f64);
    return true;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:266 (sha256:b7279e3b29d1068fca24f2e5998949f58f10138cb009b4859313664076335669)
pub fn get_particle_emitter2_d_runtime(source: &ParticleEmitter2D) -> ParticleEmitter2DRuntime {
    return {
        let __flight_source = &(get_node2_d_runtime(source));
        ParticleEmitter2DRuntime {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            anisotropy_ext: (__flight_source.anisotropy_ext).clone(),
            appearance_id: __flight_source.appearance_id,
            apply_blend_mode_parent: (__flight_source.apply_blend_mode_parent).clone(),
            binding_cache_guard: (__flight_source.binding_cache_guard).clone(),
            bounds_rectangle: (__flight_source.bounds_rectangle).clone(),
            bounds_using_local_bounds_id: __flight_source.bounds_using_local_bounds_id,
            bounds_using_local_transform_id: __flight_source.bounds_using_local_transform_id,
            bounds_version: __flight_source.bounds_version,
            build_text_layout_params: (__flight_source.build_text_layout_params).clone(),
            canvas_blend_effect_backdrops: (__flight_source.canvas_blend_effect_backdrops).clone(),
            canvas_texture_resolvers: (__flight_source.canvas_texture_resolvers).clone(),
            canvas_texture_view: (__flight_source.canvas_texture_view).clone(),
            canvas_view_cleared: __flight_source.canvas_view_cleared,
            children_id: __flight_source.children_id,
            clip_contour_pipelines: (__flight_source.clip_contour_pipelines).clone(),
            clip_contour_stack: (__flight_source.clip_contour_stack).clone(),
            clip_forms: (__flight_source.clip_forms).clone(),
            color_matrix_instanced_shader: (__flight_source.color_matrix_instanced_shader).clone(),
            color_scale_bias_instanced_shader: (__flight_source.color_scale_bias_instanced_shader)
                .clone(),
            color_tint_instanced_shader: (__flight_source.color_tint_instanced_shader).clone(),
            command_encoder: (__flight_source.command_encoder).clone(),
            compute_local_bounds_rectangle: (__flight_source.compute_local_bounds_rectangle)
                .clone(),
            current_blend_mode: (__flight_source.current_blend_mode).clone(),
            current_color_format: (__flight_source.current_color_format).clone(),
            current_framebuffer: (__flight_source.current_framebuffer).clone(),
            current_frame_id: __flight_source.current_frame_id,
            current_mask_depth: __flight_source.current_mask_depth,
            current_program: (__flight_source.current_program).clone(),
            current_texture: (__flight_source.current_texture).clone(),
            current_texture_straight_alpha: __flight_source.current_texture_straight_alpha,
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
            flush_pending_draws: (__flight_source.flush_pending_draws).clone(),
            frame_capture_buffer: (__flight_source.frame_capture_buffer).clone(),
            frame_capture_bytes_per_row: __flight_source.frame_capture_bytes_per_row,
            frame_capture_enabled: __flight_source.frame_capture_enabled,
            frame_capture_height: __flight_source.frame_capture_height,
            frame_capture_texture: (__flight_source.frame_capture_texture).clone(),
            frame_capture_width: __flight_source.frame_capture_width,
            gl_external_texture_cache: (__flight_source.gl_external_texture_cache).clone(),
            gl_render_texture_cache: (__flight_source.gl_render_texture_cache).clone(),
            gl_render_texture_guard: (__flight_source.gl_render_texture_guard).clone(),
            image_smoothing_enabled: __flight_source.image_smoothing_enabled,
            image_smoothing_quality: (__flight_source.image_smoothing_quality).clone(),
            input: (__flight_source.input).clone(),
            instance_velocities: (__flight_source.instance_velocities).clone(),
            interaction_signals: (__flight_source.interaction_signals).clone(),
            interaction_state: (__flight_source.interaction_state).clone(),
            is_local_bounds_rectangle_valid: (__flight_source.is_local_bounds_rectangle_valid)
                .clone(),
            linear_sampler: (__flight_source.linear_sampler).clone(),
            local_bounds_id: __flight_source.local_bounds_id,
            local_bounds_rectangle: (__flight_source.local_bounds_rectangle).clone(),
            local_bounds_texture: (__flight_source.local_bounds_texture).clone(),
            local_bounds_texture_version: __flight_source.local_bounds_texture_version,
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
            media_stream: (__flight_source.media_stream).clone(),
            mipmap_degraded_guard: (__flight_source.mipmap_degraded_guard).clone(),
            mipmap_generator: (__flight_source.mipmap_generator).clone(),
            mipmapped_textures: (__flight_source.mipmapped_textures).clone(),
            morph_bind_pose: (__flight_source.morph_bind_pose).clone(),
            morph_blended_weights: (__flight_source.morph_blended_weights).clone(),
            movie_clip_signals: (__flight_source.movie_clip_signals).clone(),
            nearest_sampler: (__flight_source.nearest_sampler).clone(),
            node_signals: (__flight_source.node_signals).clone(),
            pages: (__flight_source.pages).clone(),
            parent_reference_id: __flight_source.parent_reference_id,
            particle_corner_buffer: (__flight_source.particle_corner_buffer).clone(),
            particle_instance_capacity: __flight_source.particle_instance_capacity,
            particle_shader: (__flight_source.particle_shader).clone(),
            pipeline_cache: (__flight_source.pipeline_cache).clone(),
            quad_batch_corner_buffer: (__flight_source.quad_batch_corner_buffer).clone(),
            quad_batch_shader: (__flight_source.quad_batch_shader).clone(),
            quad_batch_writer_blend_mode: (__flight_source.quad_batch_writer_blend_mode).clone(),
            quad_batch_writer_buffer_cursor: __flight_source.quad_batch_writer_buffer_cursor,
            quad_batch_writer_buffer_pool: (__flight_source.quad_batch_writer_buffer_pool).clone(),
            quad_batch_writer_color_matrix_data: (__flight_source
                .quad_batch_writer_color_matrix_data)
                .clone(),
            quad_batch_writer_color_scale_bias_buffer: (__flight_source
                .quad_batch_writer_color_scale_bias_buffer)
                .clone(),
            quad_batch_writer_color_scale_bias_data: (__flight_source
                .quad_batch_writer_color_scale_bias_data)
                .clone(),
            quad_batch_writer_color_scale_bias_mode: __flight_source
                .quad_batch_writer_color_scale_bias_mode,
            quad_batch_writer_color_tint_data: (__flight_source.quad_batch_writer_color_tint_data)
                .clone(),
            quad_batch_writer_count: __flight_source.quad_batch_writer_count,
            quad_batch_writer_instance_buffer: (__flight_source.quad_batch_writer_instance_buffer)
                .clone(),
            quad_batch_writer_instance_data: (__flight_source.quad_batch_writer_instance_data)
                .clone(),
            quad_batch_writer_material: (__flight_source.quad_batch_writer_material).clone(),
            quad_batch_writer_material_buffer: (__flight_source.quad_batch_writer_material_buffer)
                .clone(),
            quad_batch_writer_material_data: (__flight_source.quad_batch_writer_material_data)
                .clone(),
            quad_batch_writer_material_floats: __flight_source.quad_batch_writer_material_floats,
            quad_batch_writer_sampler: (__flight_source.quad_batch_writer_sampler).clone(),
            quad_batch_writer_smoothing: __flight_source.quad_batch_writer_smoothing,
            quad_batch_writer_straight_alpha: __flight_source.quad_batch_writer_straight_alpha,
            quad_batch_writer_uniform_color_scale_bias: (__flight_source
                .quad_batch_writer_uniform_color_scale_bias)
                .clone(),
            quad_index_buffer: (__flight_source.quad_index_buffer).clone(),
            quad_vertex_buffer: (__flight_source.quad_vertex_buffer).clone(),
            quad_vertex_data: (__flight_source.quad_vertex_data).clone(),
            render_adapt_hook: (__flight_source.render_adapt_hook).clone(),
            renderer_map_id: __flight_source.renderer_map_id,
            render_pass: (__flight_source.render_pass).clone(),
            render_proxy_adapter_map: (__flight_source.render_proxy_adapter_map).clone(),
            render_proxy_map: (__flight_source.render_proxy_map).clone(),
            render_proxy_sources: (__flight_source.render_proxy_sources).clone(),
            render_target_stack: (__flight_source.render_target_stack).clone(),
            retired_buffers: (__flight_source.retired_buffers).clone(),
            retired_textures: (__flight_source.retired_textures).clone(),
            rich_text_content: (__flight_source.rich_text_content).clone(),
            rotation_angle: __flight_source.rotation_angle,
            rotation_cosine: __flight_source.rotation_cosine,
            rotation_sine: __flight_source.rotation_sine,
            sampler_cache: (__flight_source.sampler_cache).clone(),
            scene2d: (__flight_source.scene2d).clone(),
            scene2d_signals: (__flight_source.scene2d_signals).clone(),
            scene_mesh_upload_cache: (__flight_source.scene_mesh_upload_cache).clone(),
            selection_begin_index: __flight_source.selection_begin_index,
            selection_end_index: __flight_source.selection_end_index,
            shader_loc: (__flight_source.shader_loc).clone(),
            shape_bounds_command_registry_revision: __flight_source
                .shape_bounds_command_registry_revision,
            shape_mesh_color_matrix_shader: (__flight_source.shape_mesh_color_matrix_shader)
                .clone(),
            shape_mesh_color_scale_bias_shader: (__flight_source
                .shape_mesh_color_scale_bias_shader)
                .clone(),
            shape_mesh_pipelines: (__flight_source.shape_mesh_pipelines).clone(),
            skin_bind_pose: (__flight_source.skin_bind_pose).clone(),
            surface_antialias_enabled: __flight_source.surface_antialias_enabled,
            surface_antialias_height: __flight_source.surface_antialias_height,
            surface_antialias_resolve_bind_group: (__flight_source
                .surface_antialias_resolve_bind_group)
                .clone(),
            surface_antialias_resolve_bind_group_layout: (__flight_source
                .surface_antialias_resolve_bind_group_layout)
                .clone(),
            surface_antialias_resolve_pipeline: (__flight_source
                .surface_antialias_resolve_pipeline)
                .clone(),
            surface_antialias_texture: (__flight_source.surface_antialias_texture).clone(),
            surface_antialias_view: (__flight_source.surface_antialias_view).clone(),
            surface_antialias_width: __flight_source.surface_antialias_width,
            surface_presentation_view: (__flight_source.surface_presentation_view).clone(),
            temp_stack: (__flight_source.temp_stack).clone(),
            text_field_signals: (__flight_source.text_field_signals).clone(),
            text_layout: (__flight_source.text_layout).clone(),
            text_layout_using_content_id: __flight_source.text_layout_using_content_id,
            texture_bind_group_layout: (__flight_source.texture_bind_group_layout).clone(),
            uniform_bind_group: (__flight_source.uniform_bind_group).clone(),
            uniform_bind_group_layout: (__flight_source.uniform_bind_group_layout).clone(),
            uniform_buffer: (__flight_source.uniform_buffer).clone(),
            uniform_color_scale_bias_shader: (__flight_source.uniform_color_scale_bias_shader)
                .clone(),
            uniform_data: (__flight_source.uniform_data).clone(),
            uniform_data_u32: (__flight_source.uniform_data_u32).clone(),
            uniform_offset: __flight_source.uniform_offset,
            uniform_stride: __flight_source.uniform_stride,
            video_element: (__flight_source.video_element).clone(),
            webgl_data: (__flight_source.webgl_data).clone(),
            webgl_shader_binding_resolver: (__flight_source.webgl_shader_binding_resolver).clone(),
            webgpu_data: (__flight_source.webgpu_data).clone(),
            webgpu_shader_binding_resolver: (__flight_source.webgpu_shader_binding_resolver)
                .clone(),
            wgpu_external_texture_cache: (__flight_source.wgpu_external_texture_cache).clone(),
            wgpu_render_texture_cache: (__flight_source.wgpu_render_texture_cache).clone(),
            wgpu_render_texture_guard: (__flight_source.wgpu_render_texture_guard).clone(),
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
            color_adjustments: (__flight_source.color_adjustments).clone(),
            color_adjustments_unsupported: __flight_source.color_adjustments_unsupported,
            resolved_color_matrix: (__flight_source.resolved_color_matrix).clone(),
            resolved_color_scale_bias: (__flight_source.resolved_color_scale_bias).clone(),
            can_add_child: (__flight_source.can_add_child).clone(),
            children: (__flight_source.children).clone(),
            traits: (__flight_source.traits).clone(),
            parent: (__flight_source.parent).clone(),
        }
    };
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:275 (sha256:b4cc0ed441eb1df899df004437a26c521cc94a020a0163bf2c7941c6de8167fa)
pub fn remove_particle_emitter2_d_particle(target: &mut ParticleEmitter2D, index: f64) -> () {
    let last = (target.data.particle_count - 1.0_f64);
    if (index < 0.0_f64) || (index > last) {
        return;
    }
    if (index < last) {
        target.data.ids[index as usize] = (target.data.ids[last as usize] as f64) as u16;
        let tt = (index * PARTICLE_TRANSFORM_STRIDE);
        let tts = (last * PARTICLE_TRANSFORM_STRIDE);
        target.data.transforms[tt as usize] = (target.data.transforms[tts as usize] as f64) as f32;
        target.data.transforms[(tt + 1.0_f64) as usize] =
            (target.data.transforms[(tts + 1.0_f64) as usize] as f64) as f32;
        target.data.transforms[(tt + 2.0_f64) as usize] =
            (target.data.transforms[(tts + 2.0_f64) as usize] as f64) as f32;
        target.data.transforms[(tt + 3.0_f64) as usize] =
            (target.data.transforms[(tts + 3.0_f64) as usize] as f64) as f32;
        target.data.alphas[index as usize] = (target.data.alphas[last as usize] as f64) as f32;
        let ct = (index * PARTICLE_COLOR_STRIDE);
        let cts = (last * PARTICLE_COLOR_STRIDE);
        target.data.colors[ct as usize] = (target.data.colors[cts as usize] as f64) as f32;
        target.data.colors[(ct + 1.0_f64) as usize] =
            (target.data.colors[(cts + 1.0_f64) as usize] as f64) as f32;
        target.data.colors[(ct + 2.0_f64) as usize] =
            (target.data.colors[(cts + 2.0_f64) as usize] as f64) as f32;
        let vt = (index * PARTICLE_VELOCITY_STRIDE);
        let vts = (last * PARTICLE_VELOCITY_STRIDE);
        target.data.velocities[vt as usize] = (target.data.velocities[vts as usize] as f64) as f32;
        target.data.velocities[(vt + 1.0_f64) as usize] =
            (target.data.velocities[(vts + 1.0_f64) as usize] as f64) as f32;
        target.data.positions_z[index as usize] =
            (target.data.positions_z[last as usize] as f64) as f32;
    }
    target.data.particle_count = last;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:303 (sha256:2d6ca7518aa1ca9588c9d8b802af95a4f6c03e34f373736905df93a005581040)
pub fn reserve_particle_emitter2_d(target: &mut ParticleEmitter2D, capacity: f64) -> () {
    if (get_particle_emitter2_d_capacity(target) >= capacity) {
        return;
    }
    target.data.alphas = reserve_float32_array(&target.data.alphas, capacity);
    target.data.colors = reserve_float32_array(&target.data.colors, (capacity * 3.0_f64));
    target.data.ids = reserve_uint16_array(&target.data.ids, capacity);
    target.data.positions_z = reserve_float32_array(&target.data.positions_z, capacity);
    target.data.transforms = reserve_float32_array(
        &target.data.transforms,
        (capacity * PARTICLE_TRANSFORM_STRIDE),
    );
    target.data.velocities = reserve_float32_array(&target.data.velocities, (capacity * 2.0_f64));
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:314 (sha256:e27471ed27b12502617349a169ca71e5514153b056453349d2a53ddf5298b11c)
pub fn set_particle_emitter2_d_local_bounds_rectangle(
    target: &ParticleEmitter2D,
    rect: &Rectangle,
) -> () {
    let mut runtime = {
        let __flight_source = &(get_node2_d_runtime(target));
        ParticleEmitter2DRuntime {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            anisotropy_ext: (__flight_source.anisotropy_ext).clone(),
            appearance_id: __flight_source.appearance_id,
            apply_blend_mode_parent: (__flight_source.apply_blend_mode_parent).clone(),
            binding_cache_guard: (__flight_source.binding_cache_guard).clone(),
            bounds_rectangle: (__flight_source.bounds_rectangle).clone(),
            bounds_using_local_bounds_id: __flight_source.bounds_using_local_bounds_id,
            bounds_using_local_transform_id: __flight_source.bounds_using_local_transform_id,
            bounds_version: __flight_source.bounds_version,
            build_text_layout_params: (__flight_source.build_text_layout_params).clone(),
            canvas_blend_effect_backdrops: (__flight_source.canvas_blend_effect_backdrops).clone(),
            canvas_texture_resolvers: (__flight_source.canvas_texture_resolvers).clone(),
            canvas_texture_view: (__flight_source.canvas_texture_view).clone(),
            canvas_view_cleared: __flight_source.canvas_view_cleared,
            children_id: __flight_source.children_id,
            clip_contour_pipelines: (__flight_source.clip_contour_pipelines).clone(),
            clip_contour_stack: (__flight_source.clip_contour_stack).clone(),
            clip_forms: (__flight_source.clip_forms).clone(),
            color_matrix_instanced_shader: (__flight_source.color_matrix_instanced_shader).clone(),
            color_scale_bias_instanced_shader: (__flight_source.color_scale_bias_instanced_shader)
                .clone(),
            color_tint_instanced_shader: (__flight_source.color_tint_instanced_shader).clone(),
            command_encoder: (__flight_source.command_encoder).clone(),
            compute_local_bounds_rectangle: (__flight_source.compute_local_bounds_rectangle)
                .clone(),
            current_blend_mode: (__flight_source.current_blend_mode).clone(),
            current_color_format: (__flight_source.current_color_format).clone(),
            current_framebuffer: (__flight_source.current_framebuffer).clone(),
            current_frame_id: __flight_source.current_frame_id,
            current_mask_depth: __flight_source.current_mask_depth,
            current_program: (__flight_source.current_program).clone(),
            current_texture: (__flight_source.current_texture).clone(),
            current_texture_straight_alpha: __flight_source.current_texture_straight_alpha,
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
            flush_pending_draws: (__flight_source.flush_pending_draws).clone(),
            frame_capture_buffer: (__flight_source.frame_capture_buffer).clone(),
            frame_capture_bytes_per_row: __flight_source.frame_capture_bytes_per_row,
            frame_capture_enabled: __flight_source.frame_capture_enabled,
            frame_capture_height: __flight_source.frame_capture_height,
            frame_capture_texture: (__flight_source.frame_capture_texture).clone(),
            frame_capture_width: __flight_source.frame_capture_width,
            gl_external_texture_cache: (__flight_source.gl_external_texture_cache).clone(),
            gl_render_texture_cache: (__flight_source.gl_render_texture_cache).clone(),
            gl_render_texture_guard: (__flight_source.gl_render_texture_guard).clone(),
            image_smoothing_enabled: __flight_source.image_smoothing_enabled,
            image_smoothing_quality: (__flight_source.image_smoothing_quality).clone(),
            input: (__flight_source.input).clone(),
            instance_velocities: (__flight_source.instance_velocities).clone(),
            interaction_signals: (__flight_source.interaction_signals).clone(),
            interaction_state: (__flight_source.interaction_state).clone(),
            is_local_bounds_rectangle_valid: (__flight_source.is_local_bounds_rectangle_valid)
                .clone(),
            linear_sampler: (__flight_source.linear_sampler).clone(),
            local_bounds_id: __flight_source.local_bounds_id,
            local_bounds_rectangle: (__flight_source.local_bounds_rectangle).clone(),
            local_bounds_texture: (__flight_source.local_bounds_texture).clone(),
            local_bounds_texture_version: __flight_source.local_bounds_texture_version,
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
            media_stream: (__flight_source.media_stream).clone(),
            mipmap_degraded_guard: (__flight_source.mipmap_degraded_guard).clone(),
            mipmap_generator: (__flight_source.mipmap_generator).clone(),
            mipmapped_textures: (__flight_source.mipmapped_textures).clone(),
            morph_bind_pose: (__flight_source.morph_bind_pose).clone(),
            morph_blended_weights: (__flight_source.morph_blended_weights).clone(),
            movie_clip_signals: (__flight_source.movie_clip_signals).clone(),
            nearest_sampler: (__flight_source.nearest_sampler).clone(),
            node_signals: (__flight_source.node_signals).clone(),
            pages: (__flight_source.pages).clone(),
            parent_reference_id: __flight_source.parent_reference_id,
            particle_corner_buffer: (__flight_source.particle_corner_buffer).clone(),
            particle_instance_capacity: __flight_source.particle_instance_capacity,
            particle_shader: (__flight_source.particle_shader).clone(),
            pipeline_cache: (__flight_source.pipeline_cache).clone(),
            quad_batch_corner_buffer: (__flight_source.quad_batch_corner_buffer).clone(),
            quad_batch_shader: (__flight_source.quad_batch_shader).clone(),
            quad_batch_writer_blend_mode: (__flight_source.quad_batch_writer_blend_mode).clone(),
            quad_batch_writer_buffer_cursor: __flight_source.quad_batch_writer_buffer_cursor,
            quad_batch_writer_buffer_pool: (__flight_source.quad_batch_writer_buffer_pool).clone(),
            quad_batch_writer_color_matrix_data: (__flight_source
                .quad_batch_writer_color_matrix_data)
                .clone(),
            quad_batch_writer_color_scale_bias_buffer: (__flight_source
                .quad_batch_writer_color_scale_bias_buffer)
                .clone(),
            quad_batch_writer_color_scale_bias_data: (__flight_source
                .quad_batch_writer_color_scale_bias_data)
                .clone(),
            quad_batch_writer_color_scale_bias_mode: __flight_source
                .quad_batch_writer_color_scale_bias_mode,
            quad_batch_writer_color_tint_data: (__flight_source.quad_batch_writer_color_tint_data)
                .clone(),
            quad_batch_writer_count: __flight_source.quad_batch_writer_count,
            quad_batch_writer_instance_buffer: (__flight_source.quad_batch_writer_instance_buffer)
                .clone(),
            quad_batch_writer_instance_data: (__flight_source.quad_batch_writer_instance_data)
                .clone(),
            quad_batch_writer_material: (__flight_source.quad_batch_writer_material).clone(),
            quad_batch_writer_material_buffer: (__flight_source.quad_batch_writer_material_buffer)
                .clone(),
            quad_batch_writer_material_data: (__flight_source.quad_batch_writer_material_data)
                .clone(),
            quad_batch_writer_material_floats: __flight_source.quad_batch_writer_material_floats,
            quad_batch_writer_sampler: (__flight_source.quad_batch_writer_sampler).clone(),
            quad_batch_writer_smoothing: __flight_source.quad_batch_writer_smoothing,
            quad_batch_writer_straight_alpha: __flight_source.quad_batch_writer_straight_alpha,
            quad_batch_writer_uniform_color_scale_bias: (__flight_source
                .quad_batch_writer_uniform_color_scale_bias)
                .clone(),
            quad_index_buffer: (__flight_source.quad_index_buffer).clone(),
            quad_vertex_buffer: (__flight_source.quad_vertex_buffer).clone(),
            quad_vertex_data: (__flight_source.quad_vertex_data).clone(),
            render_adapt_hook: (__flight_source.render_adapt_hook).clone(),
            renderer_map_id: __flight_source.renderer_map_id,
            render_pass: (__flight_source.render_pass).clone(),
            render_proxy_adapter_map: (__flight_source.render_proxy_adapter_map).clone(),
            render_proxy_map: (__flight_source.render_proxy_map).clone(),
            render_proxy_sources: (__flight_source.render_proxy_sources).clone(),
            render_target_stack: (__flight_source.render_target_stack).clone(),
            retired_buffers: (__flight_source.retired_buffers).clone(),
            retired_textures: (__flight_source.retired_textures).clone(),
            rich_text_content: (__flight_source.rich_text_content).clone(),
            rotation_angle: __flight_source.rotation_angle,
            rotation_cosine: __flight_source.rotation_cosine,
            rotation_sine: __flight_source.rotation_sine,
            sampler_cache: (__flight_source.sampler_cache).clone(),
            scene2d: (__flight_source.scene2d).clone(),
            scene2d_signals: (__flight_source.scene2d_signals).clone(),
            scene_mesh_upload_cache: (__flight_source.scene_mesh_upload_cache).clone(),
            selection_begin_index: __flight_source.selection_begin_index,
            selection_end_index: __flight_source.selection_end_index,
            shader_loc: (__flight_source.shader_loc).clone(),
            shape_bounds_command_registry_revision: __flight_source
                .shape_bounds_command_registry_revision,
            shape_mesh_color_matrix_shader: (__flight_source.shape_mesh_color_matrix_shader)
                .clone(),
            shape_mesh_color_scale_bias_shader: (__flight_source
                .shape_mesh_color_scale_bias_shader)
                .clone(),
            shape_mesh_pipelines: (__flight_source.shape_mesh_pipelines).clone(),
            skin_bind_pose: (__flight_source.skin_bind_pose).clone(),
            surface_antialias_enabled: __flight_source.surface_antialias_enabled,
            surface_antialias_height: __flight_source.surface_antialias_height,
            surface_antialias_resolve_bind_group: (__flight_source
                .surface_antialias_resolve_bind_group)
                .clone(),
            surface_antialias_resolve_bind_group_layout: (__flight_source
                .surface_antialias_resolve_bind_group_layout)
                .clone(),
            surface_antialias_resolve_pipeline: (__flight_source
                .surface_antialias_resolve_pipeline)
                .clone(),
            surface_antialias_texture: (__flight_source.surface_antialias_texture).clone(),
            surface_antialias_view: (__flight_source.surface_antialias_view).clone(),
            surface_antialias_width: __flight_source.surface_antialias_width,
            surface_presentation_view: (__flight_source.surface_presentation_view).clone(),
            temp_stack: (__flight_source.temp_stack).clone(),
            text_field_signals: (__flight_source.text_field_signals).clone(),
            text_layout: (__flight_source.text_layout).clone(),
            text_layout_using_content_id: __flight_source.text_layout_using_content_id,
            texture_bind_group_layout: (__flight_source.texture_bind_group_layout).clone(),
            uniform_bind_group: (__flight_source.uniform_bind_group).clone(),
            uniform_bind_group_layout: (__flight_source.uniform_bind_group_layout).clone(),
            uniform_buffer: (__flight_source.uniform_buffer).clone(),
            uniform_color_scale_bias_shader: (__flight_source.uniform_color_scale_bias_shader)
                .clone(),
            uniform_data: (__flight_source.uniform_data).clone(),
            uniform_data_u32: (__flight_source.uniform_data_u32).clone(),
            uniform_offset: __flight_source.uniform_offset,
            uniform_stride: __flight_source.uniform_stride,
            video_element: (__flight_source.video_element).clone(),
            webgl_data: (__flight_source.webgl_data).clone(),
            webgl_shader_binding_resolver: (__flight_source.webgl_shader_binding_resolver).clone(),
            webgpu_data: (__flight_source.webgpu_data).clone(),
            webgpu_shader_binding_resolver: (__flight_source.webgpu_shader_binding_resolver)
                .clone(),
            wgpu_external_texture_cache: (__flight_source.wgpu_external_texture_cache).clone(),
            wgpu_render_texture_cache: (__flight_source.wgpu_render_texture_cache).clone(),
            wgpu_render_texture_guard: (__flight_source.wgpu_render_texture_guard).clone(),
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
            color_adjustments: (__flight_source.color_adjustments).clone(),
            color_adjustments_unsupported: __flight_source.color_adjustments_unsupported,
            resolved_color_matrix: (__flight_source.resolved_color_matrix).clone(),
            resolved_color_scale_bias: (__flight_source.resolved_color_scale_bias).clone(),
            can_add_child: (__flight_source.can_add_child).clone(),
            children: (__flight_source.children).clone(),
            traits: (__flight_source.traits).clone(),
            parent: (__flight_source.parent).clone(),
        }
    };
    if ((runtime.inner.lock().unwrap().local_bounds_rectangle).clone()).is_none() {
        {
            let __flight_runtime = runtime;
            let __flight_value = Some(create_rectangle(None, None, None, None));
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.local_bounds_rectangle = __flight_value;
        };
    }
    copy_rectangle(
        runtime
            .inner
            .lock()
            .unwrap()
            .local_bounds_rectangle
            .as_mut()
            .unwrap(),
        &{
            let __flight_source = &(rect);
            RectangleLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                height: __flight_source.height,
                width: __flight_source.width,
                x: __flight_source.x,
                y: __flight_source.y,
            }
        },
    );
    invalidate_node_local_bounds(target);
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:325 (sha256:aa2419be4505ba285d7226bc91fe2e8e6892d08e11d942689780b92eef47460b)
pub fn set_particle_emitter2_d_particle(
    target: &mut ParticleEmitter2D,
    index: f64,
    id: f64,
    x: f64,
    y: f64,
    rotation: f64,
    scale: f64,
) -> () {
    if (index < 0.0_f64) || (index >= target.data.particle_count) {
        return;
    }
    target.data.ids[index as usize] = (id) as u16;
    let tt = (index * PARTICLE_TRANSFORM_STRIDE);
    target.data.transforms[tt as usize] = (x) as f32;
    target.data.transforms[(tt + 1.0_f64) as usize] = (y) as f32;
    target.data.transforms[(tt + 2.0_f64) as usize] = (rotation) as f32;
    target.data.transforms[(tt + 3.0_f64) as usize] = (scale) as f32;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:347 (sha256:9ce7aac2c88826ecba4bd39283fb84a13ddf783ff7abcdbbeb98f1f939d86f78)
pub fn set_particle_emitter2_d_particle_alpha(
    target: &mut ParticleEmitter2D,
    index: f64,
    alpha: f64,
) -> () {
    if (index < 0.0_f64) || (index >= target.data.particle_count) {
        return;
    }
    target.data.alphas[index as usize] = (alpha) as f32;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:355 (sha256:57d82d20dcf25308521f7634213df636f6f47ac8e6137d67ceb5069704bb5410)
pub fn set_particle_emitter2_d_particle_color(
    target: &mut ParticleEmitter2D,
    index: f64,
    r: f64,
    g: f64,
    b: f64,
) -> () {
    if (index < 0.0_f64) || (index >= target.data.particle_count) {
        return;
    }
    let ct = (index * PARTICLE_COLOR_STRIDE);
    target.data.colors[ct as usize] = (r) as f32;
    target.data.colors[(ct + 1.0_f64) as usize] = (g) as f32;
    target.data.colors[(ct + 2.0_f64) as usize] = (b) as f32;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:372 (sha256:af8248de7fdb2db068c54032f4963612cf824befc3f337ee57074b6e7d515e5c)
pub fn set_particle_emitter2_d_particle_velocity(
    target: &mut ParticleEmitter2D,
    index: f64,
    vx: f64,
    vy: f64,
) -> () {
    if (index < 0.0_f64) || (index >= target.data.particle_count) {
        return;
    }
    let vt = (index * PARTICLE_VELOCITY_STRIDE);
    target.data.velocities[vt as usize] = (vx) as f32;
    target.data.velocities[(vt + 1.0_f64) as usize] = (vy) as f32;
}

// Source: upstream/packages/particleemitter/src/particleEmitter.ts:384 (sha256:a2423ac13a6d0212b1d0d6e8ddbba260e684f3837a5f46ae83a890106293a6c3)
static DEFAULT_METHODS: std::sync::LazyLock<FlightPartialRecord2> =
    std::sync::LazyLock::new(|| FlightPartialRecord2 {
        __flight_identity: std::sync::Arc::new(()),
        compute_local_bounds_rectangle: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut __flight_argument_0: Rectangle, __flight_argument_1: BoundsNodeAny| -> () {
                copy_local_bounds_rectangle(&mut __flight_argument_0, &{
                    let __flight_source = &(__flight_argument_1);
                    Node {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        __flight_entity_runtime: std::sync::Arc::clone(
                            &__flight_source.__flight_entity_runtime,
                        ),
                        data: (__flight_source.data).clone(),
                        enabled: __flight_source.enabled,
                        kind: (__flight_source.kind).clone(),
                        name: (__flight_source.name).clone(),
                    }
                })
            },
        )
            as Box<dyn FnMut(Rectangle, BoundsNodeAny) -> () + Send + 'static>))),
        anisotropy_ext: None,
        appearance_id: None,
        apply_blend_mode_parent: None,
        binding_cache_guard: None,
        bounds_rectangle: None,
        bounds_using_local_bounds_id: None,
        bounds_using_local_transform_id: None,
        bounds_version: None,
        build_text_layout_params: None,
        canvas_blend_effect_backdrops: None,
        canvas_texture_resolvers: None,
        canvas_texture_view: None,
        canvas_view_cleared: None,
        children_id: None,
        clip_contour_pipelines: None,
        clip_contour_stack: None,
        clip_forms: None,
        color_matrix_instanced_shader: None,
        color_scale_bias_instanced_shader: None,
        color_tint_instanced_shader: None,
        command_encoder: None,
        current_blend_mode: None,
        current_color_format: None,
        current_framebuffer: None,
        current_frame_id: None,
        current_mask_depth: None,
        current_program: None,
        current_texture: None,
        current_texture_straight_alpha: None,
        depth_stencil_height: None,
        depth_stencil_texture: None,
        depth_stencil_view: None,
        depth_stencil_width: None,
        dom_clip_hooks: None,
        dom_clip_stack: None,
        dom_current_element: None,
        dom_element_map: None,
        dom_next_order_list: None,
        dom_order_length: None,
        dom_order_list: None,
        element: None,
        flush_pending_draws: None,
        frame_capture_buffer: None,
        frame_capture_bytes_per_row: None,
        frame_capture_enabled: None,
        frame_capture_height: None,
        frame_capture_texture: None,
        frame_capture_width: None,
        gl_external_texture_cache: None,
        gl_render_texture_cache: None,
        gl_render_texture_guard: None,
        image_smoothing_enabled: None,
        image_smoothing_quality: None,
        input: None,
        instance_velocities: None,
        interaction_signals: None,
        interaction_state: None,
        is_local_bounds_rectangle_valid: None,
        linear_sampler: None,
        local_bounds_id: None,
        local_bounds_rectangle: None,
        local_bounds_texture: None,
        local_bounds_texture_version: None,
        local_bounds_using_local_bounds_id: None,
        local_content_id: None,
        local_matrix: None,
        local_matrix4: None,
        local_matrix4_detached: None,
        local_transform_id: None,
        local_transform_using_local_transform_id: None,
        mask_write_mode: None,
        material_bitmap_shader_map: None,
        matrix_array: None,
        max_anisotropy: None,
        measured_height: None,
        measured_width: None,
        media_stream: None,
        mipmap_degraded_guard: None,
        mipmap_generator: None,
        mipmapped_textures: None,
        morph_bind_pose: None,
        morph_blended_weights: None,
        movie_clip_signals: None,
        nearest_sampler: None,
        node_signals: None,
        pages: None,
        parent_reference_id: None,
        particle_corner_buffer: None,
        particle_instance_capacity: None,
        particle_shader: None,
        pipeline_cache: None,
        quad_batch_corner_buffer: None,
        quad_batch_shader: None,
        quad_batch_writer_blend_mode: None,
        quad_batch_writer_buffer_cursor: None,
        quad_batch_writer_buffer_pool: None,
        quad_batch_writer_color_matrix_data: None,
        quad_batch_writer_color_scale_bias_buffer: None,
        quad_batch_writer_color_scale_bias_data: None,
        quad_batch_writer_color_scale_bias_mode: None,
        quad_batch_writer_color_tint_data: None,
        quad_batch_writer_count: None,
        quad_batch_writer_instance_buffer: None,
        quad_batch_writer_instance_data: None,
        quad_batch_writer_material: None,
        quad_batch_writer_material_buffer: None,
        quad_batch_writer_material_data: None,
        quad_batch_writer_material_floats: None,
        quad_batch_writer_sampler: None,
        quad_batch_writer_smoothing: None,
        quad_batch_writer_straight_alpha: None,
        quad_batch_writer_uniform_color_scale_bias: None,
        quad_index_buffer: None,
        quad_vertex_buffer: None,
        quad_vertex_data: None,
        render_adapt_hook: None,
        renderer_map_id: None,
        render_pass: None,
        render_proxy_adapter_map: None,
        render_proxy_map: None,
        render_proxy_sources: None,
        render_target_stack: None,
        retired_buffers: None,
        retired_textures: None,
        rich_text_content: None,
        rotation_angle: None,
        rotation_cosine: None,
        rotation_sine: None,
        sampler_cache: None,
        scene2d: None,
        scene2d_signals: None,
        scene_mesh_upload_cache: None,
        selection_begin_index: None,
        selection_end_index: None,
        shader_loc: None,
        shape_bounds_command_registry_revision: None,
        shape_mesh_color_matrix_shader: None,
        shape_mesh_color_scale_bias_shader: None,
        shape_mesh_pipelines: None,
        skin_bind_pose: None,
        surface_antialias_enabled: None,
        surface_antialias_height: None,
        surface_antialias_resolve_bind_group: None,
        surface_antialias_resolve_bind_group_layout: None,
        surface_antialias_resolve_pipeline: None,
        surface_antialias_texture: None,
        surface_antialias_view: None,
        surface_antialias_width: None,
        surface_presentation_view: None,
        temp_stack: None,
        text_field_signals: None,
        text_layout: None,
        text_layout_using_content_id: None,
        texture_bind_group_layout: None,
        uniform_bind_group: None,
        uniform_bind_group_layout: None,
        uniform_buffer: None,
        uniform_color_scale_bias_shader: None,
        uniform_data: None,
        uniform_data_u32: None,
        uniform_offset: None,
        uniform_stride: None,
        video_element: None,
        webgl_data: None,
        webgl_shader_binding_resolver: None,
        webgpu_data: None,
        webgpu_shader_binding_resolver: None,
        wgpu_external_texture_cache: None,
        wgpu_render_texture_cache: None,
        wgpu_render_texture_guard: None,
        world_alpha: None,
        world_alpha_using_appearance_id: None,
        world_alpha_using_parent_appearance_id: None,
        world_appearance_id: None,
        world_bounds_rectangle: None,
        world_bounds_using_local_bounds_id: None,
        world_bounds_using_world_transform_id: None,
        world_matrix: None,
        world_matrix4: None,
        world_transform_id: None,
        world_transform_using_local_transform_id: None,
        world_transform_using_parent_transform_id: None,
        color_adjustments: None,
        color_adjustments_unsupported: None,
        resolved_color_matrix: None,
        resolved_color_scale_bias: None,
        can_add_child: None,
        children: None,
        traits: None,
        parent: None,
    });
