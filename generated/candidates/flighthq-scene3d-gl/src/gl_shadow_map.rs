// @generated from upstream/packages/scene3d-gl/src/glShadowMap.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GL_SKIN_VERTEX_DECLARATIONS_GLSL as gl_skin_vertex_declarations_glsl_constant,
    compile_gl_program, ensure_gl_mesh_upload, ensure_gl_scene3_d_program, ensure_gl_skin_palette,
    get_gl_scene3_d_runtime,
};
use flighthq_camera::{
    get_camera3_d_view_projection_matrix4, get_orthographic_projection_texel_size,
};
use flighthq_geometry::create_matrix4;
use flighthq_mesh::has_mesh_geometry_skin;
use flighthq_node::for_each_node_descendant;
use flighthq_render_gl::{create_gl_render_target, upload_gl_skin_palette_texture};
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, Camera3D, CanvasShapeCommand, CanvasTextureResolvers,
    ColorScaleBias, DIRECTIONAL_SHADOW_MAP_SIZE as directional_shadow_map_size_constant,
    DirectionalLight, ExternalTexture, GlBitmapShader, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder,
    GlMeshProgram, GlParticleShader, GlQuadBatchShader, GlRenderEffectRunner, GlRenderState,
    GlRenderTextureEntry, GlRenderTextureGuard, GlScene3DShadow, GlShaderLocations,
    GlShapeMeshColorScaleBiasShader, GlTextureResolver, GlUniformColorScaleBiasShader,
    InteractionSignals, Kind,
    MAX_DIRECTIONAL_SHADOW_PCF_RADIUS as max_directional_shadow_pcf_radius_constant, Material,
    Matrix, Matrix4, MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose, MeshSkinBindPose,
    Node, Node3D, NodeInteractionState, NodeSignals, NodeTraitsKey, Path, PathMesh, Rectangle,
    RenderEffectPaddingResolver, RenderProxy, RenderProxy2D, RenderProxyAdapter, RenderState,
    RenderTargetDescriptor, RenderTexture, Renderable, Renderer, SamplerLike, Scene2D,
    Scene2DClipHooks, Scene2DSignals, Scene3DGraphSyncPolicy, ShapeRasterizer, StrokeStyle,
    Texture, TextureFilter, TextureSourceKind, TextureWrap, TintMaterialData,
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

// Source: upstream/packages/scene3d-gl/src/glShadowMap.ts:36 (sha256:e24822f2e490d53667d861469b62cce1107612bb34b3a28e3afc0e93a2dc4f90)
pub fn draw_gl_scene3_d_shadow_map(
    mut state: GlRenderState,
    scene: &Node3D,
    shadow_camera: &Camera3D,
    directional_light: Option<DirectionalLight>,
) -> () {
    let gl: std::sync::Arc<std::sync::Mutex<crate::OpaqueHostValue>> =
        std::sync::Arc::new(std::sync::Mutex::new((state.gl).clone()));
    let mut runtime = get_gl_scene3_d_runtime(&mut state);
    let mut previous_shadow = (runtime.shadow).clone();
    if (previous_shadow).is_some() {
        previous_shadow.as_mut().unwrap().enabled = false;
    }
    if ((directional_light).is_none()) || (!directional_light.as_ref().unwrap().casts_shadow) {
        return;
    }
    if (shadow_camera.projection.kind != "orthographic") {
        panic!(
            "{}",
            "drawGlScene3DShadowMap requires an orthographic shadow camera"
        );
    }
    if ((runtime.shadow_target).clone()).is_none() {
        runtime.shadow_target = create_gl_render_target(
            &state,
            &RenderTargetDescriptor {
                __flight_identity: std::sync::Arc::new(()),
                depth: Some("depth-stencil-sampled".to_owned()),
                height: directional_shadow_map_size_constant,
                width: directional_shadow_map_size_constant,
                format: None,
                color_attachments: None,
                color_formats: None,
                sample_count: None,
                color_space: None,
                clear_colors: None,
                clear_depth: None,
            },
            None,
        );
    }
    let target = (runtime.shadow_target).clone();
    let normal_bias_world = (directional_light.as_ref().unwrap().normal_bias
        * get_orthographic_projection_texel_size(
            &shadow_camera.projection,
            target.as_ref().unwrap().width,
            target.as_ref().unwrap().height,
        ));
    let matrix: std::sync::Arc<std::sync::Mutex<Matrix4>> =
        std::sync::Arc::new(std::sync::Mutex::new(
            (previous_shadow.as_ref().map(|value| (value.matrix).clone())).unwrap_or(
                create_matrix4(
                    None, None, None, None, None, None, None, None, None, None, None, None, None,
                    None, None, None,
                ),
            ),
        ));
    get_camera3_d_view_projection_matrix4(&mut (*matrix.lock().unwrap()), shadow_camera, 1.0_f64);
    let rigid_program = ensure_gl_scene3_d_program(
        &mut state,
        "shadow:depth".to_owned(),
        &mut compile_shadow_depth_program,
    );
    let skinned_program: std::sync::Arc<std::sync::Mutex<Option<GlMeshProgram>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    let prev_framebuffer = crate::host_value::<Option<crate::OpaqueHostValue>>("host.getParameter");
    let prev_viewport = crate::host_value::<Vec<i32>>("host.getParameter");
    crate::host_value::<()>("host.bindFramebuffer");
    crate::host_value::<()>("host.viewport");
    crate::host_value::<()>("host.enable");
    crate::host_value::<()>("host.depthFunc");
    crate::host_value::<()>("host.depthMask");
    crate::host_value::<()>("host.enable");
    crate::host_value::<()>("host.cullFace");
    crate::host_value::<()>("host.clear");
    let bound_program: std::sync::Arc<std::sync::Mutex<Option<GlMeshProgram>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    for_each_node_descendant(
        &{
            let __flight_source = &(scene);
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
        },
        &mut |mut node: Node| -> () {
            let mut mesh = node;
            if ((mesh.geometry).clone()).is_none() {
                return;
            }
            let skinned =
                (((mesh.skin).clone()).is_some()) && (has_mesh_geometry_skin(&mesh.geometry));
            let program = if skinned {
                {
                    (*skinned_program.lock().unwrap())?? = Some(ensure_gl_scene3_d_program(
                        &mut state,
                        "shadow:depth:skin".to_owned(),
                        &mut compile_shadow_depth_skinned_program,
                    ));
                    (*skinned_program.lock().unwrap())
                }
            } else {
                (rigid_program).clone()
            };
            if ((program).clone() != (*bound_program.lock().unwrap()).clone()) {
                crate::host_value::<()>("host.useProgram");
                crate::host_value::<()>("host.uniformMatrix4fv");
                (*bound_program.lock().unwrap()) = Some((program).clone());
            }
            crate::host_value::<()>("host.uniformMatrix4fv");
            if skinned {
                crate::host_value::<()>("host.activeTexture");
                upload_gl_skin_palette_texture(
                    (*gl.lock().unwrap()).clone(),
                    &mut ensure_gl_skin_palette(&mut state),
                    &mesh.skin.as_mut().unwrap().skeleton.joint_matrices,
                    (__flight_js_to_i32(
                        ((mesh.skin.as_mut().unwrap().skeleton.joint_matrices.len() as f64)
                            / 16.0_f64),
                    ) | __flight_js_to_i32(0.0_f64)) as f64,
                );
                crate::host_value::<()>("host.uniform1i");
            }
            let upload = ensure_gl_mesh_upload(&mut state, &mut mesh.geometry, Some(skinned));
            crate::host_value::<()>("host.bindVertexArray");
            if ((upload.index_buffer).clone()).is_some() {
                crate::host_value::<()>("host.drawElements");
            } else {
                crate::host_value::<()>("host.drawArrays");
            }
        },
    );
    crate::host_value::<()>("host.activeTexture");
    crate::host_value::<()>("host.bindFramebuffer");
    crate::host_value::<()>("host.viewport");
    crate::host_value::<()>("host.disable");
    crate::host_value::<()>("host.cullFace");
    runtime.shadow = Some(GlScene3DShadow {
        __flight_identity: std::sync::Arc::new(()),
        enabled: true,
        matrix: (*matrix.lock().unwrap()).clone(),
        normal_bias_world: normal_bias_world,
        pcf_radius: normalize_directional_shadow_pcf_radius(
            directional_light.as_ref().unwrap().pcf_radius,
        ),
        shadow_bias: directional_light.as_ref().unwrap().shadow_bias,
        texture: {
            let __flight_portable_source = (target.as_ref().unwrap().depth_texture).clone();
            match (&__flight_portable_source).as_ref() {
                Some(value) => (value).clone(),
                None => crate::FlightValue::Null,
            }
        },
    });
}

// Source: upstream/packages/scene3d-gl/src/glShadowMap.ts:148 (sha256:8f2dfa83e46aaf82dd49793d6b7f43e17ca5afc0ab081e9f0b2cf779da6be2af)
fn normalize_directional_shadow_pcf_radius(radius: f64) -> f64 {
    if (!(radius).is_finite()) {
        return 0.0_f64;
    }
    return (max_directional_shadow_pcf_radius_constant).min((0.0_f64).max((radius).floor()));
}

// Source: upstream/packages/scene3d-gl/src/glShadowMap.ts:153 (sha256:caf241aba7522f1ee4e6972feba6e17a209eb37361e610d4232ba5dd9ec3a893)
fn compile_shadow_depth_program(gl: crate::OpaqueHostValue) -> GlMeshProgram {
    let program = compile_gl_program(
        (gl).clone(),
        (SHADOW_DEPTH_VERTEX).clone(),
        (SHADOW_DEPTH_FRAGMENT).clone(),
    );
    return GlMeshProgram {
        __flight_identity: std::sync::Arc::new(()),
        loc_model: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_normal_matrix: None,
        loc_view_projection: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        program: (program).clone(),
        loc_color_scale: None,
        loc_color_bias: None,
        loc_color_matrix0: None,
        loc_color_matrix1: None,
        loc_color_matrix2: None,
        loc_color_matrix3: None,
        loc_color_matrix_offset: None,
        loc_object_alpha: None,
        loc_alpha_is_coverage: None,
        loc_joint_texture: None,
        loc_uv_transform: None,
    };
}

// Source: upstream/packages/scene3d-gl/src/glShadowMap.ts:166 (sha256:56ff60d588c0d3209cdd1d1b0894f009fe7d12a917aa5985f3c4c9ef1925c48f)
fn compile_shadow_depth_skinned_program(gl: crate::OpaqueHostValue) -> GlMeshProgram {
    let program = compile_gl_program(
        (gl).clone(),
        (SHADOW_DEPTH_SKINNED_VERTEX).clone(),
        (SHADOW_DEPTH_FRAGMENT).clone(),
    );
    return GlMeshProgram {
        __flight_identity: std::sync::Arc::new(()),
        loc_joint_texture: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_model: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_normal_matrix: None,
        loc_view_projection: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        program: (program).clone(),
        loc_color_scale: None,
        loc_color_bias: None,
        loc_color_matrix0: None,
        loc_color_matrix1: None,
        loc_color_matrix2: None,
        loc_color_matrix3: None,
        loc_color_matrix_offset: None,
        loc_object_alpha: None,
        loc_alpha_is_coverage: None,
        loc_uv_transform: None,
    };
}

// Source: upstream/packages/scene3d-gl/src/glShadowMap.ts:177 (sha256:4b908dfae7067667aaf297ec418ff95ffb30aae196acb1afcd6a86d4901354ea)
const SHADOW_DEPTH_VERTEX: &'static str = "#version 300 es\nlayout(location = 0) in vec3 a_position;\nuniform mat4 u_viewProjection;\nuniform mat4 u_model;\nvoid main() {\n  gl_Position = u_viewProjection * u_model * vec4(a_position, 1.0);\n}\n";

// Source: upstream/packages/scene3d-gl/src/glShadowMap.ts:189 (sha256:0e6ef7941f59172549c49098338d5d1c21390e61e831e4272bf63e0960fada04)
static SHADOW_DEPTH_SKINNED_VERTEX: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "#version 300 es\n{}\nlayout(location = 0) in vec3 a_position;\nuniform mat4 u_viewProjection;\nuniform mat4 u_model;\nvoid main() {{\n  gl_Position = u_viewProjection * u_model * skinMatrix() * vec4(a_position, 1.0);\n}}\n",
        gl_skin_vertex_declarations_glsl_constant
    )
});

// Source: upstream/packages/scene3d-gl/src/glShadowMap.ts:199 (sha256:caadb2b83b29a7a394896a8a7eccd789b2e7af0866325b4a1045105ec5bb6d4b)
const SHADOW_DEPTH_FRAGMENT: &'static str = "#version 300 es\nprecision highp float;\nout vec4 fragColor;\nvoid main() {\n  fragColor = vec4(1.0);\n}\n";
