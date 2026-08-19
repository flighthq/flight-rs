// @generated from upstream/packages/scene3d-gl/src/customShaderGlMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    begin_gl_mesh_draw, compile_gl_program, draw_gl_mesh_subset, ensure_gl_scene3_d_program,
    get_gl_scene3_d_runtime, register_gl_mesh_material_renderer, set_gl_mesh_camera_position,
    set_gl_mesh_view_projection,
};
use flighthq_render_gl::resolve_gl_texture;
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny,
    CUSTOM_SHADER_MATERIAL_KIND as custom_shader_material_kind_constant, Camera3D,
    CanvasShapeCommand, CanvasTextureResolvers, ColorScaleBias, CustomShaderMaterial,
    ExternalTexture, GlBitmapShader, GlBlendRealization, GlColorAdjustmentMaterialFeature,
    GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder, GlCustomMaterialShaderSource,
    GlMeshMaterialRenderer, GlMeshProgram, GlParticleShader, GlQuadBatchShader,
    GlRenderEffectRunner, GlRenderState, GlRenderTextureEntry, GlRenderTextureGuard,
    GlShaderLocations, GlShapeMeshColorScaleBiasShader, GlTextureResolver,
    GlUniformColorScaleBiasShader, InteractionSignals, Kind, Material, Matrix, Matrix4,
    MeshGeometry, MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose, MeshSkinBindPose,
    Node, NodeInteractionState, NodeSignals, NodeTraitsKey, Path, PathMesh, Rectangle,
    RenderEffectPaddingResolver, RenderProxy, RenderProxy2D, RenderProxyAdapter, RenderState,
    RenderTexture, Renderable, Renderer, SamplerLike, Scene2D, Scene2DClipHooks, Scene2DSignals,
    Scene3DGraphSyncPolicy, Scene3DLightBlock, Scene3DRenderProxy, ShapeRasterizer, StrokeStyle,
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

// Source: upstream/packages/scene3d-gl/src/customShaderGlMeshMaterialRenderer.ts:31 (sha256:a42f5cccd06ccfde1e126b27d20be607852f9b63029078a8938b6187bf1c0ee8)
#[derive(Clone, Default)]
struct GlCustomShaderProgram {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loc_color_scale: Option<crate::OpaqueHostValue>,
    pub loc_color_bias: Option<crate::OpaqueHostValue>,
    pub loc_color_matrix0: Option<crate::OpaqueHostValue>,
    pub loc_color_matrix1: Option<crate::OpaqueHostValue>,
    pub loc_color_matrix2: Option<crate::OpaqueHostValue>,
    pub loc_color_matrix3: Option<crate::OpaqueHostValue>,
    pub loc_color_matrix_offset: Option<crate::OpaqueHostValue>,
    pub loc_object_alpha: Option<crate::OpaqueHostValue>,
    pub loc_alpha_is_coverage: Option<crate::OpaqueHostValue>,
    pub loc_joint_texture: Option<crate::OpaqueHostValue>,
    pub loc_model: Option<crate::OpaqueHostValue>,
    pub loc_normal_matrix: Option<crate::OpaqueHostValue>,
    pub loc_uv_transform: Option<crate::OpaqueHostValue>,
    pub loc_view_projection: Option<crate::OpaqueHostValue>,
    pub program: crate::OpaqueHostValue,
    pub loc_camera_position: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlCustomShaderProgram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene3d-gl/src/customShaderGlMeshMaterialRenderer.ts:41 (sha256:a3b095f8601deea21dab1bfd5dd3c2f81edf9dd336dccc7570b0e7026e710728)
pub static CUSTOM_SHADER_GL_MESH_MATERIAL_RENDERER: std::sync::LazyLock<GlMeshMaterialRenderer> =
    std::sync::LazyLock::new(|| GlMeshMaterialRenderer {
        __flight_identity: std::sync::Arc::new(()),
        bind: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut state: GlRenderState,
                  material: Option<Material>,
                  _lights: Scene3DLightBlock,
                  camera: Camera3D|
                  -> () {
                let custom = material;
                if ((custom).is_none()) || ((custom.as_ref().unwrap().shader_key).clone() == "") {
                    get_gl_scene3_d_runtime(&mut state).active_mesh_program = None;
                    return;
                }
                let source = get_gl_custom_material_shader_source(
                    &state,
                    (custom.as_ref().unwrap().shader_key).clone(),
                );
                if (source).is_none() {
                    get_gl_scene3_d_runtime(&mut state).active_mesh_program = None;
                    return;
                }
                let program = ensure_gl_custom_shader_program(
                    &mut state,
                    (custom.as_ref().unwrap().shader_key).clone(),
                    (source.as_ref().unwrap()).clone(),
                );
                {
                    let __flight_callback =
                        (get_gl_scene3_d_runtime(&mut state).custom_shader_guard).clone();
                    __flight_callback.as_ref().map(|callback| {
                        callback.lock().unwrap()(
                            (state).clone(),
                            (program.program).clone(),
                            (custom.as_ref().unwrap().shader_key).clone(),
                        )
                    })
                };
                begin_gl_mesh_draw(
                    &mut state,
                    &{
                        let __flight_source = &(program);
                        GlMeshProgram {
                            __flight_identity: std::sync::Arc::clone(
                                &__flight_source.__flight_identity,
                            ),
                            loc_color_scale: (__flight_source.loc_color_scale).clone(),
                            loc_color_bias: (__flight_source.loc_color_bias).clone(),
                            loc_color_matrix0: (__flight_source.loc_color_matrix0).clone(),
                            loc_color_matrix1: (__flight_source.loc_color_matrix1).clone(),
                            loc_color_matrix2: (__flight_source.loc_color_matrix2).clone(),
                            loc_color_matrix3: (__flight_source.loc_color_matrix3).clone(),
                            loc_color_matrix_offset: (__flight_source.loc_color_matrix_offset)
                                .clone(),
                            loc_object_alpha: (__flight_source.loc_object_alpha).clone(),
                            loc_alpha_is_coverage: (__flight_source.loc_alpha_is_coverage).clone(),
                            loc_joint_texture: (__flight_source.loc_joint_texture).clone(),
                            loc_model: (__flight_source.loc_model).clone(),
                            loc_normal_matrix: (__flight_source.loc_normal_matrix).clone(),
                            loc_uv_transform: (__flight_source.loc_uv_transform).clone(),
                            loc_view_projection: (__flight_source.loc_view_projection).clone(),
                            program: (__flight_source.program).clone(),
                        }
                    },
                    custom.as_ref().unwrap().double_sided,
                );
                set_gl_mesh_view_projection(
                    &state,
                    ((program.loc_view_projection).clone()).clone(),
                    &camera,
                );
                set_gl_mesh_camera_position(
                    (state.gl).clone(),
                    ((program.loc_camera_position).clone()).clone(),
                    &camera,
                );
                upload_custom_shader_material_uniforms(
                    (state.gl).clone(),
                    (program.program).clone(),
                    custom.as_ref().unwrap(),
                );
                upload_custom_shader_material_textures(
                    &state,
                    (program.program).clone(),
                    custom.as_ref().unwrap(),
                );
            },
        )
            as Box<
                dyn FnMut(GlRenderState, Option<Material>, Scene3DLightBlock, Camera3D) -> ()
                    + Send
                    + 'static,
            >)),
        draw: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut state: GlRenderState,
                  proxy: Scene3DRenderProxy,
                  mut geometry: MeshGeometry|
                  -> () {
                let mut program = (get_gl_scene3_d_runtime(&mut state).active_mesh_program).clone();
                if (program).is_none() {
                    return;
                }
                draw_gl_mesh_subset(
                    &mut state,
                    &mut program.as_mut().unwrap(),
                    &proxy,
                    &mut geometry,
                );
            },
        )
            as Box<
                dyn FnMut(GlRenderState, Scene3DRenderProxy, MeshGeometry) -> () + Send + 'static,
            >)),
    });

// Source: upstream/packages/scene3d-gl/src/customShaderGlMeshMaterialRenderer.ts:79 (sha256:92654ea017db7f0f469c932c791d74eff9e7b600cc208a77d1a056479cac9c8f)
pub fn get_gl_custom_material_shader_source(
    state: &GlRenderState,
    shader_key: String,
) -> Option<GlCustomMaterialShaderSource> {
    return (*_CUSTOM_MATERIAL_SHADERS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone())
        .as_mut()
        .unwrap()
        .iter()
        .find(|(key, _)| key == &(shader_key).clone())
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/scene3d-gl/src/customShaderGlMeshMaterialRenderer.ts:104 (sha256:557cfec69c4335c4c0e0398a97f33f8232116e60417b359d4802e71590a8ceae)
pub fn register_gl_custom_material_shader(
    state: &GlRenderState,
    shader_key: String,
    source: &GlCustomMaterialShaderSource,
) -> () {
    let mut registry = (*_CUSTOM_MATERIAL_SHADERS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if ((registry).clone()).is_none() {
        registry = Some(Vec::new());
        {
            let __flight_key = (*state).clone();
            let __flight_value = ((registry).clone()).clone().unwrap();
            if let Some((_, value)) = (*_CUSTOM_MATERIAL_SHADERS.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*_CUSTOM_MATERIAL_SHADERS.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
    }
    {
        let __flight_key = (shader_key).clone();
        let __flight_value = (*source).clone();
        if let Some((_, value)) = registry
            .as_mut()
            .unwrap()
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            registry
                .as_mut()
                .unwrap()
                .push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/scene3d-gl/src/customShaderGlMeshMaterialRenderer.ts:120 (sha256:1c632f71569097511d89259fc3fbbfe361b9a020dcb9b0d35dc6b21cef1a1591)
pub fn register_gl_custom_shader_material(state: &mut GlRenderState) -> () {
    register_gl_mesh_material_renderer(
        state,
        (custom_shader_material_kind_constant).to_owned(),
        &CUSTOM_SHADER_GL_MESH_MATERIAL_RENDERER,
    );
}

// Source: upstream/packages/scene3d-gl/src/customShaderGlMeshMaterialRenderer.ts:123 (sha256:3f4b170add978fb78b0f501608010004eaeebe6e36b71d61fad84eb55eac6192)
fn ensure_gl_custom_shader_program(
    state: &mut GlRenderState,
    shader_key: String,
    source: GlCustomMaterialShaderSource,
) -> GlCustomShaderProgram {
    return ensure_gl_scene3_d_program(
        state,
        format!("custom:{}", shader_key),
        &mut |gl: crate::OpaqueHostValue| -> GlCustomShaderProgram {
            compile_gl_custom_shader_program((gl).clone(), &source)
        },
    );
}

// Source: upstream/packages/scene3d-gl/src/customShaderGlMeshMaterialRenderer.ts:131 (sha256:86091581dd6a83835f1eef7a6b70ce466eee0fadae5675792f8138f4d497f03d)
fn compile_gl_custom_shader_program(
    gl: crate::OpaqueHostValue,
    source: &GlCustomMaterialShaderSource,
) -> GlCustomShaderProgram {
    let linked = compile_gl_program(
        (gl).clone(),
        (source.vertex).clone(),
        (source.fragment).clone(),
    );
    return GlCustomShaderProgram {
        __flight_identity: std::sync::Arc::new(()),
        loc_camera_position: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_model: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_normal_matrix: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_view_projection: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        program: (linked).clone(),
    };
}

// Source: upstream/packages/scene3d-gl/src/customShaderGlMeshMaterialRenderer.ts:145 (sha256:61fbcbfd9410ee112d275197e6ea5fb0d534850e2d6d24dd3315c9968eea58f7)
fn upload_custom_shader_material_uniforms(
    gl: crate::OpaqueHostValue,
    program: crate::OpaqueHostValue,
    material: &CustomShaderMaterial,
) -> () {
    let uniforms = (material.uniforms).clone();
    if (uniforms).is_none() {
        return;
    }
    for name in (crate::host_value::<()>("host.keys")).iter().cloned() {
        let location = crate::host_value::<()>("host.getUniformLocation");
        if ((location).clone()).is_none() {
            continue;
        }
        let value = uniforms
            .as_ref()
            .unwrap()
            .iter()
            .find(|(key, _)| key == &name)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone();
        if (match &((value).clone()) {
            crate::FlightUnion2::A(_) => "number",
            crate::FlightUnion2::B(value) => "object",
        } == "number")
        {
            crate::host_value::<()>("host.uniform1f");
            continue;
        }
        {
            let __switch_value = value.length;
            let __flight_case = if __switch_value == 1.0_f64 {
                0_usize
            } else if __switch_value == 2.0_f64 {
                1_usize
            } else if __switch_value == 3.0_f64 {
                2_usize
            } else if __switch_value == 4.0_f64 {
                3_usize
            } else {
                4_usize
            };
            '__flight_switch: {
                if __flight_case <= 0_usize {
                    crate::host_value::<()>("host.uniform1f");
                    break '__flight_switch;
                }
                if __flight_case <= 1_usize {
                    crate::host_value::<()>("host.uniform2fv");
                    break '__flight_switch;
                }
                if __flight_case <= 2_usize {
                    crate::host_value::<()>("host.uniform3fv");
                    break '__flight_switch;
                }
                if __flight_case <= 3_usize {
                    crate::host_value::<()>("host.uniform4fv");
                    break '__flight_switch;
                }
                if __flight_case <= 4_usize {
                    crate::host_value::<()>("host.uniform1fv");
                    break '__flight_switch;
                }
            }
        }
    }
}

// Source: upstream/packages/scene3d-gl/src/customShaderGlMeshMaterialRenderer.ts:180 (sha256:05dfce52209bb27802dd82ca0e5511c4061050f4cd1dead3105592889237e6ba)
fn upload_custom_shader_material_textures(
    state: &GlRenderState,
    program: crate::OpaqueHostValue,
    material: &CustomShaderMaterial,
) -> () {
    let textures = (material.textures).clone();
    if (textures).is_none() {
        return;
    }
    let gl = (state.gl).clone();
    let mut unit = 0.0_f64;
    for name in (crate::host_value::<()>("host.keys")).iter().cloned() {
        let texture: Texture = textures
            .as_ref()
            .unwrap()
            .iter()
            .find(|(key, _)| key == &name)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone();
        let location = crate::host_value::<()>("host.getUniformLocation");
        if ((location).clone()).is_none() {
            continue;
        }
        crate::host_value::<()>("host.activeTexture");
        if (resolve_gl_texture(state, (texture).clone(), None, None)).is_none() {
            continue;
        }
        crate::host_value::<()>("host.uniform1i");
        {
            unit += 1.0;
            unit
        };
    }
}

// Source: upstream/packages/scene3d-gl/src/customShaderGlMeshMaterialRenderer.ts:200 (sha256:fa0e78d9eae404e484a5c9f186db8993d9c3d32d44d8e376297f418533c6ca71)
static _CUSTOM_MATERIAL_SHADERS: std::sync::LazyLock<
    std::sync::Mutex<Vec<(GlRenderState, Vec<(String, GlCustomMaterialShaderSource)>)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
