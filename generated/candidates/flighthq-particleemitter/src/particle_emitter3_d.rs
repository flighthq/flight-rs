// @generated from upstream/packages/particleemitter/src/particleEmitter3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_particle_emitter_data;
use flighthq_geometry::{reserve_float32_array, reserve_uint16_array};
use flighthq_scene3d::{create_node3_d, get_node3_d_runtime};
use flighthq_types::{
    AabbLike, Adjustment, BlendMode, BoundsNodeAny, ClipRegion, ColorScaleBias, InteractionSignals,
    Kind, Material, MaterialData, Matrix, Matrix4, Matrix4Like, Node, NodeData,
    NodeInteractionState, NodeSignals, NodeTraitsKey,
    PARTICLE_EMITTER3_D_KIND as particle_emitter3_d_kind_constant, ParticleEmitter3D,
    ParticleEmitter3DRuntime, ParticleEmitterData, Quaternion, Rectangle, SamplerLike, Scene2D,
    Scene2DSignals, Texture, TextureAtlas, Vector3, Vector3Like,
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
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
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
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
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

#[derive(Clone, Default)]
pub struct FlightPartialRecord13 {
    pub __flight_identity: std::sync::Arc<()>,
    pub position: Option<Vector3>,
    pub rotation: Option<Quaternion>,
    pub scale: Option<Vector3>,
}
impl PartialEq for FlightPartialRecord13 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:17 (sha256:c3dc807b578ac94141dd73c6a0532f43a0d50d26a1aa7884792f64f943d23ca6)
const PARTICLE_TRANSFORM_STRIDE: f64 = 4.0_f64;

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:18 (sha256:a68bb9efd032a89d93f67112fefca6fd2cfff5d832357b4e7a07ee6a442870a2)
const PARTICLE_COLOR_STRIDE: f64 = 3.0_f64;

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:21 (sha256:585a7221ae5fdce81f34d23b2eddffa6eddb0882b479fae7fe335b6f12057b5d)
const PARTICLE_VELOCITY_STRIDE: f64 = 3.0_f64;

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:23 (sha256:4cf7c9a26c52e92bf2e2a17e5e34c043832d23d843a350de94e0ecbfe30f0cb0)
pub const PARTICLE_EMITTER_3_D_DELETED_ID: f64 = 65535.0_f64;

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:25 (sha256:382baec1573441aa2594ebb9b91f9da090d9b94c67decea9be81b22acbf89f9b)
pub fn append_particle_emitter3_d_particle(
    target: &mut ParticleEmitter3D,
    id: f64,
    x: f64,
    y: f64,
    z: f64,
    rotation: f64,
    scale: f64,
) -> f64 {
    let index = target.data.particle_count;
    let needed = (index + 1.0_f64);
    if (get_particle_emitter3_d_capacity(target) < needed) {
        let new_capacity = (needed).max(if (target.data.particle_count * 2.0_f64) != 0.0_f64 {
            (target.data.particle_count * 2.0_f64)
        } else {
            8.0_f64
        });
        reserve_particle_emitter3_d(target, new_capacity);
    }
    target.data.particle_count = needed;
    target.data.ids[index as usize] = (id) as u16;
    let tt = (index * PARTICLE_TRANSFORM_STRIDE);
    target.data.transforms[tt as usize] = (x) as f32;
    target.data.transforms[(tt + 1.0_f64) as usize] = (y) as f32;
    target.data.transforms[(tt + 2.0_f64) as usize] = (rotation) as f32;
    target.data.transforms[(tt + 3.0_f64) as usize] = (scale) as f32;
    target.data.positions_z[index as usize] = (z) as f32;
    target.data.alphas[index as usize] = (1.0_f64) as f32;
    let ct = (index * PARTICLE_COLOR_STRIDE);
    target.data.colors[ct as usize] = (1.0_f64) as f32;
    target.data.colors[(ct + 1.0_f64) as usize] = (1.0_f64) as f32;
    target.data.colors[(ct + 2.0_f64) as usize] = (1.0_f64) as f32;
    let vt = (index * PARTICLE_VELOCITY_STRIDE);
    target.data.velocities[vt as usize] = (0.0_f64) as f32;
    target.data.velocities[(vt + 1.0_f64) as usize] = (0.0_f64) as f32;
    target.data.velocities[(vt + 2.0_f64) as usize] = (0.0_f64) as f32;
    return index;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:60 (sha256:83f11c03e3602fc761037e0b5d4f772eeba8fdbe20577ef302752e16557cf660)
pub fn clear_particle_emitter3_d(target: &mut ParticleEmitter3D) -> () {
    target.data.particle_count = 0.0_f64;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:64 (sha256:628cd7614df48156f182128798e9fd5bf25988b8cfb9ddcd4883c0bf30a7fc1d)
pub fn clone_particle_emitter3_d(source: &ParticleEmitter3D) -> ParticleEmitter3D {
    return create_particle_emitter3_d(Some(ParticleEmitter3D {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        blend_mode: (source.blend_mode).clone(),
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

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:82 (sha256:dbd0cebd3fb2aba2a3ff0343cd713cebe18cc5b035bc1c3babb9576352bf0d71)
pub fn compact_particle_emitter3_d(target: &mut ParticleEmitter3D) -> () {
    if (target.data.particle_count == 0.0_f64) {
        return;
    }
    let mut write = 0.0_f64;
    {
        let mut read = 0.0_f64;
        while (read < target.data.particle_count) {
            if ((target.data.ids[read as usize] as f64) == PARTICLE_EMITTER_3_D_DELETED_ID) {
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
                target.data.velocities[(vt + 2.0_f64) as usize] =
                    (target.data.velocities[(vts + 2.0_f64) as usize] as f64) as f32;
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

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:114 (sha256:1983bcc1bef2efb6120b8c96a2f74fe5ad849552db249755b28612eeab5c3d61)
pub fn compute_particle_emitter3_d_local_bounds_aabb(
    out: &mut AabbLike,
    source: &ParticleEmitter3D,
) -> () {
    let particle_count = source.data.particle_count;
    if (particle_count == 0.0_f64) {
        out.min.x = 0.0_f64;
        out.min.y = 0.0_f64;
        out.min.z = 0.0_f64;
        out.max.x = 0.0_f64;
        out.max.y = 0.0_f64;
        out.max.z = 0.0_f64;
        return;
    }
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut min_z = f64::INFINITY;
    let mut max_x = (-f64::INFINITY);
    let mut max_y = (-f64::INFINITY);
    let mut max_z = (-f64::INFINITY);
    {
        let mut i = 0.0_f64;
        while (i < particle_count) {
            let tt = (i * PARTICLE_TRANSFORM_STRIDE);
            let px = (source.data.transforms[tt as usize] as f64);
            let py = (source.data.transforms[(tt + 1.0_f64) as usize] as f64);
            let pz = (source.data.positions_z[i as usize] as f64);
            let scale = (source.data.transforms[(tt + 3.0_f64) as usize] as f64);
            let r = (math.sqrt1_2 * if (scale < 0.0_f64) { (-scale) } else { scale });
            if ((px - r) < min_x) {
                min_x = (px - r);
            }
            if ((py - r) < min_y) {
                min_y = (py - r);
            }
            if ((pz - r) < min_z) {
                min_z = (pz - r);
            }
            if ((px + r) > max_x) {
                max_x = (px + r);
            }
            if ((py + r) > max_y) {
                max_y = (py + r);
            }
            if ((pz + r) > max_z) {
                max_z = (pz + r);
            }
            {
                i += 1.0;
                i
            };
        }
    }
    out.min.x = min_x;
    out.min.y = min_y;
    out.min.z = min_z;
    out.max.x = max_x;
    out.max.y = max_y;
    out.max.z = max_z;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:155 (sha256:d0475c93e75562b9e0645bee468ddee9ea59ca437cbcd970586ececfe2411f3b)
pub fn create_particle_emitter3_d(obj: Option<ParticleEmitter3D>) -> ParticleEmitter3D {
    let mut node = create_node3_d(
        Some((particle_emitter3_d_kind_constant).to_owned()),
        Some((((obj).clone()).clone().unwrap()).clone()),
    );
    node.data = create_particle_emitter_data(Some(
        (obj.as_ref().map(|value| (value.data).clone())).clone(),
    ));
    node.blend_mode = (obj.as_ref().map(|value| (value.blend_mode).clone()))
        .clone()
        .unwrap_or("normal".to_owned());
    return node;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:162 (sha256:793699623f6e6810c655e530eda5bdb9dcaf4359ac7477126b9fb762f367e789)
pub fn get_particle_emitter3_d_capacity(source: &ParticleEmitter3D) -> f64 {
    let color_capacity =
        (__flight_js_to_i32(((source.data.colors.len() as f64) / PARTICLE_COLOR_STRIDE))
            | __flight_js_to_i32(0.0_f64)) as f64;
    let transform_capacity =
        (__flight_js_to_i32(((source.data.transforms.len() as f64) / PARTICLE_TRANSFORM_STRIDE))
            | __flight_js_to_i32(0.0_f64)) as f64;
    let velocity_capacity =
        (__flight_js_to_i32(((source.data.velocities.len() as f64) / PARTICLE_VELOCITY_STRIDE))
            | __flight_js_to_i32(0.0_f64)) as f64;
    return (((((source.data.ids.len() as f64).min((source.data.alphas.len() as f64)))
        .min((source.data.positions_z.len() as f64)))
    .min(color_capacity))
    .min(transform_capacity))
    .min(velocity_capacity);
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:177 (sha256:34696172d9b8786516c98aa37ec593ad466020a6791015d1b1a764af168d7473)
pub fn get_particle_emitter3_d_particle_alpha(source: &ParticleEmitter3D, index: f64) -> f64 {
    if (index < 0.0_f64) || (index >= source.data.particle_count) {
        return (-1.0_f64);
    }
    return (source.data.alphas[index as usize] as f64);
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:182 (sha256:6da9448545e5e496112104e2f1adfd5aaf4f4d7a7e9b150a673fe50b6fd5f416)
pub fn get_particle_emitter3_d_particle_id(source: &ParticleEmitter3D, index: f64) -> f64 {
    if (index < 0.0_f64) || (index >= source.data.particle_count) {
        return (-1.0_f64);
    }
    return (source.data.ids[index as usize] as f64);
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:187 (sha256:1d55e8ba919ffb0343bd02111a9578682929d91d162d1a72fbd11bca458f6e3b)
pub fn get_particle_emitter3_d_particle_velocity(
    out: &mut Vector3Like,
    source: &ParticleEmitter3D,
    index: f64,
) -> bool {
    if (index < 0.0_f64) || (index >= source.data.particle_count) {
        return false;
    }
    let vt = (index * PARTICLE_VELOCITY_STRIDE);
    out.x = (source.data.velocities[vt as usize] as f64);
    out.y = (source.data.velocities[(vt + 1.0_f64) as usize] as f64);
    out.z = (source.data.velocities[(vt + 2.0_f64) as usize] as f64);
    return true;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:200 (sha256:b3dab07bb00851846b50b57ea44dfef5ebdc0f04a1a422abb7bb4c54e64ea55b)
pub fn get_particle_emitter3_d_runtime(source: &ParticleEmitter3D) -> ParticleEmitter3DRuntime {
    return {
        let __flight_source = &(get_node3_d_runtime(&source));
        ParticleEmitter3DRuntime {
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

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:204 (sha256:53ccdc048bf6625edc739e17e698d25ee6b76aeb8f9ba9256e6a43e004d85399)
pub fn is_particle_emitter3_d(node: &SharedStructuralRecord1) -> bool {
    return ((node.kind).clone() == particle_emitter3_d_kind_constant);
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:208 (sha256:3094d60232aefa4d76ff37c4195068cea23d41e3f31cab39040e980ebbbaaf5f)
pub fn remove_particle_emitter3_d_particle(target: &mut ParticleEmitter3D, index: f64) -> () {
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
        target.data.velocities[(vt + 2.0_f64) as usize] =
            (target.data.velocities[(vts + 2.0_f64) as usize] as f64) as f32;
        target.data.positions_z[index as usize] =
            (target.data.positions_z[last as usize] as f64) as f32;
    }
    target.data.particle_count = last;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:236 (sha256:fb7fee2176d02be2c677f1884edc603d4dcb7bc83c40c5b9221b5418d4f30506)
pub fn reserve_particle_emitter3_d(target: &mut ParticleEmitter3D, capacity: f64) -> () {
    if (get_particle_emitter3_d_capacity(target) >= capacity) {
        return;
    }
    target.data.alphas = reserve_float32_array(&target.data.alphas, capacity);
    target.data.colors =
        reserve_float32_array(&target.data.colors, (capacity * PARTICLE_COLOR_STRIDE));
    target.data.ids = reserve_uint16_array(&target.data.ids, capacity);
    target.data.positions_z = reserve_float32_array(&target.data.positions_z, capacity);
    target.data.transforms = reserve_float32_array(
        &target.data.transforms,
        (capacity * PARTICLE_TRANSFORM_STRIDE),
    );
    target.data.velocities = reserve_float32_array(
        &target.data.velocities,
        (capacity * PARTICLE_VELOCITY_STRIDE),
    );
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:247 (sha256:a3cf1d15cc63dfc25e76dfe9fb9caf5392eba4b0a2add18f6e4a26c727d40c0d)
pub fn set_particle_emitter3_d_particle(
    target: &mut ParticleEmitter3D,
    index: f64,
    id: f64,
    x: f64,
    y: f64,
    z: f64,
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
    target.data.positions_z[index as usize] = (z) as f32;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:268 (sha256:bb24af2612562f03b8bf5c95c95897383aa7f425270b71479eb20793a9dee670)
pub fn set_particle_emitter3_d_particle_alpha(
    target: &mut ParticleEmitter3D,
    index: f64,
    alpha: f64,
) -> () {
    if (index < 0.0_f64) || (index >= target.data.particle_count) {
        return;
    }
    target.data.alphas[index as usize] = (alpha) as f32;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:273 (sha256:a25ff808d8a466b32bd7d9391a881c4f79980472a0782e31f6046a73a03eada9)
pub fn set_particle_emitter3_d_particle_color(
    target: &mut ParticleEmitter3D,
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

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:287 (sha256:0d35ba999cfaff1d04815ceaa1a7f00561cd41c570a8e47473061d16aeeccd2a)
pub fn set_particle_emitter3_d_particle_velocity(
    target: &mut ParticleEmitter3D,
    index: f64,
    vx: f64,
    vy: f64,
    vz: f64,
) -> () {
    if (index < 0.0_f64) || (index >= target.data.particle_count) {
        return;
    }
    let vt = (index * PARTICLE_VELOCITY_STRIDE);
    target.data.velocities[vt as usize] = (vx) as f32;
    target.data.velocities[(vt + 1.0_f64) as usize] = (vy) as f32;
    target.data.velocities[(vt + 2.0_f64) as usize] = (vz) as f32;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:306 (sha256:1128a65a985602d2c1ee95878812fcd14b7905d3ba3f825dead6bb0a0257dc72)
pub fn sort_particle_emitter3_d_indices_by_view_depth(
    out_indices: &mut Vec<u32>,
    out_view_depths: &mut Vec<f64>,
    source: &ParticleEmitter3D,
    position_to_view: &Matrix4Like,
) -> bool {
    let count = source.data.particle_count;
    if ((((!(count).is_finite() && (count).fract() == 0.0_f64) || (count < 0.0_f64))
        || (get_particle_emitter3_d_capacity(source) < count))
        || ((out_indices.len() as f64) < count))
        || ((out_view_depths.len() as f64) < count)
    {
        return false;
    }
    {
        let mut index = 0.0_f64;
        while (index < count) {
            let transform_offset = (index * PARTICLE_TRANSFORM_STRIDE);
            let x = (source.data.transforms[transform_offset as usize] as f64);
            let y = (source.data.transforms[(transform_offset + 1.0_f64) as usize] as f64);
            let z = (source.data.positions_z[index as usize] as f64);
            out_indices[index as usize] = (index) as u32;
            out_view_depths[index as usize] = (((((position_to_view.m[2.0_f64 as usize] as f64)
                * x)
                + ((position_to_view.m[6.0_f64 as usize] as f64) * y))
                + ((position_to_view.m[10.0_f64 as usize] as f64) * z))
                + (position_to_view.m[14.0_f64 as usize] as f64));
            {
                index += 1.0;
                index
            };
        }
    }
    {
        let mut start =
            ((__flight_js_to_i32(count) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64 - 1.0_f64);
        while (start >= 0.0_f64) {
            sift_particle_depth_max_heap(out_indices, out_view_depths, start, count);
            {
                start -= 1.0;
                start
            };
        }
    }
    {
        let mut end = (count - 1.0_f64);
        while (end > 0.0_f64) {
            let first = (out_indices[0.0_f64 as usize] as f64);
            out_indices[0.0_f64 as usize] = (out_indices[end as usize] as f64) as u32;
            out_indices[end as usize] = ((first).clone()) as u32;
            sift_particle_depth_max_heap(out_indices, out_view_depths, 0.0_f64, end);
            {
                end -= 1.0;
                end
            };
        }
    }
    return true;
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:349 (sha256:dab7b55568f51fdf1416d19364385f3c826176e9c2a578e6db98decdc987fbe2)
fn is_particle_depth_greater(a: f64, b: f64, depths: &Vec<f64>) -> bool {
    let a_depth = (depths[a as usize] as f64);
    let b_depth = (depths[b as usize] as f64);
    return (a_depth > b_depth) || ((a_depth == b_depth) && (a > b));
}

// Source: upstream/packages/particleemitter/src/particleEmitter3D.ts:355 (sha256:818d916c3db316b80e6a021e30002c33fd6ced9c87743aca2559067296a04e11)
fn sift_particle_depth_max_heap(
    indices: &mut Vec<u32>,
    depths: &Vec<f64>,
    mut root: f64,
    length: f64,
) -> () {
    while true {
        let mut left = ((root * 2.0_f64) + 1.0_f64);
        if (left >= length) {
            return;
        }
        let right = (left + 1.0_f64);
        let mut greater = left;
        if (right < length)
            && (is_particle_depth_greater(
                (indices[right as usize] as f64),
                (indices[left as usize] as f64),
                depths,
            ))
        {
            greater = right;
        }
        if (!is_particle_depth_greater(
            (indices[greater as usize] as f64),
            (indices[root as usize] as f64),
            depths,
        )) {
            return;
        }
        let swap = (indices[root as usize] as f64);
        indices[root as usize] = (indices[greater as usize] as f64) as u32;
        indices[greater as usize] = ((swap).clone()) as u32;
        root = greater;
    }
}
