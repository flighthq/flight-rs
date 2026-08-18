// @generated from upstream/packages/scene3d-gl/src/glMeshUpload.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_gl_scene3_d_runtime;
use flighthq_mesh::{get_mesh_geometry_morph_bind_pose, get_mesh_geometry_skin_bind_pose};
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, CanvasShapeCommand, CanvasTextureResolvers,
    ColorScaleBias, ExternalTexture, GlBitmapShader, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder,
    GlMeshUpload, GlParticleShader, GlQuadBatchShader, GlRenderEffectRunner, GlRenderState,
    GlRenderTextureEntry, GlRenderTextureGuard, GlShaderLocations, GlShapeMeshColorScaleBiasShader,
    GlTextureResolver, GlUniformColorScaleBiasShader, InteractionSignals, Kind, Material, Matrix,
    Matrix4, MeshGeometry, MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose,
    MeshSkinBindPose, Node, NodeInteractionState, NodeSignals, NodeTraitsKey, Path, PathMesh,
    PrimitiveTopology, Rectangle, RenderEffectPaddingResolver, RenderProxy, RenderProxy2D,
    RenderProxyAdapter, RenderState, RenderTexture, Renderable, Renderer, SamplerLike, Scene2D,
    Scene2DClipHooks, Scene2DSignals, Scene3DGraphSyncPolicy, ShapeRasterizer, StrokeStyle,
    Texture, TextureFilter, TextureSourceKind, TextureWrap, TintMaterialData, VertexAttribute,
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

// Source: upstream/packages/scene3d-gl/src/glMeshUpload.ts:17 (sha256:c9ecc1866cbfa94aea8b79f7100912c81b69948ae077d85b0d888d8bf821c8c3)
pub fn destroy_gl_mesh_upload(state: &GlRenderState, upload: &GlMeshUpload) -> () {
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.deleteVertexArray");
    crate::host_value::<()>("host.deleteBuffer");
    if ((upload.index_buffer).clone()).is_some() {
        crate::host_value::<()>("host.deleteBuffer");
    }
}

// Source: upstream/packages/scene3d-gl/src/glMeshUpload.ts:47 (sha256:38328496aa73e09b19a749f38823d5722ccbc90592d6061a6ce6e180e40c54ce)
pub fn ensure_gl_mesh_upload(
    state: &mut GlRenderState,
    geometry: &mut MeshGeometry,
    gpu_skinned: Option<bool>,
) -> GlMeshUpload {
    let gpu_skinned = gpu_skinned.unwrap_or(false);
    let gl = (state.gl).clone();
    let mut upload = get_gl_scene3_d_runtime(state)
        .upload_cache
        .iter()
        .find(|(key, _)| key == &(*geometry).clone())
        .map(|(_, value)| value.clone());
    let primitive_mode = get_gl_primitive_mode((gl).clone(), (geometry.topology).clone());
    let morphed = (get_mesh_geometry_morph_bind_pose(geometry)).is_some();
    let bind_pose = if (gpu_skinned) && (!morphed) {
        get_mesh_geometry_skin_bind_pose(geometry)
    } else {
        None
    };
    if ((upload).is_some())
        && (if (bind_pose).is_some() {
            (upload.as_mut().unwrap().skin_bind_uploaded) == Some(true)
        } else {
            (upload.as_mut().unwrap().version == geometry.version)
        })
    {
        upload.as_mut().unwrap().primitive_mode = primitive_mode;
        crate::host_value::<()>("host.bindVertexArray");
        return ((upload).clone().unwrap()).clone();
    }
    if (upload).is_none() {
        upload = Some(GlMeshUpload {
            __flight_identity: std::sync::Arc::new(()),
            index_buffer: None,
            index_count: 0.0_f64,
            index_type: crate::host_value::<f64>("host.UNSIGNED_SHORT"),
            primitive_mode: primitive_mode,
            vao: crate::host_value::<crate::OpaqueHostValue>("host.createVertexArray"),
            version: (-1.0_f64),
            vertex_buffer: crate::host_value::<crate::OpaqueHostValue>("host.createBuffer"),
            skin_bind_uploaded: None,
        });
        {
            let __flight_key = (*geometry).clone();
            let __flight_value = (upload).clone().unwrap();
            if let Some((_, value)) = get_gl_scene3_d_runtime(state)
                .upload_cache
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                get_gl_scene3_d_runtime(state)
                    .upload_cache
                    .push((__flight_key, __flight_value));
            }
        };
    }
    crate::host_value::<()>("host.bindVertexArray");
    crate::host_value::<()>("host.bindBuffer");
    crate::host_value::<()>("host.bufferData");
    upload.as_mut().unwrap().skin_bind_uploaded = Some((bind_pose).is_some());
    let stride = geometry.layout.stride;
    {
        let mut i = 0.0_f64;
        while (i < (geometry.layout.attributes.len() as f64)) {
            bind_gl_vertex_attribute(
                (gl).clone(),
                &geometry.layout.attributes[i as usize],
                stride,
            );
            {
                i += 1.0;
                i
            };
        }
    }
    if ((geometry.indices).clone()).is_some() {
        if ((upload.as_mut().unwrap().index_buffer).clone()).is_none() {
            upload.as_mut().unwrap().index_buffer = Some(
                crate::host_value::<crate::OpaqueHostValue>("host.createBuffer"),
            );
        }
        crate::host_value::<()>("host.bindBuffer");
        crate::host_value::<()>("host.bufferData");
        upload.as_mut().unwrap().index_type = if ((geometry.indices).clone()).is_some() {
            crate::host_value::<f64>("host.UNSIGNED_INT")
        } else {
            crate::host_value::<f64>("host.UNSIGNED_SHORT")
        };
        upload.as_mut().unwrap().index_count = (geometry.indices.as_ref().unwrap().len() as f64);
    } else {
        if ((upload.as_mut().unwrap().index_buffer).clone()).is_some() {
            crate::host_value::<()>("host.deleteBuffer");
        }
        upload.as_mut().unwrap().index_buffer = None;
        upload.as_mut().unwrap().index_count = if (geometry.layout.stride > 0.0_f64) {
            (geometry.vertices.byte_length / geometry.layout.stride).floor()
        } else {
            0.0_f64
        };
    }
    upload.as_mut().unwrap().primitive_mode = primitive_mode;
    upload.as_mut().unwrap().version = geometry.version;
    return ((upload).clone().unwrap()).clone();
}

// Source: upstream/packages/scene3d-gl/src/glMeshUpload.ts:117 (sha256:be5b157823863c71dc9fdd29e4d44fd9bb95bca04de0066b71f1e386e193b9b1)
fn get_gl_primitive_mode(gl: crate::OpaqueHostValue, topology: PrimitiveTopology) -> f64 {
    {
        let __switch_value = topology;
        let __flight_case = if __switch_value == "line-list" {
            0_usize
        } else if __switch_value == "line-strip" {
            1_usize
        } else if __switch_value == "point-list" {
            2_usize
        } else if __switch_value == "triangle-strip" {
            3_usize
        } else {
            4_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                return crate::host_value::<f64>("host.LINES");
            }
            if __flight_case <= 1_usize {
                return crate::host_value::<f64>("host.LINE_STRIP");
            }
            if __flight_case <= 2_usize {
                return crate::host_value::<f64>("host.POINTS");
            }
            if __flight_case <= 3_usize {
                return crate::host_value::<f64>("host.TRIANGLE_STRIP");
            }
            if __flight_case <= 4_usize {
                return crate::host_value::<f64>("host.TRIANGLES");
            }
            unreachable!("exhaustive TypeScript switch completed without returning");
        }
    }
}

// Source: upstream/packages/scene3d-gl/src/glMeshUpload.ts:136 (sha256:104e51e50d249371ebecfff168d8d67ef11bb7720fd522d144e4742219343c8c)
fn build_skin_bind_vertices(geometry: &mut MeshGeometry, bind_pose: &MeshSkinBindPose) -> Vec<f32> {
    let mut out = ((geometry.vertices).clone()).clone();
    let floats_per_vertex = (geometry.layout.stride / 4.0_f64);
    let position_offset = float_offset_for_semantic(geometry, "position".to_owned());
    let normal_offset = float_offset_for_semantic(geometry, "normal".to_owned());
    let vertex_count = (__flight_js_to_i32(((bind_pose.positions.len() as f64) / 3.0_f64))
        | __flight_js_to_i32(0.0_f64)) as f64;
    {
        let mut v = 0.0_f64;
        while (v < vertex_count) {
            let base = (v * floats_per_vertex);
            let s = (v * 3.0_f64);
            if (position_offset >= 0.0_f64) {
                out[(base + position_offset) as usize] = (bind_pose.positions[s as usize] as f64);
                out[((base + position_offset) + 1.0_f64) as usize] =
                    (bind_pose.positions[(s + 1.0_f64) as usize] as f64);
                out[((base + position_offset) + 2.0_f64) as usize] =
                    (bind_pose.positions[(s + 2.0_f64) as usize] as f64);
            }
            if (normal_offset >= 0.0_f64) {
                out[(base + normal_offset) as usize] = (bind_pose.normals[s as usize] as f64);
                out[((base + normal_offset) + 1.0_f64) as usize] =
                    (bind_pose.normals[(s + 1.0_f64) as usize] as f64);
                out[((base + normal_offset) + 2.0_f64) as usize] =
                    (bind_pose.normals[(s + 2.0_f64) as usize] as f64);
            }
            {
                v += 1.0;
                v
            };
        }
    }
    return out;
}

// Source: upstream/packages/scene3d-gl/src/glMeshUpload.ts:162 (sha256:e9dcbf0a92065ea35cc1b468448302a94456c845014ed1921205c068c4246a47)
fn float_offset_for_semantic(geometry: &MeshGeometry, semantic: String) -> f64 {
    {
        let mut i = 0.0_f64;
        while (i < (geometry.layout.attributes.len() as f64)) {
            if ((geometry.layout.attributes[i as usize].semantic).clone() == semantic) {
                return (geometry.layout.attributes[i as usize].byte_offset / 4.0_f64);
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return (-1.0_f64);
}

// Source: upstream/packages/scene3d-gl/src/glMeshUpload.ts:176 (sha256:1acb12b0121b279e2d86337798144be860ffbd7b42e1e61c2cfacb92a192519d)
static ATTRIBUTE_LOCATION: std::sync::LazyLock<Vec<(String, f64)>> =
    std::sync::LazyLock::new(|| {
        let mut __flight_record = Vec::new();
        __flight_record.push(("color0".to_owned(), 4.0_f64));
        __flight_record.push(("joints0".to_owned(), 6.0_f64));
        __flight_record.push(("normal".to_owned(), 1.0_f64));
        __flight_record.push(("position".to_owned(), 0.0_f64));
        __flight_record.push(("tangent".to_owned(), 2.0_f64));
        __flight_record.push(("uv0".to_owned(), 3.0_f64));
        __flight_record.push(("uv1".to_owned(), 5.0_f64));
        __flight_record.push(("weights0".to_owned(), 7.0_f64));
        __flight_record
    });

// Source: upstream/packages/scene3d-gl/src/glMeshUpload.ts:187 (sha256:f2d55b26523bc2bce29ed22616706f51a6729456ce2f0ba1a41ef127b4214cd8)
fn bind_gl_vertex_attribute(
    gl: crate::OpaqueHostValue,
    attribute: &VertexAttribute,
    stride: f64,
) -> () {
    let location = ATTRIBUTE_LOCATION
        .iter()
        .find(|(key, _)| key == &(attribute.semantic).clone())
        .map(|(_, value)| value)
        .expect("TypeScript Record key was absent")
        .clone();
    if (location).is_none() {
        return;
    }
    let __destructure1 = resolve_gl_vertex_format((gl).clone(), (attribute.format).clone());
    let size = __destructure1[0.0_f64 as usize].clone();
    let type_ = __destructure1[1.0_f64 as usize].clone();
    let normalized = __destructure1[2.0_f64 as usize].clone();
    crate::host_value::<()>("host.enableVertexAttribArray");
    crate::host_value::<()>("host.vertexAttribPointer");
}

// Source: upstream/packages/scene3d-gl/src/glMeshUpload.ts:201 (sha256:87b501c4dd33dd29f0c0e869e5bf4524e412e034ca9834f0b57897622902338b)
fn resolve_gl_vertex_format(
    gl: crate::OpaqueHostValue,
    format: String,
) -> Vec<crate::FlightUnion2<f64, bool>> {
    {
        let __switch_value = format;
        let __flight_case = if __switch_value == "float32x2" {
            0_usize
        } else if __switch_value == "float32x3" {
            1_usize
        } else if __switch_value == "float32x4" {
            2_usize
        } else if __switch_value == "uint8x4" {
            3_usize
        } else if __switch_value == "unorm8x4" {
            4_usize
        } else if __switch_value == "uint16x4" {
            5_usize
        } else {
            6_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                return vec![
                    crate::FlightUnion2::<f64, bool>::A(2.0_f64),
                    crate::host_value::<crate::FlightUnion2<f64, bool>>("host.FLOAT"),
                    crate::FlightUnion2::<f64, bool>::B(false),
                ];
            }
            if __flight_case <= 1_usize {
                return vec![
                    crate::FlightUnion2::<f64, bool>::A(3.0_f64),
                    crate::host_value::<crate::FlightUnion2<f64, bool>>("host.FLOAT"),
                    crate::FlightUnion2::<f64, bool>::B(false),
                ];
            }
            if __flight_case <= 2_usize {
                return vec![
                    crate::FlightUnion2::<f64, bool>::A(4.0_f64),
                    crate::host_value::<crate::FlightUnion2<f64, bool>>("host.FLOAT"),
                    crate::FlightUnion2::<f64, bool>::B(false),
                ];
            }
            if __flight_case <= 3_usize {
                return vec![
                    crate::FlightUnion2::<f64, bool>::A(4.0_f64),
                    crate::host_value::<crate::FlightUnion2<f64, bool>>("host.UNSIGNED_BYTE"),
                    crate::FlightUnion2::<f64, bool>::B(false),
                ];
            }
            if __flight_case <= 4_usize {
                return vec![
                    crate::FlightUnion2::<f64, bool>::A(4.0_f64),
                    crate::host_value::<crate::FlightUnion2<f64, bool>>("host.UNSIGNED_BYTE"),
                    crate::FlightUnion2::<f64, bool>::B(true),
                ];
            }
            if __flight_case <= 5_usize {
                return vec![
                    crate::FlightUnion2::<f64, bool>::A(4.0_f64),
                    crate::host_value::<crate::FlightUnion2<f64, bool>>("host.UNSIGNED_SHORT"),
                    crate::FlightUnion2::<f64, bool>::B(false),
                ];
            }
            if __flight_case <= 6_usize {
                return vec![
                    crate::FlightUnion2::<f64, bool>::A(3.0_f64),
                    crate::host_value::<crate::FlightUnion2<f64, bool>>("host.FLOAT"),
                    crate::FlightUnion2::<f64, bool>::B(false),
                ];
            }
            unreachable!("exhaustive TypeScript switch completed without returning");
        }
    }
}
