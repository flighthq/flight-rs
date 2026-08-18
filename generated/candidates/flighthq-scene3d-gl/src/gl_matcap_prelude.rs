// @generated from upstream/packages/scene3d-gl/src/glMatcapPrelude.ts; do not edit.
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
    GlMatcapDefineKey, GlMatcapProgram, GlParticleShader, GlQuadBatchShader, GlRenderEffectRunner,
    GlRenderState, GlRenderTextureEntry, GlRenderTextureGuard, GlShaderLocations,
    GlShapeMeshColorScaleBiasShader, GlTextureResolver, GlUniformColorScaleBiasShader,
    InteractionSignals, Kind, LinearColor, Material, Matrix, Matrix4, MeshGeometryGlData,
    MeshGeometryWgpuData, MeshMorphBindPose, MeshSkinBindPose, Node, NodeInteractionState,
    NodeSignals, NodeTraitsKey, Path, PathMesh, Rectangle, RenderEffectPaddingResolver,
    RenderProxy, RenderProxy2D, RenderProxyAdapter, RenderState, RenderTexture, Renderable,
    Renderer, SamplerLike, Scene2D, Scene2DClipHooks, Scene2DSignals, Scene3DGraphSyncPolicy,
    ShapeRasterizer, StrokeStyle, Texture, TextureFilter, TextureSourceKind, TextureWrap,
    TintMaterialData,
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

// Source: upstream/packages/scene3d-gl/src/glMatcapPrelude.ts:9 (sha256:806088298426cdecdd44b0a1c683340130c6dccd996b950fa54781423f426c32)
pub fn bind_gl_matcap_surface(
    state: &GlRenderState,
    program: &GlMatcapProgram,
    tint: &LinearColor,
    matcap: Option<Texture>,
    alpha_cutoff: f64,
) -> () {
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.uniform4f");
    crate::host_value::<()>("host.uniform1f");
    if (matcap).is_some() {
        crate::host_value::<()>("host.activeTexture");
        if (resolve_gl_texture(state, (matcap.as_ref().unwrap()).clone(), None, None)).is_some() {
            crate::host_value::<()>("host.uniform1i");
        }
    }
}

// Source: upstream/packages/scene3d-gl/src/glMatcapPrelude.ts:28 (sha256:f9efdd5d40d48e51f0e0bed3ea6d2d207642b7b596d4463ba1d01ee1b638dd58)
pub fn build_gl_matcap_define_key(key: &GlMatcapDefineKey) -> String {
    return format!(
        "{}{}",
        if key.alpha_mask_enabled {
            "m".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_matcap {
            "t".to_owned()
        } else {
            "-".to_owned()
        }
    );
}

// Source: upstream/packages/scene3d-gl/src/glMatcapPrelude.ts:34 (sha256:79ee0ed950e8903915a9fa6bfa860d5e5f158191085a2c0946ffde7f6f1c6718)
pub fn compile_gl_matcap_program(
    gl: crate::OpaqueHostValue,
    key: &GlMatcapDefineKey,
) -> GlMatcapProgram {
    let program = compile_gl_program(
        (gl).clone(),
        get_gl_matcap_vertex_source_for_key(key),
        get_gl_matcap_fragment_source_for_key(key),
    );
    return GlMatcapProgram {
        __flight_identity: std::sync::Arc::new(()),
        loc_alpha_cutoff: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_matcap: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_model: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_normal_matrix: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_tint: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_view: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_view_projection: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        program: (program).clone(),
    };
}

// Source: upstream/packages/scene3d-gl/src/glMatcapPrelude.ts:50 (sha256:5826967b9f85314bc0aa4addcb181fcfe4511127098fcb5319d80dd99e0b7db6)
pub fn ensure_gl_matcap_program(
    state: &mut GlRenderState,
    key: GlMatcapDefineKey,
) -> GlMatcapProgram {
    return ensure_gl_scene3_d_program(
        state,
        format!("matcap:{}", build_gl_matcap_define_key(&key)),
        &mut |gl: crate::OpaqueHostValue| -> GlMatcapProgram {
            compile_gl_matcap_program((gl).clone(), &key)
        },
    );
}

// Source: upstream/packages/scene3d-gl/src/glMatcapPrelude.ts:57 (sha256:5bd55499e3e048359be2422c17849d9e6eb4acf58198d5c8f1f650ac77736434)
pub fn get_gl_matcap_fragment_source_for_key(key: &GlMatcapDefineKey) -> String {
    return (build_define_source(key) + MATCAP_FRAGMENT_BODY);
}

// Source: upstream/packages/scene3d-gl/src/glMatcapPrelude.ts:62 (sha256:365fe62c9e0241d4dda32c79fe2362dc60c1779fb665d74c23d641377a31c4e5)
pub fn get_gl_matcap_vertex_source_for_key(key: &GlMatcapDefineKey) -> String {
    return (build_define_source(key) + MATCAP_VERTEX_BODY);
}

// Source: upstream/packages/scene3d-gl/src/glMatcapPrelude.ts:66 (sha256:c4405eea9e4d999e38862011c1d731eccfbfab994abaac50f999a899eecd9905)
fn build_define_source(key: &GlMatcapDefineKey) -> String {
    let mut defines = "#version 300 es\n";
    if key.alpha_mask_enabled {
        defines += "#define ALPHA_MASK\n".to_owned();
    }
    if key.has_matcap {
        defines += "#define HAS_MATCAP\n".to_owned();
    }
    return defines;
}

// Source: upstream/packages/scene3d-gl/src/glMatcapPrelude.ts:73 (sha256:2eccf5a25764a81d6fea4a66dfc2ba69e04f84701530324c49d11c6f4586e2f8)
const MATCAP_VERTEX_BODY: &'static str = "\nlayout(location = 0) in vec3 a_position;\nlayout(location = 1) in vec3 a_normal;\n\nuniform mat4 u_viewProjection;\nuniform mat4 u_model;\nuniform mat4 u_view;\nuniform mat3 u_normalMatrix;\n\nout vec3 v_viewNormal;\n\nvoid main() {\n  // u_normalMatrix takes the object normal into world space (handles model rotation/scale);\n  // mat3(u_view) rotates it into view space. Normalized in the fragment scene2d.\n  v_viewNormal = mat3(u_view) * (u_normalMatrix * a_normal);\n  gl_Position = u_viewProjection * u_model * vec4(a_position, 1.0);\n}\n";

// Source: upstream/packages/scene3d-gl/src/glMatcapPrelude.ts:92 (sha256:71da6be9a5c6519feb3ac6801488a6a01db8aa2cd9512d45d224a02d708bf722)
static MATCAP_FRAGMENT_BODY: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "\nprecision highp float;\n\nin vec3 v_viewNormal;\n\nuniform vec4 u_tint;\n#ifdef HAS_MATCAP\nuniform sampler2D u_matcap;\n#endif\n#ifdef ALPHA_MASK\nuniform float u_alphaCutoff;\n#endif\n\n{}\n\nout vec4 fragColor;\n\n// Texture.colorSpace selects the GPU format, so sampled matcap color is already linear here.\nvoid main() {{\n  vec4 color = u_tint;\n#ifdef HAS_MATCAP\n  // The view-space normal projected to 2D indexes the prebaked-lit sphere: uv = n.xy * 0.5 + 0.5.\n  vec3 viewNormal = normalize(v_viewNormal);\n  vec2 matcapUv = viewNormal.xy * 0.5 + 0.5;\n  vec4 sampled = texture(u_matcap, matcapUv);\n  color.rgb *= sampled.rgb;\n  color.a *= sampled.a;\n#endif\n#ifdef ALPHA_MASK\n  if (color.a < u_alphaCutoff) discard;\n  color.a = 1.0;\n#endif\n  fragColor = color;\n{}\n}}\n",
        gl_mesh_fragment_tail_uniforms_constant, gl_mesh_fragment_tail_constant
    )
});
