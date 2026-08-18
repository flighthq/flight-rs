// @generated from upstream/packages/scene3d-gl/src/extendedPbrGlMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    begin_gl_mesh_draw, bind_gl_mesh_light_block, bind_gl_pbr_extensions,
    bind_gl_pbr_standard_block, build_gl_pbr_standard_define_key, draw_gl_mesh_subset,
    ensure_gl_pbr_program, get_gl_scene3_d_runtime, register_gl_mesh_material_renderer,
    resolve_gl_pbr_extension_contributions, set_gl_mesh_camera_position,
    set_gl_mesh_view_projection,
};
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, Camera3D, CanvasShapeCommand, CanvasTextureResolvers,
    ColorScaleBias, EXTENDED_PBR_MATERIAL_KIND as extended_pbr_material_kind_constant,
    ExternalTexture, GlBitmapShader, GlBlendRealization, GlColorAdjustmentMaterialFeature,
    GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder, GlLitProgram,
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

// Source: upstream/packages/scene3d-gl/src/extendedPbrGlMeshMaterialRenderer.ts:12 (sha256:7f54c566ed2a5495fbba6b0aff3ebd39960bf9eb7217a9d71f33fe05423a2076)
pub static EXTENDED_PBR_GL_MESH_MATERIAL_RENDERER: std::sync::LazyLock<GlMeshMaterialRenderer> =
    std::sync::LazyLock::new(|| GlMeshMaterialRenderer {
        __flight_identity: std::sync::Arc::new(()),
        bind: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut state: GlRenderState,
                  material: Option<Material>,
                  lights: Scene3DLightBlock,
                  camera: Camera3D|
                  -> () {
                let extended = material;
                let mut runtime = get_gl_scene3_d_runtime(&mut state);
                let extensions =
                    (extended.as_ref().map(|value| (value.extensions).clone())).unwrap_or(vec![]);
                let contributions = resolve_gl_pbr_extension_contributions(&mut state, &extensions);
                if (contributions).is_none() {
                    {
                        let __flight_callback = (runtime.pbr_extension_guard).clone();
                        __flight_callback
                            .as_ref()
                            .map(|callback| callback.lock().unwrap()((extensions).clone()))
                    };
                    runtime.active_mesh_program = None;
                    return;
                }
                let mut program = {
                    let __flight_argument_1 = (build_gl_pbr_standard_define_key(
                        &state,
                        (extended.as_ref().map(|value| (value.standard).clone())).clone(),
                        (extended).clone(),
                    ))
                    .clone();
                    ensure_gl_pbr_program(
                        (state).clone(),
                        &__flight_argument_1,
                        Some(((contributions.as_ref().unwrap()).clone()).clone()),
                    )
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
                    (extended.as_ref().map(|value| value.double_sided)).unwrap_or(false),
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
                bind_gl_mesh_light_block(
                    &mut state,
                    &{
                        let __flight_source = &(program);
                        GlLitProgram {
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
                            loc_ambient_count: (__flight_source.loc_ambient_count).clone(),
                            loc_ambient_radiance: (__flight_source.loc_ambient_radiance).clone(),
                            loc_camera_position: (__flight_source.loc_camera_position).clone(),
                            loc_directional: (__flight_source.loc_directional).clone(),
                            loc_directional_count: (__flight_source.loc_directional_count).clone(),
                            loc_directional_radiance: (__flight_source.loc_directional_radiance)
                                .clone(),
                            loc_hemisphere_count: (__flight_source.loc_hemisphere_count).clone(),
                            loc_hemisphere_lights: (__flight_source.loc_hemisphere_lights).clone(),
                            loc_ibl_brdf: (__flight_source.loc_ibl_brdf).clone(),
                            loc_ibl_enabled: (__flight_source.loc_ibl_enabled).clone(),
                            loc_ibl_intensity: (__flight_source.loc_ibl_intensity).clone(),
                            loc_ibl_irradiance: (__flight_source.loc_ibl_irradiance).clone(),
                            loc_ibl_max_mip: (__flight_source.loc_ibl_max_mip).clone(),
                            loc_ibl_prefiltered: (__flight_source.loc_ibl_prefiltered).clone(),
                            loc_point_count: (__flight_source.loc_point_count).clone(),
                            loc_point_lights: (__flight_source.loc_point_lights).clone(),
                            loc_shadow_bias: (__flight_source.loc_shadow_bias).clone(),
                            loc_shadow_enabled: (__flight_source.loc_shadow_enabled).clone(),
                            loc_shadow_map: (__flight_source.loc_shadow_map).clone(),
                            loc_shadow_matrix: (__flight_source.loc_shadow_matrix).clone(),
                            loc_shadow_normal_bias_world: (__flight_source
                                .loc_shadow_normal_bias_world)
                                .clone(),
                            loc_shadow_pcf_radius: (__flight_source.loc_shadow_pcf_radius).clone(),
                            loc_spot_count: (__flight_source.loc_spot_count).clone(),
                            loc_spot_lights: (__flight_source.loc_spot_lights).clone(),
                        }
                    },
                    &lights,
                );
                bind_gl_pbr_standard_block(
                    &state,
                    &mut program,
                    (extended.as_ref().map(|value| (value.standard).clone())).clone(),
                );
                crate::host_value::<()>("host.uniform1f");
                bind_gl_pbr_extensions(&mut state, (program.program).clone(), &extensions);
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

// Source: upstream/packages/scene3d-gl/src/extendedPbrGlMeshMaterialRenderer.ts:43 (sha256:36af5e27cc6a7c3f6a615632bf2722d9bccafbce39b4b2dc0a66b331c69fa797)
pub fn register_gl_extended_pbr_material(state: &mut GlRenderState) -> () {
    register_gl_mesh_material_renderer(
        state,
        (extended_pbr_material_kind_constant).to_owned(),
        &EXTENDED_PBR_GL_MESH_MATERIAL_RENDERER,
    );
}
