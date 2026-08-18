// @generated from upstream/packages/scene3d-gl/src/glPbrStandardBlock.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::bind_gl_uv_transform;
use flighthq_color::unpack_color_to_linear;
use flighthq_render_gl::resolve_gl_texture;
use flighthq_texture::has_texture_uv_transform;
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, CanvasShapeCommand, CanvasTextureResolvers,
    ColorScaleBias, ExternalTexture, GlBitmapShader, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder,
    GlParticleShader, GlPbrDefineKey, GlPbrProgram, GlQuadBatchShader, GlRenderEffectRunner,
    GlRenderState, GlRenderTextureEntry, GlRenderTextureGuard, GlShaderLocations,
    GlShapeMeshColorScaleBiasShader, GlTextureResolver, GlUniformColorScaleBiasShader,
    InteractionSignals, Kind, LinearColor, Material, Matrix, Matrix4, MeshGeometryGlData,
    MeshGeometryWgpuData, MeshMorphBindPose, MeshSkinBindPose, Node, NodeInteractionState,
    NodeSignals, NodeTraitsKey, Path, PathMesh, Rectangle, RenderEffectPaddingResolver,
    RenderProxy, RenderProxy2D, RenderProxyAdapter, RenderState, RenderTexture, Renderable,
    Renderer, SamplerLike, Scene2D, Scene2DClipHooks, Scene2DSignals, Scene3DGraphSyncPolicy,
    ShapeRasterizer, StandardPbrMaterialProperties, StrokeStyle, SurfaceMaterial, Texture,
    TextureFilter, TextureSourceKind, TextureWrap, TintMaterialData,
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

// Source: upstream/packages/scene3d-gl/src/glPbrStandardBlock.ts:19 (sha256:a6ada2a09ddedb9065e5b10c81d3fbf8aeb558b7fd0017c5055c41dfbcaa3daf)
pub const GL_PBR_BASE_COLOR_TEXTURE_UNIT: f64 = 0.0_f64;

// Source: upstream/packages/scene3d-gl/src/glPbrStandardBlock.ts:20 (sha256:9d44d3d24c1e0f90bef3b7b6fea0c52f4549ee1862262e8e3442eb3caee3de0f)
pub const GL_PBR_NORMAL_TEXTURE_UNIT: f64 = 1.0_f64;

// Source: upstream/packages/scene3d-gl/src/glPbrStandardBlock.ts:21 (sha256:e83984d78630266d83d2d0d870597252caf1a6f39e54a5ce93b492dc7d830e6a)
pub const GL_PBR_METALLIC_ROUGHNESS_TEXTURE_UNIT: f64 = 2.0_f64;

// Source: upstream/packages/scene3d-gl/src/glPbrStandardBlock.ts:22 (sha256:68018b7e986f3a40b02d65c773a7bf124b2c8268df0d1ae2b685a8cbbdc18877)
pub const GL_PBR_OCCLUSION_TEXTURE_UNIT: f64 = 3.0_f64;

// Source: upstream/packages/scene3d-gl/src/glPbrStandardBlock.ts:23 (sha256:59d640560193ac9a7a67138610e4ba916156b219ca3a2305b44e025afeeae42c)
pub const GL_PBR_EMISSIVE_TEXTURE_UNIT: f64 = 4.0_f64;

// Source: upstream/packages/scene3d-gl/src/glPbrStandardBlock.ts:24 (sha256:bff920c73be2bc666f2035d037b02aa6ed11bc51d94fda9eb7a133bb6a9e8320)
pub const GL_PBR_ALPHA_TEXTURE_UNIT: f64 = 5.0_f64;

// Source: upstream/packages/scene3d-gl/src/glPbrStandardBlock.ts:28 (sha256:a6a3c002573cf297c8464076de2200d6665046d9a209770c871d7abfa2c658c6)
pub const GL_PBR_EXTENSION_TEXTURE_UNIT: f64 = 6.0_f64;

// Source: upstream/packages/scene3d-gl/src/glPbrStandardBlock.ts:39 (sha256:7f3a9642f8e94e86f27ab2b86ee0e0671b07cf7f498d79e2c80acb7c86622ed9)
pub fn bind_gl_pbr_standard_block(
    state: &GlRenderState,
    program: &mut GlPbrProgram,
    standard: Option<StandardPbrMaterialProperties>,
) -> () {
    let gl = (state.gl).clone();
    if (standard).is_none() {
        crate::host_value::<()>("host.uniform4f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform3f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform1f");
        return;
    }
    unpack_color_to_linear(&mut SCRATCH_RGBA, standard.as_ref().unwrap().base_color);
    crate::host_value::<()>("host.uniform4f");
    crate::host_value::<()>("host.uniform1f");
    crate::host_value::<()>("host.uniform1f");
    crate::host_value::<()>("host.uniform1f");
    crate::host_value::<()>("host.uniform1f");
    unpack_color_to_linear(&mut SCRATCH_RGBA, standard.as_ref().unwrap().emissive);
    crate::host_value::<()>("host.uniform3f");
    crate::host_value::<()>("host.uniform1f");
    bind_gl_pbr_standard_texture(
        state,
        ((standard.as_ref().unwrap().base_color_map).clone()).clone(),
        ((program.loc_base_color_map).clone()).clone(),
        GL_PBR_BASE_COLOR_TEXTURE_UNIT,
    );
    bind_gl_pbr_standard_texture(
        state,
        ((standard.as_ref().unwrap().normal_map).clone()).clone(),
        ((program.loc_normal_map).clone()).clone(),
        GL_PBR_NORMAL_TEXTURE_UNIT,
    );
    bind_gl_pbr_standard_texture(
        state,
        ((standard.as_ref().unwrap().metallic_roughness_map).clone()).clone(),
        ((program.loc_metallic_roughness_map).clone()).clone(),
        GL_PBR_METALLIC_ROUGHNESS_TEXTURE_UNIT,
    );
    bind_gl_pbr_standard_texture(
        state,
        ((standard.as_ref().unwrap().occlusion_map).clone()).clone(),
        ((program.loc_occlusion_map).clone()).clone(),
        GL_PBR_OCCLUSION_TEXTURE_UNIT,
    );
    bind_gl_pbr_standard_texture(
        state,
        ((standard.as_ref().unwrap().emissive_map).clone()).clone(),
        ((program.loc_emissive_map).clone()).clone(),
        GL_PBR_EMISSIVE_TEXTURE_UNIT,
    );
    bind_gl_pbr_standard_texture(
        state,
        ((standard.as_ref().unwrap().alpha_map).clone()).clone(),
        ((program.loc_alpha_map).clone()).clone(),
        GL_PBR_ALPHA_TEXTURE_UNIT,
    );
    bind_gl_uv_transform(
        (gl).clone(),
        program,
        ((standard.as_ref().unwrap().base_color_map).clone()).clone(),
    );
}

// Source: upstream/packages/scene3d-gl/src/glPbrStandardBlock.ts:86 (sha256:0106d3af8c101e35ac9d1b0bee4969226c3de0c68bc65e7626f3fe1a44cc03bd)
pub fn bind_gl_pbr_standard_texture(
    state: &GlRenderState,
    texture: Option<Texture>,
    location: Option<crate::OpaqueHostValue>,
    unit: f64,
) -> () {
    if (texture).is_none() {
        return;
    }
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.activeTexture");
    if (resolve_gl_texture(state, (texture.as_ref().unwrap()).clone(), None, None)).is_none() {
        return;
    }
    crate::host_value::<()>("host.uniform1i");
}

// Source: upstream/packages/scene3d-gl/src/glPbrStandardBlock.ts:105 (sha256:09d9f455bdbd99cdc925bc975cd489d7c92c11366c2eb215d6f15172c93957ab)
pub fn build_gl_pbr_standard_define_key(
    state: &GlRenderState,
    standard: Option<StandardPbrMaterialProperties>,
    surface: Option<SurfaceMaterial>,
) -> GlPbrDefineKey {
    let base_color_map = standard
        .as_ref()
        .and_then(|value| (value.base_color_map).clone());
    let alpha_mode =
        (surface.as_ref().map(|value| (value.alpha_mode).clone())).unwrap_or("opaque".to_owned());
    return GlPbrDefineKey {
        __flight_identity: std::sync::Arc::new(()),
        alpha_mask_enabled: (alpha_mode == "mask"),
        has_alpha_map: (alpha_mode != "opaque")
            && (is_gl_texture_ready(
                state,
                (standard
                    .as_ref()
                    .and_then(|value| (value.alpha_map).clone()))
                .clone(),
            )),
        has_base_color_map: is_gl_texture_ready(state, ((base_color_map).clone()).clone()),
        has_emissive_map: is_gl_texture_ready(
            state,
            (standard
                .as_ref()
                .and_then(|value| (value.emissive_map).clone()))
            .clone(),
        ),
        has_metallic_roughness_map: is_gl_texture_ready(
            state,
            (standard
                .as_ref()
                .and_then(|value| (value.metallic_roughness_map).clone()))
            .clone(),
        ),
        has_normal_map: is_gl_texture_ready(
            state,
            (standard
                .as_ref()
                .and_then(|value| (value.normal_map).clone()))
            .clone(),
        ),
        has_occlusion_map: is_gl_texture_ready(
            state,
            (standard
                .as_ref()
                .and_then(|value| (value.occlusion_map).clone()))
            .clone(),
        ),
        has_uv_transform: (((base_color_map).is_some())
            && (is_gl_texture_ready(state, ((base_color_map).clone()).clone())))
            && (has_texture_uv_transform(&base_color_map)),
        has_color_adjustment: None,
        has_color_matrix: None,
        has_skin: None,
    };
}

// Source: upstream/packages/scene3d-gl/src/glPbrStandardBlock.ts:132 (sha256:1ab7a37cb70695d5b973bdfafc97faded5f33b2990541873e77e2e7cd21aa544)
pub fn is_gl_texture_ready(state: &GlRenderState, texture: Option<Texture>) -> bool {
    return ((texture).is_some())
        && ((resolve_gl_texture(state, ((texture).clone().unwrap()).clone(), None, None))
            .is_some());
}

// Source: upstream/packages/scene3d-gl/src/glPbrStandardBlock.ts:136 (sha256:33cbdb0c15208a5943cbbf7c6e6dbffb13fa7a74ea8e21fc4e9ff194027c4ad2)
static SCRATCH_RGBA: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));
