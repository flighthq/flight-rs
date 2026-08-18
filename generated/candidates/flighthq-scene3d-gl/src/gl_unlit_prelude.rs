// @generated from upstream/packages/scene3d-gl/src/glUnlitPrelude.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GL_MESH_FRAGMENT_TAIL as gl_mesh_fragment_tail_constant,
    GL_MESH_FRAGMENT_TAIL_UNIFORMS as gl_mesh_fragment_tail_uniforms_constant,
    GL_SKIN_VERTEX_DECLARATIONS_GLSL as gl_skin_vertex_declarations_glsl_constant,
    GL_UV_TRANSFORM_VERTEX_GLSL as gl_uv_transform_vertex_glsl_constant, compile_gl_program,
    ensure_gl_scene3_d_program, get_gl_scene3_d_runtime,
};
use flighthq_render_gl::resolve_gl_texture;
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, CanvasShapeCommand, CanvasTextureResolvers,
    ColorScaleBias, ExternalTexture, GlBitmapShader, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder,
    GlParticleShader, GlQuadBatchShader, GlRenderEffectRunner, GlRenderState, GlRenderTextureEntry,
    GlRenderTextureGuard, GlShaderLocations, GlShapeMeshColorScaleBiasShader, GlTextureResolver,
    GlUniformColorScaleBiasShader, GlUnlitDefineKey, GlUnlitProgram, InteractionSignals, Kind,
    LinearColor, Material, Matrix, Matrix4, MeshGeometryGlData, MeshGeometryWgpuData,
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

// Source: upstream/packages/scene3d-gl/src/glUnlitPrelude.ts:17 (sha256:fb17914e3f60574cbfd861fed9eeddf0e096bc8ac4772999b5b25208702dc35c)
pub fn bind_gl_unlit_surface(
    state: &GlRenderState,
    program: &GlUnlitProgram,
    color: &LinearColor,
    intensity: f64,
    color_map: Option<Texture>,
    alpha_cutoff: f64,
) -> () {
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.uniform4f");
    crate::host_value::<()>("host.uniform1f");
    crate::host_value::<()>("host.uniform1f");
    if (color_map).is_some() {
        crate::host_value::<()>("host.activeTexture");
        if (resolve_gl_texture(state, (color_map.as_ref().unwrap()).clone(), None, None)).is_some()
        {
            crate::host_value::<()>("host.uniform1i");
        }
    }
}

// Source: upstream/packages/scene3d-gl/src/glUnlitPrelude.ts:38 (sha256:b0f222166941c10d6b98d0eab2e7326e6fb8c308dbd31ccf5750f29c8bc910e1)
pub fn build_gl_unlit_define_key(key: &GlUnlitDefineKey) -> String {
    return format!(
        "{}{}{}{}{}",
        if key.alpha_mask_enabled {
            "m".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_color_map {
            "c".to_owned()
        } else {
            "-".to_owned()
        },
        if key.vertex_color {
            "v".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_uv_transform {
            "u".to_owned()
        } else {
            "-".to_owned()
        },
        if (key.has_skin).unwrap_or(false) {
            "k".to_owned()
        } else {
            "-".to_owned()
        }
    );
}

// Source: upstream/packages/scene3d-gl/src/glUnlitPrelude.ts:46 (sha256:3800522d0031b2c6d1168b1074fe5a14f18b753595e1f3e192e446b3566f5c98)
pub fn compile_gl_unlit_program(
    gl: crate::OpaqueHostValue,
    key: &GlUnlitDefineKey,
) -> GlUnlitProgram {
    let program = compile_gl_program(
        (gl).clone(),
        get_gl_unlit_vertex_source_for_key(key),
        get_gl_unlit_fragment_source_for_key(key),
    );
    return GlUnlitProgram {
        __flight_identity: std::sync::Arc::new(()),
        loc_alpha_cutoff: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_color: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_color_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_intensity: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_joint_texture: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_model: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_normal_matrix: None,
        loc_view_projection: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        program: (program).clone(),
    };
}

// Source: upstream/packages/scene3d-gl/src/glUnlitPrelude.ts:63 (sha256:95f79a978a5c7b7a22e70d478edbcfddd9e8f32c28e3be7dcdd164c3e402921a)
pub fn ensure_gl_unlit_program(
    state: &mut GlRenderState,
    key: &GlUnlitDefineKey,
) -> GlUnlitProgram {
    let full_key: GlUnlitDefineKey = GlUnlitDefineKey {
        has_skin: Some(get_gl_scene3_d_runtime(state).active_skinned_run),
        ..((*key).clone()).clone()
    };
    return ensure_gl_scene3_d_program(
        state,
        format!("unlit:{}", build_gl_unlit_define_key(&full_key)),
        &mut |gl: crate::OpaqueHostValue| -> GlUnlitProgram {
            compile_gl_unlit_program((gl).clone(), &full_key)
        },
    );
}

// Source: upstream/packages/scene3d-gl/src/glUnlitPrelude.ts:76 (sha256:b4486273a9149807b50ab979aa6a13d5e9d7fca25e880da4dcbc3503e475e8aa)
pub fn get_gl_unlit_fragment_source_for_key(key: &GlUnlitDefineKey) -> String {
    return (build_define_source(key) + UNLIT_FRAGMENT_BODY);
}

// Source: upstream/packages/scene3d-gl/src/glUnlitPrelude.ts:81 (sha256:2e877f631aa438dedd5d02d50b9c61a78e6cdea7460e6126960b4ae0c21939cc)
pub fn get_gl_unlit_vertex_source_for_key(key: &GlUnlitDefineKey) -> String {
    return ((build_define_source(key)
        + if (key.has_skin).unwrap_or(false) {
            (gl_skin_vertex_declarations_glsl_constant).to_owned()
        } else {
            "".to_owned()
        })
        + UNLIT_VERTEX_BODY);
}

// Source: upstream/packages/scene3d-gl/src/glUnlitPrelude.ts:85 (sha256:798eee80c3bd04b30a0b4d55056d752d4859b74ee9520ec6ed9fd3b528c54931)
fn build_define_source(key: &GlUnlitDefineKey) -> String {
    let mut defines = "#version 300 es\n";
    if key.alpha_mask_enabled {
        defines += "#define ALPHA_MASK\n".to_owned();
    }
    if key.has_color_map {
        defines += "#define HAS_COLOR_MAP\n".to_owned();
    }
    if key.has_uv_transform {
        defines += "#define HAS_UV_TRANSFORM\n".to_owned();
    }
    if key.vertex_color {
        defines += "#define VERTEX_COLOR\n".to_owned();
    }
    if (key.has_skin).unwrap_or(false) {
        defines += "#define HAS_SKIN\n".to_owned();
    }
    return defines;
}

// Source: upstream/packages/scene3d-gl/src/glUnlitPrelude.ts:95 (sha256:59decd435c72f3028dca14b7ab7c621cada8d1ab59c05f06286028c94efa5e0b)
static UNLIT_VERTEX_BODY: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "\nlayout(location = 0) in vec3 a_position;\nlayout(location = 3) in vec2 a_uv0;\n#ifdef VERTEX_COLOR\nlayout(location = 4) in vec4 a_color0;\nout vec4 v_color0;\n#endif\n\nuniform mat4 u_viewProjection;\nuniform mat4 u_model;\n{}\nout vec2 v_uv0;\n\nvoid main() {{\n  v_uv0 = applyUvTransform(a_uv0);\n#ifdef VERTEX_COLOR\n  v_color0 = a_color0;\n#endif\n#ifdef HAS_SKIN\n  gl_Position = u_viewProjection * u_model * skinMatrix() * vec4(a_position, 1.0);\n#else\n  gl_Position = u_viewProjection * u_model * vec4(a_position, 1.0);\n#endif\n}}\n",
        gl_uv_transform_vertex_glsl_constant
    )
});

// Source: upstream/packages/scene3d-gl/src/glUnlitPrelude.ts:121 (sha256:dcf153a2d3a383e5e28126f728873953ac36f99d2831686caee95089ea734754)
static UNLIT_FRAGMENT_BODY: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "\nprecision highp float;\n\nin vec2 v_uv0;\n#ifdef VERTEX_COLOR\nin vec4 v_color0;\n#endif\n\nuniform vec4 u_color;\nuniform float u_intensity;\n#ifdef HAS_COLOR_MAP\nuniform sampler2D u_colorMap;\n#endif\n#ifdef ALPHA_MASK\nuniform float u_alphaCutoff;\n#endif\n\n{}\n\nout vec4 fragColor;\n\n// Texture.colorSpace selects the GPU format, so sampled color is already linear here.\nvoid main() {{\n  vec4 color = u_color;\n#ifdef VERTEX_COLOR\n  color *= v_color0;\n#endif\n#ifdef HAS_COLOR_MAP\n  vec4 sampled = texture(u_colorMap, v_uv0);\n  color.rgb *= sampled.rgb;\n  color.a *= sampled.a;\n#endif\n#ifdef ALPHA_MASK\n  if (color.a < u_alphaCutoff) discard;\n  color.a = 1.0;\n#endif\n  fragColor = vec4(color.rgb * u_intensity, color.a);\n{}\n}}\n",
        gl_mesh_fragment_tail_uniforms_constant, gl_mesh_fragment_tail_constant
    )
});
