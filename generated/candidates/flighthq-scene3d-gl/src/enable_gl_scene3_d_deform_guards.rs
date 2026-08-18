// @generated from upstream/packages/scene3d-gl/src/enableGlScene3DDeformGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_gl_scene3_d_runtime;
use flighthq_log::log_once;
use flighthq_mesh::{get_mesh_geometry_morph_bind_pose, has_mesh_geometry_skin};
use flighthq_node::get_node_runtime;
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, CanvasShapeCommand, CanvasTextureResolvers,
    ColorScaleBias, ExternalTexture, GlBitmapShader, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder,
    GlParticleShader, GlQuadBatchShader, GlRenderEffectRunner, GlRenderState, GlRenderTextureEntry,
    GlRenderTextureGuard, GlShaderLocations, GlShapeMeshColorScaleBiasShader, GlTextureResolver,
    GlUniformColorScaleBiasShader, InteractionSignals, Kind, LogData, LogDataProvider, LogLevel,
    Material, Matrix, Matrix4, Mesh, MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose,
    MeshRuntime, MeshSkinBindPose, Node, NodeAny, NodeInteractionState, NodeSignals, NodeTraitsKey,
    Path, PathMesh, Rectangle, RenderEffectPaddingResolver, RenderProxy, RenderProxy2D,
    RenderProxyAdapter, RenderState, RenderTexture, Renderable, Renderer, SamplerLike, Scene2D,
    Scene2DClipHooks, Scene2DSignals, Scene3DGraphSyncPolicy, ShapeRasterizer, StrokeStyle,
    Texture, TextureFilter, TextureSourceKind, TextureWrap, TintMaterialData,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
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

// Source: upstream/packages/scene3d-gl/src/enableGlScene3DDeformGuards.ts:10 (sha256:d2b40726c3820c8d0ecd7bc87edf20fdfc9d982facc93fd01722be6e4865c6e7)
pub fn are_gl_scene3_d_deform_guards_enabled(state: &mut GlRenderState) -> bool {
    return ((get_gl_scene3_d_runtime(state).deform_guard).clone()).is_some();
}

// Source: upstream/packages/scene3d-gl/src/enableGlScene3DDeformGuards.ts:22 (sha256:6da7b8d046a00690492656fe7c0aef6cff09b14fbf6f9d2e45fc4061f58434b0)
pub fn enable_gl_scene3_d_deform_guards(state: &mut GlRenderState) -> () {
    get_gl_scene3_d_runtime(state).deform_guard = Some(std::sync::Arc::new(std::sync::Mutex::new(
        Box::new(move |__flight_argument_0: Mesh| -> () {
            warn_gl_scene3_d_mesh_drawn_undeformed(&__flight_argument_0)
        }) as Box<dyn FnMut(Mesh) -> () + Send + 'static>,
    )));
}

// Source: upstream/packages/scene3d-gl/src/enableGlScene3DDeformGuards.ts:32 (sha256:b3cddc66a2f1c7fcc402dd9680ef65fb636b5c6b7027b5b9e5f6b83a5fd80644)
#[derive(Clone, Default)]
struct WarnGlScene3DMeshDrawnUndeformedRecord4 {
    __flight_identity: std::sync::Arc<()>,
    message: String,
}
impl PartialEq for WarnGlScene3DMeshDrawnUndeformedRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn warn_gl_scene3_d_mesh_drawn_undeformed(mesh: &Mesh) -> () {
    if (((mesh.morph).clone()).is_some())
        && ((get_mesh_geometry_morph_bind_pose(&mesh.geometry)).is_none())
    {
        log_once(
            "scene-gl:morph-drawn-without-prepare".to_owned(),
            LogLevel::Warn,
            &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
                String,
                Vec<(String, crate::OpaqueHostValue)>,
            >::B({
                let mut __flight_record = Vec::new();
                __flight_record.push(("message".to_owned(), crate::OpaqueHostValue::String("drawGlScene3D: a morphed mesh reached the draw un-blended (it will draw the bind pose) — call prepareScene3DMorph(scene) before prepareScene3DRender.".to_owned())));
                __flight_record
            }))),
            Some(("scene-gl".to_owned()).clone()),
        );
    }
    if (((mesh.skin).clone()).is_some()) && (has_mesh_geometry_skin(&mesh.geometry)) {
        let runtime = {
            let __flight_source = &(get_node_runtime(&{
                let __flight_source = &({
                    let __flight_source = &((*mesh).clone());
                    NodeAny {
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
                });
                Node {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    __flight_entity_runtime: std::sync::Arc::clone(
                        &__flight_source.__flight_entity_runtime,
                    ),
                    data: (__flight_source.data).clone(),
                    enabled: __flight_source.enabled,
                    kind: (__flight_source.kind).clone(),
                    name: (__flight_source.name).clone(),
                }
            }));
            MeshRuntime {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                anisotropy_ext: (__flight_source.anisotropy_ext).clone(),
                appearance_id: __flight_source.appearance_id,
                binding_cache_guard: (__flight_source.binding_cache_guard).clone(),
                bounds_rectangle: (__flight_source.bounds_rectangle).clone(),
                bounds_using_local_bounds_id: __flight_source.bounds_using_local_bounds_id,
                bounds_using_local_transform_id: __flight_source.bounds_using_local_transform_id,
                bounds_version: __flight_source.bounds_version,
                build_text_layout_params: (__flight_source.build_text_layout_params).clone(),
                canvas_blend_effect_backdrops: (__flight_source.canvas_blend_effect_backdrops)
                    .clone(),
                canvas_render_effect_registry: (__flight_source.canvas_render_effect_registry)
                    .clone(),
                canvas_shape_command_registry: (__flight_source.canvas_shape_command_registry)
                    .clone(),
                canvas_texture_resolvers: (__flight_source.canvas_texture_resolvers).clone(),
                canvas_texture_view: (__flight_source.canvas_texture_view).clone(),
                canvas_view_cleared: __flight_source.canvas_view_cleared,
                children_id: __flight_source.children_id,
                clip_contour_pipelines: (__flight_source.clip_contour_pipelines).clone(),
                clip_contour_stack: (__flight_source.clip_contour_stack).clone(),
                clip_forms: (__flight_source.clip_forms).clone(),
                color_adjustment_resolver: (__flight_source.color_adjustment_resolver).clone(),
                color_adjustment_unsupported_guard: (__flight_source
                    .color_adjustment_unsupported_guard)
                    .clone(),
                color_matrix_instanced_shader: (__flight_source.color_matrix_instanced_shader)
                    .clone(),
                color_scale_bias_instanced_shader: (__flight_source
                    .color_scale_bias_instanced_shader)
                    .clone(),
                color_tint_instanced_shader: (__flight_source.color_tint_instanced_shader).clone(),
                command_encoder: (__flight_source.command_encoder).clone(),
                compressed_texture_decoder: (__flight_source.compressed_texture_decoder).clone(),
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
                dom_texture_resolver_registry: (__flight_source.dom_texture_resolver_registry)
                    .clone(),
                element: (__flight_source.element).clone(),
                flush_pending_draws: (__flight_source.flush_pending_draws).clone(),
                frame_capture_buffer: (__flight_source.frame_capture_buffer).clone(),
                frame_capture_bytes_per_row: __flight_source.frame_capture_bytes_per_row,
                frame_capture_enabled: __flight_source.frame_capture_enabled,
                frame_capture_height: __flight_source.frame_capture_height,
                frame_capture_texture: (__flight_source.frame_capture_texture).clone(),
                frame_capture_width: __flight_source.frame_capture_width,
                gl_blend_mode_registry: (__flight_source.gl_blend_mode_registry).clone(),
                gl_color_adjustment_material_feature: (__flight_source
                    .gl_color_adjustment_material_feature)
                    .clone(),
                gl_color_adjustment_material_feature_guard: (__flight_source
                    .gl_color_adjustment_material_feature_guard)
                    .clone(),
                gl_external_texture_cache: (__flight_source.gl_external_texture_cache).clone(),
                gl_render_effect_registry: (__flight_source.gl_render_effect_registry).clone(),
                gl_render_texture_cache: (__flight_source.gl_render_texture_cache).clone(),
                gl_render_texture_guard: (__flight_source.gl_render_texture_guard).clone(),
                gl_texture_resolver_registry: (__flight_source.gl_texture_resolver_registry)
                    .clone(),
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
                local_bounds_using_local_bounds_id: __flight_source
                    .local_bounds_using_local_bounds_id,
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
                quad_batch_writer_blend_mode: (__flight_source.quad_batch_writer_blend_mode)
                    .clone(),
                quad_batch_writer_buffer_cursor: __flight_source.quad_batch_writer_buffer_cursor,
                quad_batch_writer_buffer_pool: (__flight_source.quad_batch_writer_buffer_pool)
                    .clone(),
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
                quad_batch_writer_color_tint_data: (__flight_source
                    .quad_batch_writer_color_tint_data)
                    .clone(),
                quad_batch_writer_count: __flight_source.quad_batch_writer_count,
                quad_batch_writer_instance_buffer: (__flight_source
                    .quad_batch_writer_instance_buffer)
                    .clone(),
                quad_batch_writer_instance_data: (__flight_source.quad_batch_writer_instance_data)
                    .clone(),
                quad_batch_writer_material: (__flight_source.quad_batch_writer_material).clone(),
                quad_batch_writer_material_buffer: (__flight_source
                    .quad_batch_writer_material_buffer)
                    .clone(),
                quad_batch_writer_material_data: (__flight_source.quad_batch_writer_material_data)
                    .clone(),
                quad_batch_writer_material_floats: __flight_source
                    .quad_batch_writer_material_floats,
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
                render_effect_padding_resolver_registry: (__flight_source
                    .render_effect_padding_resolver_registry)
                    .clone(),
                renderer_map: (__flight_source.renderer_map).clone(),
                renderer_map_id: __flight_source.renderer_map_id,
                render_pass: (__flight_source.render_pass).clone(),
                render_proxy_adapter_map: (__flight_source.render_proxy_adapter_map).clone(),
                render_proxy_map: (__flight_source.render_proxy_map).clone(),
                render_proxy_sources: (__flight_source.render_proxy_sources).clone(),
                render_root_guard: (__flight_source.render_root_guard).clone(),
                render_target_stack: (__flight_source.render_target_stack).clone(),
                retired_buffers: (__flight_source.retired_buffers).clone(),
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
                shape_mesh_color_matrix_shader: (__flight_source.shape_mesh_color_matrix_shader)
                    .clone(),
                shape_mesh_color_scale_bias_shader: (__flight_source
                    .shape_mesh_color_scale_bias_shader)
                    .clone(),
                shape_mesh_pipelines: (__flight_source.shape_mesh_pipelines).clone(),
                shape_rasterizer: (__flight_source.shape_rasterizer).clone(),
                skin_bind_pose: (__flight_source.skin_bind_pose).clone(),
                stroke_tessellator: (__flight_source.stroke_tessellator).clone(),
                tangent_smoothing_sources: (__flight_source.tangent_smoothing_sources).clone(),
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
                webgl_shader_binding_resolver: (__flight_source.webgl_shader_binding_resolver)
                    .clone(),
                webgpu_data: (__flight_source.webgpu_data).clone(),
                webgpu_shader_binding_resolver: (__flight_source.webgpu_shader_binding_resolver)
                    .clone(),
                wgpu_color_adjustment_material_feature: (__flight_source
                    .wgpu_color_adjustment_material_feature)
                    .clone(),
                wgpu_color_adjustment_material_feature_guard: (__flight_source
                    .wgpu_color_adjustment_material_feature_guard)
                    .clone(),
                wgpu_external_texture_cache: (__flight_source.wgpu_external_texture_cache).clone(),
                wgpu_render_effect_registry: (__flight_source.wgpu_render_effect_registry).clone(),
                wgpu_render_texture_cache: (__flight_source.wgpu_render_texture_cache).clone(),
                wgpu_render_texture_guard: (__flight_source.wgpu_render_texture_guard).clone(),
                wgpu_texture_resolver_registry: (__flight_source.wgpu_texture_resolver_registry)
                    .clone(),
                world_alpha: __flight_source.world_alpha,
                world_alpha_using_appearance_id: __flight_source.world_alpha_using_appearance_id,
                world_alpha_using_parent_appearance_id: __flight_source
                    .world_alpha_using_parent_appearance_id,
                world_appearance_id: __flight_source.world_appearance_id,
                world_bounds_rectangle: (__flight_source.world_bounds_rectangle).clone(),
                world_bounds_using_local_bounds_id: __flight_source
                    .world_bounds_using_local_bounds_id,
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
                deformed_local_bounds: None,
            }
        };
        if ((runtime.inner.lock().unwrap().deformed_local_bounds).clone()).is_none() {
            log_once(
                "scene-gl:skin-drawn-without-prepare".to_owned(),
                LogLevel::Warn,
                &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
                    String,
                    Vec<(String, crate::OpaqueHostValue)>,
                >::B({
                    let mut __flight_record = Vec::new();
                    __flight_record.push(("message".to_owned(), crate::OpaqueHostValue::String("drawGlScene3D: a GPU-skinned mesh reached the draw un-posed (its joint palette is uncomputed, collapsing it to the origin) — call prepareScene3DSkinning(scene) before prepareScene3DRender.".to_owned())));
                    __flight_record
                }))),
                Some(("scene-gl".to_owned()).clone()),
            );
        }
    }
}
