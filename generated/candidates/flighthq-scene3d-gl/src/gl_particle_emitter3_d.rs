// @generated from upstream/packages/scene3d-gl/src/glParticleEmitter3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_gl_scene3_d_viewport_aspect;
use flighthq_node::{get_node_runtime, get_node_world_matrix4};
use flighthq_render::prepare_scene3_d_render;
use flighthq_render_gl::{
    create_gl_program, enable_gl_blend_mode_support, invalidate_gl_render_state_cache,
    resolve_gl_texture,
};
use flighthq_texture::{get_texture_height, get_texture_width, has_texture_source};
use flighthq_types::{
    Adjustment, BLEND_MODE as blend_mode_constant, BoundsNodeAny, Camera3D, CanvasShapeCommand,
    CanvasTextureResolvers, ColorScaleBias, ExternalTexture, GlBitmapShader, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder,
    GlParticleShader, GlQuadBatchShader, GlRenderEffectRunner, GlRenderState, GlRenderTextureEntry,
    GlRenderTextureGuard, GlShaderLocations, GlShapeMeshColorScaleBiasShader, GlTextureResolver,
    GlUniformColorScaleBiasShader, InteractionSignals, Kind, Material, Matrix, Matrix4,
    MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose, MeshSkinBindPose, Node, Node3D,
    NodeAny, NodeInteractionState, NodeSignals, NodeTraitsKey,
    PARTICLE_EMITTER3_D_KIND as particle_emitter3_d_kind_constant, ParticleBlendMode,
    ParticleEmitter3D, Path, PathMesh, Rectangle, RenderEffectPaddingResolver, RenderProxy,
    RenderProxy2D, RenderProxyAdapter, RenderState, RenderTexture, Renderable, Renderer,
    SamplerLike, Scene2D, Scene2DClipHooks, Scene2DSignals, Scene3DGraphSyncPolicy,
    Scene3DLightsLike, ShapeRasterizer, StrokeStyle, Texture, TextureFilter, TextureSourceKind,
    TextureWrap, TintMaterialData, Transform3DNode,
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

// Source: upstream/packages/scene3d-gl/src/glParticleEmitter3D.ts:42 (sha256:a898f2879e75da7211abb2adcf948b45613b4b61c68abe25c29f2ba8f13f4516)
const INSTANCE_FLOATS: f64 = 16.0_f64;

// Source: upstream/packages/scene3d-gl/src/glParticleEmitter3D.ts:43 (sha256:158d26939b3bb3d19a69d96db2f96a052f8eab461f3875a14559b034ea66c0ee)
const INSTANCE_STRIDE: f64 = 64.0_f64;

// Source: upstream/packages/scene3d-gl/src/glParticleEmitter3D.ts:45 (sha256:c3dc807b578ac94141dd73c6a0532f43a0d50d26a1aa7884792f64f943d23ca6)
const PARTICLE_TRANSFORM_STRIDE: f64 = 4.0_f64;

// Source: upstream/packages/scene3d-gl/src/glParticleEmitter3D.ts:47 (sha256:059ebc1576bb2473834543adbb26c0fe4f68d4cfef2b872b7a23c9de2c0bcde9)
const PARTICLE_3_D_VS: &'static str = "#version 300 es\nprecision highp float;\n\nin vec2 a_corner;\n\nlayout(location = 1) in vec3  a_pos;\nlayout(location = 2) in float a_cosScale;\nlayout(location = 3) in float a_sinScale;\nlayout(location = 4) in vec4  a_color;\nlayout(location = 5) in vec4  a_uvRect;\nlayout(location = 6) in vec2  a_size;\n\nuniform mat4 u_viewProjection;\nuniform vec3 u_cameraRight;\nuniform vec3 u_cameraUp;\n\nout vec2 v_uv;\nout vec4 v_color;\n\nvoid main() {\n  float lx = (a_corner.x - 0.5) * a_size.x;\n  float ly = (a_corner.y - 0.5) * a_size.y;\n  float rx = a_cosScale * lx - a_sinScale * ly;\n  float ry = a_sinScale * lx + a_cosScale * ly;\n  vec3 worldPos = a_pos + u_cameraRight * rx + u_cameraUp * ry;\n  gl_Position = u_viewProjection * vec4(worldPos, 1.0);\n  v_uv    = mix(a_uvRect.xy, a_uvRect.zw, a_corner);\n  v_color = a_color;\n}";

// Source: upstream/packages/scene3d-gl/src/glParticleEmitter3D.ts:77 (sha256:c60e9b558e5c30c70781173a1ed1c07bd18c4b24ba07341f1c3a2c52c2b33915)
const PARTICLE_3_D_FS: &'static str = "#version 300 es\nprecision highp float;\n\nin vec2 v_uv;\nin vec4 v_color;\n\nuniform sampler2D u_texture;\nuniform int u_hasTexture;\n\nout vec4 fragColor;\n\n// Both branches output premultiplied alpha (rgb already scaled by alpha), matching the codebase-wide\n// premultiplied convention the blend funcs in applyGlParticleBlendMode assume. The texture is uploaded\n// premultiplied by bindGlTexture, so tex.rgb is pre-scaled by tex.a; the trailing * v_color.a then\n// premultiplies the tint alpha. The untextured branch premultiplies v_color explicitly.\nvoid main() {\n  if (u_hasTexture != 0) {\n    vec4 tex = texture(u_texture, v_uv);\n    fragColor = vec4(tex.rgb * v_color.rgb, tex.a) * v_color.a;\n  } else {\n    fragColor = vec4(v_color.rgb * v_color.a, v_color.a);\n  }\n  if (fragColor.a <= 0.0) discard;\n}";

// Source: upstream/packages/scene3d-gl/src/glParticleEmitter3D.ts:102 (sha256:1f1edf1f5e30b1269007f163aba3c771575edb436bb6733d32491bc4da3cd063)
#[derive(Clone, Default)]
struct GlParticle3DShader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub corner_buffer: crate::OpaqueHostValue,
    pub index_buffer: crate::OpaqueHostValue,
    pub instance_buffer: crate::OpaqueHostValue,
    pub instance_data: Vec<f32>,
    pub loc_camera_right: crate::OpaqueHostValue,
    pub loc_camera_up: crate::OpaqueHostValue,
    pub loc_color: f64,
    pub loc_corner: f64,
    pub loc_cos_scale: f64,
    pub loc_has_texture: crate::OpaqueHostValue,
    pub loc_pos: f64,
    pub loc_sin_scale: f64,
    pub loc_size: f64,
    pub loc_texture: crate::OpaqueHostValue,
    pub loc_uv_rect: f64,
    pub loc_view_projection: crate::OpaqueHostValue,
    pub program: crate::OpaqueHostValue,
    pub vao: crate::OpaqueHostValue,
}
impl PartialEq for GlParticle3DShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene3d-gl/src/glParticleEmitter3D.ts:123 (sha256:dbeea52a8c70f1befe2ba665999be22458f05d63bc6d0e91daca8c11741335ec)
fn compile_particle3_d_shader(gl: crate::OpaqueHostValue) -> GlParticle3DShader {
    let program = create_gl_program(
        (gl).clone(),
        (PARTICLE_3_D_VS).clone(),
        (PARTICLE_3_D_FS).clone(),
        Some(("ParticleEmitter3D".to_owned()).clone()),
    );
    let vao = crate::host_value::<()>("host.createVertexArray");
    crate::host_value::<()>("host.bindVertexArray");
    let corner_data: Vec<f32> = (vec![
        0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 1.0_f64, 1.0_f64, 0.0_f64, 1.0_f64,
    ])
    .iter()
    .map(|value| (*value) as f32)
    .collect();
    let corner_buffer = crate::host_value::<()>("host.createBuffer");
    crate::host_value::<()>("host.bindBuffer");
    crate::host_value::<()>("host.bufferData");
    let index_data: Vec<u16> = (vec![0.0_f64, 1.0_f64, 2.0_f64, 0.0_f64, 2.0_f64, 3.0_f64])
        .iter()
        .map(|value| (*value) as u16)
        .collect();
    let index_buffer = crate::host_value::<()>("host.createBuffer");
    crate::host_value::<()>("host.bindBuffer");
    crate::host_value::<()>("host.bufferData");
    let instance_buffer = crate::host_value::<()>("host.createBuffer");
    crate::host_value::<()>("host.bindVertexArray");
    return GlParticle3DShader {
        __flight_identity: std::sync::Arc::new(()),
        corner_buffer: (corner_buffer).clone(),
        index_buffer: (index_buffer).clone(),
        instance_buffer: (instance_buffer).clone(),
        instance_data: vec![0.0_f32; (0.0_f64) as usize],
        loc_camera_right: crate::host_value::<crate::OpaqueHostValue>("host.getUniformLocation"),
        loc_camera_up: crate::host_value::<crate::OpaqueHostValue>("host.getUniformLocation"),
        loc_color: 4.0_f64,
        loc_corner: crate::host_value::<f64>("host.getAttribLocation"),
        loc_cos_scale: 2.0_f64,
        loc_has_texture: crate::host_value::<crate::OpaqueHostValue>("host.getUniformLocation"),
        loc_pos: 1.0_f64,
        loc_sin_scale: 3.0_f64,
        loc_size: 6.0_f64,
        loc_texture: crate::host_value::<crate::OpaqueHostValue>("host.getUniformLocation"),
        loc_uv_rect: 5.0_f64,
        loc_view_projection: crate::host_value::<crate::OpaqueHostValue>("host.getUniformLocation"),
        program: (program).clone(),
        vao: (vao).clone(),
    };
}

// Source: upstream/packages/scene3d-gl/src/glParticleEmitter3D.ts:169 (sha256:b200551ac2857dd49680502cac2b7718f95d2649cb37dd60dc3563a42ab345d8)
fn ensure_particle3_d_shader(state: &GlRenderState) -> GlParticle3DShader {
    let mut shader = (*SHADER_CACHE.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if ((shader).clone()).is_some() {
        return ((shader.as_mut().unwrap()).clone()).clone();
    }
    shader = Some(compile_particle3_d_shader((state.gl).clone()));
    {
        let __flight_key = (*state).clone();
        let __flight_value = ((shader).clone()).clone().unwrap();
        if let Some((_, value)) = (*SHADER_CACHE.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*SHADER_CACHE.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    return ((shader).clone().unwrap()).clone();
}

// Source: upstream/packages/scene3d-gl/src/glParticleEmitter3D.ts:177 (sha256:44c4c263ba931e144774f4307b2e234ca9ea8762ec213c8ea6e8dc9e41c479e3)
fn ensure_instance_capacity(
    shader: &mut GlParticle3DShader,
    gl: crate::OpaqueHostValue,
    count: f64,
) -> () {
    let needed = (count * INSTANCE_FLOATS);
    if ((shader.instance_data.len() as f64) >= needed) {
        return;
    }
    let new_size = (needed).max(((shader.instance_data.len() as f64) * 2.0_f64));
    shader.instance_data = vec![0.0_f32; (new_size) as usize];
    crate::host_value::<()>("host.bindBuffer");
    crate::host_value::<()>("host.bufferData");
}

// Source: upstream/packages/scene3d-gl/src/glParticleEmitter3D.ts:186 (sha256:fc4e6d87b88c59699e6605ad8776109903079db2ac9efb3e69412aa7d25329a5)
fn collect_particle_emitter3_d_nodes(node: &NodeAny, out: &mut Vec<ParticleEmitter3D>) -> () {
    if (!node.enabled) {
        return;
    }
    if ((node.kind).clone() == particle_emitter3_d_kind_constant) {
        out.push(node);
    }
    let children = {
        let __flight_slot = get_node_runtime(&{
            let __flight_source = &(node);
            Node {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                data: (__flight_source.data).clone(),
                enabled: __flight_source.enabled,
                kind: (__flight_source.kind).clone(),
                name: (__flight_source.name).clone(),
            }
        })
        .__flight_generic_slot::<crate::NodeRuntimeStorage<Traits>>();
        let __flight_storage = __flight_slot.lock().unwrap();
        (__flight_storage.children).clone()
    };
    if (children).is_some() {
        {
            let mut i = 0.0_f64;
            while (i < (children.as_ref().unwrap().len() as f64)) {
                collect_particle_emitter3_d_nodes(
                    &{
                        let __flight_source = &(children.as_ref().unwrap()[i as usize]);
                        NodeAny {
                            __flight_identity: std::sync::Arc::clone(
                                &__flight_source.__flight_identity,
                            ),
                            __flight_entity_runtime: std::sync::Arc::clone(
                                &__flight_source.__flight_entity_runtime,
                            ),
                            data: (__flight_source.data).clone(),
                            enabled: __flight_source.enabled,
                            kind: (__flight_source.kind).clone(),
                            name: (__flight_source.name).clone(),
                        }
                    },
                    out,
                );
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
}

// Source: upstream/packages/scene3d-gl/src/glParticleEmitter3D.ts:202 (sha256:6c7ea481d1df8bb14283ee2e78386d831f5d9205e26e38e890e6d618f1962851)
fn apply_gl_particle_blend_mode(state: &mut GlRenderState, mode: ParticleBlendMode) -> () {
    if ((state.apply_blend_mode).clone()).is_none() {
        enable_gl_blend_mode_support(state);
    }
    {
        let __switch_value = mode;
        let __flight_case = if __switch_value == "add" {
            0_usize
        } else if __switch_value == "multiply" {
            1_usize
        } else if __switch_value == "screen" {
            2_usize
        } else {
            3_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                {
                    let __flight_callback = state.apply_blend_mode.as_ref().unwrap().clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        (*state).clone(),
                        Some((blend_mode_constant.add).clone()),
                    );
                    __flight_result
                };
                return;
            }
            if __flight_case <= 1_usize {
                {
                    let __flight_callback = state.apply_blend_mode.as_ref().unwrap().clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        (*state).clone(),
                        Some((blend_mode_constant.multiply).clone()),
                    );
                    __flight_result
                };
                return;
            }
            if __flight_case <= 2_usize {
                {
                    let __flight_callback = state.apply_blend_mode.as_ref().unwrap().clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        (*state).clone(),
                        Some((blend_mode_constant.screen).clone()),
                    );
                    __flight_result
                };
                return;
            }
            if __flight_case <= 3_usize {
                {
                    let __flight_callback = state.apply_blend_mode.as_ref().unwrap().clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        (*state).clone(),
                        Some((blend_mode_constant.normal).clone()),
                    );
                    __flight_result
                };
            }
        }
    }
}

// Source: upstream/packages/scene3d-gl/src/glParticleEmitter3D.ts:219 (sha256:e8c6d2cee99da33dd98b194c94a36ec63a1042e3d31c92ed739fa34029a4a079)
fn draw_particle_emitter3_d_node(
    state: &GlRenderState,
    shader: &mut GlParticle3DShader,
    emitter: &ParticleEmitter3D,
) -> () {
    let gl = (state.gl).clone();
    let atlas = (emitter.data.atlas).clone();
    let particle_count = emitter.data.particle_count;
    if (particle_count == 0.0_f64) {
        return;
    }
    ensure_instance_capacity(shader, (gl).clone(), particle_count);
    let atlas_texture = atlas.as_ref().and_then(|value| (value.texture).clone());
    let resolved_atlas = if (((atlas_texture).clone()).is_some())
        && (has_texture_source((((atlas_texture).clone()).clone().unwrap()).clone()))
    {
        resolve_gl_texture(
            state,
            (((atlas_texture).clone()).clone().unwrap()).clone(),
            Some(true),
            None,
        )
    } else {
        None
    };
    let has_atlas = (resolved_atlas).is_some();
    let regions = if has_atlas {
        Some((atlas.as_ref().unwrap().regions).clone())
    } else {
        None
    };
    let num_regions = if (regions).is_some() {
        (regions.as_ref().unwrap().len() as f64)
    } else {
        0.0_f64
    };
    let iw = if has_atlas {
        (1.0_f64
            / (1.0_f64).max(get_texture_width(
                (((atlas_texture).clone()).clone().unwrap()).clone(),
            )))
    } else {
        0.0_f64
    };
    let ih = if has_atlas {
        (1.0_f64
            / (1.0_f64).max(get_texture_height(
                ((atlas_texture).clone().unwrap()).clone(),
            )))
    } else {
        0.0_f64
    };
    let world_matrix = {
        let __flight_source = &(get_node_world_matrix4(&{
            let __flight_source = &(emitter);
            Transform3DNode {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                data: (__flight_source.data).clone(),
                enabled: __flight_source.enabled,
                kind: (__flight_source.kind).clone(),
                name: (__flight_source.name).clone(),
                position: (__flight_source.position).clone(),
                rotation: (__flight_source.rotation).clone(),
                scale: (__flight_source.scale).clone(),
            }
        }));
        Matrix4 {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            m: (__flight_source.m).clone(),
        }
    };
    let world_space = emitter.data.world_space;
    let mut base = 0.0_f64;
    let mut draw_count = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < particle_count) {
            let tt = (i * PARTICLE_TRANSFORM_STRIDE);
            let lx = (emitter.data.transforms[tt as usize] as f64);
            let ly = (emitter.data.transforms[(tt + 1.0_f64) as usize] as f64);
            let rotation = (emitter.data.transforms[(tt + 2.0_f64) as usize] as f64);
            let scale = (emitter.data.transforms[(tt + 3.0_f64) as usize] as f64);
            let lz = (emitter.data.positions_z[i as usize] as f64);
            let wx = if world_space {
                ((lx).clone()) as f32
            } else {
                (((((world_matrix.m[0.0_f64 as usize] as f64) * (lx).clone())
                    + ((world_matrix.m[4.0_f64 as usize] as f64) * (ly).clone()))
                    + ((world_matrix.m[8.0_f64 as usize] as f64) * (lz).clone()))
                    + (world_matrix.m[12.0_f64 as usize] as f64)) as f32
            };
            let wy = if world_space {
                ((ly).clone()) as f32
            } else {
                (((((world_matrix.m[1.0_f64 as usize] as f64) * (lx).clone())
                    + ((world_matrix.m[5.0_f64 as usize] as f64) * (ly).clone()))
                    + ((world_matrix.m[9.0_f64 as usize] as f64) * (lz).clone()))
                    + (world_matrix.m[13.0_f64 as usize] as f64)) as f32
            };
            let wz = if world_space {
                ((lz).clone()) as f32
            } else {
                (((((world_matrix.m[2.0_f64 as usize] as f64) * (lx).clone())
                    + ((world_matrix.m[6.0_f64 as usize] as f64) * (ly).clone()))
                    + ((world_matrix.m[10.0_f64 as usize] as f64) * (lz).clone()))
                    + (world_matrix.m[14.0_f64 as usize] as f64)) as f32
            };
            let cos_r = (((rotation).clone()).cos() * scale);
            let sin_r = (((rotation).clone()).sin() * scale);
            let ct = (i * 3.0_f64);
            let has_colors = ((emitter.data.colors).is_some())
                && ((emitter.data.colors.len() as f64) > (ct + 2.0_f64));
            let r = if has_colors {
                (emitter.data.colors[ct as usize] as f64) as f32
            } else {
                (1.0_f64) as f32
            };
            let g = if has_colors {
                (emitter.data.colors[(ct + 1.0_f64) as usize] as f64) as f32
            } else {
                (1.0_f64) as f32
            };
            let b = if has_colors {
                (emitter.data.colors[(ct + 2.0_f64) as usize] as f64) as f32
            } else {
                (1.0_f64) as f32
            };
            let mut u0 = 0.0_f64;
            let mut v0 = 0.0_f64;
            let mut u1 = 1.0_f64;
            let mut v1 = 1.0_f64;
            let mut region_w = 1.0_f64;
            let mut region_h = 1.0_f64;
            if (regions).is_some() {
                let id = (emitter.data.ids[i as usize] as f64);
                if (id >= num_regions) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                let region = regions.as_ref().unwrap()[id as usize].clone();
                if (region.width <= 0.0_f64) || (region.height <= 0.0_f64) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                u0 = (region.x * iw);
                v0 = (region.y * ih);
                u1 = ((region.x + region.width) * iw);
                v1 = ((region.y + region.height) * ih);
                region_w = region.width;
                region_h = region.height;
            }
            shader.instance_data[base as usize] = ((wx).clone()) as f32;
            shader.instance_data[(base + 1.0_f64) as usize] = ((wy).clone()) as f32;
            shader.instance_data[(base + 2.0_f64) as usize] = ((wz).clone()) as f32;
            shader.instance_data[(base + 3.0_f64) as usize] = (cos_r) as f32;
            shader.instance_data[(base + 4.0_f64) as usize] = (sin_r) as f32;
            shader.instance_data[(base + 5.0_f64) as usize] = ((r).clone()) as f32;
            shader.instance_data[(base + 6.0_f64) as usize] = ((g).clone()) as f32;
            shader.instance_data[(base + 7.0_f64) as usize] = ((b).clone()) as f32;
            shader.instance_data[(base + 8.0_f64) as usize] =
                (emitter.data.alphas[i as usize] as f64) as f32;
            shader.instance_data[(base + 9.0_f64) as usize] = (u0) as f32;
            shader.instance_data[(base + 10.0_f64) as usize] = (v0) as f32;
            shader.instance_data[(base + 11.0_f64) as usize] = (u1) as f32;
            shader.instance_data[(base + 12.0_f64) as usize] = (v1) as f32;
            let max_dim = if (region_w >= region_h) {
                region_w
            } else {
                region_h
            };
            shader.instance_data[(base + 13.0_f64) as usize] = (region_w / max_dim) as f32;
            shader.instance_data[(base + 14.0_f64) as usize] = (region_h / max_dim) as f32;
            shader.instance_data[(base + 15.0_f64) as usize] = (0.0_f64) as f32;
            base += INSTANCE_FLOATS;
            {
                draw_count += 1.0;
                draw_count
            };
            {
                i += 1.0;
                i
            };
        }
    }
    if (draw_count == 0.0_f64) {
        return;
    }
    crate::host_value::<()>("host.bindVertexArray");
    crate::host_value::<()>("host.bindBuffer");
    crate::host_value::<()>("host.bufferSubData");
    crate::host_value::<()>("host.uniform1i");
    if has_atlas {
        crate::host_value::<()>("host.activeTexture");
        crate::host_value::<()>("host.bindTexture");
        crate::host_value::<()>("host.uniform1i");
    }
    crate::host_value::<()>("host.bindBuffer");
    crate::host_value::<()>("host.enableVertexAttribArray");
    crate::host_value::<()>("host.vertexAttribPointer");
    crate::host_value::<()>("host.vertexAttribDivisor");
    crate::host_value::<()>("host.bindBuffer");
    crate::host_value::<()>("host.enableVertexAttribArray");
    crate::host_value::<()>("host.vertexAttribPointer");
    crate::host_value::<()>("host.vertexAttribDivisor");
    crate::host_value::<()>("host.enableVertexAttribArray");
    crate::host_value::<()>("host.vertexAttribPointer");
    crate::host_value::<()>("host.vertexAttribDivisor");
    crate::host_value::<()>("host.enableVertexAttribArray");
    crate::host_value::<()>("host.vertexAttribPointer");
    crate::host_value::<()>("host.vertexAttribDivisor");
    crate::host_value::<()>("host.enableVertexAttribArray");
    crate::host_value::<()>("host.vertexAttribPointer");
    crate::host_value::<()>("host.vertexAttribDivisor");
    crate::host_value::<()>("host.enableVertexAttribArray");
    crate::host_value::<()>("host.vertexAttribPointer");
    crate::host_value::<()>("host.vertexAttribDivisor");
    crate::host_value::<()>("host.enableVertexAttribArray");
    crate::host_value::<()>("host.vertexAttribPointer");
    crate::host_value::<()>("host.vertexAttribDivisor");
    crate::host_value::<()>("host.bindBuffer");
    crate::host_value::<()>("host.drawElementsInstanced");
    crate::host_value::<()>("host.bindVertexArray");
}

// Source: upstream/packages/scene3d-gl/src/glParticleEmitter3D.ts:375 (sha256:f0f254e098bcce94c5f96531b4cf8a5267ba0524bcf3f4b066a2fae00b570a97)
pub fn destroy_gl_particle_emitter3_d_shader(state: &GlRenderState) -> () {
    let shader = (*SHADER_CACHE.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (shader).is_none() {
        return;
    }
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.deleteProgram");
    crate::host_value::<()>("host.deleteBuffer");
    crate::host_value::<()>("host.deleteBuffer");
    crate::host_value::<()>("host.deleteBuffer");
    {
        let __flight_key = (*state).clone();
        if let Some(__flight_index) = (*SHADER_CACHE.lock().unwrap())
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            (*SHADER_CACHE.lock().unwrap()).remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/scene3d-gl/src/glParticleEmitter3D.ts:386 (sha256:c51309d5cabfef842c24f1958be9f9c2cdf175328d97348ff770e9935df6cd73)
pub fn draw_gl_scene3_d_particle_emitter3_ds(
    state: &mut GlRenderState,
    scene: &mut Node3D,
    camera: &Camera3D,
    lights: &Scene3DLightsLike,
) -> () {
    EMITTER_SCRATCH.lock().unwrap().clear();
    collect_particle_emitter3_d_nodes(
        &{
            let __flight_source = &(scene);
            NodeAny {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                data: (__flight_source.data).clone(),
                enabled: __flight_source.enabled,
                kind: (__flight_source.kind).clone(),
                name: (__flight_source.name).clone(),
            }
        },
        &mut EMITTER_SCRATCH,
    );
    if ((EMITTER_SCRATCH.lock().unwrap().len() as f64) == 0.0_f64) {
        return;
    }
    let list = prepare_scene3_d_render(
        &{
            let __flight_source = &(state);
            RenderState {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                allow_smoothing: __flight_source.allow_smoothing,
                background_color: __flight_source.background_color,
                background_color_rgba: (__flight_source.background_color_rgba).clone(),
                background_color_string: (__flight_source.background_color_string).clone(),
                current_clip_depth: __flight_source.current_clip_depth,
                display_object_clip_hooks: (__flight_source.display_object_clip_hooks).clone(),
                pixel_ratio: __flight_source.pixel_ratio,
                render_alpha: __flight_source.render_alpha,
                render_blend_mode: (__flight_source.render_blend_mode).clone(),
                render_transform2_d: (__flight_source.render_transform2_d).clone(),
                scene_graph_sync_policy: (__flight_source.scene_graph_sync_policy).clone(),
                round_pixels: __flight_source.round_pixels,
            }
        },
        scene,
        camera,
        lights,
        Some(get_gl_scene3_d_viewport_aspect(state)),
    );
    let mut shader = ensure_particle3_d_shader(state);
    let gl = (state.gl).clone();
    crate::host_value::<()>("host.useProgram");
    crate::host_value::<()>("host.uniformMatrix4fv");
    crate::host_value::<()>("host.uniform3f");
    crate::host_value::<()>("host.uniform3f");
    crate::host_value::<()>("host.enable");
    crate::host_value::<()>("host.depthMask");
    crate::host_value::<()>("host.enable");
    {
        let mut i = 0.0_f64;
        while (i < (EMITTER_SCRATCH.lock().unwrap().len() as f64)) {
            let emitter = EMITTER_SCRATCH[i as usize].clone();
            apply_gl_particle_blend_mode(state, (emitter.blend_mode).clone());
            draw_particle_emitter3_d_node(state, &mut shader, &emitter);
            {
                i += 1.0;
                i
            };
        }
    }
    crate::host_value::<()>("host.depthMask");
    crate::host_value::<()>("host.disable");
    invalidate_gl_render_state_cache(state);
}

// Source: upstream/packages/scene3d-gl/src/glParticleEmitter3D.ts:428 (sha256:8dd70c86c3ccb149585022e65d72b53233f93fe2903f5bdb3f0f61dc264c3d74)
static EMITTER_SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<ParticleEmitter3D>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));

// Source: upstream/packages/scene3d-gl/src/glParticleEmitter3D.ts:429 (sha256:46d570ec549d7e0373ab15be52ef938b0549421fb109f3158f21131692a2fab8)
static SHADER_CACHE: std::sync::LazyLock<
    std::sync::Mutex<Vec<(GlRenderState, GlParticle3DShader)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
