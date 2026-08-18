// @generated from upstream/packages/scene3d-gl/src/glDebugPrelude.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GL_MESH_FRAGMENT_TAIL as gl_mesh_fragment_tail_constant,
    GL_MESH_FRAGMENT_TAIL_UNIFORMS as gl_mesh_fragment_tail_uniforms_constant, compile_gl_program,
    ensure_gl_scene3_d_program,
};
use flighthq_render_gl::resolve_gl_texture;
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, CanvasShapeCommand, CanvasTextureResolvers,
    ColorScaleBias, ExternalTexture, GlBitmapShader, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder,
    GlDebugDefineKey, GlDebugProgram, GlParticleShader, GlQuadBatchShader, GlRenderEffectRunner,
    GlRenderState, GlRenderTextureEntry, GlRenderTextureGuard, GlShaderLocations,
    GlShapeMeshColorScaleBiasShader, GlTextureResolver, GlUniformColorScaleBiasShader,
    InteractionSignals, Kind, Material, Matrix, Matrix4, MeshGeometryGlData, MeshGeometryWgpuData,
    MeshMorphBindPose, MeshSkinBindPose, Node, NodeInteractionState, NodeSignals, NodeTraitsKey,
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

// Source: upstream/packages/scene3d-gl/src/glDebugPrelude.ts:9 (sha256:af925c54bfdbe2afefb2ed95d11d0a3ebd785aac4c42ef18fabbca8ab1d64094)
pub fn bind_gl_debug_normal_map(
    state: &GlRenderState,
    program: &GlDebugProgram,
    normal_map: Option<Texture>,
    normal_scale: f64,
) -> () {
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.uniform1f");
    if (normal_map).is_some() {
        crate::host_value::<()>("host.activeTexture");
        if (resolve_gl_texture(state, (normal_map.as_ref().unwrap()).clone(), None, None)).is_some()
        {
            crate::host_value::<()>("host.uniform1i");
        }
    }
}

// Source: upstream/packages/scene3d-gl/src/glDebugPrelude.ts:27 (sha256:cca10a750a9c52420d73fc66a8a63f4ae930cae48fc69c67f368b4e5d8f62754)
pub fn bind_gl_debug_range(
    state: &GlRenderState,
    program: &GlDebugProgram,
    near: f64,
    far: f64,
) -> () {
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.uniform1f");
    crate::host_value::<()>("host.uniform1f");
}

// Source: upstream/packages/scene3d-gl/src/glDebugPrelude.ts:40 (sha256:19b71fef9692ea32a7d91fa76a93441f91eac6dc11a4d6f76a717cce9a4b331a)
pub fn build_gl_debug_define_key(key: &GlDebugDefineKey) -> String {
    return format!(
        "{}{}",
        if ((key.mode).clone() == "depth") {
            "d".to_owned()
        } else {
            "n".to_owned()
        },
        if key.has_normal_map {
            "m".to_owned()
        } else {
            "-".to_owned()
        }
    );
}

// Source: upstream/packages/scene3d-gl/src/glDebugPrelude.ts:46 (sha256:06ecf1287e67e8d9ca1e39d778e16459c7326e2de93cf81f8c5f970f0ba051c9)
pub fn compile_gl_debug_program(
    gl: crate::OpaqueHostValue,
    key: &GlDebugDefineKey,
) -> GlDebugProgram {
    let program = compile_gl_program(
        (gl).clone(),
        get_gl_debug_vertex_source_for_key(key),
        get_gl_debug_fragment_source_for_key(key),
    );
    return GlDebugProgram {
        __flight_identity: std::sync::Arc::new(()),
        loc_far: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_model: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_near: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_normal_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_normal_matrix: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_normal_scale: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_view_projection: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        program: (program).clone(),
    };
}

// Source: upstream/packages/scene3d-gl/src/glDebugPrelude.ts:62 (sha256:a20d380f29974cd4cd38b200b69144597a81954830349d066c2f46738fd7f736)
pub fn ensure_gl_debug_program(state: &mut GlRenderState, key: GlDebugDefineKey) -> GlDebugProgram {
    return ensure_gl_scene3_d_program(
        state,
        format!("debug:{}", build_gl_debug_define_key(&key)),
        &mut |gl: crate::OpaqueHostValue| -> GlDebugProgram {
            compile_gl_debug_program((gl).clone(), &key)
        },
    );
}

// Source: upstream/packages/scene3d-gl/src/glDebugPrelude.ts:67 (sha256:831b73d5309a3e69ba02462d2e756ae2c170dddaba591134b65f815414e91e52)
pub fn get_gl_debug_fragment_source_for_key(key: &GlDebugDefineKey) -> String {
    return (build_define_source(key) + DEBUG_FRAGMENT_BODY);
}

// Source: upstream/packages/scene3d-gl/src/glDebugPrelude.ts:72 (sha256:481a96657085a3d07faa4b24e25e2d98b2518fdd247fe887f1e6a6a0e91d1c71)
pub fn get_gl_debug_vertex_source_for_key(key: &GlDebugDefineKey) -> String {
    return (build_define_source(key) + DEBUG_VERTEX_BODY);
}

// Source: upstream/packages/scene3d-gl/src/glDebugPrelude.ts:76 (sha256:29bb5d56e377a60c582365846354a6d0d10fa826ad78bc05b40bf72b9bc5f503)
fn build_define_source(key: &GlDebugDefineKey) -> String {
    let mut defines = "#version 300 es\n";
    if ((key.mode).clone() == "depth") {
        defines += "#define DEPTH_MODE\n".to_owned();
    } else {
        defines += "#define NORMAL_MODE\n".to_owned();
    }
    if key.has_normal_map {
        defines += "#define HAS_NORMAL_MAP\n".to_owned();
    }
    return defines;
}

// Source: upstream/packages/scene3d-gl/src/glDebugPrelude.ts:84 (sha256:3d3058793e98c9e2e6b260565c1d0b3a9d7b569c71f843c8e553ad3fb9b4f0b9)
const DEBUG_VERTEX_BODY: &'static str = "\nlayout(location = 0) in vec3 a_position;\nlayout(location = 1) in vec3 a_normal;\nlayout(location = 2) in vec4 a_tangent;\nlayout(location = 3) in vec2 a_uv0;\n\nuniform mat4 u_viewProjection;\nuniform mat4 u_model;\nuniform mat3 u_normalMatrix;\n\nout vec3 v_worldPosition;\nout vec3 v_normal;\nout vec4 v_tangent;\nout vec2 v_uv0;\n\nvoid main() {\n  vec4 worldPosition = u_model * vec4(a_position, 1.0);\n  v_worldPosition = worldPosition.xyz;\n  v_normal = u_normalMatrix * a_normal;\n  v_tangent = vec4(u_normalMatrix * a_tangent.xyz, a_tangent.w);\n  v_uv0 = a_uv0;\n  gl_Position = u_viewProjection * worldPosition;\n}\n";

// Source: upstream/packages/scene3d-gl/src/glDebugPrelude.ts:109 (sha256:bbeb624affdcad7a560c71bf55bdb14679a3bb651d38c105a4298b9490b97b30)
static DEBUG_FRAGMENT_BODY: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "\nprecision highp float;\n\nin vec3 v_worldPosition;\nin vec3 v_normal;\nin vec4 v_tangent;\nin vec2 v_uv0;\n\n#ifdef DEPTH_MODE\nuniform float u_near;\nuniform float u_far;\n#endif\n#ifdef NORMAL_MODE\nuniform float u_normalScale;\n#ifdef HAS_NORMAL_MAP\nuniform sampler2D u_normalMap;\n#endif\n#endif\n\n{}\n\nout vec4 fragColor;\n\nvoid main() {{\n#ifdef DEPTH_MODE\n  // Linear view-space distance is the perspective w: 1.0 / gl_FragCoord.w == w_clip == eye distance.\n  // This is camera-agnostic (no camera near/far needed); map it across the material's [u_near, u_far]\n  // visualization window to grayscale [0, 1].\n  float eyeDepth = 1.0 / gl_FragCoord.w;\n  float d = clamp((eyeDepth - u_near) / max(u_far - u_near, 1e-6), 0.0, 1.0);\n  fragColor = vec4(vec3(d), 1.0);\n#endif\n#ifdef NORMAL_MODE\n  // Visualize the WORLD-space surface normal (the geometric normal carried through u_normalMatrix).\n  vec3 geometricNormal = normalize(v_normal);\n  if (!gl_FrontFacing) geometricNormal = -geometricNormal;\n\n  vec3 normal = geometricNormal;\n#ifdef HAS_NORMAL_MAP\n  vec3 tangent = normalize(v_tangent.xyz);\n  vec3 bitangent = cross(geometricNormal, tangent) * v_tangent.w;\n  vec3 tangentNormal = texture(u_normalMap, v_uv0).xyz * 2.0 - 1.0;\n  tangentNormal.xy *= u_normalScale;\n  mat3 tbn = mat3(tangent, bitangent, geometricNormal);\n  normal = normalize(tbn * tangentNormal);\n#endif\n\n  fragColor = vec4(normal * 0.5 + 0.5, 1.0);\n#endif\n{}\n}}\n",
        gl_mesh_fragment_tail_uniforms_constant, gl_mesh_fragment_tail_constant
    )
});
