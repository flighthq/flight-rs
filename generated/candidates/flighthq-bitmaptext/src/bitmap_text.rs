// @generated from upstream/packages/bitmaptext/src/bitmapText.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::{
    copy_rectangle, create_rectangle, reserve_float32_array, reserve_uint16_array,
};
use flighthq_scene2d::{create_node2_d, create_node2_d_runtime, get_node2_d_runtime};
use flighthq_textureatlas::create_texture_atlas;
use flighthq_types::{
    Adjustment, BITMAP_TEXT_KIND as bitmap_text_kind_constant, BitmapText, BitmapTextAlign,
    BitmapTextData, BitmapTextOptions, BitmapTextPage, BitmapTextRuntime, BlendMode, BoundsNodeAny,
    ClipRegion, ColorScaleBias, GlyphSource, InteractionSignals, Kind, Material, MaterialData,
    Matrix, Matrix4, Node, Node2DData, NodeInteractionState, NodeSignals, NodeTraitsKey, Rectangle,
    RectangleLike, SamplerLike, Scene2D, Scene2DSignals, Texture, Texture2D, TextureAtlasRegion,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub align: Option<BitmapTextAlign>,
    pub glyph_source: Option<GlyphSource>,
    pub letter_spacing: Option<f64>,
    pub line_height: Option<f64>,
    pub text: Option<String>,
    pub wrap_width: Option<f64>,
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
    pub texture: Option<Texture2D>,
    pub regions: Option<Vec<TextureAtlasRegion>>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord7 {
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
impl PartialEq for FlightPartialRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord8 {
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for FlightPartialRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord9 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: Option<ClipRegion>,
}
impl PartialEq for FlightPartialRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord10 {
    pub __flight_identity: std::sync::Arc<()>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
}
impl PartialEq for FlightPartialRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord11 {
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
impl PartialEq for FlightPartialRecord11 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:20 (sha256:98a66b7283f76f6b66ced65175063259191b7b200ae9c10b40e5376aaca08dc0)
const BITMAP_TEXT_TRANSFORM_STRIDE: f64 = 2.0_f64;

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:24 (sha256:74345c09b6118d995cb94c502521f9f903abb86f6daf73a7906935cfe65324da)
pub fn compute_bitmap_text_local_bounds_rectangle(out: &mut Rectangle, source: &BitmapText) -> () {
    let runtime = {
        let __flight_source = &(get_node2_d_runtime(source));
        BitmapTextRuntime {
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
    let bounds = (runtime.inner.lock().unwrap().local_bounds_rectangle).clone();
    if (bounds).is_none() {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        out.width = 0.0_f64;
        out.height = 0.0_f64;
        return;
    }
    copy_rectangle(out, &{
        let __flight_source = &(bounds.as_ref().unwrap());
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
    });
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:44 (sha256:6f07b3ab25b39914170c6b46b5359489f5a586293575259a551ffd1ee9b4f37c)
pub fn create_bitmap_text(
    glyph_source: &Option<GlyphSource>,
    options: Option<BitmapTextOptions>,
) -> BitmapText {
    let mut bitmap_text = create_node2_d(
        (bitmap_text_kind_constant).to_owned(),
        Some(undefined),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<FlightPartialRecord8>| -> Node2DData {
                create_bitmap_text_data(Some(((__flight_argument_0).clone().unwrap()).clone()))
            },
        )
            as Box<
                dyn FnMut(Option<FlightPartialRecord8>) -> Node2DData + Send + 'static,
            >))),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<R>| -> R { create_bitmap_text_runtime() },
        )
            as Box<dyn FnMut(Option<R>) -> R + Send + 'static>))),
    );
    bitmap_text.data.glyph_source = (*glyph_source).clone();
    if (options).is_some() {
        apply_bitmap_text_options(&mut bitmap_text.data, &options.as_ref().unwrap());
    }
    let mut runtime = {
        let __flight_source = &(get_node2_d_runtime(&bitmap_text));
        BitmapTextRuntime {
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
    runtime
        .inner
        .lock()
        .unwrap()
        .pages
        .push(create_bitmap_text_page());
    return bitmap_text;
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:59 (sha256:01913cbf4c72f6782b6f38c10dadbd46eff16e3a791c99bcd4c696a6f75b371c)
pub fn create_bitmap_text_data(data: Option<FlightPartialRecord1>) -> BitmapTextData {
    return BitmapTextData {
        __flight_identity: std::sync::Arc::new(()),
        align: (data.as_ref().and_then(|value| (value.align).clone()))
            .clone()
            .unwrap_or("left".to_owned()),
        glyph_source: data.as_ref().and_then(|value| (value.glyph_source).clone()),
        letter_spacing: (data.as_ref().and_then(|value| value.letter_spacing))
            .clone()
            .unwrap_or(0.0_f64),
        line_height: (data.as_ref().and_then(|value| value.line_height))
            .clone()
            .unwrap_or(1.0_f64),
        text: (data.as_ref().and_then(|value| (value.text).clone()))
            .clone()
            .unwrap_or("".to_owned()),
        wrap_width: data.as_ref().and_then(|value| value.wrap_width),
    };
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:70 (sha256:0493418e9be43f909b3424628930cd73bbd0b87e0b5f4488453693440a21a796)
pub fn create_bitmap_text_runtime() -> BitmapTextRuntime {
    let mut runtime = {
        let __flight_source = &(create_node2_d_runtime(Some(((*DEFAULT_METHODS).clone()).clone())));
        BitmapTextRuntime {
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
    {
        let __flight_runtime = runtime;
        let __flight_value = vec![];
        let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
        __flight_storage.pages = __flight_value;
    };
    return runtime;
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:79 (sha256:beb2063df494c0cf5820839f49241ce13a95548f488d2e11f0cd9133bea028f2)
pub fn get_bitmap_text_bounds(source: &BitmapText) -> Rectangle {
    let mut out = create_rectangle(None, None, None, None);
    compute_bitmap_text_local_bounds_rectangle(&mut out, source);
    return out;
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:87 (sha256:0207075e815b656dfcfdaf8e0b711ab3e401f34380a4242456cf4cfcd3c0049b)
pub fn get_bitmap_text_pages(source: &BitmapText) -> Vec<BitmapTextPage> {
    return ({
        let __flight_source = &(get_node2_d_runtime(source));
        BitmapTextRuntime {
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
    }
    .inner
    .lock()
    .unwrap()
    .pages)
        .clone();
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:94 (sha256:75b6933233dd86f11f7940d0ecdfb2cbd3cd885458532122caf7093427ccfc45)
pub fn reserve_bitmap_text(target: &BitmapText, glyph_capacity: f64) -> () {
    let runtime = {
        let __flight_source = &(get_node2_d_runtime(target));
        BitmapTextRuntime {
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
    for mut page in ((runtime.inner.lock().unwrap().pages).clone())
        .iter()
        .cloned()
    {
        page.ids = reserve_uint16_array(&page.ids, glyph_capacity);
        page.transforms = reserve_float32_array(
            &page.transforms,
            (glyph_capacity * BITMAP_TEXT_TRANSFORM_STRIDE),
        );
    }
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:103 (sha256:86964ba65f3f4396e7ffef7642d63385d310454ca142fe20064743c04b44f7fc)
pub fn set_bitmap_text_align(target: &mut BitmapText, align: BitmapTextAlign) -> () {
    target.data.align = (align).clone();
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:107 (sha256:f3979369c293d3771fc2e96a8d8dfb6ba5251701b4afc9a02b5e1fe11752ae7e)
pub fn set_bitmap_text_glyph_source(
    target: &mut BitmapText,
    glyph_source: &Option<GlyphSource>,
) -> () {
    target.data.glyph_source = (*glyph_source).clone();
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:111 (sha256:2248db62f27e90cd43fff4588f3775db6d8cac400b6addd2ae87914f04ce47c7)
pub fn set_bitmap_text_letter_spacing(target: &mut BitmapText, letter_spacing: f64) -> () {
    target.data.letter_spacing = letter_spacing;
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:115 (sha256:eba236685e316a1bd57865a42ab1ce5479dafd9e2964cb1309691861e538bb9e)
pub fn set_bitmap_text_line_height(target: &mut BitmapText, line_height: f64) -> () {
    target.data.line_height = line_height;
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:119 (sha256:72271f9f1f7e6213b541cc3731589dabb4a1674ee6c7472c0a249f2230b25f1a)
pub fn set_bitmap_text_text(target: &mut BitmapText, text: String) -> () {
    target.data.text = (text).clone();
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:123 (sha256:909454d4aa245693104ad943c8572177af39c646c872ef9c17a8044509b922ea)
pub fn set_bitmap_text_wrap_width(target: &mut BitmapText, wrap_width: Option<f64>) -> () {
    target.data.wrap_width = wrap_width;
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:127 (sha256:432c34b0b4afaa00cd14997f44872fdb1df731599a7b8d11fcfd6099413ecd19)
fn apply_bitmap_text_options(data: &mut BitmapTextData, options: &BitmapTextOptions) -> () {
    if ((options.align).clone()).is_some() {
        data.align = ((options.align).clone()).unwrap();
    }
    if (options.letter_spacing).is_some() {
        data.letter_spacing = (options.letter_spacing).unwrap();
    }
    if (options.line_height).is_some() {
        data.line_height = (options.line_height).unwrap();
    }
    if ((options.text).clone()).is_some() {
        data.text = ((options.text).clone()).unwrap();
    }
    if (options.wrap_width).is_some() {
        data.wrap_width = options.wrap_width;
    }
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:135 (sha256:1ae29f9a6dd898a30f79df0e3b823e567acff9d21a407686a608ad0558066db2)
fn copy_local_bounds_rectangle(out: &mut Rectangle, source: &Node) -> () {
    let runtime = {
        let __flight_source = &(get_node2_d_runtime(&source));
        BitmapTextRuntime {
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

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:141 (sha256:4a58cea3bf3bf3fd72fee4662c084a9e6cea15bf7f1f26351a58153f6003d324)
fn create_bitmap_text_page() -> BitmapTextPage {
    return BitmapTextPage {
        __flight_identity: std::sync::Arc::new(()),
        atlas: create_texture_atlas(None),
        ids: vec![0_u16; (0.0_f64) as usize],
        instance_count: 0.0_f64,
        transforms: vec![0.0_f32; (0.0_f64) as usize],
    };
}

// Source: upstream/packages/bitmaptext/src/bitmapText.ts:145 (sha256:8a9b089bd51cc37c07cebbbfddf583d54c250e80e084a008dab75e82e397a0d0)
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
