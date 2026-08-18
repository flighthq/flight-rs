// @generated from upstream/packages/scene3d-gl/src/glWireframeUpload.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ensure_gl_mesh_upload;
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, CanvasShapeCommand, CanvasTextureResolvers,
    ColorScaleBias, ExternalTexture, GlBitmapShader, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder,
    GlParticleShader, GlQuadBatchShader, GlRenderEffectRunner, GlRenderState, GlRenderTextureEntry,
    GlRenderTextureGuard, GlShaderLocations, GlShapeMeshColorScaleBiasShader, GlTextureResolver,
    GlUniformColorScaleBiasShader, GlWireframeUpload, InteractionSignals, Kind, Material, Matrix,
    Matrix4, MeshGeometry, MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose,
    MeshSkinBindPose, Node, NodeInteractionState, NodeSignals, NodeTraitsKey, Path, PathMesh,
    Rectangle, RenderEffectPaddingResolver, RenderProxy, RenderProxy2D, RenderProxyAdapter,
    RenderState, RenderTexture, Renderable, Renderer, SamplerLike, Scene2D, Scene2DClipHooks,
    Scene2DSignals, Scene3DGraphSyncPolicy, ShapeRasterizer, StrokeStyle, Texture, TextureFilter,
    TextureSourceKind, TextureWrap, TintMaterialData, VertexAttribute,
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

// Source: upstream/packages/scene3d-gl/src/glWireframeUpload.ts:15 (sha256:2722692c06a02ad080a04f5bbb764133d119e707e60870ee500aced50ccefd4e)
pub fn destroy_gl_wireframe_upload(state: &GlRenderState, upload: &GlWireframeUpload) -> () {
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.deleteVertexArray");
    crate::host_value::<()>("host.deleteBuffer");
}

// Source: upstream/packages/scene3d-gl/src/glWireframeUpload.ts:21 (sha256:8983eb93142ceb6d43234d10e6fab543bdffee1a21529bc60f11f7397453cc0b)
pub fn ensure_gl_wireframe_upload(
    state: &mut GlRenderState,
    geometry: &mut MeshGeometry,
) -> GlWireframeUpload {
    let gl = (state.gl).clone();
    let mesh_upload = ensure_gl_mesh_upload(state, geometry, None);
    let mut per_state = (*WIREFRAME_UPLOADS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (per_state).is_none() {
        per_state = Some(Vec::new());
        {
            let __flight_key = (*state).clone();
            let __flight_value = (per_state).clone().unwrap();
            if let Some((_, value)) = (*WIREFRAME_UPLOADS.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*WIREFRAME_UPLOADS.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
    }
    let mut upload = per_state
        .as_mut()
        .unwrap()
        .iter()
        .find(|(key, _)| key == &(*geometry).clone())
        .map(|(_, value)| value.clone());
    if ((upload).is_some()) && (upload.as_mut().unwrap().version == geometry.version) {
        crate::host_value::<()>("host.bindVertexArray");
        return ((upload).clone().unwrap()).clone();
    }
    let line_indices = build_line_indices(geometry);
    let index_type = if true {
        crate::host_value::<crate::OpaqueHostValue>("host.UNSIGNED_INT")
    } else {
        crate::host_value::<crate::OpaqueHostValue>("host.UNSIGNED_SHORT")
    };
    if (upload).is_none() {
        upload = Some(GlWireframeUpload {
            __flight_identity: std::sync::Arc::new(()),
            index_type: index_type,
            line_index_buffer: crate::host_value::<crate::OpaqueHostValue>("host.createBuffer"),
            vao: crate::host_value::<crate::OpaqueHostValue>("host.createVertexArray"),
            version: (-1.0_f64),
        });
        {
            let __flight_key = (*geometry).clone();
            let __flight_value = (upload).clone().unwrap();
            if let Some((_, value)) = per_state
                .as_mut()
                .unwrap()
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                per_state
                    .as_mut()
                    .unwrap()
                    .push((__flight_key, __flight_value));
            }
        };
    }
    upload.as_mut().unwrap().index_type = index_type;
    crate::host_value::<()>("host.bindVertexArray");
    crate::host_value::<()>("host.bindBuffer");
    let stride = geometry.layout.stride;
    let position = ((geometry.layout.attributes).clone())
        .iter()
        .find(|value| {
            (|a: VertexAttribute| -> bool { ((a.semantic).clone() == "position") })(
                (*value).clone(),
            )
        })
        .cloned();
    let byte_offset = if (position).is_some() {
        position.as_ref().unwrap().byte_offset
    } else {
        0.0_f64
    };
    crate::host_value::<()>("host.enableVertexAttribArray");
    crate::host_value::<()>("host.vertexAttribPointer");
    crate::host_value::<()>("host.bindBuffer");
    crate::host_value::<()>("host.bufferData");
    upload.as_mut().unwrap().version = geometry.version;
    return ((upload).clone().unwrap()).clone();
}

// Source: upstream/packages/scene3d-gl/src/glWireframeUpload.ts:71 (sha256:209cd37ce18bdfc0cebbd58b3ad7a663bc748f852db73608c929d9bd32a4db14)
fn build_line_indices(geometry: &MeshGeometry) -> Vec<u32> {
    let triangle_indices = (geometry.indices).clone();
    let triangle_count = if (triangle_indices).is_some() {
        ((triangle_indices.as_ref().unwrap().len() as f64) / 3.0_f64).floor()
    } else {
        ((((geometry.vertices.len() as f64) * 4.0_f64) / geometry.layout.stride) / 3.0_f64).floor()
    };
    let line_count = (triangle_count * 6.0_f64);
    let use_uint32 = ((triangle_indices).is_some()) || (line_count > 65535.0_f64);
    let mut lines = if use_uint32 {
        vec![0_u32; (line_count) as usize]
    } else {
        vec![0_u32; (line_count) as usize]
    };
    {
        let mut t = 0.0_f64;
        while (t < triangle_count) {
            let base = (t * 3.0_f64);
            let i0 = if (triangle_indices).is_some() {
                (triangle_indices.as_ref().unwrap()[base as usize] as f64) as u32
            } else {
                (base) as u32
            };
            let i1 = if (triangle_indices).is_some() {
                (triangle_indices.as_ref().unwrap()[(base + 1.0_f64) as usize] as f64) as u32
            } else {
                (base + 1.0_f64) as u32
            };
            let i2 = if (triangle_indices).is_some() {
                (triangle_indices.as_ref().unwrap()[(base + 2.0_f64) as usize] as f64) as u32
            } else {
                (base + 2.0_f64) as u32
            };
            let out = (t * 6.0_f64);
            lines[out as usize] = ((i0).clone()) as u32;
            lines[(out + 1.0_f64) as usize] = ((i1).clone()) as u32;
            lines[(out + 2.0_f64) as usize] = ((i1).clone()) as u32;
            lines[(out + 3.0_f64) as usize] = ((i2).clone()) as u32;
            lines[(out + 4.0_f64) as usize] = ((i2).clone()) as u32;
            lines[(out + 5.0_f64) as usize] = ((i0).clone()) as u32;
            {
                t += 1.0;
                t
            };
        }
    }
    return lines;
}

// Source: upstream/packages/scene3d-gl/src/glWireframeUpload.ts:100 (sha256:d0fd91cc11cfecef0f3cb55b7ea2c9f98c08df683a5fefc5f58dbbe889c19f4d)
static WIREFRAME_UPLOADS: std::sync::LazyLock<
    std::sync::Mutex<Vec<(GlRenderState, Vec<(MeshGeometry, GlWireframeUpload)>)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
