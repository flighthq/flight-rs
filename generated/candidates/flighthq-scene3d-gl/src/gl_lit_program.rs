// @generated from upstream/packages/scene3d-gl/src/glLitProgram.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_gl_scene3_d_runtime;
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, CanvasShapeCommand, CanvasTextureResolvers,
    ColorScaleBias, ExternalTexture, GlBitmapShader, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder,
    GlLitProgram, GlParticleShader, GlQuadBatchShader, GlRenderEffectRunner, GlRenderState,
    GlRenderTextureEntry, GlRenderTextureGuard, GlShaderLocations, GlShapeMeshColorScaleBiasShader,
    GlTextureResolver, GlUniformColorScaleBiasShader, InteractionSignals, Kind,
    MAX_DIRECTIONAL_SHADOW_PCF_RADIUS as max_directional_shadow_pcf_radius_constant, Material,
    Matrix, Matrix4, MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose, MeshSkinBindPose,
    Node, NodeInteractionState, NodeSignals, NodeTraitsKey, Path, PathMesh, Rectangle,
    RenderEffectPaddingResolver, RenderProxy, RenderProxy2D, RenderProxyAdapter, RenderState,
    RenderTexture, Renderable, Renderer, SamplerLike, Scene2D, Scene2DClipHooks, Scene2DSignals,
    Scene3DGraphSyncPolicy, Scene3DLightBlock, ShapeRasterizer, StrokeStyle, Texture,
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

// Source: upstream/packages/scene3d-gl/src/glLitProgram.ts:17 (sha256:9a1b35287d8f162c495fdafbb86162e91a5f9c6f8dad57c692641a3faa442739)
const SHADOW_MAP_TEXTURE_UNIT: f64 = 8.0_f64;

// Source: upstream/packages/scene3d-gl/src/glLitProgram.ts:18 (sha256:097e6357aabcbe0e4bb771579b520f780372d29ef1551a14f95e2dfede08d87f)
const IBL_IRRADIANCE_TEXTURE_UNIT: f64 = 9.0_f64;

// Source: upstream/packages/scene3d-gl/src/glLitProgram.ts:19 (sha256:3d95bb469c80c852641a8872c263bec28b7a3b888d9cea8eefda2e8c9d4a0390)
const IBL_PREFILTERED_TEXTURE_UNIT: f64 = 10.0_f64;

// Source: upstream/packages/scene3d-gl/src/glLitProgram.ts:20 (sha256:1d1b914fc8e9f173701b4dd5a9b1fab93ca79debadf7344e673878863be66fe2)
const IBL_BRDF_TEXTURE_UNIT: f64 = 11.0_f64;

// Source: upstream/packages/scene3d-gl/src/glLitProgram.ts:22 (sha256:24b69273437c7506f6535403f4a6d4289d7eb7cd63aef98d51875284f3293376)
#[derive(Clone, Default)]
struct GlIblPlaceholders {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cube: crate::OpaqueHostValue,
    pub lut: crate::OpaqueHostValue,
}
impl PartialEq for GlIblPlaceholders {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene3d-gl/src/glLitProgram.ts:27 (sha256:08d104b5a9cafd11b4fee9a2cf09130aa50054db39990bd5d31202138b6edea0)
static _IBL_PLACEHOLDERS: std::sync::LazyLock<
    std::sync::Mutex<Vec<(GlRenderState, GlIblPlaceholders)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/scene3d-gl/src/glLitProgram.ts:31 (sha256:460538ea4a5cf78a0d196ce4acfcf0bb157b4e3d415365caf24c60abfce17741)
static _UPLOADED_LIGHT_VERSION: std::sync::LazyLock<std::sync::Mutex<Vec<(GlLitProgram, f64)>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/scene3d-gl/src/glLitProgram.ts:32 (sha256:04d180acaad28d2274f8d9825f880abbf68b50fb56d052a8b31ba8a097ac6c9d)
static _UPLOADED_LIGHT_BLOCK: std::sync::LazyLock<
    std::sync::Mutex<Vec<(GlLitProgram, Scene3DLightBlock)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/scene3d-gl/src/glLitProgram.ts:40 (sha256:c95d655dcb2233410e4652552d1db9d3675504c051732d1343bcae06a116df02)
pub fn bind_gl_mesh_light_block(
    state: &mut GlRenderState,
    program: &GlLitProgram,
    lights: &Scene3DLightBlock,
) -> () {
    let gl = (state.gl).clone();
    if (!(((*_UPLOADED_LIGHT_BLOCK.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*program).clone())
        .map(|(_, value)| value.clone()))
        == Some((*lights).clone())))
        || (!(((*_UPLOADED_LIGHT_VERSION.lock().unwrap())
            .iter()
            .find(|(key, _)| key == &(*program).clone())
            .map(|(_, value)| value.clone()))
            == Some(lights.version)))
    {
        crate::host_value::<()>("host.uniform4f");
        crate::host_value::<()>("host.uniform4f");
        crate::host_value::<()>("host.uniform3f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform4fv");
        crate::host_value::<()>("host.uniform4fv");
        crate::host_value::<()>("host.uniform4fv");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.uniform1i");
        {
            let __flight_key = (*program).clone();
            let __flight_value = (*lights).clone();
            if let Some((_, value)) = (*_UPLOADED_LIGHT_BLOCK.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*_UPLOADED_LIGHT_BLOCK.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
        {
            let __flight_key = (*program).clone();
            let __flight_value = lights.version;
            if let Some((_, value)) = (*_UPLOADED_LIGHT_VERSION.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*_UPLOADED_LIGHT_VERSION.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
    }
    let runtime = get_gl_scene3_d_runtime(state);
    let shadow = (runtime.shadow).clone();
    if ((shadow).is_some()) && (shadow.as_ref().unwrap().enabled) {
        crate::host_value::<()>("host.activeTexture");
        crate::host_value::<()>("host.bindTexture");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.uniformMatrix4fv");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.uniform1f");
    } else {
        crate::host_value::<()>("host.uniform1f");
    }
    let ibl = (runtime.ibl).clone();
    if (ibl).is_some() {
        crate::host_value::<()>("host.activeTexture");
        crate::host_value::<()>("host.bindTexture");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.activeTexture");
        crate::host_value::<()>("host.bindTexture");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.activeTexture");
        crate::host_value::<()>("host.bindTexture");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.activeTexture");
    } else {
        let placeholders = ensure_gl_ibl_placeholders(state);
        crate::host_value::<()>("host.activeTexture");
        crate::host_value::<()>("host.bindTexture");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.activeTexture");
        crate::host_value::<()>("host.bindTexture");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.activeTexture");
        crate::host_value::<()>("host.bindTexture");
        crate::host_value::<()>("host.uniform1i");
        crate::host_value::<()>("host.uniform1f");
        crate::host_value::<()>("host.activeTexture");
    }
}

// Source: upstream/packages/scene3d-gl/src/glLitProgram.ts:143 (sha256:1c012768e63de7ba17f613a0e1fe7351a021c8e85bd2bbbc1a4265962e5a1815)
fn ensure_gl_ibl_placeholders(state: &GlRenderState) -> GlIblPlaceholders {
    let mut placeholders = (*_IBL_PLACEHOLDERS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (placeholders).is_some() {
        return ((placeholders.as_mut().unwrap()).clone()).clone();
    }
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.activeTexture");
    let black: Vec<u8> = (vec![0.0_f64, 0.0_f64, 0.0_f64, 255.0_f64])
        .iter()
        .map(|value| (*value) as u8)
        .collect();
    let cube = crate::host_value::<()>("host.createTexture");
    crate::host_value::<()>("host.bindTexture");
    {
        let mut face = 0.0_f64;
        while (face < 6.0_f64) {
            crate::host_value::<()>("host.texImage2D");
            {
                face += 1.0;
                face
            };
        }
    }
    crate::host_value::<()>("host.texParameteri");
    crate::host_value::<()>("host.texParameteri");
    let lut = crate::host_value::<()>("host.createTexture");
    crate::host_value::<()>("host.bindTexture");
    crate::host_value::<()>("host.texImage2D");
    crate::host_value::<()>("host.texParameteri");
    crate::host_value::<()>("host.texParameteri");
    placeholders = Some(GlIblPlaceholders {
        __flight_identity: std::sync::Arc::new(()),
        cube: (cube).clone(),
        lut: (lut).clone(),
    });
    {
        let __flight_key = (*state).clone();
        let __flight_value = (placeholders).clone().unwrap();
        if let Some((_, value)) = (*_IBL_PLACEHOLDERS.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_IBL_PLACEHOLDERS.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    return ((placeholders).clone().unwrap()).clone();
}

// Source: upstream/packages/scene3d-gl/src/glLitProgram.ts:176 (sha256:7678e558bb28c5689c46e0f9cd7d3d1e7407abef6d8023dd55a7afb1028030c2)
pub fn resolve_gl_lit_locations(
    gl: crate::OpaqueHostValue,
    program: crate::OpaqueHostValue,
) -> GlLitProgram {
    return GlLitProgram {
        __flight_identity: std::sync::Arc::new(()),
        loc_ambient_count: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_ambient_radiance: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_camera_position: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_directional: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_directional_count: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_directional_radiance: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_hemisphere_count: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_hemisphere_lights: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_ibl_brdf: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_ibl_enabled: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_ibl_intensity: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_ibl_irradiance: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_ibl_max_mip: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_ibl_prefiltered: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_point_count: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_point_lights: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_shadow_bias: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_shadow_enabled: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_shadow_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_shadow_matrix: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_shadow_normal_bias_world: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_shadow_pcf_radius: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_spot_count: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_spot_lights: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
    };
}

// Source: upstream/packages/scene3d-gl/src/glLitProgram.ts:212 (sha256:0a344a71e9d6e1abf27a5122974657093defdbda194b5082ce377b7cbcc796a0)
pub static GL_DIRECTIONAL_SHADOW_GLSL: std::sync::LazyLock<String> = std::sync::LazyLock::new(
    || {
        format!(
            "\nuniform sampler2D u_shadowMap;       // directional shadow depth map\nuniform mat4 u_shadowMatrix;         // world -> shadow light-clip\nuniform float u_shadowEnabled;       // 0 or 1 — gates shadow sampling\nuniform int u_shadowPcfRadius;       // integer kernel radius: 0 = one tap, 1 = 3x3\nuniform float u_shadowBias;          // normalized depth-compare bias\nuniform float u_shadowNormalBiasWorld; // receiver offset along the geometric normal in world units\n\n// Directional shadow factor at a world position: 1.0 fully lit, 0.0 fully shadowed. The compile-time\n// radius cap bounds fragment cost. Radius 0 and 1 take dedicated one-tap and 3x3 paths, so the default\n// does not execute the maximum kernel; radius 2 takes the bounded 5x5 path. Fragments outside the\n// shadow frustum are treated as lit.\nfloat compareDirectionalShadow(vec2 uv, float current) {{\n  float closest = texture(u_shadowMap, uv).r;\n  return current <= closest ? 1.0 : 0.0;\n}}\n\nfloat sampleDirectionalShadow(vec3 worldPos, vec3 geometricNormal) {{\n  if (u_shadowEnabled < 0.5) return 1.0;\n  vec3 biasedWorldPos = worldPos + geometricNormal * u_shadowNormalBiasWorld;\n  vec4 clip = u_shadowMatrix * vec4(biasedWorldPos, 1.0);\n  vec3 ndc = clip.xyz / clip.w;\n  vec3 uvz = ndc * 0.5 + 0.5;\n  if (uvz.x < 0.0 || uvz.x > 1.0 || uvz.y < 0.0 || uvz.y > 1.0 || uvz.z > 1.0) return 1.0;\n  float current = uvz.z - u_shadowBias;\n  vec2 texel = 1.0 / vec2(textureSize(u_shadowMap, 0));\n  int radius = clamp(u_shadowPcfRadius, 0, {});\n  if (radius == 0) return compareDirectionalShadow(uvz.xy, current);\n\n  float sum = 0.0;\n  if (radius == 1) {{\n    for (int x = -1; x <= 1; ++x) {{\n      for (int y = -1; y <= 1; ++y) {{\n        sum += compareDirectionalShadow(uvz.xy + vec2(float(x), float(y)) * texel, current);\n      }}\n    }}\n    return sum / 9.0;\n  }}\n  for (int x = -{}; x <= {}; ++x) {{\n    for (int y = -{}; y <= {}; ++y) {{\n      sum += compareDirectionalShadow(uvz.xy + vec2(float(x), float(y)) * texel, current);\n    }}\n  }}\n  float diameter = float({});\n  return sum / (diameter * diameter);\n}}\n",
            max_directional_shadow_pcf_radius_constant,
            max_directional_shadow_pcf_radius_constant,
            max_directional_shadow_pcf_radius_constant,
            max_directional_shadow_pcf_radius_constant,
            max_directional_shadow_pcf_radius_constant,
            ((max_directional_shadow_pcf_radius_constant * 2.0_f64) + 1.0_f64)
        )
    },
);

// Source: upstream/packages/scene3d-gl/src/glLitProgram.ts:260 (sha256:eb5569df24eee6641e33298572d597c822041a42fb6d98f84fb4dab64ea59275)
pub static GL_MESH_LIGHT_BLOCK_GLSL: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "\nuniform vec4 u_directional;          // xyz = light travel direction (surface->light is -xyz)\nuniform vec4 u_directionalRadiance;  // rgb = linear radiance, premultiplied by intensity\nuniform vec3 u_ambientRadiance;      // linear ambient irradiance\nuniform float u_directionalCount;    // 0 or 1 — gates the directional term\nuniform float u_ambientCount;        // 0 or 1 — gates the ambient term\nuniform vec3 u_cameraPosition;       // world-space camera position for view-dependent terms\n\n// Punctual (point/spot/hemisphere) forward-light arrays. Fixed MAX_FORWARD_LIGHTS-wide; each count\n// uniform bounds its loop. Layout matches Scene3DLightBlock.data (packScene3DLightBlock) byte-for-byte:\n//   point[i]      = u_pointLights[i*2+0]={{pos.xyz,range}}, [i*2+1]={{radiance.rgb,invSqrRange}}\n//   spot[i]       = u_spotLights[i*4+0..1] as point, [i*4+2]={{dir.xyz,_}}, [i*4+3]={{cosInner,cosOuter,_,_}}\n//   hemisphere[i] = u_hemisphereLights[i*3+0]={{sky.rgb,_}}, [i*3+1]={{ground.rgb,_}}, [i*3+2]={{up.xyz,_}}\nuniform vec4 u_pointLights[MAX_FORWARD_LIGHTS * 2];\nuniform vec4 u_spotLights[MAX_FORWARD_LIGHTS * 4];\nuniform vec4 u_hemisphereLights[MAX_FORWARD_LIGHTS * 3];\nuniform int u_pointCount;\nuniform int u_spotCount;\nuniform int u_hemisphereCount;\n\n// Smooth inverse-square range window (glTF/UE4): 1 near the light, eased to 0 at the range. invSqrRange\n// is 1/range^2 (0 = infinite range, no cutoff). dist2 is the squared surface->light distance.\nfloat rangeWindow(float dist2, float invSqrRange) {{\n  float factor = dist2 * invSqrRange;\n  float windowed = clamp(1.0 - factor * factor, 0.0, 1.0);\n  return windowed * windowed;\n}}\n{}\n",
        GL_DIRECTIONAL_SHADOW_GLSL
    )
});
