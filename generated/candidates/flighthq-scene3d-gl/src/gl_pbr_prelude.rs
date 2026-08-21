// @generated from upstream/packages/scene3d-gl/src/glPbrPrelude.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GL_DIRECTIONAL_SHADOW_GLSL as gl_directional_shadow_glsl_constant,
    GL_MESH_FRAGMENT_TAIL as gl_mesh_fragment_tail_constant,
    GL_MESH_FRAGMENT_TAIL_UNIFORMS as gl_mesh_fragment_tail_uniforms_constant,
    GL_SKIN_VERTEX_DECLARATIONS_GLSL as gl_skin_vertex_declarations_glsl_constant,
    GL_UV_TRANSFORM_VERTEX_GLSL as gl_uv_transform_vertex_glsl_constant,
};
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, CanvasShapeCommand, CanvasTextureResolvers,
    ColorScaleBias, ExternalTexture, GlBitmapShader, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder,
    GlParticleShader, GlPbrDefineKey, GlPbrExtensionShaderContribution, GlQuadBatchShader,
    GlRenderEffectRunner, GlRenderState, GlRenderTextureEntry, GlRenderTextureGuard,
    GlShaderLocations, GlShapeMeshColorScaleBiasShader, GlTextureResolver,
    GlUniformColorScaleBiasShader, Image, InteractionSignals, Kind,
    MAX_FORWARD_LIGHTS as max_forward_lights_constant, Material, Matrix, Matrix4,
    MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose, MeshSkinBindPose, Node,
    NodeInteractionState, NodeSignals, NodeTraitsKey, Path, PathMesh, Rectangle,
    RenderEffectPaddingResolver, RenderProxy, RenderProxy2D, RenderProxyAdapter, RenderState,
    RenderTexture, Renderable, Renderer, SamplerLike, Scene2D, Scene2DClipHooks, Scene2DSignals,
    Scene3DGraphSyncPolicy, Scene3DLightBlock, ShapeRasterizer, StrokeStyle, Texture,
    TextureFilter, TextureSourceKind, TextureWrap, TintMaterialData,
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

// Source: upstream/packages/scene3d-gl/src/glPbrPrelude.ts:38 (sha256:e2e0e2935056abedf6db2e2e77704b7d6ae2a6cadca9ef8e975c7fcc5a60bca1)
pub fn build_gl_pbr_define_key(key: &GlPbrDefineKey) -> String {
    return (((((((((format!(
        "{}",
        if key.alpha_mask_enabled {
            "m".to_owned()
        } else {
            "-".to_owned()
        }
    ) + format!(
        "{}",
        if key.has_base_color_map {
            "b".to_owned()
        } else {
            "-".to_owned()
        }
    )) + format!(
        "{}",
        if key.has_normal_map {
            "n".to_owned()
        } else {
            "-".to_owned()
        }
    )) + format!(
        "{}",
        if key.has_metallic_roughness_map {
            "r".to_owned()
        } else {
            "-".to_owned()
        }
    )) + format!(
        "{}",
        if key.has_occlusion_map {
            "o".to_owned()
        } else {
            "-".to_owned()
        }
    )) + format!(
        "{}",
        if key.has_emissive_map {
            "e".to_owned()
        } else {
            "-".to_owned()
        }
    )) + format!(
        "{}",
        if key.has_alpha_map {
            "a".to_owned()
        } else {
            "-".to_owned()
        }
    )) + format!(
        "{}",
        if key.has_uv_transform {
            "u".to_owned()
        } else {
            "-".to_owned()
        }
    )) + format!(
        ":{}",
        if (key.has_skin).unwrap_or(false) {
            "k".to_owned()
        } else {
            "-".to_owned()
        }
    )) + format!(
        "{}",
        if (key.has_color_matrix).unwrap_or(false) {
            "x".to_owned()
        } else {
            if (key.has_color_adjustment).unwrap_or(false) {
                "c".to_owned()
            } else {
                "".to_owned()
            }
        }
    ));
}

// Source: upstream/packages/scene3d-gl/src/glPbrPrelude.ts:56 (sha256:f94dd968db612834bf6bfb7e8329f2e2d85afe05301849fb9686b720fb907ddc)
pub fn build_gl_pbr_define_source(key: &GlPbrDefineKey) -> String {
    let mut defines = format!(
        "#version 300 es\n#define MAX_FORWARD_LIGHTS {}\n",
        max_forward_lights_constant
    );
    if key.alpha_mask_enabled {
        defines += "#define ALPHA_MASK\n".to_owned();
    }
    if key.has_base_color_map {
        defines += "#define HAS_BASE_COLOR_MAP\n".to_owned();
    }
    if key.has_uv_transform {
        defines += "#define HAS_UV_TRANSFORM\n".to_owned();
    }
    if key.has_normal_map {
        defines += "#define HAS_NORMAL_MAP\n".to_owned();
    }
    if key.has_metallic_roughness_map {
        defines += "#define HAS_METALLIC_ROUGHNESS_MAP\n".to_owned();
    }
    if key.has_occlusion_map {
        defines += "#define HAS_OCCLUSION_MAP\n".to_owned();
    }
    if key.has_emissive_map {
        defines += "#define HAS_EMISSIVE_MAP\n".to_owned();
    }
    if key.has_alpha_map {
        defines += "#define HAS_ALPHA_MAP\n".to_owned();
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

// Source: upstream/packages/scene3d-gl/src/glPbrPrelude.ts:75 (sha256:10501a46836f95fc4fb2b6409b12a7c82d3902170a394e4130fac04461bdc5fe)
pub fn get_gl_pbr_fragment_source() -> String {
    return ((PBR_FRAGMENT_BODY).clone()).to_owned();
}

// Source: upstream/packages/scene3d-gl/src/glPbrPrelude.ts:81 (sha256:e1cea1f6ad066f8261c94cf3e04b77df5ad81315701c0db01bfcbc768e5184cb)
pub fn get_gl_pbr_fragment_source_for_key(
    key: &GlPbrDefineKey,
    contributions: Option<Vec<GlPbrExtensionShaderContribution>>,
    color_adjustment_feature: Option<GlColorAdjustmentMaterialFeature>,
) -> String {
    let contributions = contributions.unwrap_or(vec![]);
    let mut body = compose_gl_pbr_extension_source((PBR_FRAGMENT_BODY).clone(), &contributions);
    if (((key.has_color_adjustment).unwrap_or(false)) || ((key.has_color_matrix).unwrap_or(false)))
        && ((color_adjustment_feature).is_some())
    {
        body = (body.replace)(
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
    return ((build_gl_pbr_define_source(key)
        + if ((contributions.len() as f64) > 0.0_f64) {
            "#define HAS_PBR_EXTENSIONS\n".to_owned()
        } else {
            "".to_owned()
        })
        + body);
}

// Source: upstream/packages/scene3d-gl/src/glPbrPrelude.ts:100 (sha256:86567072be7eaee3c3674072dcca3d29ff277f24afc5c9fffbebe55461880867)
fn compose_gl_pbr_extension_source(
    body: String,
    contributions: &Vec<GlPbrExtensionShaderContribution>,
) -> String {
    return ((((((body.replace)(
        PBR_EXTENSION_DECLARATIONS,
        ((contributions)
            .iter()
            .cloned()
            .map(
                |value: GlPbrExtensionShaderContribution| -> crate::OpaqueHostValue {
                    {
                        let __flight_portable_source = (value.fragment_declarations).clone();
                        crate::FlightValue::String((&__flight_portable_source).clone())
                    }
                },
            )
            .collect()
            .join)("\n"),
    )
    .replace)(
        PBR_EXTENSION_FUNCTIONS,
        ((contributions)
            .iter()
            .cloned()
            .map(
                |value: GlPbrExtensionShaderContribution| -> crate::OpaqueHostValue {
                    {
                        let __flight_portable_source = (value.fragment_functions).clone();
                        crate::FlightValue::String((&__flight_portable_source).clone())
                    }
                },
            )
            .collect()
            .join)("\n"),
    )
    .replace)(
        PBR_EXTENSION_SURFACE,
        ((contributions)
            .iter()
            .cloned()
            .map(
                |value: GlPbrExtensionShaderContribution| -> crate::OpaqueHostValue {
                    {
                        let __flight_portable_source = (value.apply_surface).clone();
                        crate::FlightValue::String((&__flight_portable_source).clone())
                    }
                },
            )
            .collect()
            .join)("\n"),
    )
    .replace)(
        PBR_EXTENSION_PUNCTUAL,
        ((contributions)
            .iter()
            .cloned()
            .map(
                |value: GlPbrExtensionShaderContribution| -> crate::OpaqueHostValue {
                    {
                        let __flight_portable_source = (value.contribute_punctual).clone();
                        crate::FlightValue::String((&__flight_portable_source).clone())
                    }
                },
            )
            .collect()
            .join)("\n"),
    )
    .replace)(
        PBR_EXTENSION_IBL,
        ((contributions)
            .iter()
            .cloned()
            .map(
                |value: GlPbrExtensionShaderContribution| -> crate::OpaqueHostValue {
                    {
                        let __flight_portable_source = (value.contribute_ibl).clone();
                        crate::FlightValue::String((&__flight_portable_source).clone())
                    }
                },
            )
            .collect()
            .join)("\n"),
    )
    .replace)(
        PBR_EXTENSION_FINALIZE,
        ((contributions)
            .iter()
            .cloned()
            .map(
                |value: GlPbrExtensionShaderContribution| -> crate::OpaqueHostValue {
                    {
                        let __flight_portable_source = (value.finalize).clone();
                        crate::FlightValue::String((&__flight_portable_source).clone())
                    }
                },
            )
            .collect()
            .join)("\n"),
    );
}

// Source: upstream/packages/scene3d-gl/src/glPbrPrelude.ts:113 (sha256:8d08d08fc250fd853e234875bd1dcadc45904504cc6b536f7441f1f41b66f309)
pub fn get_gl_pbr_vertex_source() -> String {
    return ((PBR_VERTEX_BODY).clone()).to_owned();
}

// Source: upstream/packages/scene3d-gl/src/glPbrPrelude.ts:120 (sha256:e39826bbc1391968260829d50e0e2ebde35f98d762a1f40ca08cdddc1e2af6dd)
pub fn get_gl_pbr_vertex_source_for_key(key: &GlPbrDefineKey) -> String {
    return ((build_gl_pbr_define_source(key)
        + if (key.has_skin).unwrap_or(false) {
            (gl_skin_vertex_declarations_glsl_constant).to_owned()
        } else {
            "".to_owned()
        })
        + PBR_VERTEX_BODY);
}

// Source: upstream/packages/scene3d-gl/src/glPbrPrelude.ts:124 (sha256:3d680aebbf495fdee2d053a32409f776c6822d509942390d305f7e4583da9d0d)
const PBR_EXTENSION_DECLARATIONS: &'static str = "/*__PBR_EXTENSION_DECLARATIONS__*/";

// Source: upstream/packages/scene3d-gl/src/glPbrPrelude.ts:125 (sha256:acefa98e6b213d9fd5d1c3435c94749f6539e03500874c14eabb63c7d8388977)
const PBR_EXTENSION_FINALIZE: &'static str = "/*__PBR_EXTENSION_FINALIZE__*/";

// Source: upstream/packages/scene3d-gl/src/glPbrPrelude.ts:126 (sha256:21bafbfdcf81ed2fb8923fd25ad868265e1bebc0324330b41b356b4d9083864e)
const PBR_EXTENSION_FUNCTIONS: &'static str = "/*__PBR_EXTENSION_FUNCTIONS__*/";

// Source: upstream/packages/scene3d-gl/src/glPbrPrelude.ts:127 (sha256:772d86a8485e447480461b28690764546e8be0da88750a0b5014e80cce0399c4)
const PBR_EXTENSION_IBL: &'static str = "/*__PBR_EXTENSION_IBL__*/";

// Source: upstream/packages/scene3d-gl/src/glPbrPrelude.ts:128 (sha256:61e62c67f3e79ff0c047428a969f54b606fe3e923f3cff254c252d58d44fd012)
const PBR_EXTENSION_PUNCTUAL: &'static str = "/*__PBR_EXTENSION_PUNCTUAL__*/";

// Source: upstream/packages/scene3d-gl/src/glPbrPrelude.ts:129 (sha256:94822478b8c25cde4991c0e8a8ae35aaffabe7f1c60ffe6bb4996683a6b2852b)
const PBR_EXTENSION_SURFACE: &'static str = "/*__PBR_EXTENSION_SURFACE__*/";

// Source: upstream/packages/scene3d-gl/src/glPbrPrelude.ts:131 (sha256:d0f61bc8b9405dba5d9005aa2db9598df4c943b0786e12db8e77c8ba7f703bc8)
static PBR_VERTEX_BODY: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "\nlayout(location = 0) in vec3 a_position;\nlayout(location = 1) in vec3 a_normal;\nlayout(location = 2) in vec4 a_tangent;\nlayout(location = 3) in vec2 a_uv0;\nlayout(location = 5) in vec2 a_uv1;\n\nuniform mat4 u_viewProjection;\nuniform mat4 u_model;\nuniform mat3 u_normalMatrix;\n{}\nout vec3 v_worldPosition;\nout vec3 v_normal;\nout vec4 v_tangent;\nout vec2 v_uv0;\nout vec2 v_pbrExtensionUv0;\nout vec2 v_pbrExtensionUv1;\n\nvoid main() {{\n#ifdef HAS_SKIN\n  mat4 skin = skinMatrix();\n  vec4 localPosition = skin * vec4(a_position, 1.0);\n  vec3 localNormal = mat3(skin) * a_normal;\n  vec3 localTangent = mat3(skin) * a_tangent.xyz;\n#else\n  vec4 localPosition = vec4(a_position, 1.0);\n  vec3 localNormal = a_normal;\n  vec3 localTangent = a_tangent.xyz;\n#endif\n  vec4 worldPosition = u_model * localPosition;\n  v_worldPosition = worldPosition.xyz;\n  v_normal = u_normalMatrix * localNormal;\n  v_tangent = vec4(u_normalMatrix * localTangent, a_tangent.w);\n  v_uv0 = applyUvTransform(a_uv0);\n  v_pbrExtensionUv0 = a_uv0;\n  v_pbrExtensionUv1 = a_uv1;\n  gl_Position = u_viewProjection * worldPosition;\n}}\n",
        gl_uv_transform_vertex_glsl_constant
    )
});

// Source: upstream/packages/scene3d-gl/src/glPbrPrelude.ts:171 (sha256:a55a6c41adc69b3a2c90f47b5fc56c66776ca232a9c3199fee2e91d11afcdb1a)
static PBR_FRAGMENT_BODY: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "\nprecision highp float;\n\nin vec3 v_worldPosition;\nin vec3 v_normal;\nin vec4 v_tangent;\nin vec2 v_uv0;\nin vec2 v_pbrExtensionUv0;\nin vec2 v_pbrExtensionUv1;\n\nuniform vec4 u_baseColor;\n#ifdef HAS_COLOR_MATRIX\nuniform vec4 u_flightColorMatrix0;\nuniform vec4 u_flightColorMatrix1;\nuniform vec4 u_flightColorMatrix2;\nuniform vec4 u_flightColorMatrix3;\nuniform vec4 u_flightColorMatrixOffset;\n#elif defined(HAS_COLOR_ADJUSTMENT)\nuniform vec4 u_flightColorScale;\nuniform vec4 u_flightColorBias;\n#endif\nuniform float u_metallic;\nuniform float u_roughness;\nuniform float u_normalScale;\nuniform vec3 u_emissive;\nuniform float u_emissiveStrength;\nuniform float u_occlusionStrength;\nuniform float u_alphaCutoff;\nuniform vec3 u_cameraPosition;\n\nuniform vec4 u_directional;\nuniform vec4 u_directionalRadiance;\nuniform vec3 u_ambientRadiance;\nuniform float u_directionalCount;\nuniform float u_ambientCount;\n\n// Punctual (point/spot/hemisphere) forward-light arrays — layout mirrors Scene3DLightBlock.data exactly\n// (packScene3DLightBlock), matching GL_MESH_LIGHT_BLOCK_GLSL used by the classic prelude. Fixed\n// MAX_FORWARD_LIGHTS-wide; each count bounds its loop.\n//   point[i]      = u_pointLights[i*2+0]={{pos.xyz,range}}, [i*2+1]={{radiance.rgb,invSqrRange}}\n//   spot[i]       = u_spotLights[i*4+0..1] as point, [i*4+2]={{dir.xyz,_}}, [i*4+3]={{cosInner,cosOuter,_,_}}\n//   hemisphere[i] = u_hemisphereLights[i*3+0]={{sky.rgb,_}}, [i*3+1]={{ground.rgb,_}}, [i*3+2]={{up.xyz,_}}\nuniform vec4 u_pointLights[MAX_FORWARD_LIGHTS * 2];\nuniform vec4 u_spotLights[MAX_FORWARD_LIGHTS * 4];\nuniform vec4 u_hemisphereLights[MAX_FORWARD_LIGHTS * 3];\nuniform int u_pointCount;\nuniform int u_spotCount;\nuniform int u_hemisphereCount;\n\n{}\n\n{}\n\nuniform samplerCube u_iblIrradiance;  // diffuse irradiance cubemap\nuniform samplerCube u_iblPrefiltered; // roughness-mipped prefiltered specular cubemap\nuniform sampler2D u_iblBrdf;          // split-sum BRDF integration LUT (RG)\nuniform float u_iblEnabled;           // 0 or 1 — gates image-based ambient\nuniform float u_iblIntensity;         // environment contribution scale\nuniform float u_iblMaxMip;            // highest prefiltered mip index (roughness 1.0)\n\n// Roughness-aware Fresnel for the IBL specular term (Sébastien Lagarde): rougher surfaces reflect less\n// at grazing angles than the smooth Schlick approximation.\nvec3 fresnelSchlickRoughness(float cosTheta, vec3 F0, float roughness) {{\n  return F0 + (max(vec3(1.0 - roughness), F0) - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);\n}}\n\n#ifdef HAS_BASE_COLOR_MAP\nuniform sampler2D u_baseColorMap;\n#endif\n#ifdef HAS_NORMAL_MAP\nuniform sampler2D u_normalMap;\n#endif\n#ifdef HAS_METALLIC_ROUGHNESS_MAP\nuniform sampler2D u_metallicRoughnessMap;\n#endif\n#ifdef HAS_OCCLUSION_MAP\nuniform sampler2D u_occlusionMap;\n#endif\n#ifdef HAS_EMISSIVE_MAP\nuniform sampler2D u_emissiveMap;\n#endif\n#ifdef HAS_ALPHA_MAP\nuniform sampler2D u_alphaMap;\n#endif\n\n{}\n\nout vec4 fragColor;\n\nconst float PI = 3.14159265359;\n\nfloat distributionGgx(float nDotH, float roughness) {{\n  float a = roughness * roughness;\n  float a2 = a * a;\n  float d = nDotH * nDotH * (a2 - 1.0) + 1.0;\n  return a2 / max(PI * d * d, 1e-7);\n}}\n\nfloat visibilitySmith(float nDotV, float nDotL, float roughness) {{\n  float a = roughness * roughness;\n  float k = a * 0.5;\n  float gv = nDotV / (nDotV * (1.0 - k) + k);\n  float gl = nDotL / (nDotL * (1.0 - k) + k);\n  return gv * gl;\n}}\n\nvec3 fresnelSchlick(float cosTheta, vec3 f0) {{\n  return f0 + (1.0 - f0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);\n}}\n\n{}\n\n// Image-based ambient via the split-sum approximation: diffuse irradiance over the albedo plus\n// prefiltered specular weighted by the BRDF LUT. Replaces the flat ambient term when an environment\n// is baked (bakeGlEnvironmentIbl). All three cubemap/LUT samples are already linear (baked from\n// sRGB-decoded sources), so no decode here. This function follows the contributed helpers because\n// GLSL ES requires a function to be declared before a caller uses it.\nvec3 sampleIblAmbient(\n  vec3 N, vec3 V, vec3 tangentDir, vec3 bitangentDir, float rough, vec3 F0, vec3 diffuseColor, float occ\n) {{\n  float nv = max(dot(N, V), 1e-4);\n  vec3 F = fresnelSchlickRoughness(nv, F0, rough);\n  vec3 diffuse = texture(u_iblIrradiance, N).rgb * diffuseColor;\n  vec3 R = reflect(-V, N);\n  vec3 prefiltered = textureLod(u_iblPrefiltered, R, rough * u_iblMaxMip).rgb;\n  vec2 brdf = texture(u_iblBrdf, vec2(nv, rough)).rg;\n  vec3 specular = prefiltered * (F * brdf.x + brdf.y);\n  vec3 ambient = ((vec3(1.0) - F) * diffuse + specular) * occ * u_iblIntensity;\n{}\n  return ambient;\n}}\n\n// Smooth inverse-square range window (glTF/UE4): 1 near the light, eased to 0 at the range. invSqrRange\n// is 1/range^2 (0 = infinite range, no cutoff); dist2 is the squared surface->light distance.\nfloat rangeWindow(float dist2, float invSqrRange) {{\n  float factor = dist2 * invSqrRange;\n  float windowed = clamp(1.0 - factor * factor, 0.0, 1.0);\n  return windowed * windowed;\n}}\n\n// The full Cook-Torrance shading (plus every enabled extension lobe) for ONE light. Directional,\n// point, and spot lights all route through this one BRDF so punctual lights never fork the shading\n// model — the caller passes the surface->light direction L and that light's (attenuated, cone-scaled)\n// radiance. The anisotropic tangent frame is rebuilt here per light from the surface tangent frame so\n// the function stays self-contained; f0/diffuseColor/roughness/metallic are the finalized surface\n// values from main. Returns the light's linear radiance contribution (shadowing applied by the caller).\nvec3 shadePbrPunctual(vec3 N, vec3 V, vec3 tangentDir, vec3 bitangentDir, vec3 L, vec3 lightColor,\n                      vec3 f0, vec3 diffuseColor, float roughness, float metallic) {{\n  float nDotV = max(dot(N, V), 1e-4);\n  vec3 halfVec = normalize(V + L);\n  float nDotL = max(dot(N, L), 0.0);\n  float nDotH = max(dot(N, halfVec), 0.0);\n  float vDotH = max(dot(V, halfVec), 0.0);\n\n  float d = distributionGgx(nDotH, roughness);\n  float vis = visibilitySmith(nDotV, nDotL, roughness);\n  vec3 fresnel = fresnelSchlick(vDotH, f0);\n\n  vec3 specular = d * vis * fresnel;\n  vec3 kd = (1.0 - fresnel) * (1.0 - metallic);\n  vec3 brdf = kd * diffuseColor / PI + specular;\n  vec3 direct = brdf * lightColor * nDotL;\n\n{}\n\n  return direct;\n}}\n\nvoid main() {{\n  vec4 baseColor = u_baseColor;\n#ifdef HAS_BASE_COLOR_MAP\n  vec4 sampled = texture(u_baseColorMap, v_uv0);\n  baseColor.rgb *= sampled.rgb;\n  baseColor.a *= sampled.a;\n#endif\n\n  // Dedicated coverage (opacity) map: its green channel is linear data, multiplied into alpha before\n  // the alpha-mask cutoff so 'mask' cutout and 'blend' transparency both see the combined coverage.\n#ifdef HAS_ALPHA_MAP\n  baseColor.a *= texture(u_alphaMap, v_uv0).g;\n#endif\n\n#ifdef ALPHA_MASK\n  if (baseColor.a < u_alphaCutoff) discard;\n  baseColor.a = 1.0;\n#endif\n\n  vec3 geometricNormal = normalize(v_normal);\n  if (!gl_FrontFacing) geometricNormal = -geometricNormal;\n\n#if defined(HAS_NORMAL_MAP) || defined(HAS_PBR_EXTENSIONS)\n  vec3 tangent = normalize(v_tangent.xyz - geometricNormal * dot(v_tangent.xyz, geometricNormal));\n  vec3 bitangent = cross(geometricNormal, tangent) * v_tangent.w;\n#else\n  vec3 tangent = vec3(1.0, 0.0, 0.0);\n  vec3 bitangent = vec3(0.0, 1.0, 0.0);\n#endif\n\n  vec3 normal = geometricNormal;\n#ifdef HAS_NORMAL_MAP\n  vec3 tangentNormal = texture(u_normalMap, v_uv0).xyz * 2.0 - 1.0;\n  tangentNormal.xy *= u_normalScale;\n  mat3 tbn = mat3(tangent, bitangent, geometricNormal);\n  normal = normalize(tbn * tangentNormal);\n#endif\n\n  vec3 viewDir = normalize(u_cameraPosition - v_worldPosition);\n  float nDotV = max(dot(normal, viewDir), 1e-4);\n\n  float roughness = clamp(u_roughness, 0.04, 1.0);\n  float metallic = clamp(u_metallic, 0.0, 1.0);\n#ifdef HAS_METALLIC_ROUGHNESS_MAP\n  // glTF packing: roughness in G, metallic in B (R is occlusion if combined, ignored here).\n  vec4 mr = texture(u_metallicRoughnessMap, v_uv0);\n  roughness = clamp(roughness * mr.g, 0.04, 1.0);\n  metallic = clamp(metallic * mr.b, 0.0, 1.0);\n#endif\n\n  float occlusion = 1.0;\n#ifdef HAS_OCCLUSION_MAP\n  // Occlusion in R; strength lerps between full ambient (1.0) and the sampled value.\n  float ao = texture(u_occlusionMap, v_uv0).r;\n  occlusion = mix(1.0, ao, clamp(u_occlusionStrength, 0.0, 1.0));\n#endif\n\n  vec3 albedo = baseColor.rgb;\n  vec3 f0 = mix(vec3(0.04), albedo, metallic);\n\n  vec3 diffuseColor = albedo * (1.0 - metallic);\n\n{}\n\n  vec3 radiance = vec3(0.0);\n\n  // Directional light: -direction is the surface-to-light vector (light travels along direction).\n  if (u_directionalCount > 0.5) {{\n    vec3 lightDir = normalize(-u_directional.xyz);\n    vec3 direct = shadePbrPunctual(normal, viewDir, tangent, bitangent, lightDir,\n                                   u_directionalRadiance.rgb, f0, diffuseColor, roughness, metallic);\n    radiance += direct * sampleDirectionalShadow(v_worldPosition, geometricNormal);\n  }}\n\n  // Point lights: surface->light direction with a smooth inverse-square range falloff, same BRDF.\n  for (int i = 0; i < MAX_FORWARD_LIGHTS; i++) {{\n    if (i >= u_pointCount) break;\n    vec3 toLight = u_pointLights[i * 2 + 0].xyz - v_worldPosition;\n    float dist2 = dot(toLight, toLight);\n    vec3 lightDir = toLight * inversesqrt(max(dist2, 1e-8));\n    float atten = rangeWindow(dist2, u_pointLights[i * 2 + 1].w) / max(dist2, 1e-4);\n    radiance += shadePbrPunctual(normal, viewDir, tangent, bitangent, lightDir,\n                                 u_pointLights[i * 2 + 1].rgb * atten, f0, diffuseColor, roughness, metallic);\n  }}\n\n  // Spot lights: point attenuation times a smooth cone falloff between the inner/outer cosines.\n  for (int i = 0; i < MAX_FORWARD_LIGHTS; i++) {{\n    if (i >= u_spotCount) break;\n    vec3 toLight = u_spotLights[i * 4 + 0].xyz - v_worldPosition;\n    float dist2 = dot(toLight, toLight);\n    vec3 lightDir = toLight * inversesqrt(max(dist2, 1e-8));\n    float atten = rangeWindow(dist2, u_spotLights[i * 4 + 1].w) / max(dist2, 1e-4);\n    float cone = smoothstep(u_spotLights[i * 4 + 3].y, u_spotLights[i * 4 + 3].x,\n                            dot(normalize(u_spotLights[i * 4 + 2].xyz), -lightDir));\n    radiance += shadePbrPunctual(normal, viewDir, tangent, bitangent, lightDir,\n                                 u_spotLights[i * 4 + 1].rgb * atten * cone, f0, diffuseColor, roughness, metallic);\n  }}\n\n  // Ambient term: image-based lighting (diffuse irradiance + prefiltered specular) when an environment\n  // is baked, else the flat ambient irradiance over the diffuse albedo. Both are attenuated by AO.\n  if (u_iblEnabled > 0.5) {{\n    radiance += sampleIblAmbient(normal, viewDir, tangent, bitangent, roughness, f0, diffuseColor, occlusion);\n  }} else if (u_ambientCount > 0.5) {{\n    radiance += diffuseColor * u_ambientRadiance * occlusion;\n  }}\n\n  // Hemisphere fill: sky/ground gradient blended by the normal's vertical component, AO-attenuated.\n  for (int i = 0; i < MAX_FORWARD_LIGHTS; i++) {{\n    if (i >= u_hemisphereCount) break;\n    float f = 0.5 + 0.5 * dot(normal, u_hemisphereLights[i * 3 + 2].xyz);\n    radiance += mix(u_hemisphereLights[i * 3 + 1].rgb, u_hemisphereLights[i * 3 + 0].rgb, f)\n                * diffuseColor * occlusion;\n  }}\n\n  vec3 emissive = u_emissive;\n#ifdef HAS_EMISSIVE_MAP\n  emissive *= texture(u_emissiveMap, v_uv0).rgb;\n#endif\n  radiance += emissive * u_emissiveStrength;\n\n  float alpha = baseColor.a;\n{}\n\n  fragColor = vec4(radiance, alpha);\n#ifdef HAS_COLOR_MATRIX\n  fragColor = applyFlightColorMatrix(fragColor, u_flightColorMatrix0, u_flightColorMatrix1,\n    u_flightColorMatrix2, u_flightColorMatrix3, u_flightColorMatrixOffset);\n#elif defined(HAS_COLOR_ADJUSTMENT)\n  fragColor = applyFlightColorAdjustment(fragColor, u_flightColorScale, u_flightColorBias);\n#endif\n{}\n}}\n",
        PBR_EXTENSION_DECLARATIONS,
        gl_directional_shadow_glsl_constant,
        gl_mesh_fragment_tail_uniforms_constant,
        PBR_EXTENSION_FUNCTIONS,
        PBR_EXTENSION_IBL,
        PBR_EXTENSION_PUNCTUAL,
        PBR_EXTENSION_SURFACE,
        PBR_EXTENSION_FINALIZE,
        gl_mesh_fragment_tail_constant
    )
});
