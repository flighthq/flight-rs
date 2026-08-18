// @generated from upstream/packages/scene3d-gl/src/glShadedBuiltInModifiers.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::register_gl_modifier_snippet;
use flighthq_color::unpack_color_to_linear;
use flighthq_render_gl::resolve_gl_texture;
use flighthq_shading::{
    ANIMATED_NORMAL_MODIFIER_DEFINITION as animated_normal_modifier_definition_constant,
    DISSOLVE_MODIFIER_DEFINITION as dissolve_modifier_definition_constant,
    EMISSIVE_MODIFIER_DEFINITION as emissive_modifier_definition_constant,
    ENV_REFLECT_MODIFIER_DEFINITION as env_reflect_modifier_definition_constant,
    FOG_MODIFIER_DEFINITION as fog_modifier_definition_constant,
    RIM_MODIFIER_DEFINITION as rim_modifier_definition_constant,
    TOON_MODIFIER_DEFINITION as toon_modifier_definition_constant,
    VERTEX_DISPLACE_MODIFIER_DEFINITION as vertex_displace_modifier_definition_constant,
};
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, CanvasShapeCommand, CanvasTextureResolvers,
    ColorScaleBias, DissolveModifier,
    EMISSIVE_MODIFIER_FACING as emissive_modifier_facing_constant, EmissiveModifier,
    EnvReflectModifier, ExternalTexture, FOG_MODIFIER_MODE as fog_modifier_mode_constant,
    FogModifier, GlBitmapShader, GlBlendRealization, GlColorAdjustmentMaterialFeature,
    GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder, GlModifierBindContext,
    GlModifierSnippet, GlParticleShader, GlQuadBatchShader, GlRenderEffectRunner, GlRenderState,
    GlRenderTextureEntry, GlRenderTextureGuard, GlShaderLocations, GlShapeMeshColorScaleBiasShader,
    GlTextureResolver, GlUniformColorScaleBiasShader, InteractionSignals, Kind, LinearColor,
    Material, Matrix, Matrix4, MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose,
    MeshSkinBindPose, Modifier, Node, NodeInteractionState, NodeSignals, NodeTraitsKey, Path,
    PathMesh, Rectangle, RenderEffectPaddingResolver, RenderProxy, RenderProxy2D,
    RenderProxyAdapter, RenderState, RenderTexture, Renderable, Renderer, RimModifier, SamplerLike,
    Scene2D, Scene2DClipHooks, Scene2DSignals, Scene3DGraphSyncPolicy, ShapeRasterizer,
    StrokeStyle, Texture, TextureFilter, TextureSourceKind, TextureWrap, TintMaterialData,
    ToonModifier, VERTEX_DISPLACE_MODIFIER_SOURCE as vertex_displace_modifier_source_constant,
    Vector3Like, VertexDisplaceModifier,
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

// Source: upstream/packages/scene3d-gl/src/glShadedBuiltInModifiers.ts:45 (sha256:62421877eead4b7aedce116ff8bab476bc3546c5dbe39f44522a8a54aaf20c28)
pub static ANIMATED_NORMAL_GL_MODIFIER_SNIPPET: std::sync::LazyLock<GlModifierSnippet> =
    std::sync::LazyLock::new(|| GlModifierSnippet {
        bind: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, context: GlModifierBindContext| -> () {
                let animated = modifier;
                if ((animated.map).clone()).is_none() {
                    return;
                }
                let gl = (context.state.gl).clone();
                let suffix = format!("_{}", context.index);
                bind_gl_modifier_texture(
                    &context,
                    animated.map.as_ref().unwrap(),
                    format!("u_animNormalMap{}", suffix),
                );
                crate::host_value::<()>("host.uniform2f");
                crate::host_value::<()>("host.uniform1f");
                if ((animated.secondary_map).clone()).is_some() {
                    bind_gl_modifier_texture(
                        &context,
                        animated.secondary_map.as_ref().unwrap(),
                        format!("u_animNormalMap2{}", suffix),
                    );
                    let secondary =
                        ((animated.secondary_scroll).clone()).unwrap_or((animated.scroll).clone());
                    crate::host_value::<()>("host.uniform2f");
                }
            },
        )
            as Box<
                dyn FnMut(Modifier, GlModifierBindContext) -> () + Send + 'static,
            >))),
        contribution: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, index: f64| -> String {
                let animated = modifier;
                if ((animated.map).clone()).is_none() {
                    return "".to_owned();
                }
                let suffix = format!("_{}", index);
                let dual = if ((animated.secondary_map).clone()).is_some() {
                    format!(
                        "  animNormal += texture(u_animNormalMap2{}, v_uv0 + u_animNormalScroll2{} * u_time).xyz * 2.0 - 1.0;\n",
                        suffix, suffix
                    )
                } else {
                    "".to_owned()
                };
                return ((((("{\n"
                    + format!(
                        "  vec3 animNormal = texture(u_animNormalMap{}, v_uv0 + u_animNormalScroll{} * u_time).xyz * 2.0 - 1.0;\n",
                        suffix, suffix
                    ))
                    + dual)
                    + format!("  animNormal.xy *= u_animNormalStrength{};\n", suffix))
                    + "  normal = normalize(tbn * animNormal);\n")
                    + "}");
            },
        )
            as Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>)),
        declarations: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, index: f64| -> String {
                let animated = modifier;
                if ((animated.map).clone()).is_none() {
                    return "".to_owned();
                }
                let suffix = format!("_{}", index);
                let mut source = ((format!("uniform sampler2D u_animNormalMap{};\n", suffix)
                    + format!("uniform vec2 u_animNormalScroll{};\n", suffix))
                    + format!("uniform float u_animNormalStrength{};\n", suffix));
                if ((animated.secondary_map).clone()).is_some() {
                    source += format!(
                        "uniform sampler2D u_animNormalMap2{};\nuniform vec2 u_animNormalScroll2{};\n",
                        suffix, suffix
                    );
                }
                return source;
            },
        )
            as Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>))),
        ..(animated_normal_modifier_definition_constant).clone()
    });

// Source: upstream/packages/scene3d-gl/src/glShadedBuiltInModifiers.ts:97 (sha256:c7349a00a1c1316de6cef3a75d477b0bbf4c950f52a03058b3ca45901c800014)
pub static EMISSIVE_GL_MODIFIER_SNIPPET: std::sync::LazyLock<GlModifierSnippet> =
    std::sync::LazyLock::new(|| GlModifierSnippet {
        bind: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, context: GlModifierBindContext| -> () {
                let emissive = {
                    let __flight_source = &((modifier).clone());
                    EmissiveModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        steps: __flight_source.steps,
                        smoothness: __flight_source.smoothness,
                        color: __flight_source.color,
                        power: __flight_source.power,
                        intensity: __flight_source.intensity,
                        bias: __flight_source.bias,
                        tint: __flight_source.tint,
                        fresnel_bias: __flight_source.fresnel_bias,
                        roughness: __flight_source.roughness,
                        threshold: __flight_source.threshold,
                        edge_color: __flight_source.edge_color,
                        edge_width: __flight_source.edge_width,
                        scale: __flight_source.scale,
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        ..Default::default()
                    }
                };
                let gl = (context.state.gl).clone();
                let suffix = format!("_{}", context.index);
                unpack_color_to_linear(&mut SCRATCH_RGBA, emissive.color);
                crate::host_value::<()>("host.uniform3f");
                crate::host_value::<()>("host.uniform1f");
                if ((emissive.mask).clone()).is_some() {
                    bind_gl_modifier_texture(
                        &context,
                        emissive.mask.as_ref().unwrap(),
                        format!("u_emissiveMask{}", suffix),
                    );
                }
                if is_emissive_gated(&emissive) {
                    let sign = if ((emissive.facing).clone()
                        == emissive_modifier_facing_constant.away_from_light)
                    {
                        (-1.0_f64)
                    } else {
                        1.0_f64
                    };
                    crate::host_value::<()>("host.uniform1f");
                    crate::host_value::<()>("host.uniform1f");
                }
            },
        )
            as Box<
                dyn FnMut(Modifier, GlModifierBindContext) -> () + Send + 'static,
            >))),
        contribution: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, index: f64| -> String {
                let emissive = {
                    let __flight_source = &((modifier).clone());
                    EmissiveModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        steps: __flight_source.steps,
                        smoothness: __flight_source.smoothness,
                        color: __flight_source.color,
                        power: __flight_source.power,
                        intensity: __flight_source.intensity,
                        bias: __flight_source.bias,
                        tint: __flight_source.tint,
                        fresnel_bias: __flight_source.fresnel_bias,
                        roughness: __flight_source.roughness,
                        threshold: __flight_source.threshold,
                        edge_color: __flight_source.edge_color,
                        edge_width: __flight_source.edge_width,
                        scale: __flight_source.scale,
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        ..Default::default()
                    }
                };
                let suffix = format!("_{}", index);
                let mut body = ("{\n"
                    + format!(
                        "  vec3 emissiveTerm = u_emissiveColor{} * u_emissiveStrength{};\n",
                        suffix, suffix
                    ));
                if ((emissive.mask).clone()).is_some() {
                    body += format!(
                        "  emissiveTerm *= texture(u_emissiveMask{}, v_uv0).rgb;\n",
                        suffix
                    );
                }
                if is_emissive_gated(&emissive) {
                    body += ((("  vec3 emissiveLightDir = u_directionalCount > 0.5 ? normalize(-u_directional.xyz) : vec3(0.0, 0.0, 1.0);\n"
                        + format!(
                            "  float emissiveFacing = dot(normal, emissiveLightDir) * u_emissiveFacingSign{};\n",
                            suffix
                        ))
                        + format!(
                            "  float emissiveSoft = max(u_emissiveFacingSoftness{}, 1e-4);\n",
                            suffix
                        ))
                        + "  emissiveTerm *= smoothstep(-emissiveSoft, emissiveSoft, emissiveFacing);\n");
                }
                body += "  emissive += emissiveTerm;\n}".to_owned();
                return body;
            },
        )
            as Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>)),
        declarations: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, index: f64| -> String {
                let emissive = {
                    let __flight_source = &((modifier).clone());
                    EmissiveModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        steps: __flight_source.steps,
                        smoothness: __flight_source.smoothness,
                        color: __flight_source.color,
                        power: __flight_source.power,
                        intensity: __flight_source.intensity,
                        bias: __flight_source.bias,
                        tint: __flight_source.tint,
                        fresnel_bias: __flight_source.fresnel_bias,
                        roughness: __flight_source.roughness,
                        threshold: __flight_source.threshold,
                        edge_color: __flight_source.edge_color,
                        edge_width: __flight_source.edge_width,
                        scale: __flight_source.scale,
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        ..Default::default()
                    }
                };
                let suffix = format!("_{}", index);
                let mut source = format!(
                    "uniform vec3 u_emissiveColor{};\nuniform float u_emissiveStrength{};\n",
                    suffix, suffix
                );
                if ((emissive.mask).clone()).is_some() {
                    source += format!("uniform sampler2D u_emissiveMask{};\n", suffix);
                }
                if is_emissive_gated(&emissive) {
                    source += format!(
                        "uniform float u_emissiveFacingSign{};\nuniform float u_emissiveFacingSoftness{};\n",
                        suffix, suffix
                    );
                }
                return source;
            },
        )
            as Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>))),
        ..(emissive_modifier_definition_constant).clone()
    });

// Source: upstream/packages/scene3d-gl/src/glShadedBuiltInModifiers.ts:150 (sha256:bf97e93f3407d863b44546eaeba70ddc7477506a974bffaa4e6b4bf4dd643da1)
pub static RIM_GL_MODIFIER_SNIPPET: std::sync::LazyLock<GlModifierSnippet> =
    std::sync::LazyLock::new(|| GlModifierSnippet {
        bind: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, context: GlModifierBindContext| -> () {
                let rim = {
                    let __flight_source = &((modifier).clone());
                    RimModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        steps: __flight_source.steps,
                        smoothness: __flight_source.smoothness,
                        color: __flight_source.color,
                        power: __flight_source.power,
                        intensity: __flight_source.intensity,
                        bias: __flight_source.bias,
                        tint: __flight_source.tint,
                        fresnel_bias: __flight_source.fresnel_bias,
                        roughness: __flight_source.roughness,
                        threshold: __flight_source.threshold,
                        edge_color: __flight_source.edge_color,
                        edge_width: __flight_source.edge_width,
                        scale: __flight_source.scale,
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        ..Default::default()
                    }
                };
                let gl = (context.state.gl).clone();
                let suffix = format!("_{}", context.index);
                unpack_color_to_linear(&mut SCRATCH_RGBA, rim.color);
                crate::host_value::<()>("host.uniform3f");
                crate::host_value::<()>("host.uniform1f");
                crate::host_value::<()>("host.uniform1f");
                crate::host_value::<()>("host.uniform1f");
            },
        )
            as Box<
                dyn FnMut(Modifier, GlModifierBindContext) -> () + Send + 'static,
            >))),
        contribution: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |_modifier: Modifier, index: f64| -> String {
                let suffix = format!("_{}", index);
                return ((("{\n"
                    + format!(
                        "  float rim = clamp(u_rimBias{} + u_rimIntensity{} * pow(1.0 - max(dot(normal, viewDir), 0.0), max(u_rimPower{}, 0.0001)), 0.0, 1.0);\n",
                        suffix, suffix, suffix
                    ))
                    + format!("  radiance += u_rimColor{} * rim;\n", suffix))
                    + "}");
            },
        )
            as Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>)),
        declarations: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |_modifier: Modifier, index: f64| -> String {
                let suffix = format!("_{}", index);
                return (((format!("uniform vec3 u_rimColor{};\n", suffix)
                    + format!("uniform float u_rimPower{};\n", suffix))
                    + format!("uniform float u_rimIntensity{};\n", suffix))
                    + format!("uniform float u_rimBias{};\n", suffix));
            },
        )
            as Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>))),
        ..(rim_modifier_definition_constant).clone()
    });

// Source: upstream/packages/scene3d-gl/src/glShadedBuiltInModifiers.ts:189 (sha256:78b5f29d30363f500e07435d22b4e3e64a9aa8bdfc2450031479097cebc1f682)
pub static DISSOLVE_GL_MODIFIER_SNIPPET: std::sync::LazyLock<GlModifierSnippet> =
    std::sync::LazyLock::new(|| GlModifierSnippet {
        bind: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, context: GlModifierBindContext| -> () {
                let dissolve = {
                    let __flight_source = &((modifier).clone());
                    DissolveModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        steps: __flight_source.steps,
                        smoothness: __flight_source.smoothness,
                        color: __flight_source.color,
                        power: __flight_source.power,
                        intensity: __flight_source.intensity,
                        bias: __flight_source.bias,
                        tint: __flight_source.tint,
                        fresnel_bias: __flight_source.fresnel_bias,
                        roughness: __flight_source.roughness,
                        threshold: __flight_source.threshold,
                        edge_color: __flight_source.edge_color,
                        edge_width: __flight_source.edge_width,
                        scale: __flight_source.scale,
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        map: None,
                        ..Default::default()
                    }
                };
                let gl = (context.state.gl).clone();
                let suffix = format!("_{}", context.index);
                crate::host_value::<()>("host.uniform1f");
                crate::host_value::<()>("host.uniform1f");
                unpack_color_to_linear(&mut SCRATCH_RGBA, dissolve.edge_color);
                crate::host_value::<()>("host.uniform3f");
                if ((dissolve.map).clone()).is_some() {
                    bind_gl_modifier_texture(
                        &context,
                        dissolve.map.as_ref().unwrap(),
                        format!("u_dissolveMap{}", suffix),
                    );
                } else {
                    crate::host_value::<()>("host.uniform1f");
                }
            },
        )
            as Box<
                dyn FnMut(Modifier, GlModifierBindContext) -> () + Send + 'static,
            >))),
        contribution: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, index: f64| -> String {
                let dissolve = {
                    let __flight_source = &((modifier).clone());
                    DissolveModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        steps: __flight_source.steps,
                        smoothness: __flight_source.smoothness,
                        color: __flight_source.color,
                        power: __flight_source.power,
                        intensity: __flight_source.intensity,
                        bias: __flight_source.bias,
                        tint: __flight_source.tint,
                        fresnel_bias: __flight_source.fresnel_bias,
                        roughness: __flight_source.roughness,
                        threshold: __flight_source.threshold,
                        edge_color: __flight_source.edge_color,
                        edge_width: __flight_source.edge_width,
                        scale: __flight_source.scale,
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        map: None,
                        ..Default::default()
                    }
                };
                let suffix = format!("_{}", index);
                let noise = if ((dissolve.map).clone()).is_some() {
                    format!(
                        "  float dissolveNoise = texture(u_dissolveMap{}, v_uv0).r;\n",
                        suffix
                    )
                } else {
                    format!(
                        "  float dissolveNoise = shadedValueNoise(v_uv0 * u_dissolveScale{});\n",
                        suffix
                    )
                };
                return ((((("{\n" + noise)
                    + format!(
                        "  if (dissolveNoise < u_dissolveThreshold{}) discard;\n",
                        suffix
                    ))
                    + format!(
                        "  float dissolveEdge = 1.0 - smoothstep(u_dissolveThreshold{}, u_dissolveThreshold{} + max(u_dissolveEdgeWidth{}, 1e-4), dissolveNoise);\n",
                        suffix, suffix, suffix
                    ))
                    + format!(
                        "  radiance = mix(radiance, u_dissolveEdgeColor{}, dissolveEdge);\n",
                        suffix
                    ))
                    + "}");
            },
        )
            as Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>)),
        declarations: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, index: f64| -> String {
                let dissolve = {
                    let __flight_source = &((modifier).clone());
                    DissolveModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        steps: __flight_source.steps,
                        smoothness: __flight_source.smoothness,
                        color: __flight_source.color,
                        power: __flight_source.power,
                        intensity: __flight_source.intensity,
                        bias: __flight_source.bias,
                        tint: __flight_source.tint,
                        fresnel_bias: __flight_source.fresnel_bias,
                        roughness: __flight_source.roughness,
                        threshold: __flight_source.threshold,
                        edge_color: __flight_source.edge_color,
                        edge_width: __flight_source.edge_width,
                        scale: __flight_source.scale,
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        map: None,
                        ..Default::default()
                    }
                };
                let suffix = format!("_{}", index);
                let mut source = ((format!("uniform float u_dissolveThreshold{};\n", suffix)
                    + format!("uniform float u_dissolveEdgeWidth{};\n", suffix))
                    + format!("uniform vec3 u_dissolveEdgeColor{};\n", suffix));
                source += if ((dissolve.map).clone()).is_some() {
                    format!("uniform sampler2D u_dissolveMap{};\n", suffix)
                } else {
                    format!("uniform float u_dissolveScale{};\n", suffix)
                };
                return source;
            },
        )
            as Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>))),
        ..(dissolve_modifier_definition_constant).clone()
    });

// Source: upstream/packages/scene3d-gl/src/glShadedBuiltInModifiers.ts:243 (sha256:36934c7fe6bd7012444a302cfb935b9a30bf937ab63f628f4589772fc25b835f)
pub static ENV_REFLECT_GL_MODIFIER_SNIPPET: std::sync::LazyLock<GlModifierSnippet> =
    std::sync::LazyLock::new(|| GlModifierSnippet {
        bind: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, context: GlModifierBindContext| -> () {
                let reflect = {
                    let __flight_source = &((modifier).clone());
                    EnvReflectModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        steps: __flight_source.steps,
                        smoothness: __flight_source.smoothness,
                        color: __flight_source.color,
                        power: __flight_source.power,
                        intensity: __flight_source.intensity,
                        bias: __flight_source.bias,
                        tint: __flight_source.tint,
                        fresnel_bias: __flight_source.fresnel_bias,
                        roughness: __flight_source.roughness,
                        threshold: __flight_source.threshold,
                        edge_color: __flight_source.edge_color,
                        edge_width: __flight_source.edge_width,
                        scale: __flight_source.scale,
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        ..Default::default()
                    }
                };
                let gl = (context.state.gl).clone();
                let suffix = format!("_{}", context.index);
                unpack_color_to_linear(&mut SCRATCH_RGBA, reflect.tint);
                crate::host_value::<()>("host.uniform3f");
                crate::host_value::<()>("host.uniform1f");
                crate::host_value::<()>("host.uniform1f");
                crate::host_value::<()>("host.uniform1f");
            },
        )
            as Box<
                dyn FnMut(Modifier, GlModifierBindContext) -> () + Send + 'static,
            >))),
        contribution: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |_modifier: Modifier, index: f64| -> String {
                let suffix = format!("_{}", index);
                return (((((("{\n" + "  vec3 envReflectDir = reflect(-viewDir, normal);\n")
                    + format!(
                        "  float envReflectMip = clamp(u_envReflectRoughness{}, 0.0, 1.0) * max(u_iblMaxMip, 0.0);\n",
                        suffix
                    ))
                    + format!(
                        "  vec3 envReflectSample = u_iblEnabled > 0.5 ? textureLod(u_iblPrefiltered, envReflectDir, envReflectMip).rgb : u_envReflectTint{};\n",
                        suffix
                    ))
                    + format!(
                        "  float envReflectFresnel = u_envReflectFresnel{} + (1.0 - u_envReflectFresnel{}) * pow(1.0 - max(dot(normal, viewDir), 0.0), 5.0);\n",
                        suffix, suffix
                    ))
                    + format!(
                        "  radiance += envReflectSample * u_envReflectTint{} * (u_envReflectIntensity{} * envReflectFresnel);\n",
                        suffix, suffix
                    ))
                    + "}");
            },
        )
            as Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>)),
        declarations: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |_modifier: Modifier, index: f64| -> String {
                let suffix = format!("_{}", index);
                return (((((("uniform samplerCube u_iblPrefiltered;\n"
                    + "uniform float u_iblEnabled;\n")
                    + "uniform float u_iblMaxMip;\n")
                    + format!("uniform vec3 u_envReflectTint{};\n", suffix))
                    + format!("uniform float u_envReflectIntensity{};\n", suffix))
                    + format!("uniform float u_envReflectFresnel{};\n", suffix))
                    + format!("uniform float u_envReflectRoughness{};\n", suffix));
            },
        )
            as Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>))),
        ..(env_reflect_modifier_definition_constant).clone()
    });

// Source: upstream/packages/scene3d-gl/src/glShadedBuiltInModifiers.ts:288 (sha256:d97e641e6fcb1b349aab33378020a67cee834ddfa9c93bc65a7a2ead722c8a97)
pub static FOG_GL_MODIFIER_SNIPPET: std::sync::LazyLock<GlModifierSnippet> =
    std::sync::LazyLock::new(|| GlModifierSnippet {
        bind: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, context: GlModifierBindContext| -> () {
                let fog = {
                    let __flight_source = &((modifier).clone());
                    FogModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        steps: __flight_source.steps,
                        smoothness: __flight_source.smoothness,
                        color: __flight_source.color,
                        power: __flight_source.power,
                        intensity: __flight_source.intensity,
                        bias: __flight_source.bias,
                        tint: __flight_source.tint,
                        fresnel_bias: __flight_source.fresnel_bias,
                        roughness: __flight_source.roughness,
                        threshold: __flight_source.threshold,
                        edge_color: __flight_source.edge_color,
                        edge_width: __flight_source.edge_width,
                        scale: __flight_source.scale,
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        ..Default::default()
                    }
                };
                let gl = (context.state.gl).clone();
                let suffix = format!("_{}", context.index);
                unpack_color_to_linear(&mut SCRATCH_RGBA, fog.color);
                crate::host_value::<()>("host.uniform3f");
                if (((fog.mode).clone()).is_none())
                    || ((fog.mode).clone() == fog_modifier_mode_constant.linear)
                {
                    crate::host_value::<()>("host.uniform1f");
                    crate::host_value::<()>("host.uniform1f");
                } else {
                    crate::host_value::<()>("host.uniform1f");
                }
            },
        )
            as Box<
                dyn FnMut(Modifier, GlModifierBindContext) -> () + Send + 'static,
            >))),
        contribution: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, index: f64| -> String {
                let fog = {
                    let __flight_source = &((modifier).clone());
                    FogModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        steps: __flight_source.steps,
                        smoothness: __flight_source.smoothness,
                        color: __flight_source.color,
                        power: __flight_source.power,
                        intensity: __flight_source.intensity,
                        bias: __flight_source.bias,
                        tint: __flight_source.tint,
                        fresnel_bias: __flight_source.fresnel_bias,
                        roughness: __flight_source.roughness,
                        threshold: __flight_source.threshold,
                        edge_color: __flight_source.edge_color,
                        edge_width: __flight_source.edge_width,
                        scale: __flight_source.scale,
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        ..Default::default()
                    }
                };
                let suffix = format!("_{}", index);
                let mut factor: String;
                if ((fog.mode).clone() == fog_modifier_mode_constant.exponential) {
                    factor = format!(
                        "  float fogFactor = 1.0 - exp(-u_fogDensity{} * fogDist);\n",
                        suffix
                    );
                } else {
                    if ((fog.mode).clone() == fog_modifier_mode_constant.exponential2) {
                        factor = format!(
                            "  float fogTerm = u_fogDensity{} * fogDist;\n  float fogFactor = 1.0 - exp(-fogTerm * fogTerm);\n",
                            suffix
                        );
                    } else {
                        factor = format!(
                            "  float fogFactor = clamp((fogDist - u_fogNear{}) / max(u_fogFar{} - u_fogNear{}, 1e-4), 0.0, 1.0);\n",
                            suffix, suffix, suffix
                        );
                    }
                }
                return (((("{\n"
                    + "  float fogDist = length(u_cameraPosition - v_worldPosition);\n")
                    + factor)
                    + format!(
                        "  radiance = mix(radiance, u_fogColor{}, clamp(fogFactor, 0.0, 1.0));\n",
                        suffix
                    ))
                    + "}");
            },
        )
            as Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>)),
        declarations: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, index: f64| -> String {
                let fog = {
                    let __flight_source = &((modifier).clone());
                    FogModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        steps: __flight_source.steps,
                        smoothness: __flight_source.smoothness,
                        color: __flight_source.color,
                        power: __flight_source.power,
                        intensity: __flight_source.intensity,
                        bias: __flight_source.bias,
                        tint: __flight_source.tint,
                        fresnel_bias: __flight_source.fresnel_bias,
                        roughness: __flight_source.roughness,
                        threshold: __flight_source.threshold,
                        edge_color: __flight_source.edge_color,
                        edge_width: __flight_source.edge_width,
                        scale: __flight_source.scale,
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        ..Default::default()
                    }
                };
                let suffix = format!("_{}", index);
                let mut source = format!("uniform vec3 u_fogColor{};\n", suffix);
                source += if (((fog.mode).clone()).is_none())
                    || ((fog.mode).clone() == fog_modifier_mode_constant.linear)
                {
                    format!(
                        "uniform float u_fogNear{};\nuniform float u_fogFar{};\n",
                        suffix, suffix
                    )
                } else {
                    format!("uniform float u_fogDensity{};\n", suffix)
                };
                return source;
            },
        )
            as Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>))),
        ..(fog_modifier_definition_constant).clone()
    });

// Source: upstream/packages/scene3d-gl/src/glShadedBuiltInModifiers.ts:341 (sha256:f1d745ccedfe52e00526f40c66070e45c10ac3aa2ec3c5702b6f1d91facaf9a1)
pub static TOON_GL_MODIFIER_SNIPPET: std::sync::LazyLock<GlModifierSnippet> =
    std::sync::LazyLock::new(|| GlModifierSnippet {
        bind: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, context: GlModifierBindContext| -> () {
                let toon = {
                    let __flight_source = &((modifier).clone());
                    ToonModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        steps: __flight_source.steps,
                        smoothness: __flight_source.smoothness,
                        color: __flight_source.color,
                        power: __flight_source.power,
                        intensity: __flight_source.intensity,
                        bias: __flight_source.bias,
                        tint: __flight_source.tint,
                        fresnel_bias: __flight_source.fresnel_bias,
                        roughness: __flight_source.roughness,
                        threshold: __flight_source.threshold,
                        edge_color: __flight_source.edge_color,
                        edge_width: __flight_source.edge_width,
                        scale: __flight_source.scale,
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        ..Default::default()
                    }
                };
                let gl = (context.state.gl).clone();
                let suffix = format!("_{}", context.index);
                crate::host_value::<()>("host.uniform1f");
                crate::host_value::<()>("host.uniform1f");
            },
        )
            as Box<
                dyn FnMut(Modifier, GlModifierBindContext) -> () + Send + 'static,
            >))),
        contribution: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |_modifier: Modifier, index: f64| -> String {
                let suffix = format!("_{}", index);
                return ((((((((("{\n"
                    + "  float toonLum = dot(radiance, vec3(0.2126, 0.7152, 0.0722));\n")
                    + format!("  float toonSteps = max(u_toonSteps{}, 2.0);\n", suffix))
                    + "  float toonScaled = toonLum * toonSteps;\n")
                    + "  float toonBand = floor(toonScaled);\n")
                    + "  float toonFrac = toonScaled - toonBand;\n")
                    + format!(
                        "  float toonSoft = max(u_toonSmoothness{}, 1e-4);\n",
                        suffix
                    ))
                    + "  float toonQuant = (toonBand + smoothstep(0.5 - toonSoft, 0.5 + toonSoft, toonFrac)) / toonSteps;\n")
                    + "  radiance *= toonLum > 1e-4 ? toonQuant / toonLum : 1.0;\n")
                    + "}");
            },
        )
            as Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>)),
        declarations: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |_modifier: Modifier, index: f64| -> String {
                let suffix = format!("_{}", index);
                return format!(
                    "uniform float u_toonSteps{};\nuniform float u_toonSmoothness{};\n",
                    suffix, suffix
                );
            },
        )
            as Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>))),
        ..(toon_modifier_definition_constant).clone()
    });

// Source: upstream/packages/scene3d-gl/src/glShadedBuiltInModifiers.ts:374 (sha256:bba8b9afdcaebb00a76f9b3688a98cdd90d41059d62a46f0ff2f7b65bd19acb2)
pub static VERTEX_DISPLACE_GL_MODIFIER_SNIPPET: std::sync::LazyLock<GlModifierSnippet> =
    std::sync::LazyLock::new(|| GlModifierSnippet {
        bind: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, context: GlModifierBindContext| -> () {
                let displace = {
                    let __flight_source = &((modifier).clone());
                    VertexDisplaceModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        steps: __flight_source.steps,
                        smoothness: __flight_source.smoothness,
                        color: __flight_source.color,
                        power: __flight_source.power,
                        intensity: __flight_source.intensity,
                        bias: __flight_source.bias,
                        tint: __flight_source.tint,
                        fresnel_bias: __flight_source.fresnel_bias,
                        roughness: __flight_source.roughness,
                        threshold: __flight_source.threshold,
                        edge_color: __flight_source.edge_color,
                        edge_width: __flight_source.edge_width,
                        scale: __flight_source.scale,
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        map: None,
                        ..Default::default()
                    }
                };
                let gl = (context.state.gl).clone();
                let suffix = format!("_{}", context.index);
                crate::host_value::<()>("host.uniform1f");
                if ((displace.axis).clone()).is_some() {
                    crate::host_value::<()>("host.uniform3f");
                }
                if ((displace.source).clone()
                    == vertex_displace_modifier_source_constant.height_map)
                {
                    if ((displace.map).clone()).is_some() {
                        bind_gl_modifier_texture(
                            &context,
                            displace.map.as_ref().unwrap(),
                            format!("u_vDisplaceMap{}", suffix),
                        );
                    }
                } else {
                    crate::host_value::<()>("host.uniform1f");
                    crate::host_value::<()>("host.uniform1f");
                    let dir = ((displace.direction).clone()).unwrap_or(Vector3Like {
                        __flight_identity: std::sync::Arc::new(()),
                        __flight_entity_runtime: Default::default(),
                        x: 1.0_f64,
                        y: 0.0_f64,
                        z: 0.0_f64,
                    });
                    crate::host_value::<()>("host.uniform3f");
                }
            },
        )
            as Box<
                dyn FnMut(Modifier, GlModifierBindContext) -> () + Send + 'static,
            >))),
        contribution: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, index: f64| -> String {
                let displace = {
                    let __flight_source = &((modifier).clone());
                    VertexDisplaceModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        steps: __flight_source.steps,
                        smoothness: __flight_source.smoothness,
                        color: __flight_source.color,
                        power: __flight_source.power,
                        intensity: __flight_source.intensity,
                        bias: __flight_source.bias,
                        tint: __flight_source.tint,
                        fresnel_bias: __flight_source.fresnel_bias,
                        roughness: __flight_source.roughness,
                        threshold: __flight_source.threshold,
                        edge_color: __flight_source.edge_color,
                        edge_width: __flight_source.edge_width,
                        scale: __flight_source.scale,
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        map: None,
                        ..Default::default()
                    }
                };
                let suffix = format!("_{}", index);
                let axis = if ((displace.axis).clone()).is_some() {
                    format!(
                        "  vec3 vDisplaceAxis = normalize(u_vDisplaceAxis{});\n",
                        suffix
                    )
                } else {
                    "  vec3 vDisplaceAxis = normalize(localNormal);\n".to_owned()
                };
                let amount = if ((displace.source).clone()
                    == vertex_displace_modifier_source_constant.height_map)
                {
                    format!(
                        "  float vDisplaceAmount = texture(u_vDisplaceMap{}, vertexUv).r * u_vDisplaceAmplitude{};\n",
                        suffix, suffix
                    )
                } else {
                    format!(
                        "  float vDisplacePhase = dot(localPosition.xyz, normalize(u_vDisplaceDir{})) * u_vDisplaceFrequency{} + u_time * u_vDisplaceSpeed{};\n  float vDisplaceAmount = sin(vDisplacePhase) * u_vDisplaceAmplitude{};\n",
                        suffix, suffix, suffix, suffix
                    )
                };
                return (((("{\n" + axis) + amount)
                    + "  localPosition.xyz += vDisplaceAxis * vDisplaceAmount;\n")
                    + "}");
            },
        )
            as Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>)),
        declarations: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |modifier: Modifier, index: f64| -> String {
                let displace = {
                    let __flight_source = &((modifier).clone());
                    VertexDisplaceModifier {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        kind: (__flight_source.kind).clone(),
                        slot: (__flight_source.slot).clone(),
                        steps: __flight_source.steps,
                        smoothness: __flight_source.smoothness,
                        color: __flight_source.color,
                        power: __flight_source.power,
                        intensity: __flight_source.intensity,
                        bias: __flight_source.bias,
                        tint: __flight_source.tint,
                        fresnel_bias: __flight_source.fresnel_bias,
                        roughness: __flight_source.roughness,
                        threshold: __flight_source.threshold,
                        edge_color: __flight_source.edge_color,
                        edge_width: __flight_source.edge_width,
                        scale: __flight_source.scale,
                        source: (__flight_source.source).clone(),
                        amplitude: __flight_source.amplitude,
                        axis: (__flight_source.axis).clone(),
                        frequency: __flight_source.frequency,
                        speed: __flight_source.speed,
                        direction: (__flight_source.direction).clone(),
                        mode: (__flight_source.mode).clone(),
                        near: __flight_source.near,
                        far: __flight_source.far,
                        density: __flight_source.density,
                        strength: __flight_source.strength,
                        mask: (__flight_source.mask).clone(),
                        facing: (__flight_source.facing).clone(),
                        facing_softness: __flight_source.facing_softness,
                        scroll: (__flight_source.scroll).clone(),
                        secondary_map: (__flight_source.secondary_map).clone(),
                        secondary_scroll: (__flight_source.secondary_scroll).clone(),
                        map: None,
                        ..Default::default()
                    }
                };
                let suffix = format!("_{}", index);
                let mut source = format!("uniform float u_vDisplaceAmplitude{};\n", suffix);
                if ((displace.axis).clone()).is_some() {
                    source += format!("uniform vec3 u_vDisplaceAxis{};\n", suffix);
                }
                source += if ((displace.source).clone()
                    == vertex_displace_modifier_source_constant.height_map)
                {
                    format!("uniform sampler2D u_vDisplaceMap{};\n", suffix)
                } else {
                    format!(
                        "uniform float u_vDisplaceFrequency{};\nuniform float u_vDisplaceSpeed{};\nuniform vec3 u_vDisplaceDir{};\n",
                        suffix, suffix, suffix
                    )
                };
                return source;
            },
        )
            as Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>))),
        ..(vertex_displace_modifier_definition_constant).clone()
    });

// Source: upstream/packages/scene3d-gl/src/glShadedBuiltInModifiers.ts:428 (sha256:769350e159f0952a16d25d47da8a13849a4e28c131df1b4253422166aedb24d5)
pub fn register_built_in_gl_modifier_snippets(state: &mut GlRenderState) -> () {
    register_gl_modifier_snippet(state, &ANIMATED_NORMAL_GL_MODIFIER_SNIPPET);
    register_gl_modifier_snippet(state, &DISSOLVE_GL_MODIFIER_SNIPPET);
    register_gl_modifier_snippet(state, &EMISSIVE_GL_MODIFIER_SNIPPET);
    register_gl_modifier_snippet(state, &ENV_REFLECT_GL_MODIFIER_SNIPPET);
    register_gl_modifier_snippet(state, &FOG_GL_MODIFIER_SNIPPET);
    register_gl_modifier_snippet(state, &RIM_GL_MODIFIER_SNIPPET);
    register_gl_modifier_snippet(state, &TOON_GL_MODIFIER_SNIPPET);
    register_gl_modifier_snippet(state, &VERTEX_DISPLACE_GL_MODIFIER_SNIPPET);
}

// Source: upstream/packages/scene3d-gl/src/glShadedBuiltInModifiers.ts:445 (sha256:524d9e69b8812aa0f0d10881f43cf433d5d0c489cf52d1d75544f7bec703ca4a)
fn bind_gl_modifier_texture(
    context: &GlModifierBindContext,
    texture: &Texture,
    uniform_name: String,
) -> () {
    let gl = (context.state.gl).clone();
    let unit = {
        let __flight_callback = (context.acquire_modifier_texture_unit).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
    if (unit < 0.0_f64) {
        return;
    }
    crate::host_value::<()>("host.activeTexture");
    resolve_gl_texture(&context.state, (texture).clone(), None, None);
    crate::host_value::<()>("host.uniform1i");
    crate::host_value::<()>("host.activeTexture");
}

// Source: upstream/packages/scene3d-gl/src/glShadedBuiltInModifiers.ts:460 (sha256:35f1557dbe5fbeebf699dae882927e55477b02464db02ca4ba841d154f694a9c)
fn is_emissive_gated(modifier: &EmissiveModifier) -> bool {
    return (((modifier.facing).clone()).is_some())
        && ((modifier.facing).clone() != emissive_modifier_facing_constant.ignore);
}

// Source: upstream/packages/scene3d-gl/src/glShadedBuiltInModifiers.ts:464 (sha256:33cbdb0c15208a5943cbbf7c6e6dbffb13fa7a74ea8e21fc4e9ff194027c4ad2)
static SCRATCH_RGBA: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));
