// @generated from upstream/packages/bitmaptext/src/updateBitmapText.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::{create_rectangle, reserve_float32_array, reserve_uint16_array};
use flighthq_node::invalidate_node_local_bounds;
use flighthq_scene2d::get_node2_d_runtime;
use flighthq_texture::{create_texture, set_texture_source};
use flighthq_textureatlas::{add_texture_atlas_region, create_texture_atlas};
use flighthq_types::{
    Adjustment, BitmapText, BitmapTextData, BitmapTextPage, BitmapTextRuntime, BlendMode,
    BoundsNodeAny, ClipRegion, ColorScaleBias, GlyphEntry, GlyphSource, InteractionSignals, Kind,
    Material, MaterialData, Matrix, Matrix4, Node, NodeInteractionState, NodeSignals,
    NodeTraitsKey, Rectangle, SamplerLike, Scene2D, Scene2DSignals, Texture, Texture2D,
    TextureAtlasRegion, TextureFilter, TextureSource, TextureWrap,
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
    pub anisotropy: Option<f64>,
    pub mag_filter: Option<TextureFilter>,
    pub min_filter: Option<TextureFilter>,
    pub mipmaps: Option<bool>,
    pub wrap_u: Option<TextureWrap>,
    pub wrap_v: Option<TextureWrap>,
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
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord7 {
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for FlightPartialRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord8 {
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
impl PartialEq for FlightPartialRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord9 {
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for FlightPartialRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord10 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: Option<ClipRegion>,
}
impl PartialEq for FlightPartialRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord11 {
    pub __flight_identity: std::sync::Arc<()>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
}
impl PartialEq for FlightPartialRecord11 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord12 {
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
impl PartialEq for FlightPartialRecord12 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:16 (sha256:98a66b7283f76f6b66ced65175063259191b7b200ae9c10b40e5376aaca08dc0)
const BITMAP_TEXT_TRANSFORM_STRIDE: f64 = 2.0_f64;

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:17 (sha256:945bc25bcb4a4dbbb1555bb1828c3a1d08003f223c5b9e79cdc1cce606986637)
const CARRIAGE_RETURN: f64 = 13.0_f64;

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:18 (sha256:111c1172e5ab8d948ad84285fdac7880dc972715f0b0a9c7da683887c231e300)
const SPACE: f64 = 32.0_f64;

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:30 (sha256:bd2b06d994ae60462395ac35249381574ea63f8d43f23f9016a5118daab81e6a)
pub fn update_bitmap_text(bitmap_text: &BitmapText) -> () {
    let mut runtime = {
        let __flight_source = &(get_node2_d_runtime(bitmap_text));
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
    let mut bounds = ensure_bounds_rectangle((runtime).clone());
    for mut page in ((runtime.inner.lock().unwrap().pages).clone())
        .iter()
        .cloned()
    {
        page.instance_count = 0.0_f64;
        page.atlas.regions.clear();
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
    let lines = layout_bitmap_text_lines(&glyph_source.as_ref().unwrap(), &bitmap_text.data);
    let ref_width = (bitmap_text.data.wrap_width)
        .clone()
        .unwrap_or(max_line_width(&lines));
    let mut pages: Vec<(f64, BitmapTextPageContext)> = Vec::new();
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
                        let mut context = ensure_bitmap_text_page(
                            (runtime).clone(),
                            &glyph_source.as_ref().unwrap(),
                            &mut pages,
                            glyph.entry.page,
                        );
                        if (context).is_none() {
                            continue;
                        }
                        let quad_x = ((pen_x + glyph.pen_within_word) + glyph.entry.bearing_x);
                        let quad_y = (baseline_y - glyph.entry.bearing_y);
                        let mut region_id = context
                            .as_mut()
                            .unwrap()
                            .region_by_codepoint
                            .iter()
                            .find(|(entry_key, _)| entry_key == &glyph.codepoint)
                            .map(|(_, value)| value.clone());
                        if (region_id).is_none() {
                            add_texture_atlas_region(
                                &mut context.as_mut().unwrap().page.atlas,
                                glyph.entry.x,
                                glyph.entry.y,
                                glyph.entry.width,
                                glyph.entry.height,
                                None,
                                None,
                                None,
                            );
                            region_id = Some(
                                ((context.as_mut().unwrap().page.atlas.regions.len() as f64)
                                    - 1.0_f64),
                            );
                            {
                                let __flight_key = glyph.codepoint;
                                let __flight_value = (region_id).clone().unwrap();
                                if let Some((_, value)) = context
                                    .as_mut()
                                    .unwrap()
                                    .region_by_codepoint
                                    .iter_mut()
                                    .find(|(key, _)| key == &__flight_key)
                                {
                                    *value = __flight_value;
                                } else {
                                    context
                                        .as_mut()
                                        .unwrap()
                                        .region_by_codepoint
                                        .push((__flight_key, __flight_value));
                                }
                            };
                        }
                        append_bitmap_text_page_quad(
                            &mut context.as_mut().unwrap().page,
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

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:109 (sha256:15f88c93898029f3a56cee487e3f6b88d051520e28a4562a19dd8b555ab20548)
fn append_bitmap_text_page_quad(page: &mut BitmapTextPage, id: f64, x: f64, y: f64) -> () {
    let index = page.instance_count;
    let capacity = (page.ids.len() as f64).min(
        (__flight_js_to_i32(((page.transforms.len() as f64) / BITMAP_TEXT_TRANSFORM_STRIDE))
            | __flight_js_to_i32(0.0_f64)) as f64,
    );
    if (index >= capacity) {
        let next = ((index + 1.0_f64).max((capacity * 2.0_f64))).max(8.0_f64);
        page.ids = reserve_uint16_array(&page.ids, next);
        page.transforms =
            reserve_float32_array(&page.transforms, (next * BITMAP_TEXT_TRANSFORM_STRIDE));
    }
    page.ids[index as usize] = (id) as u16;
    let o = (index * BITMAP_TEXT_TRANSFORM_STRIDE);
    page.transforms[o as usize] = (x) as f32;
    page.transforms[(o + 1.0_f64) as usize] = (y) as f32;
    page.instance_count = (index + 1.0_f64);
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:127 (sha256:4f3667584f9d957b98c2e77b16859c5a22f92185621203b3ac759aa1ee2e5efb)
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
        if ((entry).clone()).is_none() {
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
                entry: ((entry).clone()).clone().unwrap(),
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

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:169 (sha256:d1b21906289569a7ceb39d700792ecaefefd445962ffe405a8146983f6e4dcde)
#[derive(Clone, Default)]
struct EnsureBitmapTextPageSynthesizedRecord1990218478 {
    __flight_identity: std::sync::Arc<()>,
    dimension: String,
    source: TextureSource,
}
impl PartialEq for EnsureBitmapTextPageSynthesizedRecord1990218478 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn ensure_bitmap_text_page(
    mut runtime: BitmapTextRuntime,
    glyph_source: &GlyphSource,
    pages: &mut Vec<(f64, BitmapTextPageContext)>,
    page: f64,
) -> Option<BitmapTextPageContext> {
    let cached = pages
        .iter()
        .find(|(entry_key, _)| entry_key == &page)
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
    while ((runtime.inner.lock().unwrap().pages.len() as f64) <= page) {
        runtime.inner.lock().unwrap().pages.push(BitmapTextPage {
            __flight_identity: std::sync::Arc::new(()),
            atlas: create_texture_atlas(None),
            ids: vec![0_u16; (0.0_f64) as usize],
            instance_count: 0.0_f64,
            transforms: vec![0.0_f32; (0.0_f64) as usize],
        });
    }
    let mut page_data = runtime.inner.lock().unwrap().pages[page as usize].clone();
    if ((page_data.atlas.texture).clone()).is_none() {
        page_data.atlas.texture = Some(create_texture(Some(
            EnsureBitmapTextPageSynthesizedRecord1990218478 {
                __flight_identity: std::sync::Arc::new(()),
                dimension: "2d".to_owned(),
                source: (image.as_ref().unwrap()).clone(),
            },
        )));
    } else {
        set_texture_source(
            {
                let __flight_portable_source = (page_data.atlas.texture).clone();
                match (&__flight_portable_source).as_ref() {
                    Some(value) => crate::FlightValue::Record({
                        let mut __flight_record = Vec::new();
                        __flight_record.push((
                            "flipX".to_owned(),
                            crate::FlightValue::Bool(*(&((value).flip_x))),
                        ));
                        __flight_record.push((
                            "flipY".to_owned(),
                            crate::FlightValue::Bool(*(&((value).flip_y))),
                        ));
                        __flight_record.push((
                            "uvOffset".to_owned(),
                            crate::FlightValue::Record({
                                let mut __flight_record = Vec::new();
                                __flight_record.push((
                                    "x".to_owned(),
                                    crate::FlightValue::Number(
                                        *(&((&((value).uv_offset)).x)) as f64,
                                    ),
                                ));
                                __flight_record.push((
                                    "y".to_owned(),
                                    crate::FlightValue::Number(
                                        *(&((&((value).uv_offset)).y)) as f64,
                                    ),
                                ));
                                __flight_record
                            }),
                        ));
                        __flight_record.push((
                            "uvRotation".to_owned(),
                            crate::FlightValue::Number(*(&((value).uv_rotation)) as f64),
                        ));
                        __flight_record.push((
                            "uvScale".to_owned(),
                            crate::FlightValue::Record({
                                let mut __flight_record = Vec::new();
                                __flight_record.push((
                                    "x".to_owned(),
                                    crate::FlightValue::Number(
                                        *(&((&((value).uv_scale)).x)) as f64,
                                    ),
                                ));
                                __flight_record.push((
                                    "y".to_owned(),
                                    crate::FlightValue::Number(
                                        *(&((&((value).uv_scale)).y)) as f64,
                                    ),
                                ));
                                __flight_record
                            }),
                        ));
                        __flight_record.push((
                            "colorSpace".to_owned(),
                            crate::FlightValue::String((&((value).color_space)).clone()),
                        ));
                        __flight_record.push((
                            "sampler".to_owned(),
                            crate::FlightValue::Record({
                                let mut __flight_record = Vec::new();
                                __flight_record.push((
                                    "anisotropy".to_owned(),
                                    crate::FlightValue::Number(
                                        *(&((&((value).sampler)).anisotropy)) as f64,
                                    ),
                                ));
                                __flight_record.push((
                                    "magFilter".to_owned(),
                                    crate::FlightValue::String(
                                        (&((&((value).sampler)).mag_filter)).clone(),
                                    ),
                                ));
                                __flight_record.push((
                                    "minFilter".to_owned(),
                                    crate::FlightValue::String(
                                        (&((&((value).sampler)).min_filter)).clone(),
                                    ),
                                ));
                                __flight_record.push((
                                    "mipmaps".to_owned(),
                                    crate::FlightValue::Bool(*(&((&((value).sampler)).mipmaps))),
                                ));
                                __flight_record.push((
                                    "wrapU".to_owned(),
                                    crate::FlightValue::String(
                                        (&((&((value).sampler)).wrap_u)).clone(),
                                    ),
                                ));
                                __flight_record.push((
                                    "wrapV".to_owned(),
                                    crate::FlightValue::String(
                                        (&((&((value).sampler)).wrap_v)).clone(),
                                    ),
                                ));
                                __flight_record
                            }),
                        ));
                        __flight_record.push((
                            "version".to_owned(),
                            crate::FlightValue::Number(*(&((value).version)) as f64),
                        ));
                        __flight_record.push((
                            "dimension".to_owned(),
                            crate::FlightValue::String((&((value).dimension)).clone()),
                        ));
                        __flight_record.push(("source".to_owned(), match (&((value).source)).as_ref() { Some(value) => crate::FlightValue::Record({ let mut __flight_record = Vec::new(); __flight_record.push(("alphaType".to_owned(), crate::FlightValue::String((&((value).alpha_type)).clone()))); __flight_record.push(("gamut".to_owned(), crate::FlightValue::String((&((value).gamut)).clone()))); __flight_record.push(("height".to_owned(), crate::FlightValue::Number(*(&((value).height)) as f64))); __flight_record.push(("kind".to_owned(), crate::FlightValue::String((&((value).kind)).clone()))); __flight_record.push(("version".to_owned(), crate::FlightValue::Number(*(&((value).version)) as f64))); __flight_record.push(("width".to_owned(), crate::FlightValue::Number(*(&((value).width)) as f64))); __flight_record.push(("format".to_owned(), crate::FlightValue::String((&((value).format)).clone()))); if let Some(value) = (value).color_attachments.as_ref() { __flight_record.push(("colorAttachments".to_owned(), crate::FlightValue::Number(*(value) as f64))); } if let Some(value) = (value).color_formats.as_ref() { __flight_record.push(("colorFormats".to_owned(), crate::FlightValue::Array((value).iter().map(|value| crate::FlightValue::String((value).clone())).collect()))); } if let Some(value) = (value).sample_count.as_ref() { __flight_record.push(("sampleCount".to_owned(), crate::FlightValue::Number(*(value) as f64))); } if let Some(value) = (value).color_space.as_ref() { __flight_record.push(("colorSpace".to_owned(), crate::FlightValue::String((value).clone()))); } if let Some(value) = (value).clear_colors.as_ref() { __flight_record.push(("clearColors".to_owned(), crate::FlightValue::Array((value).iter().map(|value| crate::FlightValue::Number(*(value) as f64)).collect()))); } if let Some(value) = (value).clear_depth.as_ref() { __flight_record.push(("clearDepth".to_owned(), crate::FlightValue::Number(*(value) as f64))); } __flight_record.push(("source".to_owned(), (&((value).source)).clone())); __flight_record.push(("compressed".to_owned(), crate::FlightValue::Record({ let mut __flight_record = Vec::new(); __flight_record.push(("container".to_owned(), crate::FlightValue::Record({ let mut __flight_record = Vec::new(); __flight_record.push(("format".to_owned(), crate::FlightValue::String((&((&((&((value).compressed)).container)).format)).clone()))); __flight_record.push(("width".to_owned(), crate::FlightValue::Number(*(&((&((&((value).compressed)).container)).width)) as f64))); __flight_record.push(("height".to_owned(), crate::FlightValue::Number(*(&((&((&((value).compressed)).container)).height)) as f64))); __flight_record.push(("depth".to_owned(), crate::FlightValue::Number(*(&((&((&((value).compressed)).container)).depth)) as f64))); __flight_record.push(("mipLevels".to_owned(), crate::FlightValue::Number(*(&((&((&((value).compressed)).container)).mip_levels)) as f64))); __flight_record.push(("layers".to_owned(), crate::FlightValue::Number(*(&((&((&((value).compressed)).container)).layers)) as f64))); __flight_record.push(("faces".to_owned(), crate::FlightValue::Number(*(&((&((&((value).compressed)).container)).faces)) as f64))); __flight_record.push(("supercompression".to_owned(), crate::FlightValue::String((&((&((&((value).compressed)).container)).supercompression)).clone()))); __flight_record.push(("levels".to_owned(), crate::FlightValue::Array((&((&((&((value).compressed)).container)).levels)).iter().map(|value| crate::FlightValue::Record({ let mut __flight_record = Vec::new(); __flight_record.push(("byteOffset".to_owned(), crate::FlightValue::Number(*(&((value).byte_offset)) as f64))); __flight_record.push(("byteLength".to_owned(), crate::FlightValue::Number(*(&((value).byte_length)) as f64))); __flight_record.push(("width".to_owned(), crate::FlightValue::Number(*(&((value).width)) as f64))); __flight_record.push(("height".to_owned(), crate::FlightValue::Number(*(&((value).height)) as f64))); __flight_record })).collect()))); __flight_record }))); __flight_record.push(("payload".to_owned(), crate::FlightValue::Array((&((&((value).compressed)).payload)).iter().map(|value| crate::FlightValue::Number((*value) as f64)).collect()))); __flight_record }))); __flight_record }), None => crate::FlightValue::Null }));
                        __flight_record
                    }),
                    None => crate::FlightValue::Null,
                }
            },
            &(Some((image.as_ref().unwrap()).clone())),
        );
    }
    let mut context: BitmapTextPageContext = BitmapTextPageContext {
        __flight_identity: std::sync::Arc::new(()),
        page: (page_data).clone(),
        region_by_codepoint: Vec::new(),
    };
    {
        let __flight_key = page;
        let __flight_value = (context).clone();
        if let Some((_, value)) = pages.iter_mut().find(|(key, _)| key == &__flight_key) {
            *value = __flight_value;
        } else {
            pages.push((__flight_key, __flight_value));
        }
    };
    return Some((context).clone());
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:200 (sha256:3a3b63bd630db2bd5b2173e069198940411e4fa8618ab60eed8210d1ef0855cb)
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

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:208 (sha256:37a28d4828e8076d45bbf780947328d6faebfad02cf933c4140e5b2eed58534d)
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
                    && ((data.wrap_width).as_ref().is_some_and(|value| {
                        ((current.width + token.gap) + token.word.width) > *value
                    }));
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

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:237 (sha256:d2a73c20d95a1a1512ff1f0278c6bd97e0108f4d220b7013c3ed2c4ffbff3066)
fn max_line_width(lines: &Vec<BitmapTextLine>) -> f64 {
    let mut max = 0.0_f64;
    for line in (lines).iter().cloned() {
        if (line.width > max) {
            max = line.width;
        }
    }
    return max;
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:243 (sha256:a394c69d44085bed7f8938b68dae56d8b1073fef074f7cca0b7872f173cda766)
fn set_empty_rectangle(out: &mut Rectangle) -> () {
    out.x = 0.0_f64;
    out.y = 0.0_f64;
    out.width = 0.0_f64;
    out.height = 0.0_f64;
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:252 (sha256:addde9091e717fe715e2337f95e4c64b535673b79412c3fccda76136eb05f90a)
#[derive(Clone, Default)]
pub(crate) struct BitmapTextGlyph {
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

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:260 (sha256:aa105752080eae23030ab6b7e7cb099c5772109d43c47f1c571e606979dbaa1e)
#[derive(Clone, Default)]
pub(crate) struct BitmapTextPageContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub page: BitmapTextPage,
    pub region_by_codepoint: Vec<(f64, f64)>,
}
impl PartialEq for BitmapTextPageContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:267 (sha256:e16d053098a238a17b8aff009c25ee0710c6f3819a3a825092cf06b4ed9783c0)
#[derive(Clone, Default)]
pub(crate) struct BitmapTextLine {
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

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:275 (sha256:2dafecf5ba6b88df3930aa9c085040985a8191db73398e385aacc144cc6698c4)
#[derive(Clone, Default)]
pub(crate) struct BitmapTextToken {
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

// Source: upstream/packages/bitmaptext/src/updateBitmapText.ts:281 (sha256:3401bd7270b995d95bd0883e9b0104675fac441bbb5d1b368e2ff25dc110b1b9)
#[derive(Clone, Default)]
pub(crate) struct BitmapTextWord {
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
