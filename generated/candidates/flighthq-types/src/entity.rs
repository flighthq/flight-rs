// @generated from upstream/packages/types/src/Entity.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Adjustment, BlendMode, BoundsNodeAny, ColorTransform, DomClipEntry, DomClipHooks,
    GlBitmapShader, GlBlendRealization, GlColorAdjustmentFold, GlColorTransformInstancedShader,
    GlCompressedTextureDecoder, GlCompressedTextureUploader, GlParticleShader, GlQuadBatchShader,
    GlRenderState, GlRenderTarget, GlShaderLocations, GlShapeMeshColorTransformShader,
    GlUniformColorTransformShader, ImageResource, InteractionSignals, Material, Matrix, Matrix4,
    MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose, MeshSkinBindPose,
    MovieClipSignals, NodeInteractionState, NodeSignals, QuadBatch, Rectangle, RenderProxy,
    RenderProxy2D, RenderProxyAdapter, RenderState, Renderable, Renderer, RichTextContent, Stage,
    StageSignals, TextFieldSignals, TextInputState, TextLabel, TextLayoutParams, TextLayoutResult,
    TextMeasureFunction, WgpuBitmapShader, WgpuClipContourEntry, WgpuClipContourPipelines,
    WgpuColorAdjustmentFold, WgpuRenderState, WgpuSavedPassState, WgpuShapeMeshPipeline,
    WgpuSpriteBatchBufferSlot,
};

// Source: upstream/packages/types/src/Entity.ts:1 (sha256:cdb46c2fe96dc3464760172db6afea575505426c2d0a8207914ee42315a65204)
pub type Kind = String;

// Source: upstream/packages/types/src/Entity.ts:2 (sha256:f85f87cc89d4e93a438fb81e5b911bfb1ecfdec7d52c81fa9ae258bec22021a6)
#[derive(Clone, Default)]
pub struct Entity {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<EntityRuntime>>>,
}
impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
#[doc(hidden)]
pub trait FlightEntity {
    fn __flight_entity_runtime(&self) -> &std::sync::Arc<std::sync::Mutex<Option<EntityRuntime>>>;
    fn __flight_fresh_clone(&self) -> Self
    where
        Self: Sized;
}
impl FlightEntity for Entity {
    fn __flight_entity_runtime(&self) -> &std::sync::Arc<std::sync::Mutex<Option<EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/Entity.ts:5 (sha256:e8922dec976bcfcb17943d6646ad2d8a649cf0c12bc2a77930283dacd421e57a)
pub type EntityWithoutRuntime<Type> = Type;

// Source: upstream/packages/types/src/Entity.ts:6 (sha256:2442a2b2f11e739d0ec1d2f573d38c0fe55fef3a64723ef98ff3c0f7b4981bdb)
#[derive(Clone, Default)]
pub struct EntityRuntime {
    #[doc(hidden)]
    pub inner: std::sync::Arc<std::sync::Mutex<EntityRuntimeStorage>>,
}
#[doc(hidden)]
#[derive(Default)]
pub struct EntityRuntimeStorage {
    pub anisotropy_ext: Option<crate::OpaqueHostValue>,
    pub appearance_id: f64,
    pub binding: Option<crate::OpaqueHostValue>,
    pub bounds_rectangle: Option<Rectangle>,
    pub bounds_using_local_bounds_id: f64,
    pub bounds_using_local_transform_id: f64,
    pub build_text_layout_params: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(TextLabel, TextMeasureFunction) -> TextLayoutParams + Send + 'static>,
            >,
        >,
    >,
    pub canvas_texture_view: Option<crate::OpaqueHostValue>,
    pub canvas_view_cleared: bool,
    pub clip_contour_pipelines: Option<Vec<(crate::OpaqueHostValue, WgpuClipContourPipelines)>>,
    pub clip_contour_stack: Vec<WgpuClipContourEntry>,
    pub clip_forms: Vec<String>,
    pub color_adjustment_channel_mixing_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RenderState, Renderable) -> () + Send + 'static>>,
        >,
    >,
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub color_adjustments_channel_mixing: bool,
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
    pub current_frame_id: f64,
    pub current_mask_depth: f64,
    pub current_program: Option<crate::OpaqueHostValue>,
    pub current_render_target: Option<GlRenderTarget>,
    pub current_texture: Option<crate::OpaqueHostValue>,
    pub depth_stencil_height: f64,
    pub depth_stencil_texture: Option<crate::OpaqueHostValue>,
    pub depth_stencil_view: Option<crate::OpaqueHostValue>,
    pub depth_stencil_width: f64,
    pub dom_clip_hooks: Option<DomClipHooks>,
    pub dom_clip_stack: Vec<DomClipEntry>,
    pub dom_current_element: Option<crate::OpaqueHostValue>,
    pub dom_element_map: Vec<(RenderProxy2D, crate::OpaqueHostValue)>,
    pub dom_next_order_list: Vec<RenderProxy2D>,
    pub dom_order_length: f64,
    pub dom_order_list: Vec<RenderProxy2D>,
    pub element: Option<crate::OpaqueHostValue>,
    pub frame_capture_buffer: Option<crate::OpaqueHostValue>,
    pub frame_capture_bytes_per_row: f64,
    pub frame_capture_enabled: bool,
    pub frame_capture_height: f64,
    pub frame_capture_texture: Option<crate::OpaqueHostValue>,
    pub frame_capture_width: f64,
    pub gl_blend_mode_registry: Option<Vec<(BlendMode, GlBlendRealization)>>,
    pub gl_color_adjustment_fold: Option<GlColorAdjustmentFold>,
    pub gl_color_adjustment_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(GlRenderState, ColorTransform) -> () + Send + 'static>>,
        >,
    >,
    pub image_smoothing_enabled: bool,
    pub image_smoothing_quality: crate::OpaqueHostValue,
    pub input: Option<TextInputState>,
    pub instance_velocities: Option<Vec<f32>>,
    pub interaction_signals: Option<InteractionSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub linear_sampler: crate::OpaqueHostValue,
    pub local_bounds_id: f64,
    pub local_bounds_rectangle: Option<Rectangle>,
    pub local_bounds_using_local_bounds_id: f64,
    pub local_content_id: f64,
    pub local_matrix: Option<Matrix>,
    pub local_matrix4: Option<Matrix4>,
    pub local_matrix4_detached: bool,
    pub local_transform_id: f64,
    pub local_transform_using_local_transform_id: f64,
    pub mask_write_mode: bool,
    pub material_bitmap_shader_map: Option<Vec<(Kind, GlBitmapShader)>>,
    pub matrix_array: Vec<f32>,
    pub max_anisotropy: Option<f64>,
    pub measured_height: f64,
    pub measured_width: f64,
    pub mipmap_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub mipmapped_textures: Option<Vec<crate::OpaqueHostValue>>,
    pub mipmap_pipeline: Option<crate::OpaqueHostValue>,
    pub morph_bind_pose: Option<MeshMorphBindPose>,
    pub movie_clip_signals: Option<MovieClipSignals>,
    pub nearest_sampler: crate::OpaqueHostValue,
    pub node_signals: Option<NodeSignals>,
    pub particle_corner_buffer: Option<crate::OpaqueHostValue>,
    pub particle_instance_capacity: f64,
    pub particle_shader: Option<GlParticleShader>,
    pub pipeline_cache: Vec<(String, crate::OpaqueHostValue)>,
    pub quad_batch_corner_buffer: Option<crate::OpaqueHostValue>,
    pub quad_batches: Vec<QuadBatch>,
    pub quad_batch_shader: Option<GlQuadBatchShader>,
    pub quad_index_buffer: crate::OpaqueHostValue,
    pub quad_vertex_buffer: crate::OpaqueHostValue,
    pub quad_vertex_data: Vec<f32>,
    pub render_adapt_hook: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(RenderState, Renderable, RenderProxy2D) -> () + Send + 'static>,
            >,
        >,
    >,
    pub renderer_map: Vec<(Kind, Renderer)>,
    pub renderer_map_id: f64,
    pub render_pass: Option<crate::OpaqueHostValue>,
    pub render_proxy_adapter_map: Vec<(Renderable, RenderProxyAdapter)>,
    pub render_proxy_map: Vec<(Renderable, RenderProxy)>,
    pub render_target_stack: Vec<WgpuSavedPassState>,
    pub resolved_color_transform: Option<ColorTransform>,
    pub retired_buffers: Option<Vec<crate::OpaqueHostValue>>,
    pub rich_text_content: Option<RichTextContent>,
    pub rotation_angle: f64,
    pub rotation_cosine: f64,
    pub rotation_sine: f64,
    pub sampler_cache: Vec<(String, crate::OpaqueHostValue)>,
    pub scene_mesh_upload_cache: Option<Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>>,
    pub selection_begin_index: f64,
    pub selection_end_index: f64,
    pub shader_loc: GlShaderLocations,
    pub shape_mesh_color_transform_shader: Option<GlShapeMeshColorTransformShader>,
    pub shape_mesh_pipelines: Option<Vec<(crate::OpaqueHostValue, WgpuShapeMeshPipeline)>>,
    pub skin_bind_pose: Option<MeshSkinBindPose>,
    pub sprite_batch_blend_mode: Option<BlendMode>,
    pub sprite_batch_buffer_cursor: f64,
    pub sprite_batch_buffer_pool: Vec<WgpuSpriteBatchBufferSlot>,
    pub sprite_batch_color_transform_buffer: Option<crate::OpaqueHostValue>,
    pub sprite_batch_color_transform_data: Vec<f32>,
    pub sprite_batch_color_transform_mode: f64,
    pub sprite_batch_count: f64,
    pub sprite_batch_instance_buffer: Option<crate::OpaqueHostValue>,
    pub sprite_batch_instance_data: Vec<f32>,
    pub sprite_batch_material: Option<Material>,
    pub sprite_batch_material_buffer: Option<crate::OpaqueHostValue>,
    pub sprite_batch_material_data: Vec<f32>,
    pub sprite_batch_material_floats: f64,
    pub sprite_batch_texture: Option<ImageResource>,
    pub sprite_batch_uniform_color_transform: Option<ColorTransform>,
    pub stage: Option<Stage>,
    pub stage_signals: Option<StageSignals>,
    pub temp_stack: Vec<Renderable>,
    pub text_field_signals: Option<TextFieldSignals>,
    pub text_layout: Option<TextLayoutResult>,
    pub text_layout_using_content_id: f64,
    pub texture_bind_group_layout: crate::OpaqueHostValue,
    pub uniform_bind_group: crate::OpaqueHostValue,
    pub uniform_bind_group_layout: crate::OpaqueHostValue,
    pub uniform_buffer: crate::OpaqueHostValue,
    pub uniform_color_transform_shader: Option<GlUniformColorTransformShader>,
    pub uniform_data: Vec<f32>,
    pub uniform_data_u32: Vec<u32>,
    pub uniform_offset: f64,
    pub uniform_stride: f64,
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
    pub world_alpha_using_appearance_id: f64,
    pub world_alpha_using_parent_appearance_id: f64,
    pub world_appearance_id: f64,
    pub world_bounds_rectangle: Option<Rectangle>,
    pub world_bounds_using_local_bounds_id: f64,
    pub world_bounds_using_world_transform_id: f64,
    pub world_matrix: Option<Matrix>,
    pub world_matrix4: Option<Matrix4>,
    pub world_transform_id: f64,
    pub world_transform_using_local_transform_id: f64,
    pub world_transform_using_parent_transform_id: f64,
    pub canvas_render_state_runtime: crate::CanvasRenderStateRuntimeStorage,
    pub dom_render_state_runtime: crate::DomRenderStateRuntimeStorage,
    pub gl_render_state_runtime: crate::GlRenderStateRuntimeStorage,
    pub wgpu_render_state_runtime: crate::WgpuRenderStateRuntimeStorage,
}
impl PartialEq for EntityRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.inner, &other.inner)
    }
}
#[doc(hidden)]
pub trait FlightEntityRuntimeMarker {
    type Runtime;
}
impl<Marker> FlightEntityRuntimeMarker for std::marker::PhantomData<Marker> {
    type Runtime = EntityRuntime;
}

// Source: upstream/packages/types/src/Entity.ts:9 (sha256:9f3e8b58b7216dc7038f2b87e275302645d5d4dce805b89b90c83f9094f1f048)
pub static ENTITY_RUNTIME_KEY: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::for_name(&("EntityRuntime".to_owned())));
