// @generated from upstream/packages/scene3d-gl/src/glMeshProgram.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    ensure_gl_mesh_upload, ensure_gl_skin_palette, get_gl_scene3_d_runtime,
    get_gl_scene3_d_viewport_aspect,
};
use flighthq_camera::get_camera3_d_view_projection_matrix4;
use flighthq_geometry::{create_matrix3, create_matrix4, get_matrix4_position, inverse_matrix4};
use flighthq_render_gl::{create_gl_program, upload_gl_skin_palette_texture};
use flighthq_texture::{get_texture_uv_matrix, has_texture_source, has_texture_uv_transform};
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, Camera3D, CanvasShapeCommand, CanvasTextureResolvers,
    ColorScaleBias, ExternalTexture, GlBitmapShader, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder,
    GlMeshProgram, GlParticleShader, GlQuadBatchShader, GlRenderEffectRunner, GlRenderState,
    GlRenderTextureEntry, GlRenderTextureGuard, GlShaderLocations, GlShapeMeshColorScaleBiasShader,
    GlTextureResolver, GlUniformColorScaleBiasShader, InteractionSignals, Kind, Material, Matrix,
    Matrix3, Matrix4, Matrix4Like, Mesh, MeshGeometry, MeshGeometryGlData, MeshGeometryWgpuData,
    MeshMorphBindPose, MeshSkinBindPose, Node, NodeInteractionState, NodeSignals, NodeTraitsKey,
    Path, PathMesh, Rectangle, RenderEffectPaddingResolver, RenderProxy, RenderProxy2D,
    RenderProxyAdapter, RenderState, RenderTexture, Renderable, Renderer, SamplerLike, Scene2D,
    Scene2DClipHooks, Scene2DSignals, Scene3DGraphSyncPolicy, Scene3DRenderProxy, ShapeRasterizer,
    StrokeStyle, Texture, TextureFilter, TextureLike, TextureSourceKind, TextureWrap,
    TintMaterialData, Vector3,
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

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:25 (sha256:5aa6343ffff2f93b96994d9bb463d2e5890c856585dd82387a31de43b6536009)
pub fn begin_gl_mesh_draw(
    state: &mut GlRenderState,
    program: &GlMeshProgram,
    double_sided: bool,
) -> () {
    let gl = (state.gl).clone();
    get_gl_scene3_d_runtime(state).active_mesh_program = Some((*program).clone());
    crate::host_value::<()>("host.useProgram");
    crate::host_value::<()>("host.enable");
    crate::host_value::<()>("host.depthFunc");
    crate::host_value::<()>("host.depthMask");
    if double_sided {
        crate::host_value::<()>("host.disable");
    } else {
        crate::host_value::<()>("host.enable");
        crate::host_value::<()>("host.cullFace");
    }
}

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:48 (sha256:2ac1acc64f92d278e866fd89a14e9e7d359f4683de4da33da6a2729ee01281cb)
pub fn bind_gl_uv_transform(
    gl: crate::OpaqueHostValue,
    program: &mut GlMeshProgram,
    texture: Option<TextureLike>,
) -> () {
    let mut loc = (program.loc_uv_transform).clone();
    if (loc).is_none() {
        loc = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.getUniformLocation",
        ));
        (*program).clone().loc_uv_transform = (loc).clone();
    }
    if ((loc).is_none()) || ((texture).is_none()) {
        return;
    }
    get_texture_uv_matrix(&mut (*SCRATCH_UV_MATRIX.lock().unwrap()), &texture);
    crate::host_value::<()>("host.uniformMatrix3fv");
}

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:66 (sha256:814105e9040265ec59347e4749f31bd781b5966862113f4ef792d4635416bc30)
pub fn compile_gl_program(
    gl: crate::OpaqueHostValue,
    vertex_source: String,
    fragment_source: String,
) -> crate::OpaqueHostValue {
    return create_gl_program(
        (gl).clone(),
        (vertex_source).clone(),
        (fragment_source).clone(),
        Some(("Mesh".to_owned()).clone()),
    );
}

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:78 (sha256:4091e5800bc256caac7be00044c1b9edfc70d8b7e223b22a6acf3ecaded55391)
pub fn destroy_gl_mesh_program(state: &GlRenderState, program: &GlMeshProgram) -> () {
    crate::host_value::<()>("host.deleteProgram");
}

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:82 (sha256:c8a9815e931757bd6e4b10c327c5d3552a4168533cd91c358b8306de1fb86808)
pub fn draw_gl_mesh_subset(
    state: &mut GlRenderState,
    program: &mut GlMeshProgram,
    proxy: &Scene3DRenderProxy,
    geometry: &mut MeshGeometry,
) -> () {
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.uniformMatrix4fv");
    if ((program.loc_normal_matrix).clone()).is_some() {
        crate::host_value::<()>("host.uniformMatrix3fv");
    }
    upload_gl_mesh_draw_alpha(
        (gl).clone(),
        program,
        (proxy.alpha).unwrap_or(1.0_f64),
        Some(((proxy.material).clone()).clone()),
    );
    let color_matrix = (proxy.color_matrix).clone();
    let color_scale_bias = (proxy.color_scale_bias).clone();
    if (color_matrix).is_some() {
        let mut loc0 = (program.loc_color_matrix0).clone();
        if (loc0).is_none() {
            loc0 = Some(crate::host_value::<crate::OpaqueHostValue>(
                "host.getUniformLocation",
            ));
            (*program).clone().loc_color_matrix0 = (loc0).clone();
            (*program).clone().loc_color_matrix1 = Some(
                crate::host_value::<crate::OpaqueHostValue>("host.getUniformLocation"),
            );
            (*program).clone().loc_color_matrix2 = Some(
                crate::host_value::<crate::OpaqueHostValue>("host.getUniformLocation"),
            );
            (*program).clone().loc_color_matrix3 = Some(
                crate::host_value::<crate::OpaqueHostValue>("host.getUniformLocation"),
            );
            (*program).clone().loc_color_matrix_offset =
                Some(crate::host_value::<crate::OpaqueHostValue>(
                    "host.getUniformLocation",
                ));
        }
        if (loc0).is_some() {
            crate::host_value::<()>("host.uniform4f");
            crate::host_value::<()>("host.uniform4f");
            crate::host_value::<()>("host.uniform4f");
            crate::host_value::<()>("host.uniform4f");
            crate::host_value::<()>("host.uniform4f");
        }
    } else {
        if (color_scale_bias).is_some() {
            let mut loc_color_scale = (program.loc_color_scale).clone();
            let mut loc_color_bias = (program.loc_color_bias).clone();
            if (loc_color_scale).is_none() {
                loc_color_scale = Some(crate::host_value::<crate::OpaqueHostValue>(
                    "host.getUniformLocation",
                ));
                loc_color_bias = Some(crate::host_value::<crate::OpaqueHostValue>(
                    "host.getUniformLocation",
                ));
                (*program).clone().loc_color_scale = (loc_color_scale).clone();
                (*program).clone().loc_color_bias = (loc_color_bias).clone();
            }
            if ((loc_color_scale).is_some()) && ((loc_color_bias).is_some()) {
                crate::host_value::<()>("host.uniform4f");
                crate::host_value::<()>("host.uniform4f");
            }
        }
    }
    let joint_matrices = (proxy.joint_matrices).clone();
    let gpu_skinned =
        (((program.loc_joint_texture).clone()).is_some()) && ((joint_matrices).is_some());
    if gpu_skinned {
        let mut palette = ensure_gl_skin_palette(state);
        crate::host_value::<()>("host.activeTexture");
        upload_gl_skin_palette_texture(
            (gl).clone(),
            &mut palette,
            joint_matrices.as_ref().unwrap(),
            (__flight_js_to_i32(((joint_matrices.as_ref().unwrap().len() as f64) / 16.0_f64))
                | __flight_js_to_i32(0.0_f64)) as f64,
        );
        crate::host_value::<()>("host.uniform1i");
    }
    let upload = ensure_gl_mesh_upload(state, geometry, Some(gpu_skinned));
    if ((upload.index_buffer).clone()).is_some() {
        let element_size = if (upload.index_type == crate::host_value::<f64>("host.UNSIGNED_INT")) {
            4.0_f64
        } else {
            2.0_f64
        };
        crate::host_value::<()>("host.drawElements");
    } else {
        crate::host_value::<()>("host.drawArrays");
    }
}

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:178 (sha256:10501c128ca88bc32127cdbc80ed716e1c15b4bfd82f723be6fd0ea55e7dc003)
pub fn ensure_gl_scene3_d_program<T: Clone>(
    state: &mut GlRenderState,
    key: String,
    compile: &mut impl FnMut(crate::OpaqueHostValue) -> T,
) -> T {
    let mut runtime = get_gl_scene3_d_runtime(state);
    let mut program = runtime
        .program_cache
        .iter()
        .find(|(key, _)| key == &(key).clone())
        .map(|(_, value)| value.clone());
    if (program).is_none() {
        program = Some(compile((state.gl).clone()));
        {
            let __flight_key = (key).clone();
            let __flight_value = (program).clone().unwrap();
            if let Some((_, value)) = runtime
                .program_cache
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                runtime.program_cache.push((__flight_key, __flight_value));
            }
        };
    }
    return (program).clone().unwrap();
}

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:196 (sha256:1716342fb8f589bd4ce1c2b9aeb5bcd3bee64ba088206e973b704206c57e1c60)
pub fn has_gl_uv_transform(texture: Option<TextureLike>) -> bool {
    return (((texture).is_some()) && (has_texture_source(((texture).clone().unwrap()).clone())))
        && (has_texture_uv_transform(&texture));
}

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:203 (sha256:c7f2848423ee466967778637ab6dbc1ac33e021c186e8bb10ba9e99fcb4592fe)
pub fn set_gl_mesh_camera_position(
    gl: crate::OpaqueHostValue,
    loc_camera_position: Option<crate::OpaqueHostValue>,
    camera: &Camera3D,
) -> () {
    inverse_matrix4(&mut (*SCRATCH_INVERSE_VIEW.lock().unwrap()), &{
        let __flight_source = &(camera.view);
        Matrix4Like {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            m: (__flight_source.m).clone(),
        }
    });
    get_matrix4_position(&mut (*SCRATCH_CAMERA_POSITION.lock().unwrap()), &{
        let __flight_source = &(*SCRATCH_INVERSE_VIEW.lock().unwrap());
        Matrix4Like {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            m: (__flight_source.m).clone(),
        }
    });
    crate::host_value::<()>("host.uniform3f");
}

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:216 (sha256:f59fd79a24a1a8838cc6492506a6e0735553767cd468f56855a59a28f955ea04)
pub fn set_gl_mesh_view_projection(
    state: &GlRenderState,
    loc_view_projection: Option<crate::OpaqueHostValue>,
    camera: &Camera3D,
) -> () {
    get_camera3_d_view_projection_matrix4(
        &mut (*SCRATCH_VIEW_PROJECTION.lock().unwrap()),
        camera,
        get_gl_scene3_d_viewport_aspect(state),
    );
    crate::host_value::<()>("host.uniformMatrix4fv");
}

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:237 (sha256:2ed6bedc087c5c19496cdcc083e61c7abc60225390913acaf8d69e3a14892025)
pub fn upload_gl_mesh_draw_alpha(
    gl: crate::OpaqueHostValue,
    program: &mut GlMeshProgram,
    alpha: f64,
    material: Option<Material>,
) -> () {
    let mut location = (program.loc_object_alpha).clone();
    if (location).is_none() {
        location = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.getUniformLocation",
        ));
        (*program).clone().loc_object_alpha = (location).clone();
    }
    if (location).is_some() {
        crate::host_value::<()>("host.uniform1f");
    }
    let mut coverage_location = (program.loc_alpha_is_coverage).clone();
    if (coverage_location).is_none() {
        coverage_location = Some(crate::host_value::<crate::OpaqueHostValue>(
            "host.getUniformLocation",
        ));
        (*program).clone().loc_alpha_is_coverage = (coverage_location).clone();
    }
    if (coverage_location).is_some() {
        crate::host_value::<()>("host.uniform1f");
    }
}

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:264 (sha256:47ea816a7bee0f309a53ed31c9eb650760b598883d64c9ea2cf788c4e050a9dc)
fn is_gl_mesh_alpha_coverage(material: Option<Material>) -> bool {
    return ((material).is_some()) && (((material).clone().unwrap().alpha_mode).clone() == "blend");
}

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:274 (sha256:a6dc7d15a6fa74a62f77ba73d3aea675675b4ca20400d5c772f6bf771f93a39a)
pub const GL_UV_TRANSFORM_VERTEX_GLSL: &'static str = "\n#ifdef HAS_UV_TRANSFORM\nuniform mat3 u_uvTransform;\nvec2 applyUvTransform(vec2 uv) { return (u_uvTransform * vec3(uv, 1.0)).xy; }\n#else\nvec2 applyUvTransform(vec2 uv) { return uv; }\n#endif\n";

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:286 (sha256:bba7e900335a674af9bb9d38ae7eb9fa2e5560ca9ee7bc9fbd4f6f823ca1c390)
pub const SKIN_PALETTE_TEXTURE_UNIT: f64 = 12.0_f64;

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:296 (sha256:a7d9d67ffd8b62f2dbad61a192fa342e425d59fd51cd9bad9da54de0acb14008)
pub const GL_SKIN_VERTEX_DECLARATIONS_GLSL: &'static str = "\nlayout(location = 6) in vec4 a_joints0;\nlayout(location = 7) in vec4 a_weights0;\nuniform highp sampler2D u_jointTexture;\n\nmat4 fetchJointMatrix(int joint) {\n  int x = joint * 4;\n  return mat4(\n    texelFetch(u_jointTexture, ivec2(x, 0), 0),\n    texelFetch(u_jointTexture, ivec2(x + 1, 0), 0),\n    texelFetch(u_jointTexture, ivec2(x + 2, 0), 0),\n    texelFetch(u_jointTexture, ivec2(x + 3, 0), 0)\n  );\n}\n\nmat4 skinMatrix() {\n  return a_weights0.x * fetchJointMatrix(int(a_joints0.x))\n       + a_weights0.y * fetchJointMatrix(int(a_joints0.y))\n       + a_weights0.z * fetchJointMatrix(int(a_joints0.z))\n       + a_weights0.w * fetchJointMatrix(int(a_joints0.w));\n}\n";

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:319 (sha256:140acb0d499b3786d700284ab9e3540997031e7613e61aec3eb90cf6d2ab88c6)
static SCRATCH_VIEW_PROJECTION: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:320 (sha256:cac89509b1c8459b541129553f50270085b134abaef34130e7309e6463eaf999)
static SCRATCH_INVERSE_VIEW: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:321 (sha256:87ef2e4557c2d0eb9483fdfe75d849407b3f25f82a43c851b3a0ff34fb85a5b9)
static SCRATCH_CAMERA_POSITION: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(Vector3 {
            __flight_identity: std::sync::Arc::new(()),
            __flight_entity_runtime: Default::default(),
            x: 0.0_f64,
            y: 0.0_f64,
            z: 0.0_f64,
        })
    });

// Source: upstream/packages/scene3d-gl/src/glMeshProgram.ts:324 (sha256:591628d13476cd1379f3e3321adebd8dde23da6c434a9b73ccb6259319613fd2)
static SCRATCH_UV_MATRIX: std::sync::LazyLock<std::sync::Mutex<Matrix3>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix3(
            None, None, None, None, None, None, None, None, None,
        ))
    });
