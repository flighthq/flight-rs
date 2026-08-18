// @generated from upstream/packages/scene3d-gl/src/glScene3DTestHelper.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_render::create_render_state;
use flighthq_types::{
    BlendMode, BoundsNodeAny, CanvasShapeCommand, CanvasTextureResolvers, ColorScaleBias,
    ExternalTexture, GlBitmapShader, GlBlendRealization, GlColorAdjustmentMaterialFeature,
    GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder, GlCompressedTextureUploader,
    GlMaterialRenderer, GlMeshMaterialRenderer, GlParticleShader, GlQuadBatchShader,
    GlRenderEffectRunner, GlRenderState, GlRenderStateRuntime, GlRenderTarget,
    GlRenderTextureEntry, GlRenderTextureGuard, GlScissorRect, GlShaderLocations,
    GlShapeMeshColorScaleBiasShader, GlTextureResolver, GlUniformColorScaleBiasShader,
    GlViewportRect, Image, InteractionSignals, Kind, Material, Matrix, Matrix4, MeshGeometryGlData,
    MeshGeometryWgpuData, MeshMorphBindPose, MeshSkinBindPose, NodeInteractionState, NodeSignals,
    Path, PathMesh, Rectangle, RenderEffectPaddingResolver, RenderProxy, RenderProxy2D,
    RenderProxyAdapter, RenderRegistrySignals, RenderState, RenderTargetDimensions, RenderTexture,
    Renderable, Renderer, SamplerLike, Scene2D, Scene2DClipHooks, Scene2DSignals,
    Scene3DGraphSyncPolicy, ShapeRasterizer, StrokeStyle, Texture, TextureSource,
    TextureSourceKind, TintMaterialData,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub name: String,
    pub args: Vec<crate::OpaqueHostValue>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub active_uniforms: Option<Vec<SharedStructuralRecord3>>,
    pub compile_ok: Option<bool>,
    pub link_ok: Option<bool>,
}
impl PartialEq for SharedStructuralRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub name: String,
    pub type_: f64,
}
impl PartialEq for SharedStructuralRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub state: GlRenderState,
    pub gl: FakeGl2,
}
impl PartialEq for SharedStructuralRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub anisotropy_ext: Option<crate::OpaqueHostValue>,
    pub appearance_id: Option<f64>,
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
    pub canvas_render_effect_registry: Option<Vec<(String, CanvasRenderEffectRunner)>>,
    pub canvas_shape_command_registry:
        Option<Vec<(String, CanvasShapeCommand<crate::OpaqueHostValue>)>>,
    pub canvas_texture_resolvers: Option<CanvasTextureResolvers>,
    pub canvas_texture_view: Option<crate::OpaqueHostValue>,
    pub canvas_view_cleared: Option<bool>,
    pub children_id: Option<f64>,
    pub clip_contour_pipelines: Option<Vec<(crate::OpaqueHostValue, WgpuClipContourPipelines)>>,
    pub clip_contour_stack: Option<Vec<WgpuClipContourEntry>>,
    pub clip_forms: Option<Vec<String>>,
    pub color_adjustment_resolver: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(RenderState, RenderProxy, Option<RenderProxy>) -> () + Send + 'static,
                >,
            >,
        >,
    >,
    pub color_adjustment_unsupported_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RenderState, Renderable) -> () + Send + 'static>>,
        >,
    >,
    pub color_matrix_instanced_shader: Option<GlColorScaleBiasInstancedShader>,
    pub color_scale_bias_instanced_shader: Option<GlColorScaleBiasInstancedShader>,
    pub color_tint_instanced_shader: Option<GlColorScaleBiasInstancedShader>,
    pub command_encoder: Option<crate::OpaqueHostValue>,
    pub compressed_texture_decoder: Option<GlCompressedTextureDecoder>,
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
    pub dom_texture_resolver_registry: Option<Vec<(TextureSourceKind, DomTextureResolver)>>,
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
    pub gl_blend_mode_registry: Option<Vec<(BlendMode, GlBlendRealization)>>,
    pub gl_color_adjustment_material_feature: Option<GlColorAdjustmentMaterialFeature>,
    pub gl_color_adjustment_material_feature_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            GlRenderState,
                            crate::FlightUnion2<
                                ColorScaleBias,
                                crate::FlightUnion2<TintMaterialData, Vec<f64>>,
                            >,
                        ) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub gl_external_texture_cache: Option<Vec<(ExternalTexture, crate::OpaqueHostValue)>>,
    pub gl_render_effect_registry: Option<Vec<(Kind, GlRenderEffectRunner)>>,
    pub gl_render_texture_cache: Option<Vec<(RenderTexture, GlRenderTextureEntry)>>,
    pub gl_render_texture_guard: Option<GlRenderTextureGuard>,
    pub gl_texture_resolver_registry: Option<Vec<(TextureSourceKind, GlTextureResolver)>>,
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
    pub render_effect_padding_resolver_registry: Option<Vec<(Kind, RenderEffectPaddingResolver)>>,
    pub renderer_map: Option<Vec<(Kind, Renderer)>>,
    pub renderer_map_id: Option<f64>,
    pub render_pass: Option<crate::OpaqueHostValue>,
    pub render_proxy_adapter_map: Option<Vec<(Renderable, RenderProxyAdapter)>>,
    pub render_proxy_map: Option<Vec<(Renderable, RenderProxy)>>,
    pub render_proxy_sources: Option<Vec<Renderable>>,
    pub render_root_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RenderState, Renderable) -> () + Send + 'static>>,
        >,
    >,
    pub render_target_stack: Option<Vec<WgpuSavedPassState>>,
    pub retired_buffers: Option<Vec<crate::OpaqueHostValue>>,
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
    pub shape_mesh_color_matrix_shader: Option<GlShapeMeshColorScaleBiasShader>,
    pub shape_mesh_color_scale_bias_shader: Option<GlShapeMeshColorScaleBiasShader>,
    pub shape_mesh_pipelines: Option<Vec<(String, WgpuShapeMeshPipeline)>>,
    pub shape_rasterizer: Option<ShapeRasterizer>,
    pub skin_bind_pose: Option<MeshSkinBindPose>,
    pub stroke_tessellator: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(Path, StrokeStyle, Option<f64>) -> Option<PathMesh> + Send + 'static>,
            >,
        >,
    >,
    pub tangent_smoothing_sources: Option<Vec<u32>>,
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
    pub wgpu_color_adjustment_material_feature: Option<WgpuColorAdjustmentMaterialFeature>,
    pub wgpu_color_adjustment_material_feature_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            WgpuRenderState,
                            crate::FlightUnion2<
                                ColorScaleBias,
                                crate::FlightUnion2<TintMaterialData, Vec<f64>>,
                            >,
                        ) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub wgpu_external_texture_cache: Option<Vec<(ExternalTexture, WgpuTextureEntry)>>,
    pub wgpu_render_effect_registry: Option<Vec<(Kind, WgpuRenderEffectRunner)>>,
    pub wgpu_render_texture_cache: Option<Vec<(RenderTexture, WgpuRenderTextureEntry)>>,
    pub wgpu_render_texture_guard: Option<WgpuRenderTextureGuard>,
    pub wgpu_texture_resolver_registry: Option<Vec<(TextureSourceKind, WgpuTextureResolver)>>,
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
    pub registry_miss: Option<flighthq_types::RenderStateRuntimeRecord1>,
    pub default_bitmap_shader: Option<GlBitmapShader>,
    pub particle_instance_buffer: Option<crate::OpaqueHostValue>,
    pub particle_instance_data: Option<Vec<f32>>,
    pub material_renderer_map: Option<Vec<(Kind, GlMaterialRenderer)>>,
    pub scene_mesh_material_registry: Option<Vec<(Kind, GlMeshMaterialRenderer)>>,
    pub quad_batch_writer_material_renderer: Option<GlMaterialRenderer>,
    pub quad_batch_writer_texture: Option<crate::OpaqueHostValue>,
    pub current_scissor_rect: Option<GlScissorRect>,
    pub current_render_target: Option<GlRenderTarget>,
    pub render_target_viewport: Option<GlViewportRect>,
    pub texture_cache: Option<Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>>,
    pub texture_source_premultiplied_texture_cache:
        Option<Vec<(TextureSource, ModuleSynthesizedRecord2691800350)>>,
    pub texture_source_premultiplied_srgb_texture_cache:
        Option<Vec<(TextureSource, ModuleSynthesizedRecord2691800350)>>,
    pub texture_source_straight_texture_cache:
        Option<Vec<(TextureSource, ModuleSynthesizedRecord2691800350)>>,
    pub texture_source_straight_srgb_texture_cache:
        Option<Vec<(TextureSource, ModuleSynthesizedRecord2691800350)>>,
    pub compressed_texture_upload: Option<GlCompressedTextureUploader>,
    pub video_texture_cache: Option<Vec<(Image, ModuleSynthesizedRecord2980518618)>>,
    pub video_srgb_texture_cache: Option<Vec<(Image, ModuleSynthesizedRecord2980518618)>>,
    pub scissor_stack: Option<Vec<GlScissorRect>>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub allow_smoothing: Option<bool>,
    pub background_color: Option<f64>,
    pub background_color_rgba: Option<Vec<f64>>,
    pub background_color_string: Option<String>,
    pub current_clip_depth: Option<f64>,
    pub display_object_clip_hooks: Option<Scene2DClipHooks>,
    pub pixel_ratio: Option<f64>,
    pub render_alpha: Option<f64>,
    pub render_blend_mode: Option<BlendMode>,
    pub render_transform2_d: Option<Matrix>,
    pub scene_graph_sync_policy: Option<Scene3DGraphSyncPolicy>,
    pub round_pixels: Option<bool>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct ModuleSynthesizedRecord1949170252 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clear: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub signals: RenderRegistrySignals,
}
impl PartialEq for ModuleSynthesizedRecord1949170252 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ModuleSynthesizedRecord2691800350 {
    pub __flight_identity: std::sync::Arc<()>,
    pub texture: crate::OpaqueHostValue,
    pub version: f64,
}
impl PartialEq for ModuleSynthesizedRecord2691800350 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ModuleSynthesizedRecord2980518618 {
    pub __flight_identity: std::sync::Arc<()>,
    pub texture: crate::OpaqueHostValue,
    pub uploaded_version: f64,
}
impl PartialEq for ModuleSynthesizedRecord2980518618 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene3d-gl/src/glScene3DTestHelper.ts:12 (sha256:8fbe6b4fb6edc7c6fe472c236bfd68c962398f2bdd4f0244898a3c86e61b30d0)
#[derive(Clone, Default)]
pub struct FakeGl2 {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub calls: Vec<SharedStructuralRecord1>,
}
impl PartialEq for FakeGl2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene3d-gl/src/glScene3DTestHelper.ts:18 (sha256:8d2a8914b1d70356fca600a8c668bde1e4b903036d960687e3732801e0a27e6d)
#[derive(Clone, Default)]
struct MakeFakeGl2Record10 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for MakeFakeGl2Record10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
struct MakeFakeGl2SynthesizedRecord1118266146 {
    __flight_identity: std::sync::Arc<()>,
    active_uniforms: f64,
    active_texture: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    array_buffer: f64,
    attach_shader: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    back: f64,
    bind_buffer: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    bind_framebuffer: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    bind_texture: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    bind_vertex_array: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    blend: f64,
    blend_equation: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    blend_func: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    buffer_data: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    buffer_sub_data: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    calls: Vec<SharedStructuralRecord1>,
    clamp_to_edge: f64,
    clear: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    clear_bufferfi: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    clear_bufferfv: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    clear_color: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    clear_depth: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    color: f64,
    color_buffer_bit: f64,
    compile_status: f64,
    compile_shader: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    create_buffer: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    create_program: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    create_shader: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    create_texture: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    create_vertex_array: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    cull_face: f64,
    cull_face: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    delete_buffer: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    delete_framebuffer: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    delete_program: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    delete_renderbuffer: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    delete_shader: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    delete_texture: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    delete_vertex_array: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    depth_buffer_bit: f64,
    depth_stencil: f64,
    depth_test: f64,
    depth_func: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    depth_mask: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    disable: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    draw_arrays: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    draw_elements: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    draw_elements_instanced: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    element_array_buffer: f64,
    enable: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    enable_vertex_attrib_array: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    float: f64,
    float_mat2: f64,
    float_mat3: f64,
    float_mat4: f64,
    float_vec2: f64,
    float_vec3: f64,
    float_vec4: f64,
    flush: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    fragment_shader: f64,
    framebuffer: f64,
    func_add: f64,
    generate_mipmap: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    get_active_uniform: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue, f64) -> () + Send + 'static>>,
    >,
    get_attrib_location: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    get_extension: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    get_parameter: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>>,
    get_program_info_log: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    get_program_parameter: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue, f64) -> () + Send + 'static>>,
    >,
    get_shader_info_log: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    get_shader_parameter: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    get_uniform_location: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue, String) -> () + Send + 'static>>,
    >,
    less: f64,
    line_strip: f64,
    linear: f64,
    linear_mipmap_linear: f64,
    lines: f64,
    link_status: f64,
    link_program: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    max_texture_image_units: f64,
    max_vertex_uniform_vectors: f64,
    nearest: f64,
    one: f64,
    one_minus_src_alpha: f64,
    pixel_storei: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    points: f64,
    rgba: f64,
    rgba32_f: f64,
    shader_source: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    src_alpha: f64,
    srgb8_alpha8: f64,
    static_draw: f64,
    tex_image2_d: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    tex_parameteri: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    tex_sub_image2_d: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    texture_2_d: f64,
    texture_mag_filter: f64,
    texture_min_filter: f64,
    texture_wrap_s: f64,
    texture_wrap_t: f64,
    texture0: f64,
    texture1: f64,
    triangle_strip: f64,
    triangles: f64,
    uniform1f: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    uniform1fv: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    uniform1i: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    uniform2f: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    uniform2fv: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    uniform3f: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    uniform3fv: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    uniform4f: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    uniform4fv: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    uniform_matrix3fv: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    uniform_matrix4fv: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    unsigned_byte: f64,
    unsigned_int: f64,
    unsigned_short: f64,
    use_program: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    vertex_shader: f64,
    vertex_attrib4f: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    vertex_attrib_divisor: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    vertex_attrib_i_pointer: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    vertex_attrib_pointer: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    viewport: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
}
impl PartialEq for MakeFakeGl2SynthesizedRecord1118266146 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn make_fake_gl2(options: Option<SharedStructuralRecord2>) -> FakeGl2 {
    let compile_ok = (options.as_ref().and_then(|value| value.compile_ok)).unwrap_or(true);
    let link_ok = (options.as_ref().and_then(|value| value.link_ok)).unwrap_or(true);
    let active_uniforms = (options
        .as_ref()
        .and_then(|value| (value.active_uniforms).clone()))
    .unwrap_or(vec![]);
    let calls: std::sync::Arc<std::sync::Mutex<Vec<SharedStructuralRecord1>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let mut record: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        crate::OpaqueHostValue,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<
                            Box<
                                dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue
                                    + Send
                                    + 'static,
                            >,
                        >,
                    > + Send
                    + 'static,
            >,
        >,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut calls = calls.clone();
        move |name: String,
              result: Option<crate::OpaqueHostValue>|
              -> std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue
                        + Send
                        + 'static,
                >,
            >,
        > {
            std::sync::Arc::new(std::sync::Mutex::new(Box::new({
                let mut calls = calls.clone();
                move |args: Vec<crate::OpaqueHostValue>| -> crate::OpaqueHostValue {
                    (*calls.lock().unwrap()).push(SharedStructuralRecord1 {
                        __flight_identity: std::sync::Arc::new(()),
                        name: (name).clone(),
                        args: (args).clone(),
                    });
                    return ((result).clone().unwrap()).clone();
                }
            })
                as Box<
                    dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue
                        + Send
                        + 'static,
                >))
        }
    })
        as Box<
            dyn FnMut(
                    String,
                    crate::OpaqueHostValue,
                ) -> std::sync::Arc<
                    std::sync::Mutex<
                        Box<
                            dyn FnMut(Vec<crate::OpaqueHostValue>) -> crate::OpaqueHostValue
                                + Send
                                + 'static,
                        >,
                    >,
                > + Send
                + 'static,
        >));
    let mut gl = {
        let __flight_source = &(MakeFakeGl2SynthesizedRecord1118266146 {
            __flight_identity: std::sync::Arc::new(()),
            calls: (*calls.lock().unwrap()).clone(),
            array_buffer: 34962.0_f64,
            element_array_buffer: 34963.0_f64,
            static_draw: 35044.0_f64,
            float: 5126.0_f64,
            unsigned_byte: 5121.0_f64,
            unsigned_short: 5123.0_f64,
            unsigned_int: 5125.0_f64,
            lines: 1.0_f64,
            line_strip: 3.0_f64,
            points: 0.0_f64,
            triangles: 4.0_f64,
            triangle_strip: 5.0_f64,
            texture0: 33984.0_f64,
            texture1: 33985.0_f64,
            texture_2_d: 3553.0_f64,
            vertex_shader: 35633.0_f64,
            fragment_shader: 35632.0_f64,
            compile_status: 35713.0_f64,
            link_status: 35714.0_f64,
            active_uniforms: 35718.0_f64,
            float_vec2: 35664.0_f64,
            float_vec3: 35665.0_f64,
            float_vec4: 35666.0_f64,
            float_mat2: 35674.0_f64,
            float_mat3: 35675.0_f64,
            float_mat4: 35676.0_f64,
            blend: 3042.0_f64,
            cull_face: 2884.0_f64,
            back: 1029.0_f64,
            depth_test: 2929.0_f64,
            less: 513.0_f64,
            one: 1.0_f64,
            one_minus_src_alpha: 771.0_f64,
            src_alpha: 770.0_f64,
            func_add: 32774.0_f64,
            framebuffer: 36160.0_f64,
            color_buffer_bit: 16384.0_f64,
            depth_buffer_bit: 256.0_f64,
            color: 6144.0_f64,
            depth_stencil: 34041.0_f64,
            max_texture_image_units: 34930.0_f64,
            max_vertex_uniform_vectors: 36347.0_f64,
            rgba32_f: 34836.0_f64,
            rgba: 6408.0_f64,
            srgb8_alpha8: 35907.0_f64,
            nearest: 9728.0_f64,
            linear: 9729.0_f64,
            linear_mipmap_linear: 9987.0_f64,
            clamp_to_edge: 33071.0_f64,
            texture_min_filter: 10241.0_f64,
            texture_mag_filter: 10240.0_f64,
            texture_wrap_s: 10242.0_f64,
            texture_wrap_t: 10243.0_f64,
            get_parameter: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
                move |pname: f64| -> f64 {
                    if (pname == 36347.0_f64) {
                        1024.0_f64
                    } else {
                        if (pname == 34930.0_f64) {
                            16.0_f64
                        } else {
                            0.0_f64
                        }
                    }
                },
            )
                as Box<dyn FnMut(f64) -> f64 + Send + 'static>)),
            get_extension: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    "getExtension".to_owned(),
                    crate::OpaqueHostValue::Null,
                );
                __flight_result
            },
            create_shader: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    "createShader".to_owned(),
                    MakeFakeGl2Record10 {
                        __flight_identity: std::sync::Arc::new(()),
                    },
                );
                __flight_result
            },
            shader_source: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("shaderSource".to_owned());
                __flight_result
            },
            compile_shader: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("compileShader".to_owned());
                __flight_result
            },
            get_shader_parameter: {
                let __flight_callback = (record).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()("getShaderParameter".to_owned(), compile_ok);
                __flight_result
            },
            get_shader_info_log: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    "getShaderInfoLog".to_owned(),
                    crate::OpaqueHostValue::String("".to_owned()),
                );
                __flight_result
            },
            delete_shader: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("deleteShader".to_owned());
                __flight_result
            },
            create_program: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    "createProgram".to_owned(),
                    MakeFakeGl2Record10 {
                        __flight_identity: std::sync::Arc::new(()),
                    },
                );
                __flight_result
            },
            attach_shader: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("attachShader".to_owned());
                __flight_result
            },
            link_program: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("linkProgram".to_owned());
                __flight_result
            },
            get_program_parameter: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
                let active_uniforms = active_uniforms.clone();
                let mut calls = calls.clone();
                move |_program: crate::OpaqueHostValue, pname: f64| -> () {
                    (*calls.lock().unwrap()).push(SharedStructuralRecord1 {
                        __flight_identity: std::sync::Arc::new(()),
                        name: "getProgramParameter".to_owned(),
                        args: vec![pname],
                    });
                    return if (pname == 35718.0_f64) {
                        (active_uniforms.len() as f64)
                    } else {
                        link_ok
                    };
                }
            })
                as Box<dyn FnMut(crate::OpaqueHostValue, f64) -> () + Send + 'static>)),
            get_active_uniform: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
                let active_uniforms = active_uniforms.clone();
                let mut calls = calls.clone();
                move |_program: crate::OpaqueHostValue, index: f64| -> () {
                    (*calls.lock().unwrap()).push(SharedStructuralRecord1 {
                        __flight_identity: std::sync::Arc::new(()),
                        name: "getActiveUniform".to_owned(),
                        args: vec![index],
                    });
                    let info = active_uniforms[index as usize].clone();
                    return if (info).is_none() {
                        None
                    } else {
                        ClosureSynthesizedRecord3120208375 {
                            __flight_identity: std::sync::Arc::new(()),
                            name: (info.name).clone(),
                            size: 1.0_f64,
                            type_: info.type_,
                        }
                    };
                }
            })
                as Box<dyn FnMut(crate::OpaqueHostValue, f64) -> () + Send + 'static>)),
            get_program_info_log: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    "getProgramInfoLog".to_owned(),
                    crate::OpaqueHostValue::String("".to_owned()),
                );
                __flight_result
            },
            use_program: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("useProgram".to_owned());
                __flight_result
            },
            get_uniform_location: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
                let mut calls = calls.clone();
                move |_program: crate::OpaqueHostValue, name: String| -> () {
                    (*calls.lock().unwrap()).push(SharedStructuralRecord1 {
                        __flight_identity: std::sync::Arc::new(()),
                        name: "getUniformLocation".to_owned(),
                        args: vec![name],
                    });
                    return Font {
                        __flight_identity: std::sync::Arc::new(()),
                        __flight_entity_runtime: Default::default(),
                        name: (name).clone(),
                    };
                }
            })
                as Box<dyn FnMut(crate::OpaqueHostValue, String) -> () + Send + 'static>)),
            create_buffer: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    "createBuffer".to_owned(),
                    MakeFakeGl2Record10 {
                        __flight_identity: std::sync::Arc::new(()),
                    },
                );
                __flight_result
            },
            bind_buffer: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("bindBuffer".to_owned());
                __flight_result
            },
            buffer_data: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("bufferData".to_owned());
                __flight_result
            },
            create_vertex_array: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    "createVertexArray".to_owned(),
                    MakeFakeGl2Record10 {
                        __flight_identity: std::sync::Arc::new(()),
                    },
                );
                __flight_result
            },
            bind_vertex_array: {
                let __flight_callback = (record).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()("bindVertexArray".to_owned());
                __flight_result
            },
            delete_buffer: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("deleteBuffer".to_owned());
                __flight_result
            },
            delete_framebuffer: {
                let __flight_callback = (record).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()("deleteFramebuffer".to_owned());
                __flight_result
            },
            delete_program: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("deleteProgram".to_owned());
                __flight_result
            },
            delete_renderbuffer: {
                let __flight_callback = (record).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()("deleteRenderbuffer".to_owned());
                __flight_result
            },
            delete_texture: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("deleteTexture".to_owned());
                __flight_result
            },
            delete_vertex_array: {
                let __flight_callback = (record).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()("deleteVertexArray".to_owned());
                __flight_result
            },
            enable_vertex_attrib_array: {
                let __flight_callback = (record).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()("enableVertexAttribArray".to_owned());
                __flight_result
            },
            get_attrib_location: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    "getAttribLocation".to_owned(),
                    crate::OpaqueHostValue::Number(0.0_f64),
                );
                __flight_result
            },
            vertex_attrib_pointer: {
                let __flight_callback = (record).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()("vertexAttribPointer".to_owned());
                __flight_result
            },
            vertex_attrib_i_pointer: {
                let __flight_callback = (record).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()("vertexAttribIPointer".to_owned());
                __flight_result
            },
            vertex_attrib4f: {
                let __flight_callback = (record).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()("vertexAttrib4f".to_owned());
                __flight_result
            },
            vertex_attrib_divisor: {
                let __flight_callback = (record).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()("vertexAttribDivisor".to_owned());
                __flight_result
            },
            buffer_sub_data: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("bufferSubData".to_owned());
                __flight_result
            },
            bind_framebuffer: {
                let __flight_callback = (record).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()("bindFramebuffer".to_owned());
                __flight_result
            },
            blend_equation: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("blendEquation".to_owned());
                __flight_result
            },
            blend_func: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("blendFunc".to_owned());
                __flight_result
            },
            clear: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("clear".to_owned());
                __flight_result
            },
            clear_color: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("clearColor".to_owned());
                __flight_result
            },
            clear_depth: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("clearDepth".to_owned());
                __flight_result
            },
            clear_bufferfv: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("clearBufferfv".to_owned());
                __flight_result
            },
            clear_bufferfi: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("clearBufferfi".to_owned());
                __flight_result
            },
            cull_face: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("cullFace".to_owned());
                __flight_result
            },
            depth_func: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("depthFunc".to_owned());
                __flight_result
            },
            depth_mask: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("depthMask".to_owned());
                __flight_result
            },
            flush: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("flush".to_owned());
                __flight_result
            },
            viewport: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("viewport".to_owned());
                __flight_result
            },
            disable: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("disable".to_owned());
                __flight_result
            },
            enable: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("enable".to_owned());
                __flight_result
            },
            draw_elements: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("drawElements".to_owned());
                __flight_result
            },
            draw_elements_instanced: {
                let __flight_callback = (record).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()("drawElementsInstanced".to_owned());
                __flight_result
            },
            draw_arrays: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("drawArrays".to_owned());
                __flight_result
            },
            active_texture: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("activeTexture".to_owned());
                __flight_result
            },
            bind_texture: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("bindTexture".to_owned());
                __flight_result
            },
            create_texture: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    "createTexture".to_owned(),
                    MakeFakeGl2Record10 {
                        __flight_identity: std::sync::Arc::new(()),
                    },
                );
                __flight_result
            },
            tex_parameteri: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("texParameteri".to_owned());
                __flight_result
            },
            generate_mipmap: {
                let __flight_callback = (record).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()("generateMipmap".to_owned());
                __flight_result
            },
            tex_image2_d: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("texImage2D".to_owned());
                __flight_result
            },
            tex_sub_image2_d: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("texSubImage2D".to_owned());
                __flight_result
            },
            pixel_storei: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("pixelStorei".to_owned());
                __flight_result
            },
            uniform1i: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("uniform1i".to_owned());
                __flight_result
            },
            uniform1f: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("uniform1f".to_owned());
                __flight_result
            },
            uniform1fv: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("uniform1fv".to_owned());
                __flight_result
            },
            uniform2f: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("uniform2f".to_owned());
                __flight_result
            },
            uniform2fv: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("uniform2fv".to_owned());
                __flight_result
            },
            uniform3f: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("uniform3f".to_owned());
                __flight_result
            },
            uniform3fv: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("uniform3fv".to_owned());
                __flight_result
            },
            uniform4f: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("uniform4f".to_owned());
                __flight_result
            },
            uniform4fv: {
                let __flight_callback = (record).clone();
                let __flight_result = __flight_callback.lock().unwrap()("uniform4fv".to_owned());
                __flight_result
            },
            uniform_matrix3fv: {
                let __flight_callback = (record).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()("uniformMatrix3fv".to_owned());
                __flight_result
            },
            uniform_matrix4fv: {
                let __flight_callback = (record).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()("uniformMatrix4fv".to_owned());
                __flight_result
            },
        });
        FakeGl2 {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            calls: (__flight_source.calls).clone(),
        }
    };
    return {
        let __flight_source = &(gl);
        FakeGl2 {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            calls: (__flight_source.calls).clone(),
        }
    };
}

// Source: upstream/packages/scene3d-gl/src/glScene3DTestHelper.ts:180 (sha256:a6afefa2a88aef7181e942b54776bb07c1b69cc2bd9705ebd3295ad44dd8c5f0)
#[derive(Clone, Default)]
struct MakeGlScene3DStateRecord10 {
    __flight_identity: std::sync::Arc<()>,
    width: f64,
    height: f64,
}
impl PartialEq for MakeGlScene3DStateRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct MakeGlScene3DStateRecord11 {
    __flight_identity: std::sync::Arc<()>,
    allow_smoothing: bool,
    background_color_rgba: Vec<f64>,
}
impl PartialEq for MakeGlScene3DStateRecord11 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct MakeGlScene3DStateRecord12 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for MakeGlScene3DStateRecord12 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn make_gl_scene3_d_state(gl: Option<FakeGl2>) -> SharedStructuralRecord4 {
    let context = (gl).unwrap_or(make_fake_gl2(None));
    let canvas = RenderTargetDimensions {
        __flight_identity: std::sync::Arc::new(()),
        width: 256.0_f64,
        height: 256.0_f64,
    };
    let mut state = create_render_state(Some(FlightPartialRecord6 {
        __flight_identity: std::sync::Arc::new(()),
        allow_smoothing: Some(true),
        background_color_rgba: Some(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]),
        background_color: None,
        background_color_string: None,
        current_clip_depth: None,
        display_object_clip_hooks: None,
        pixel_ratio: None,
        render_alpha: None,
        render_blend_mode: None,
        render_transform2_d: None,
        scene_graph_sync_policy: None,
        round_pixels: None,
    }));
    crate::host_value::<()>("host.assign");
    let runtime = (|| -> GlRenderStateRuntime {
        let mut runtime = create_render_state_runtime();
        let context_runtime: std::sync::Arc<std::sync::Mutex<ClosureRecord13>> =
            std::sync::Arc::new(std::sync::Mutex::new(if (shared_runtime).is_none() {
                ClosureRecord13 {
                    __flight_identity: std::sync::Arc::new(()),
                    fields: MakeGlScene3DStateRecord12 {
                        __flight_identity: std::sync::Arc::new(()),
                    },
                    references: 0.0_f64,
                }
            } else {
                (|| -> GlContextRuntime {
                    let context_runtime = (_context_runtime_by_state_runtime.get)(shared_runtime);
                    if ((*context_runtime.lock().unwrap()).clone()).is_none() {
                        panic!("{}", "GlRenderState runtime has no context tier");
                    }
                    return (*context_runtime.lock().unwrap()).clone();
                })()
            }));
        {
            (*context_runtime.lock().unwrap()).references += 1.0;
            (*context_runtime.lock().unwrap()).references
        };
        (_context_runtime_by_state_runtime.set)(
            (runtime).clone(),
            (*context_runtime.lock().unwrap()).clone(),
        );
        for key in (crate::GL_CONTEXT_RUNTIME_KEYS).iter().cloned() {
            crate::host_value::<()>("host.defineProperty");
        }
        {
            let __flight_runtime = runtime;
            let __flight_value = None;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage
                .gl_render_state_runtime
                .current_render_target = __flight_value;
        };
        {
            let __flight_runtime = runtime;
            let __flight_value = None;
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.binding_cache_guard = __flight_value;
        };
        return runtime;
    })();
    crate::host_value::<()>("host.assign");
    *flighthq_types::FlightEntity::__flight_entity_runtime(&(state))
        .lock()
        .unwrap() = Some((runtime).clone());
    return SharedStructuralRecord4 {
        __flight_identity: std::sync::Arc::new(()),
        state: (state).clone(),
        gl: (context).clone(),
    };
}
