// @generated from upstream/packages/scene3d-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    begin_gl_mesh_draw, bind_gl_mesh_light_block, bind_gl_pbr_standard_block,
    build_gl_pbr_standard_define_key, draw_gl_mesh_subset, ensure_gl_pbr_program,
    get_gl_scene3_d_runtime, register_gl_mesh_material_renderer, set_gl_mesh_camera_position,
    set_gl_mesh_view_projection,
};
use flighthq_color::unpack_color_to_linear;
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, Camera3D, CanvasShapeCommand, CanvasTextureResolvers,
    ColorScaleBias, ExternalTexture, GlBitmapShader, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder,
    GlLitProgram, GlMeshMaterialRenderer, GlMeshProgram, GlParticleShader, GlQuadBatchShader,
    GlRenderEffectRunner, GlRenderState, GlRenderTextureEntry, GlRenderTextureGuard,
    GlShaderLocations, GlShapeMeshColorScaleBiasShader, GlTextureResolver,
    GlUniformColorScaleBiasShader, InteractionSignals, Kind, LinearColor, Material, Matrix,
    Matrix4, MeshGeometry, MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose,
    MeshSkinBindPose, Node, NodeInteractionState, NodeSignals, NodeTraitsKey, Path, PathMesh,
    Rectangle, RenderEffectPaddingResolver, RenderProxy, RenderProxy2D, RenderProxyAdapter,
    RenderState, RenderTexture, Renderable, Renderer,
    SPECULAR_GLOSSINESS_PBR_MATERIAL_KIND as specular_glossiness_pbr_material_kind_constant,
    SamplerLike, Scene2D, Scene2DClipHooks, Scene2DSignals, Scene3DGraphSyncPolicy,
    Scene3DLightBlock, Scene3DRenderProxy, ShapeRasterizer, SpecularGlossinessPbrMaterial,
    StandardPbrMaterialProperties, StrokeStyle, Texture, TextureFilter, TextureSourceKind,
    TextureWrap, TintMaterialData,
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

// Source: upstream/packages/scene3d-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:46 (sha256:b56ea44bd79a8f0f509935fdda100b87aac690771e8f0d443ec7731c164d0468)
pub static SPECULAR_GLOSSINESS_PBR_GL_MESH_MATERIAL_RENDERER: std::sync::LazyLock<
    GlMeshMaterialRenderer,
> = std::sync::LazyLock::new(|| GlMeshMaterialRenderer {
    __flight_identity: std::sync::Arc::new(()),
    bind: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
        move |mut state: GlRenderState,
              material: Option<Material>,
              lights: Scene3DLightBlock,
              camera: Camera3D|
              -> () {
            let gl = (state.gl).clone();
            let spec_gloss = material;
            let standard = if (spec_gloss).is_some() {
                Some(convert_specular_glossiness_to_standard(
                    &spec_gloss.as_ref().unwrap(),
                ))
            } else {
                None
            };
            let mut program = {
                let __flight_argument_1 = (build_gl_pbr_standard_define_key(
                    &state,
                    ((standard).clone()).clone(),
                    (spec_gloss).clone(),
                ))
                .clone();
                ensure_gl_pbr_program((state).clone(), &__flight_argument_1, None)
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
                        loc_color_matrix_offset: (__flight_source.loc_color_matrix_offset).clone(),
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
                ((spec_gloss).is_some()) && (spec_gloss.as_ref().unwrap().double_sided),
            );
            set_gl_mesh_view_projection(
                &state,
                ((program.loc_view_projection).clone()).clone(),
                &camera,
            );
            set_gl_mesh_camera_position(
                (gl).clone(),
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
                        loc_color_matrix_offset: (__flight_source.loc_color_matrix_offset).clone(),
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
            bind_gl_pbr_standard_block(&state, &mut program, ((standard).clone()).clone());
            crate::host_value::<()>("host.uniform1f");
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

// Source: upstream/packages/scene3d-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:76 (sha256:05900f2601fe994989b1b349124c0d7477755945f3e6444c25a42808619432f1)
pub fn register_gl_specular_glossiness_pbr_material(state: &mut GlRenderState) -> () {
    register_gl_mesh_material_renderer(
        state,
        (specular_glossiness_pbr_material_kind_constant).to_owned(),
        &SPECULAR_GLOSSINESS_PBR_GL_MESH_MATERIAL_RENDERER,
    );
}

// Source: upstream/packages/scene3d-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:85 (sha256:4dff35280bbc8e9613af7f0bf4009a2d310220a34efab68cb7b78fbad3a7c42d)
fn convert_specular_glossiness_to_standard(
    material: &SpecularGlossinessPbrMaterial,
) -> StandardPbrMaterialProperties {
    unpack_color_to_linear(&mut SCRATCH_DIFFUSE, material.diffuse);
    unpack_color_to_linear(&mut SCRATCH_SPECULAR, material.specular);
    let specular_brightness = ((SCRATCH_SPECULAR[0.0_f64 as usize].clone())
        .max(SCRATCH_SPECULAR[1.0_f64 as usize].clone()))
    .max(SCRATCH_SPECULAR[2.0_f64 as usize].clone());
    let one_minus_specular_strength = (1.0_f64 - specular_brightness);
    let diffuse_brightness = ((SCRATCH_DIFFUSE[0.0_f64 as usize].clone())
        .max(SCRATCH_DIFFUSE[1.0_f64 as usize].clone()))
    .max(SCRATCH_DIFFUSE[2.0_f64 as usize].clone());
    let metallic = solve_metallic(
        diffuse_brightness,
        specular_brightness,
        one_minus_specular_strength,
    );
    let denom = (1.0_f64 - DIELECTRIC_SPECULAR).max(0.0001_f64);
    let r = lerp(
        ((SCRATCH_DIFFUSE[0.0_f64 as usize].clone() * one_minus_specular_strength) / denom),
        SCRATCH_SPECULAR[0.0_f64 as usize].clone(),
        metallic,
    );
    let g = lerp(
        ((SCRATCH_DIFFUSE[1.0_f64 as usize].clone() * one_minus_specular_strength) / denom),
        SCRATCH_SPECULAR[1.0_f64 as usize].clone(),
        metallic,
    );
    let b = lerp(
        ((SCRATCH_DIFFUSE[2.0_f64 as usize].clone() * one_minus_specular_strength) / denom),
        SCRATCH_SPECULAR[2.0_f64 as usize].clone(),
        metallic,
    );
    return StandardPbrMaterialProperties {
        __flight_identity: std::sync::Arc::new(()),
        alpha_map: None,
        base_color: pack_linear_rgba(r, g, b, SCRATCH_DIFFUSE[3.0_f64 as usize].clone()),
        base_color_map: (material.diffuse_map).clone(),
        emissive: material.emissive,
        emissive_map: (material.emissive_map).clone(),
        emissive_strength: material.emissive_strength,
        metallic: metallic,
        metallic_roughness_map: None,
        normal_map: (material.normal_map).clone(),
        normal_scale: material.normal_scale,
        occlusion_map: (material.occlusion_map).clone(),
        occlusion_strength: material.occlusion_strength,
        roughness: (1.0_f64 - material.glossiness),
    };
}

// Source: upstream/packages/scene3d-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:120 (sha256:cd0b0c7f12d62073921747b45ae6e65432836c014bfddce7b6618c8baafc1450)
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    return (a + ((b - a) * t));
}

// Source: upstream/packages/scene3d-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:126 (sha256:626a62e8ce825f13d669ae542bb99d554ef9e1c4c6ea66eb51587029fa9494eb)
fn pack_linear_rgba(r: f64, g: f64, b: f64, a: f64) -> f64 {
    let mut to_byte: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new(move |linear: f64| -> f64 {
            let clamped = ((linear).max(0.0_f64)).min(1.0_f64);
            let srgb = if (clamped <= 0.0031308_f64) {
                (clamped * 12.92_f64)
            } else {
                ((1.055_f64 * (clamped).powf((1.0_f64 / 2.4_f64))) - 0.055_f64)
            };
            return (__flight_js_to_i32((srgb * 255.0_f64).round()) & __flight_js_to_i32(255.0_f64))
                as f64;
        })
            as Box<dyn FnMut(f64) -> f64 + Send + 'static>));
    let alpha = (__flight_js_to_i32((((a).max(0.0_f64)).min(1.0_f64) * 255.0_f64).round())
        & __flight_js_to_i32(255.0_f64)) as f64;
    return (__flight_js_to_u32(
        (__flight_js_to_i32(
            (__flight_js_to_i32(
                (__flight_js_to_i32(
                    __flight_js_to_i32({
                        let __flight_callback = (to_byte).clone();
                        let __flight_result = __flight_callback.lock().unwrap()(r);
                        __flight_result
                    })
                    .wrapping_shl((__flight_js_to_u32(24.0_f64) & 31)) as f64,
                ) | __flight_js_to_i32(
                    __flight_js_to_i32({
                        let __flight_callback = (to_byte).clone();
                        let __flight_result = __flight_callback.lock().unwrap()(g);
                        __flight_result
                    })
                    .wrapping_shl((__flight_js_to_u32(16.0_f64) & 31)) as f64,
                )) as f64,
            ) | __flight_js_to_i32(
                __flight_js_to_i32({
                    let __flight_callback = (to_byte).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(b);
                    __flight_result
                })
                .wrapping_shl((__flight_js_to_u32(8.0_f64) & 31)) as f64,
            )) as f64,
        ) | __flight_js_to_i32(alpha)) as f64,
    ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/scene3d-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:139 (sha256:a1c15f5161cfc62eb7be16df30b9216d1c76369868335a877e1e596db421e329)
fn solve_metallic(diffuse: f64, specular: f64, one_minus_specular_strength: f64) -> f64 {
    if (specular < DIELECTRIC_SPECULAR) {
        return 0.0_f64;
    }
    let a = DIELECTRIC_SPECULAR;
    let b = ((((diffuse * one_minus_specular_strength) / (1.0_f64 - DIELECTRIC_SPECULAR))
        + specular)
        - (2.0_f64 * DIELECTRIC_SPECULAR));
    let c = (DIELECTRIC_SPECULAR - specular);
    let discriminant = ((b * b) - ((4.0_f64 * a) * c)).max(0.0_f64);
    return ((((-b) + (discriminant).sqrt()) / (2.0_f64 * a)).max(0.0_f64)).min(1.0_f64);
}

// Source: upstream/packages/scene3d-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:148 (sha256:6eafeff718d709fcec713ab73bf33641a15f4332172b7fc86e7d4648e9f4863c)
const DIELECTRIC_SPECULAR: f64 = 0.04_f64;

// Source: upstream/packages/scene3d-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:149 (sha256:584fea5993f7928db33b03e244cc367cf635b860292ffc9d4823806de8e1ce72)
static SCRATCH_DIFFUSE: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/scene3d-gl/src/specularGlossinessPbrGlMeshMaterialRenderer.ts:150 (sha256:4386710dd492cfc412409f758b2713609d4d20b0babac253dcb67a2987dc9561)
static SCRATCH_SPECULAR: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));
