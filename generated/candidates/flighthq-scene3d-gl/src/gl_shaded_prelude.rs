// @generated from upstream/packages/scene3d-gl/src/glShadedPrelude.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GL_MESH_FRAGMENT_TAIL as gl_mesh_fragment_tail_constant,
    GL_MESH_FRAGMENT_TAIL_UNIFORMS as gl_mesh_fragment_tail_uniforms_constant,
    GL_MESH_LIGHT_BLOCK_GLSL as gl_mesh_light_block_glsl_constant,
    GL_SKIN_VERTEX_DECLARATIONS_GLSL as gl_skin_vertex_declarations_glsl_constant,
    GL_UV_TRANSFORM_VERTEX_GLSL as gl_uv_transform_vertex_glsl_constant, compile_gl_program,
    ensure_gl_scene3_d_program, get_gl_scene3_d_runtime, resolve_gl_lit_locations,
};
use flighthq_render_gl::get_gl_render_state_runtime;
use flighthq_shading::{
    create_modifier_registry, get_modifier_define_key, order_modifier_stack, resolve_modifier,
};
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, CanvasShapeCommand, CanvasTextureResolvers,
    ColorScaleBias, ExternalTexture, GlBitmapShader, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder,
    GlParticleShader, GlQuadBatchShader, GlRenderEffectRunner, GlRenderState, GlRenderTextureEntry,
    GlRenderTextureGuard, GlShadedDefineKey, GlShadedProgram, GlShaderLocations,
    GlShapeMeshColorScaleBiasShader, GlTextureResolver, GlUniformColorScaleBiasShader,
    InteractionSignals, Kind, MAX_FORWARD_LIGHTS as max_forward_lights_constant,
    MODIFIER_SLOT as modifier_slot_constant, Material, Matrix, Matrix4, MeshGeometryGlData,
    MeshGeometryWgpuData, MeshMorphBindPose, MeshSkinBindPose, Modifier, ModifierRegistry, Node,
    NodeInteractionState, NodeSignals, NodeTraitsKey, Path, PathMesh, Rectangle,
    RenderEffectPaddingResolver, RenderProxy, RenderProxy2D, RenderProxyAdapter, RenderState,
    RenderTexture, Renderable, Renderer, SamplerLike, Scene2D, Scene2DClipHooks, Scene2DSignals,
    Scene3DGraphSyncPolicy, ShapeRasterizer, StrokeStyle, Texture, TextureFilter,
    TextureSourceKind, TextureWrap, TintMaterialData,
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

// Source: upstream/packages/scene3d-gl/src/glShadedPrelude.ts:33 (sha256:cd6bb685dcba3672b7676a42829e4452599ccd949a701c8726d25d75979dc4e4)
pub fn build_gl_shaded_cache_key(key: &GlShadedDefineKey, modifier_define_key: String) -> String {
    let base = format!(
        "{}{}{}{}{}{}{}",
        if key.alpha_mask_enabled {
            "m".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_diffuse_map {
            "d".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_specular_map {
            "s".to_owned()
        } else {
            "-".to_owned()
        },
        if key.has_normal_map {
            "n".to_owned()
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
        },
        if (key.has_color_matrix).unwrap_or(false) {
            "x".to_owned()
        } else {
            if (key.has_color_adjustment).unwrap_or(false) {
                "c".to_owned()
            } else {
                "".to_owned()
            }
        }
    );
    return format!("shaded:{}|{}", base, modifier_define_key);
}

// Source: upstream/packages/scene3d-gl/src/glShadedPrelude.ts:47 (sha256:c8dec8a54d4a84e375eb65425f4afecdf0a46327a4db5f2523de8a91a4987c13)
pub fn compile_gl_shaded_program(
    gl: crate::OpaqueHostValue,
    key: &GlShadedDefineKey,
    ordered_modifiers: &Vec<Modifier>,
    registry: &ModifierRegistry,
    color_adjustment_feature: Option<GlColorAdjustmentMaterialFeature>,
) -> GlShadedProgram {
    let define_source = build_gl_shaded_define_source(key);
    let vertex_source = ((define_source
        + if (key.has_skin).unwrap_or(false) {
            (gl_skin_vertex_declarations_glsl_constant).to_owned()
        } else {
            "".to_owned()
        })
        + assemble_gl_shaded_vertex_body(ordered_modifiers, registry));
    let mut fragment_body = assemble_gl_shaded_fragment_body(ordered_modifiers, registry);
    if (((key.has_color_adjustment).unwrap_or(false)) || ((key.has_color_matrix).unwrap_or(false)))
        && ((color_adjustment_feature).is_some())
    {
        fragment_body = (fragment_body.replace)(
            "precision highp float;",
            format!(
                "precision highp float;\n{}",
                if (key.has_color_matrix).unwrap_or(false) {
                    (color_adjustment_feature
                        .as_ref()
                        .unwrap()
                        .matrix_fragment_shader_chunk)
                        .clone()
                } else {
                    (color_adjustment_feature
                        .as_ref()
                        .unwrap()
                        .fragment_shader_chunk)
                        .clone()
                }
            ),
        );
    }
    let fragment_source = (define_source + fragment_body);
    let program = compile_gl_program(
        (gl).clone(),
        (vertex_source).clone(),
        (fragment_source).clone(),
    );
    return {
        let __flight_spread_0 = resolve_gl_lit_locations((gl).clone(), (program).clone());
        GlShadedProgram {
            __flight_identity: std::sync::Arc::new(()),
            loc_color_scale: (__flight_spread_0.loc_color_scale).clone(),
            loc_color_bias: (__flight_spread_0.loc_color_bias).clone(),
            loc_color_matrix0: (__flight_spread_0.loc_color_matrix0).clone(),
            loc_color_matrix1: (__flight_spread_0.loc_color_matrix1).clone(),
            loc_color_matrix2: (__flight_spread_0.loc_color_matrix2).clone(),
            loc_color_matrix3: (__flight_spread_0.loc_color_matrix3).clone(),
            loc_color_matrix_offset: (__flight_spread_0.loc_color_matrix_offset).clone(),
            loc_object_alpha: (__flight_spread_0.loc_object_alpha).clone(),
            loc_alpha_is_coverage: (__flight_spread_0.loc_alpha_is_coverage).clone(),
            loc_joint_texture: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_model: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_normal_matrix: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_uv_transform: (__flight_spread_0.loc_uv_transform).clone(),
            loc_view_projection: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            program: (program).clone(),
            loc_ambient_count: (__flight_spread_0.loc_ambient_count).clone(),
            loc_ambient_radiance: (__flight_spread_0.loc_ambient_radiance).clone(),
            loc_camera_position: (__flight_spread_0.loc_camera_position).clone(),
            loc_directional: (__flight_spread_0.loc_directional).clone(),
            loc_directional_count: (__flight_spread_0.loc_directional_count).clone(),
            loc_directional_radiance: (__flight_spread_0.loc_directional_radiance).clone(),
            loc_hemisphere_count: (__flight_spread_0.loc_hemisphere_count).clone(),
            loc_hemisphere_lights: (__flight_spread_0.loc_hemisphere_lights).clone(),
            loc_ibl_brdf: (__flight_spread_0.loc_ibl_brdf).clone(),
            loc_ibl_enabled: (__flight_spread_0.loc_ibl_enabled).clone(),
            loc_ibl_intensity: (__flight_spread_0.loc_ibl_intensity).clone(),
            loc_ibl_irradiance: (__flight_spread_0.loc_ibl_irradiance).clone(),
            loc_ibl_max_mip: (__flight_spread_0.loc_ibl_max_mip).clone(),
            loc_ibl_prefiltered: (__flight_spread_0.loc_ibl_prefiltered).clone(),
            loc_point_count: (__flight_spread_0.loc_point_count).clone(),
            loc_point_lights: (__flight_spread_0.loc_point_lights).clone(),
            loc_shadow_bias: (__flight_spread_0.loc_shadow_bias).clone(),
            loc_shadow_enabled: (__flight_spread_0.loc_shadow_enabled).clone(),
            loc_shadow_map: (__flight_spread_0.loc_shadow_map).clone(),
            loc_shadow_matrix: (__flight_spread_0.loc_shadow_matrix).clone(),
            loc_shadow_normal_bias_world: (__flight_spread_0.loc_shadow_normal_bias_world).clone(),
            loc_shadow_pcf_radius: (__flight_spread_0.loc_shadow_pcf_radius).clone(),
            loc_spot_count: (__flight_spread_0.loc_spot_count).clone(),
            loc_spot_lights: (__flight_spread_0.loc_spot_lights).clone(),
            loc_alpha_cutoff: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_diffuse: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_diffuse_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_normal_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_normal_scale: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_shininess: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_specular: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_specular_map: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
            loc_time: crate::host_value::<Option<crate::OpaqueHostValue>>(
                "host.getUniformLocation",
            ),
        }
    };
}

// Source: upstream/packages/scene3d-gl/src/glShadedPrelude.ts:99 (sha256:9d5f166f0dc11a97138eafbd83638e783d717b584e6da674666e5be78cf7f148)
pub fn ensure_gl_shaded_program(
    mut state: GlRenderState,
    key: &GlShadedDefineKey,
    modifiers: &mut Vec<Modifier>,
) -> GlShadedProgram {
    let registry = ((get_gl_scene3_d_runtime(&mut state).modifier_snippet_registry).clone())
        .unwrap_or(((*EMPTY_MODIFIER_REGISTRY).clone()).clone());
    let ordered = order_modifier_stack(modifiers);
    let full_key: GlShadedDefineKey = GlShadedDefineKey {
        has_color_adjustment: Some(get_gl_scene3_d_runtime(&mut state).active_color_adjustment_run),
        has_color_matrix: Some(get_gl_scene3_d_runtime(&mut state).active_color_matrix_run),
        has_skin: Some(get_gl_scene3_d_runtime(&mut state).active_skinned_run),
        ..((*key).clone()).clone()
    };
    let cache_key = build_gl_shaded_cache_key(
        &full_key,
        get_modifier_define_key(modifiers, Some(((registry).clone()).clone())),
    );
    return {
        let __flight_argument_2 = (std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let full_key = full_key.clone();
            let ordered = ordered.clone();
            let registry = registry.clone();
            let mut state = state.clone();
            move |gl: crate::OpaqueHostValue| -> f64 {
                compile_gl_shaded_program(
                    (gl).clone(),
                    &full_key,
                    &ordered,
                    &registry,
                    ((get_gl_render_state_runtime(&state)
                        .inner
                        .lock()
                        .unwrap()
                        .gl_color_adjustment_material_feature)
                        .clone())
                    .clone(),
                )
            }
        })
            as Box<dyn FnMut(crate::OpaqueHostValue) -> f64 + Send + 'static>)))
        .clone();
        ensure_gl_scene3_d_program(&mut state, (cache_key).clone(), &__flight_argument_2)
    };
}

// Source: upstream/packages/scene3d-gl/src/glShadedPrelude.ts:136 (sha256:77468258dcddb0e2588d22400b53becb88f35abff896bc012bb7cc45bb78b6b0)
fn assemble_gl_shaded_fragment_body(
    ordered_modifiers: &Vec<Modifier>,
    registry: &ModifierRegistry,
) -> String {
    let mut declarations = "";
    let mut normal = "";
    let mut diffuse = "";
    let mut specular = "";
    let mut emissive = "";
    let mut effect = "";
    {
        let mut index = 0.0_f64;
        while (index < (ordered_modifiers.len() as f64)) {
            let modifier = ordered_modifiers[index as usize].clone();
            let snippet = resolve_modifier(registry, (modifier.kind).clone());
            if ((snippet).is_none())
                || ((snippet.as_ref().unwrap().slot).clone() == modifier_slot_constant.vertex)
            {
                {
                    index += 1.0;
                    index
                };
                continue;
            }
            if ((snippet.as_ref().unwrap().declarations).clone()).is_some() {
                declarations += format!("{}\n", {
                    let __flight_callback = snippet
                        .as_ref()
                        .unwrap()
                        .declarations
                        .as_ref()
                        .unwrap()
                        .clone();
                    let __flight_result =
                        __flight_callback.lock().unwrap()((modifier).clone(), index);
                    __flight_result
                });
            }
            let contribution = format!("{}\n", {
                let __flight_callback = (snippet.as_ref().unwrap().contribution).clone();
                let __flight_result = __flight_callback.lock().unwrap()((modifier).clone(), index);
                __flight_result
            });
            if ((snippet.as_ref().unwrap().slot).clone() == modifier_slot_constant.normal) {
                normal += (contribution).clone();
            } else {
                if ((snippet.as_ref().unwrap().slot).clone() == modifier_slot_constant.diffuse) {
                    diffuse += (contribution).clone();
                } else {
                    if ((snippet.as_ref().unwrap().slot).clone() == modifier_slot_constant.specular)
                    {
                        specular += (contribution).clone();
                    } else {
                        if ((snippet.as_ref().unwrap().slot).clone()
                            == modifier_slot_constant.emissive)
                        {
                            emissive += (contribution).clone();
                        } else {
                            if ((snippet.as_ref().unwrap().slot).clone()
                                == modifier_slot_constant.effect)
                            {
                                effect += (contribution).clone();
                            }
                        }
                    }
                }
            }
            {
                index += 1.0;
                index
            };
        }
    }
    return ((((((SHADED_FRAGMENT_TEMPLATE.replace)(
        "//@DECLARATIONS",
        dedupe_gl_shaded_declarations((declarations).clone()),
    )
    .replace)("//@NORMAL", normal)
    .replace)("//@DIFFUSE", diffuse)
    .replace)("//@SPECULAR", specular)
    .replace)("//@EMISSIVE", emissive)
    .replace)("//@EFFECT", effect);
}

// Source: upstream/packages/scene3d-gl/src/glShadedPrelude.ts:172 (sha256:4ec73b9b641d9937109befdedca35dc9c1e42ff43725c0ff351d5f1c089bec31)
fn assemble_gl_shaded_vertex_body(
    ordered_modifiers: &Vec<Modifier>,
    registry: &ModifierRegistry,
) -> String {
    let mut declarations = "";
    let mut vertex = "";
    {
        let mut index = 0.0_f64;
        while (index < (ordered_modifiers.len() as f64)) {
            let modifier = ordered_modifiers[index as usize].clone();
            let snippet = resolve_modifier(registry, (modifier.kind).clone());
            if ((snippet).is_none())
                || ((snippet.as_ref().unwrap().slot).clone() != modifier_slot_constant.vertex)
            {
                {
                    index += 1.0;
                    index
                };
                continue;
            }
            if ((snippet.as_ref().unwrap().declarations).clone()).is_some() {
                declarations += format!("{}\n", {
                    let __flight_callback = snippet
                        .as_ref()
                        .unwrap()
                        .declarations
                        .as_ref()
                        .unwrap()
                        .clone();
                    let __flight_result =
                        __flight_callback.lock().unwrap()((modifier).clone(), index);
                    __flight_result
                });
            }
            vertex += format!("{}\n", {
                let __flight_callback = (snippet.as_ref().unwrap().contribution).clone();
                let __flight_result = __flight_callback.lock().unwrap()((modifier).clone(), index);
                __flight_result
            });
            {
                index += 1.0;
                index
            };
        }
    }
    return ((SHADED_VERTEX_BODY.replace)(
        "//@VERTEX_DECLARATIONS",
        dedupe_gl_shaded_declarations((declarations).clone()),
    )
    .replace)("//@VERTEX", vertex);
}

// Source: upstream/packages/scene3d-gl/src/glShadedPrelude.ts:198 (sha256:afb913e10182b3b772e8fb2f78b0db14f75f099c6ed302f399e0ed9f981a3a7b)
fn dedupe_gl_shaded_declarations(declarations: String) -> String {
    let mut seen: Vec<String> = Vec::new();
    let mut result = "";
    for line in ((declarations)
        .split("\n".to_owned().as_str())
        .map(|part| part.to_owned())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        let trimmed = (line).trim().to_owned();
        let is_shared_declaration = (((trimmed).clone())
            .starts_with(("uniform ".to_owned()).as_str()))
            && (!(regex::RegexBuilder::new("_\\d+\\b")
                .case_insensitive(false)
                .multi_line(false)
                .dot_matches_new_line(false)
                .build()
                .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
            .is_match(&((trimmed).clone())));
        if is_shared_declaration {
            if seen.iter().any(|item| item == &(trimmed).clone()) {
                continue;
            }
            {
                let __flight_value = (trimmed).clone();
                if !seen.contains(&__flight_value) {
                    seen.push(__flight_value);
                }
            };
        }
        result += format!("{}\n", line);
    }
    return result;
}

// Source: upstream/packages/scene3d-gl/src/glShadedPrelude.ts:216 (sha256:d1596c2224d6188e277569c932e3a3b47c7e0e1f8a832f1c8870327bc2b2f700)
fn build_gl_shaded_define_source(key: &GlShadedDefineKey) -> String {
    let mut defines = format!(
        "#version 300 es\n#define MAX_FORWARD_LIGHTS {}\n",
        max_forward_lights_constant
    );
    if key.alpha_mask_enabled {
        defines += "#define ALPHA_MASK\n".to_owned();
    }
    if key.has_diffuse_map {
        defines += "#define HAS_DIFFUSE_MAP\n".to_owned();
    }
    if key.has_specular_map {
        defines += "#define HAS_SPECULAR_MAP\n".to_owned();
    }
    if key.has_normal_map {
        defines += "#define HAS_NORMAL_MAP\n".to_owned();
    }
    if key.has_uv_transform {
        defines += "#define HAS_UV_TRANSFORM\n".to_owned();
    }
    if (key.has_skin).unwrap_or(false) {
        defines += "#define HAS_SKIN\n".to_owned();
    }
    if (key.has_color_matrix).unwrap_or(false) {
        defines += "#define HAS_COLOR_MATRIX\n".to_owned();
    } else {
        if (key.has_color_adjustment).unwrap_or(false) {
            defines += "#define HAS_COLOR_ADJUSTMENT\n".to_owned();
        }
    }
    return defines;
}

// Source: upstream/packages/scene3d-gl/src/glShadedPrelude.ts:229 (sha256:39ed8cccc3fabd132ff381c7237d90811ffa805d62901d71f3fa92c003b4dd3d)
static SHADED_VERTEX_BODY: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "\nlayout(location = 0) in vec3 a_position;\nlayout(location = 1) in vec3 a_normal;\nlayout(location = 2) in vec4 a_tangent;\nlayout(location = 3) in vec2 a_uv0;\n\nuniform mat4 u_viewProjection;\nuniform mat4 u_model;\nuniform mat3 u_normalMatrix;\nuniform float u_time;\n{}\nout vec3 v_worldPosition;\nout vec3 v_normal;\nout vec4 v_tangent;\nout vec2 v_uv0;\n\n//@VERTEX_DECLARATIONS\n\nvoid main() {{\n#ifdef HAS_SKIN\n  mat4 skin = skinMatrix();\n  vec4 localPosition = skin * vec4(a_position, 1.0);\n  vec3 localNormal = mat3(skin) * a_normal;\n  vec3 localTangent = mat3(skin) * a_tangent.xyz;\n#else\n  vec4 localPosition = vec4(a_position, 1.0);\n  vec3 localNormal = a_normal;\n  vec3 localTangent = a_tangent.xyz;\n#endif\n  vec2 vertexUv = a_uv0;\n\n  // Vertex slot: read/write `localPosition` (the pre-model local vertex) and `localNormal`; the\n  // procedural cases scroll by `u_time` and read `vertexUv` (the raw uv, before the uv transform).\n  //@VERTEX\n\n  vec4 worldPosition = u_model * localPosition;\n  v_worldPosition = worldPosition.xyz;\n  v_normal = u_normalMatrix * localNormal;\n  v_tangent = vec4(u_normalMatrix * localTangent, a_tangent.w);\n  v_uv0 = applyUvTransform(a_uv0);\n  gl_Position = u_viewProjection * worldPosition;\n}}\n",
        gl_uv_transform_vertex_glsl_constant
    )
});

// Source: upstream/packages/scene3d-gl/src/glShadedPrelude.ts:278 (sha256:d7541674ef4d375402fe15799e10c3df1d8f5b96ae0d73db63e5eb8d2c24a738)
static SHADED_FRAGMENT_TEMPLATE: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "\nprecision highp float;\n\nin vec3 v_worldPosition;\nin vec3 v_normal;\nin vec4 v_tangent;\nin vec2 v_uv0;\n\nuniform vec4 u_diffuse;\n#ifdef HAS_COLOR_MATRIX\nuniform vec4 u_flightColorMatrix0;\nuniform vec4 u_flightColorMatrix1;\nuniform vec4 u_flightColorMatrix2;\nuniform vec4 u_flightColorMatrix3;\nuniform vec4 u_flightColorMatrixOffset;\n#elif defined(HAS_COLOR_ADJUSTMENT)\nuniform vec4 u_flightColorScale;\nuniform vec4 u_flightColorBias;\n#endif\nuniform vec4 u_specular;\nuniform float u_shininess;\nuniform float u_normalScale;\nuniform float u_alphaCutoff;\nuniform float u_time;\n{}\n\n#ifdef HAS_DIFFUSE_MAP\nuniform sampler2D u_diffuseMap;\n#endif\n#ifdef HAS_SPECULAR_MAP\nuniform sampler2D u_specularMap;\n#endif\n#ifdef HAS_NORMAL_MAP\nuniform sampler2D u_normalMap;\n#endif\n\n{}\n\nout vec4 fragColor;\n\n// Deterministic 2D value noise, declared in the base so any Effect-slot modifier (the procedural\n// dissolve mask) can call it without redeclaring a function — a GLSL compiler drops it when no\n// modifier references it, so a plain ShadedMaterial pays nothing for it.\nfloat shadedHashNoise(vec2 p) {{\n  return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453123);\n}}\nfloat shadedValueNoise(vec2 p) {{\n  vec2 i = floor(p);\n  vec2 f = fract(p);\n  vec2 u = f * f * (3.0 - 2.0 * f);\n  float a = shadedHashNoise(i + vec2(0.0, 0.0));\n  float b = shadedHashNoise(i + vec2(1.0, 0.0));\n  float c = shadedHashNoise(i + vec2(0.0, 1.0));\n  float d = shadedHashNoise(i + vec2(1.0, 1.0));\n  return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);\n}}\n\n// Diffuse + half-vector (BlinnPhong) specular for ONE light. Every light type routes through this so\n// they never fork the shading model — the caller supplies the surface->light direction and the\n// (attenuated) radiance.\nvec3 shadeShadedLight(vec3 normal, vec3 lightDir, vec3 lightColor, vec3 diffuseRgb, vec3 specularColor, float shininess) {{\n  float nDotL = max(dot(normal, lightDir), 0.0);\n  vec3 result = diffuseRgb * nDotL * lightColor;\n  if (nDotL > 0.0) {{\n    vec3 viewDir = normalize(u_cameraPosition - v_worldPosition);\n    vec3 halfVec = normalize(lightDir + viewDir);\n    float specAngle = max(dot(normal, halfVec), 0.0);\n    result += pow(specAngle, max(shininess, 1.0)) * specularColor * lightColor;\n  }}\n  return result;\n}}\n\n//@DECLARATIONS\n\nvoid main() {{\n  vec4 diffuse = u_diffuse;\n#ifdef HAS_DIFFUSE_MAP\n  vec4 sampledDiffuse = texture(u_diffuseMap, v_uv0);\n  diffuse.rgb *= sampledDiffuse.rgb;\n  diffuse.a *= sampledDiffuse.a;\n#endif\n\n  vec3 geometricNormal = normalize(v_normal);\n  if (!gl_FrontFacing) geometricNormal = -geometricNormal;\n  // Gram-Schmidt-reorthogonalize the interpolated tangent against the interpolated normal before\n  // building the TBN: linear interpolation across a triangle leaves v_tangent no longer perpendicular\n  // to v_normal, and skipping this step skews the tangent frame — the normal-map artifact this base\n  // shader previously exhibited. Mirrors the PBR prelude's TBN construction exactly.\n  vec3 tangent = normalize(v_tangent.xyz - geometricNormal * dot(v_tangent.xyz, geometricNormal));\n  vec3 bitangent = cross(geometricNormal, tangent) * v_tangent.w;\n  mat3 tbn = mat3(tangent, bitangent, geometricNormal);\n\n  vec3 normal = geometricNormal;\n#ifdef HAS_NORMAL_MAP\n  vec3 baseTangentNormal = texture(u_normalMap, v_uv0).xyz * 2.0 - 1.0;\n  baseTangentNormal.xy *= u_normalScale;\n  normal = normalize(tbn * baseTangentNormal);\n#endif\n\n  // Normal slot: read/write `normal` (the world-space shading normal). `tbn` maps tangent- to\n  // world-space; `v_uv0` and `u_time` drive scrolling perturbations.\n  //@NORMAL\n\n  vec3 specularColor = u_specular.rgb;\n#ifdef HAS_SPECULAR_MAP\n  specularColor *= texture(u_specularMap, v_uv0).rgb;\n#endif\n  float shininess = u_shininess;\n\n  // Diffuse slot: read/write `diffuse` (vec4 linear albedo + alpha).\n  //@DIFFUSE\n  // Specular slot: read/write `specularColor` (linear) and `shininess`.\n  //@SPECULAR\n\n#ifdef ALPHA_MASK\n  if (diffuse.a < u_alphaCutoff) discard;\n  diffuse.a = 1.0;\n#endif\n\n  vec3 radiance = vec3(0.0);\n\n  // Directional light: -direction is the surface->light vector; modulated by the shared shadow term.\n  if (u_directionalCount > 0.5) {{\n    vec3 lightDir = normalize(-u_directional.xyz);\n    float shadow = sampleDirectionalShadow(v_worldPosition, geometricNormal);\n    radiance += shadeShadedLight(normal, lightDir, u_directionalRadiance.rgb, diffuse.rgb, specularColor, shininess) * shadow;\n  }}\n\n  // Point lights: surface->light direction with a smooth inverse-square range falloff.\n  for (int i = 0; i < MAX_FORWARD_LIGHTS; i++) {{\n    if (i >= u_pointCount) break;\n    vec3 toLight = u_pointLights[i * 2 + 0].xyz - v_worldPosition;\n    float dist2 = dot(toLight, toLight);\n    vec3 lightDir = toLight * inversesqrt(max(dist2, 1e-8));\n    float atten = rangeWindow(dist2, u_pointLights[i * 2 + 1].w) / max(dist2, 1e-4);\n    radiance += shadeShadedLight(normal, lightDir, u_pointLights[i * 2 + 1].rgb * atten, diffuse.rgb, specularColor, shininess);\n  }}\n\n  // Spot lights: point attenuation times a smooth cone falloff between the inner/outer cosines.\n  for (int i = 0; i < MAX_FORWARD_LIGHTS; i++) {{\n    if (i >= u_spotCount) break;\n    vec3 toLight = u_spotLights[i * 4 + 0].xyz - v_worldPosition;\n    float dist2 = dot(toLight, toLight);\n    vec3 lightDir = toLight * inversesqrt(max(dist2, 1e-8));\n    float atten = rangeWindow(dist2, u_spotLights[i * 4 + 1].w) / max(dist2, 1e-4);\n    float cone = smoothstep(u_spotLights[i * 4 + 3].y, u_spotLights[i * 4 + 3].x,\n                            dot(normalize(u_spotLights[i * 4 + 2].xyz), -lightDir));\n    radiance += shadeShadedLight(normal, lightDir, u_spotLights[i * 4 + 1].rgb * atten * cone, diffuse.rgb, specularColor, shininess);\n  }}\n\n  // Ambient term: flat irradiance over the diffuse albedo.\n  if (u_ambientCount > 0.5) {{\n    radiance += diffuse.rgb * u_ambientRadiance;\n  }}\n\n  // Hemisphere fill: sky/ground gradient blended by the normal's vertical component.\n  for (int i = 0; i < MAX_FORWARD_LIGHTS; i++) {{\n    if (i >= u_hemisphereCount) break;\n    float f = 0.5 + 0.5 * dot(normal, u_hemisphereLights[i * 3 + 2].xyz);\n    radiance += mix(u_hemisphereLights[i * 3 + 1].rgb, u_hemisphereLights[i * 3 + 0].rgb, f) * diffuse.rgb;\n  }}\n\n  // Emissive slot: add self-illumination into `emissive` (linear radiance). `normal`, the light\n  // uniforms, and `v_uv0` are available for facing gates and masks.\n  vec3 emissive = vec3(0.0);\n  //@EMISSIVE\n  radiance += emissive;\n\n  // Effect slot: post-process the shaded `radiance` (view-dependent rim, tint, etc). `viewDir` is\n  // the world-space surface->camera direction; `normal` and `v_uv0` are available.\n  vec3 viewDir = normalize(u_cameraPosition - v_worldPosition);\n  //@EFFECT\n\n  fragColor = vec4(radiance, diffuse.a);\n#ifdef HAS_COLOR_MATRIX\n  fragColor = applyFlightColorMatrix(fragColor, u_flightColorMatrix0, u_flightColorMatrix1,\n    u_flightColorMatrix2, u_flightColorMatrix3, u_flightColorMatrixOffset);\n#elif defined(HAS_COLOR_ADJUSTMENT)\n  fragColor = applyFlightColorAdjustment(fragColor, u_flightColorScale, u_flightColorBias);\n#endif\n{}\n}}\n",
        gl_mesh_light_block_glsl_constant,
        gl_mesh_fragment_tail_uniforms_constant,
        gl_mesh_fragment_tail_constant
    )
});

// Source: upstream/packages/scene3d-gl/src/glShadedPrelude.ts:464 (sha256:85c6dd8686f8049af784c7ee2f61bbc86a7eca4e66509f0b668419a9c8b8cd00)
static EMPTY_MODIFIER_REGISTRY: std::sync::LazyLock<ModifierRegistry> =
    std::sync::LazyLock::new(|| create_modifier_registry());
