// @generated from upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ensure_gl_environment_source_cube, get_gl_scene3_d_runtime};
use flighthq_render_gl::create_gl_program;
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, CanvasShapeCommand, CanvasTextureResolvers,
    ColorScaleBias, Environment, ExternalTexture, GlBitmapShader, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder,
    GlParticleShader, GlQuadBatchShader, GlRenderEffectRunner, GlRenderState, GlRenderTextureEntry,
    GlRenderTextureGuard, GlScene3DIbl, GlShaderLocations, GlShapeMeshColorScaleBiasShader,
    GlTextureResolver, GlUniformColorScaleBiasShader, InteractionSignals, Kind, Material, Matrix,
    Matrix4, MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose, MeshSkinBindPose, Node,
    NodeInteractionState, NodeSignals, NodeTraitsKey, Path, PathMesh, Rectangle,
    RenderEffectPaddingResolver, RenderProxy, RenderProxy2D, RenderProxyAdapter, RenderState,
    RenderTexture, Renderable, Renderer, SamplerLike, Scene2D, Scene2DClipHooks, Scene2DSignals,
    Scene3DGraphSyncPolicy, ShapeRasterizer, StrokeStyle, Texture, TextureFilter,
    TextureSourceKind, TextureWrap, TintMaterialData,
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
    pub prefiltered_cube: crate::OpaqueHostValue,
    pub prefiltered_mip_count: f64,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
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
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub anisotropy: Option<f64>,
    pub mag_filter: Option<TextureFilter>,
    pub min_filter: Option<TextureFilter>,
    pub mipmaps: Option<bool>,
    pub wrap_u: Option<TextureWrap>,
    pub wrap_v: Option<TextureWrap>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:17 (sha256:c9c9a3589694c4f1142e51ac08311a602c9aface1623cfbd20b0aac441706bff)
pub fn bake_gl_environment_ibl(state: &mut GlRenderState, environment: &Environment) -> () {
    let source_cube = ensure_gl_environment_source_cube(state, environment);
    if (source_cube).is_none() {
        return;
    }
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.getExtension");
    crate::host_value::<()>("host.getExtension");
    let mut runtime = get_gl_scene3_d_runtime(state);
    if ((runtime.ibl_bake_framebuffer).clone()).is_none() {
        runtime.ibl_bake_framebuffer = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.createFramebuffer",
        ));
    }
    let fbo = (runtime.ibl_bake_framebuffer).clone();
    let prev_framebuffer = crate::host_value::<Option<crate::OpaqueHostValue>>("host.getParameter");
    let prev_viewport = crate::host_value::<Vec<i32>>("host.getParameter");
    crate::host_value::<()>("host.disable");
    crate::host_value::<()>("host.disable");
    crate::host_value::<()>("host.disable");
    let irradiance_cube = bake_gl_irradiance(
        state,
        ((fbo).clone().unwrap()).clone(),
        (source_cube.as_ref().unwrap()).clone(),
    );
    let __destructure0 = bake_gl_prefiltered(
        state,
        ((fbo).clone().unwrap()).clone(),
        (source_cube.as_ref().unwrap()).clone(),
    );
    let prefiltered_cube = (__destructure0.prefiltered_cube).clone();
    let prefiltered_mip_count = __destructure0.prefiltered_mip_count;
    let brdf_lut = (runtime.ibl.as_ref().map(|value| (value.brdf_lut).clone()))
        .unwrap_or(bake_gl_brdf_lut(state, ((fbo).clone().unwrap()).clone()));
    crate::host_value::<()>("host.bindFramebuffer");
    crate::host_value::<()>("host.viewport");
    crate::host_value::<()>("host.bindVertexArray");
    runtime.ibl = Some(GlScene3DIbl {
        __flight_identity: std::sync::Arc::new(()),
        brdf_lut: (brdf_lut).clone(),
        intensity: environment.intensity,
        irradiance_cube: (irradiance_cube).clone(),
        prefiltered_cube: (prefiltered_cube).clone(),
        prefiltered_mip_count: prefiltered_mip_count,
    });
}

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:57 (sha256:22a8a3cc077f20f02f6b9daf77d257fc6d5b3f04bc04c9f2d2c2cfe697c4bc8f)
pub fn destroy_gl_environment_ibl_bake_programs(state: &GlRenderState) -> () {
    let by_state = (*_BAKE_PROGRAMS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (by_state).is_none() {
        return;
    }
    let gl = (state.gl).clone();
    for baked in (by_state
        .as_ref()
        .unwrap()
        .iter()
        .map(|(_, value)| value.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        crate::host_value::<()>("host.deleteProgram");
        crate::host_value::<()>("host.deleteVertexArray");
        crate::host_value::<()>("host.deleteBuffer");
    }
    {
        let __flight_key = (*state).clone();
        if let Some(__flight_index) = (*_BAKE_PROGRAMS.lock().unwrap())
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            (*_BAKE_PROGRAMS.lock().unwrap()).remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:69 (sha256:b09beb67d6ef12529280918f86f0ffb7be6d746584ee9a30f061feba50cfaf5e)
fn bake_gl_irradiance(
    state: &GlRenderState,
    fbo: crate::OpaqueHostValue,
    source_cube: crate::OpaqueHostValue,
) -> crate::OpaqueHostValue {
    let gl = (state.gl).clone();
    let cube = create_gl_bake_cube((gl).clone(), IRRADIANCE_SIZE, false);
    let program = ensure_gl_bake_program(
        state,
        "irradiance".to_owned(),
        (IRRADIANCE_FRAGMENT).clone(),
    );
    crate::host_value::<()>("host.bindFramebuffer");
    crate::host_value::<()>("host.useProgram");
    bind_gl_bake_source_cube((gl).clone(), &program, (source_cube).clone());
    render_gl_bake_cube_faces(
        state,
        (fbo).clone(),
        &program,
        (cube).clone(),
        IRRADIANCE_SIZE,
        0.0_f64,
    );
    return cube;
}

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:80 (sha256:dc07d61bb8a9828fe83cc09106cd06ca3fd45606700029774dea9200907ccc2f)
fn bake_gl_prefiltered(
    state: &GlRenderState,
    fbo: crate::OpaqueHostValue,
    source_cube: crate::OpaqueHostValue,
) -> SharedStructuralRecord1 {
    let gl = (state.gl).clone();
    let cube = create_gl_bake_cube((gl).clone(), PREFILTERED_SIZE, true);
    let program = ensure_gl_bake_program(
        state,
        "prefiltered".to_owned(),
        (PREFILTERED_FRAGMENT).clone(),
    );
    crate::host_value::<()>("host.bindFramebuffer");
    crate::host_value::<()>("host.useProgram");
    bind_gl_bake_source_cube((gl).clone(), &program, (source_cube).clone());
    let mip_count = PREFILTERED_MIPS;
    {
        let mut mip = 0.0_f64;
        while (mip < mip_count) {
            let mip_size = (1.0_f64).max(
                (__flight_js_to_i32(PREFILTERED_SIZE) >> (__flight_js_to_u32(mip) & 31)) as f64,
            );
            let roughness = if (mip_count > 1.0_f64) {
                (mip / (mip_count - 1.0_f64))
            } else {
                0.0_f64
            };
            crate::host_value::<()>("host.uniform1f");
            render_gl_bake_cube_faces(
                state,
                (fbo).clone(),
                &program,
                (cube).clone(),
                mip_size,
                mip,
            );
            {
                mip += 1.0;
                mip
            };
        }
    }
    return SharedStructuralRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        prefiltered_cube: (cube).clone(),
        prefiltered_mip_count: mip_count,
    };
}

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:102 (sha256:ec9eb9e839f45d490e311a31014a475ae2c915e01bead6576accceb63fd11087)
fn bake_gl_brdf_lut(state: &GlRenderState, fbo: crate::OpaqueHostValue) -> crate::OpaqueHostValue {
    let gl = (state.gl).clone();
    let texture = crate::host_value::<()>("host.createTexture");
    crate::host_value::<()>("host.bindTexture");
    crate::host_value::<()>("host.texImage2D");
    crate::host_value::<()>("host.texParameteri");
    crate::host_value::<()>("host.texParameteri");
    crate::host_value::<()>("host.texParameteri");
    crate::host_value::<()>("host.texParameteri");
    let program = ensure_gl_bake_program(state, "brdf".to_owned(), (BRDF_LUT_FRAGMENT).clone());
    crate::host_value::<()>("host.bindFramebuffer");
    crate::host_value::<()>("host.framebufferTexture2D");
    crate::host_value::<()>("host.viewport");
    crate::host_value::<()>("host.useProgram");
    draw_gl_bake_quad(state, &program);
    return texture;
}

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:125 (sha256:aa413ea6ccff966c279bb3d15252994fe4be328030073b819344de3b02a29c2c)
fn render_gl_bake_cube_faces(
    state: &GlRenderState,
    fbo: crate::OpaqueHostValue,
    program: &GlBakeProgram,
    cube: crate::OpaqueHostValue,
    size: f64,
    mip_level: f64,
) -> () {
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.viewport");
    {
        let mut face = 0.0_f64;
        while (face < 6.0_f64) {
            crate::host_value::<()>("host.framebufferTexture2D");
            let b = CUBE_FACE_BASIS[face as usize].clone();
            crate::host_value::<()>("host.uniform3f");
            crate::host_value::<()>("host.uniform3f");
            crate::host_value::<()>("host.uniform3f");
            draw_gl_bake_quad(state, program);
            {
                face += 1.0;
                face
            };
        }
    }
    {
        fbo;
        ()
    };
}

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:146 (sha256:d07d3deb08df85aa6e4c4560deb4dee077d7bc5eb7a2883a8bef2079cbaaba8d)
fn create_gl_bake_cube(
    gl: crate::OpaqueHostValue,
    size: f64,
    mipped: bool,
) -> crate::OpaqueHostValue {
    let texture = crate::host_value::<()>("host.createTexture");
    crate::host_value::<()>("host.bindTexture");
    let levels = if mipped { PREFILTERED_MIPS } else { 1.0_f64 };
    {
        let mut mip = 0.0_f64;
        while (mip < levels) {
            let mip_size =
                (1.0_f64).max((__flight_js_to_i32(size) >> (__flight_js_to_u32(mip) & 31)) as f64);
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
            {
                mip += 1.0;
                mip
            };
        }
    }
    let min_filter = if mipped {
        crate::host_value::<crate::OpaqueHostValue>("host.LINEAR_MIPMAP_LINEAR")
    } else {
        crate::host_value::<crate::OpaqueHostValue>("host.LINEAR")
    };
    crate::host_value::<()>("host.texParameteri");
    crate::host_value::<()>("host.texParameteri");
    crate::host_value::<()>("host.texParameteri");
    crate::host_value::<()>("host.texParameteri");
    crate::host_value::<()>("host.texParameteri");
    if mipped {
        crate::host_value::<()>("host.texParameteri");
    }
    return texture;
}

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:169 (sha256:ec1126cf7292819df0679b8ae0a0fc822735c322d187f64346bf7f48fa3dc5cc)
fn bind_gl_bake_source_cube(
    gl: crate::OpaqueHostValue,
    program: &GlBakeProgram,
    source_cube: crate::OpaqueHostValue,
) -> () {
    crate::host_value::<()>("host.activeTexture");
    crate::host_value::<()>("host.bindTexture");
    crate::host_value::<()>("host.uniform1i");
}

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:175 (sha256:50d28137033757686d11bdd4c671d6ae36ca3e026e32c55a476e44a863fc37cc)
#[derive(Clone, Default)]
struct GlBakeProgram {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub buffer: crate::OpaqueHostValue,
    pub loc_env_cube: Option<crate::OpaqueHostValue>,
    pub loc_face_forward: Option<crate::OpaqueHostValue>,
    pub loc_face_right: Option<crate::OpaqueHostValue>,
    pub loc_face_up: Option<crate::OpaqueHostValue>,
    pub loc_roughness: Option<crate::OpaqueHostValue>,
    pub program: crate::OpaqueHostValue,
    pub vao: crate::OpaqueHostValue,
}
impl PartialEq for GlBakeProgram {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:186 (sha256:8b5973ba6b26695381612a282f30894ae9632e06ca8f5bea9a1bdd2e04e36dcb)
fn ensure_gl_bake_program(state: &GlRenderState, key: String, fragment: String) -> GlBakeProgram {
    let gl = (state.gl).clone();
    let mut by_state = (*_BAKE_PROGRAMS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (by_state).is_none() {
        by_state = Some(Vec::new());
        {
            let __flight_key = (*state).clone();
            let __flight_value = (by_state).clone().unwrap();
            if let Some((_, value)) = (*_BAKE_PROGRAMS.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*_BAKE_PROGRAMS.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
    }
    let mut baked = by_state
        .as_mut()
        .unwrap()
        .iter()
        .find(|(key, _)| key == &(key).clone())
        .map(|(_, value)| value.clone());
    if (baked).is_some() {
        return ((baked.as_mut().unwrap()).clone()).clone();
    }
    let program = link_gl_bake_program((gl).clone(), (fragment).clone());
    let vao = crate::host_value::<()>("host.createVertexArray");
    crate::host_value::<()>("host.bindVertexArray");
    let buffer = crate::host_value::<()>("host.createBuffer");
    crate::host_value::<()>("host.bindBuffer");
    crate::host_value::<()>("host.bufferData");
    crate::host_value::<()>("host.enableVertexAttribArray");
    crate::host_value::<()>("host.vertexAttribPointer");
    crate::host_value::<()>("host.bindVertexArray");
    baked = Some(GlBakeProgram {
        __flight_identity: std::sync::Arc::new(()),
        buffer: (buffer).clone(),
        loc_env_cube: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_face_forward: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_face_right: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_face_up: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_roughness: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        program: (program).clone(),
        vao: (vao).clone(),
    });
    {
        let __flight_key = (key).clone();
        let __flight_value = (baked).clone().unwrap();
        if let Some((_, value)) = by_state
            .as_mut()
            .unwrap()
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            by_state
                .as_mut()
                .unwrap()
                .push((__flight_key, __flight_value));
        }
    };
    return ((baked).clone().unwrap()).clone();
}

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:220 (sha256:818b4987dd275a958dfb348428a0425e6a75e0fb07d7d7d403bc903c45c349cb)
fn draw_gl_bake_quad(state: &GlRenderState, program: &GlBakeProgram) -> () {
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.bindVertexArray");
    crate::host_value::<()>("host.drawArrays");
}

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:226 (sha256:7b8da98ffb8d965da478a4783f7f3fa8a040cb8ab956d64486da47794b806e85)
fn link_gl_bake_program(gl: crate::OpaqueHostValue, fragment: String) -> crate::OpaqueHostValue {
    return create_gl_program(
        (gl).clone(),
        (BAKE_VERTEX).clone(),
        (fragment).clone(),
        Some(("IBL bake".to_owned()).clone()),
    );
}

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:230 (sha256:5e89e94a1bc7ae843942497522c48ebed2208dd7344e31082823e3fdf8ea423e)
const IRRADIANCE_SIZE: f64 = 16.0_f64;

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:231 (sha256:4ebb079b70adb55fb4d0920d186d68cb1b8cfb745de25a5811540369b2a4150b)
const PREFILTERED_SIZE: f64 = 64.0_f64;

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:232 (sha256:7c822a5cabc4a4fb565f49e0b2392dce59bffdde9b41c00645869bd4e7417a17)
const PREFILTERED_MIPS: f64 = 5.0_f64;

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:233 (sha256:6d44ae039a3168e1b77d4f527c615f1a45e614de6ffb746d3fd19a35a646634b)
const BRDF_LUT_SIZE: f64 = 128.0_f64;

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:239 (sha256:3aafcf7c2b93fc31e9366bd44f7a59804c80950a4aeb40c29c1682a914b92d9c)
static CUBE_FACE_BASIS: std::sync::LazyLock<Vec<Vec<f64>>> = std::sync::LazyLock::new(|| {
    vec![
        vec![
            1.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            (-1.0_f64),
            0.0_f64,
            (-1.0_f64),
            0.0_f64,
        ],
        vec![
            (-1.0_f64),
            0.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            1.0_f64,
            0.0_f64,
            (-1.0_f64),
            0.0_f64,
        ],
        vec![
            0.0_f64, 1.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64,
        ],
        vec![
            0.0_f64,
            (-1.0_f64),
            0.0_f64,
            1.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            (-1.0_f64),
        ],
        vec![
            0.0_f64,
            0.0_f64,
            1.0_f64,
            1.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            (-1.0_f64),
            0.0_f64,
        ],
        vec![
            0.0_f64,
            0.0_f64,
            (-1.0_f64),
            (-1.0_f64),
            0.0_f64,
            0.0_f64,
            0.0_f64,
            (-1.0_f64),
            0.0_f64,
        ],
    ]
});

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:248 (sha256:ab8af434cb511536b9b65d28a287d306880acdf6b11f4e5ed6adaad64ee8d496)
static _QUAD: std::sync::LazyLock<Vec<f32>> = std::sync::LazyLock::new(|| {
    (vec![
        (-1.0_f64),
        (-1.0_f64),
        1.0_f64,
        (-1.0_f64),
        (-1.0_f64),
        1.0_f64,
        1.0_f64,
        1.0_f64,
    ])
    .iter()
    .map(|value| (*value) as f32)
    .collect()
});

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:249 (sha256:045bc0b71ebd80a5548b0d3d738372fcc2bb2a8a90ae1973da35a50281c1fe21)
static _BAKE_PROGRAMS: std::sync::LazyLock<
    std::sync::Mutex<Vec<(GlRenderState, Vec<(String, GlBakeProgram)>)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:253 (sha256:88dd65ba77ae1ec9708dd6849aabb4cd190871cb58b6f11c5c8e930a67007350)
const BAKE_VERTEX: &'static str = "#version 300 es\nlayout(location = 0) in vec2 a_position;\nout vec2 v_uv;\nvoid main() {\n  v_uv = a_position;\n  gl_Position = vec4(a_position, 0.0, 1.0);\n}\n";

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:262 (sha256:9c8559f21587a02049816223d64be1de0b88dd2511991ba898e0fb5c62ace50a)
const BAKE_COMMON: &'static str = "precision highp float;\nin vec2 v_uv;\nuniform samplerCube u_envCube;\nuniform vec3 u_faceForward;\nuniform vec3 u_faceRight;\nuniform vec3 u_faceUp;\nout vec4 fragColor;\nconst float PI = 3.14159265359;\n\nvec3 faceDirection() {\n  return normalize(u_faceForward + v_uv.x * u_faceRight + v_uv.y * u_faceUp);\n}\n";

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:277 (sha256:48d7621e429bf959cef4b6d813d3652ab7a7c0b2669b1b4db7ec5b022c0bf7e4)
static IRRADIANCE_FRAGMENT: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "#version 300 es\n{}\nvoid main() {{\n  vec3 N = faceDirection();\n  vec3 up = abs(N.z) < 0.999 ? vec3(0.0, 0.0, 1.0) : vec3(1.0, 0.0, 0.0);\n  vec3 right = normalize(cross(up, N));\n  up = normalize(cross(N, right));\n\n  vec3 irradiance = vec3(0.0);\n  float samples = 0.0;\n  const float delta = 0.15;\n  for (float phi = 0.0; phi < 2.0 * PI; phi += delta) {{\n    for (float theta = 0.0; theta < 0.5 * PI; theta += delta) {{\n      vec3 tangent = vec3(sin(theta) * cos(phi), sin(theta) * sin(phi), cos(theta));\n      vec3 sampleVec = tangent.x * right + tangent.y * up + tangent.z * N;\n      irradiance += texture(u_envCube, sampleVec).rgb * cos(theta) * sin(theta);\n      samples += 1.0;\n    }}\n  }}\n  fragColor = vec4(PI * irradiance / samples, 1.0);\n}}\n",
        BAKE_COMMON
    )
});

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:301 (sha256:6cdde5002694a605ee043e92c9de7c721e5c00035b4473d074cb3957fa916038)
static PREFILTERED_FRAGMENT: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "#version 300 es\n{}\nuniform float u_roughness;\n\nfloat radicalInverse(uint bits) {{\n  bits = (bits << 16u) | (bits >> 16u);\n  bits = ((bits & 0x55555555u) << 1u) | ((bits & 0xAAAAAAAAu) >> 1u);\n  bits = ((bits & 0x33333333u) << 2u) | ((bits & 0xCCCCCCCCu) >> 2u);\n  bits = ((bits & 0x0F0F0F0Fu) << 4u) | ((bits & 0xF0F0F0F0u) >> 4u);\n  bits = ((bits & 0x00FF00FFu) << 8u) | ((bits & 0xFF00FF00u) >> 8u);\n  return float(bits) * 2.3283064365386963e-10;\n}}\nvec2 hammersley(uint i, uint n) {{\n  return vec2(float(i) / float(n), radicalInverse(i));\n}}\nvec3 importanceSampleGGX(vec2 Xi, vec3 N, float roughness) {{\n  float a = roughness * roughness;\n  float phi = 2.0 * PI * Xi.x;\n  float cosTheta = sqrt((1.0 - Xi.y) / (1.0 + (a * a - 1.0) * Xi.y));\n  float sinTheta = sqrt(1.0 - cosTheta * cosTheta);\n  vec3 H = vec3(cos(phi) * sinTheta, sin(phi) * sinTheta, cosTheta);\n  vec3 up = abs(N.z) < 0.999 ? vec3(0.0, 0.0, 1.0) : vec3(1.0, 0.0, 0.0);\n  vec3 tangent = normalize(cross(up, N));\n  vec3 bitangent = cross(N, tangent);\n  return normalize(tangent * H.x + bitangent * H.y + N * H.z);\n}}\nvoid main() {{\n  vec3 N = faceDirection();\n  vec3 V = N;\n  const uint SAMPLE_COUNT = 48u;\n  vec3 prefiltered = vec3(0.0);\n  float totalWeight = 0.0;\n  for (uint i = 0u; i < SAMPLE_COUNT; i++) {{\n    vec2 Xi = hammersley(i, SAMPLE_COUNT);\n    vec3 H = importanceSampleGGX(Xi, N, u_roughness);\n    vec3 L = normalize(2.0 * dot(V, H) * H - V);\n    float nDotL = max(dot(N, L), 0.0);\n    if (nDotL > 0.0) {{\n      prefiltered += texture(u_envCube, L).rgb * nDotL;\n      totalWeight += nDotL;\n    }}\n  }}\n  fragColor = vec4(totalWeight > 0.0 ? prefiltered / totalWeight : texture(u_envCube, N).rgb, 1.0);\n}}\n",
        BAKE_COMMON
    )
});

// Source: upstream/packages/scene3d-gl/src/glEnvironmentIblBake.ts:348 (sha256:dbd602724be2b67244f31c440330e7abb9466d3413c8361b92359b43ba08cac4)
const BRDF_LUT_FRAGMENT: &'static str = "#version 300 es\nprecision highp float;\nin vec2 v_uv;\nout vec4 fragColor;\nconst float PI = 3.14159265359;\n\nfloat radicalInverse(uint bits) {\n  bits = (bits << 16u) | (bits >> 16u);\n  bits = ((bits & 0x55555555u) << 1u) | ((bits & 0xAAAAAAAAu) >> 1u);\n  bits = ((bits & 0x33333333u) << 2u) | ((bits & 0xCCCCCCCCu) >> 2u);\n  bits = ((bits & 0x0F0F0F0Fu) << 4u) | ((bits & 0xF0F0F0F0u) >> 4u);\n  bits = ((bits & 0x00FF00FFu) << 8u) | ((bits & 0xFF00FF00u) >> 8u);\n  return float(bits) * 2.3283064365386963e-10;\n}\nvec2 hammersley(uint i, uint n) {\n  return vec2(float(i) / float(n), radicalInverse(i));\n}\nvec3 importanceSampleGGX(vec2 Xi, vec3 N, float roughness) {\n  float a = roughness * roughness;\n  float phi = 2.0 * PI * Xi.x;\n  float cosTheta = sqrt((1.0 - Xi.y) / (1.0 + (a * a - 1.0) * Xi.y));\n  float sinTheta = sqrt(1.0 - cosTheta * cosTheta);\n  vec3 H = vec3(cos(phi) * sinTheta, sin(phi) * sinTheta, cosTheta);\n  vec3 up = abs(N.z) < 0.999 ? vec3(0.0, 0.0, 1.0) : vec3(1.0, 0.0, 0.0);\n  vec3 tangent = normalize(cross(up, N));\n  vec3 bitangent = cross(N, tangent);\n  return normalize(tangent * H.x + bitangent * H.y + N * H.z);\n}\nfloat geometrySchlickGGX(float nDotV, float roughness) {\n  float k = roughness * roughness / 2.0;\n  return nDotV / (nDotV * (1.0 - k) + k);\n}\nfloat geometrySmith(vec3 N, vec3 V, vec3 L, float roughness) {\n  return geometrySchlickGGX(max(dot(N, L), 0.0), roughness) * geometrySchlickGGX(max(dot(N, V), 0.0), roughness);\n}\nvoid main() {\n  vec2 uv = v_uv * 0.5 + 0.5;\n  float nDotV = max(uv.x, 0.001);\n  float roughness = uv.y;\n  vec3 V = vec3(sqrt(1.0 - nDotV * nDotV), 0.0, nDotV);\n  vec3 N = vec3(0.0, 0.0, 1.0);\n  float A = 0.0;\n  float B = 0.0;\n  const uint SAMPLE_COUNT = 256u;\n  for (uint i = 0u; i < SAMPLE_COUNT; i++) {\n    vec2 Xi = hammersley(i, SAMPLE_COUNT);\n    vec3 H = importanceSampleGGX(Xi, N, roughness);\n    vec3 L = normalize(2.0 * dot(V, H) * H - V);\n    float nDotL = max(L.z, 0.0);\n    float nDotH = max(H.z, 0.0);\n    float vDotH = max(dot(V, H), 0.0);\n    if (nDotL > 0.0) {\n      float G = geometrySmith(N, V, L, roughness);\n      float gVis = (G * vDotH) / (nDotH * nDotV);\n      float Fc = pow(1.0 - vDotH, 5.0);\n      A += (1.0 - Fc) * gVis;\n      B += Fc * gVis;\n    }\n  }\n  fragColor = vec4(A / float(SAMPLE_COUNT), B / float(SAMPLE_COUNT), 0.0, 1.0);\n}\n";
