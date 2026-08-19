// @generated from upstream/packages/scene3d-gl/src/glPbrExtensionRegistry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_gl_scene3_d_runtime, is_gl_texture_ready};
use flighthq_color::unpack_color_to_linear;
use flighthq_geometry::create_matrix3;
use flighthq_render_gl::{get_gl_render_state_runtime, resolve_gl_texture};
use flighthq_texture::get_texture_uv_matrix;
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, CanvasShapeCommand, CanvasTextureResolvers,
    ColorScaleBias, ExternalTexture, GlBitmapShader, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder,
    GlParticleShader, GlPbrExtensionBindContext, GlPbrExtensionIssue, GlPbrExtensionRegistration,
    GlPbrExtensionShaderContext, GlPbrExtensionShaderContribution, GlQuadBatchShader,
    GlRenderEffectRunner, GlRenderState, GlRenderTextureEntry, GlRenderTextureGuard,
    GlShaderLocations, GlShapeMeshColorScaleBiasShader, GlTextureResolver,
    GlUniformColorScaleBiasShader, InteractionSignals, Kind, LinearColor, Material, Matrix,
    Matrix3, Matrix4, MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose,
    MeshSkinBindPose, Node, NodeInteractionState, NodeSignals, NodeTraitsKey, Path, PathMesh,
    PbrExtension, PbrUvSet, Rectangle, RenderEffectPaddingResolver, RenderProxy, RenderProxy2D,
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

// Source: upstream/packages/scene3d-gl/src/glPbrExtensionRegistry.ts:21 (sha256:8c32361c0768fdaac28776c09edf812dd8c242ee0bd9ff7c0ea8bed8dbbe6de7)
pub fn bind_gl_pbr_extensions(
    state: &mut GlRenderState,
    program: crate::OpaqueHostValue,
    extensions: &Vec<PbrExtension>,
) -> bool {
    let context = create_gl_pbr_extension_bind_context((state).clone(), (program).clone());
    {
        let mut i = 0.0_f64;
        while (i < (extensions.len() as f64)) {
            let registration = get_gl_scene3_d_runtime(state)
                .pbr_extension_registry
                .iter()
                .find(|(key, _)| key == &(extensions[i as usize].kind).clone())
                .map(|(_, value)| value.clone());
            if (registration).is_none() {
                return false;
            }
            {
                let __flight_callback = (registration.as_ref().unwrap().bind).clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    (context).clone(),
                    extensions[i as usize].clone(),
                );
                __flight_result
            };
            {
                i += 1.0;
                i
            };
        }
    }
    return true;
}

// Source: upstream/packages/scene3d-gl/src/glPbrExtensionRegistry.ts:36 (sha256:4cd2098fbbc662eef580eee961801fc70554e527dafccb978b746a98d0c5cb58)
#[derive(Clone, Default)]
struct ExplainGlPbrExtensionsRecord4 {
    __flight_identity: std::sync::Arc<()>,
    code: String,
    kind: String,
}
impl PartialEq for ExplainGlPbrExtensionsRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn explain_gl_pbr_extensions(
    state: &mut GlRenderState,
    extensions: &Vec<PbrExtension>,
) -> Vec<GlPbrExtensionIssue> {
    let mut issues: Vec<GlPbrExtensionIssue> = vec![];
    let mut kinds: Vec<Kind> = Vec::new();
    let mut transmission_scene_color_kind: Option<Kind> = None;
    let mut texture_count = 0.0_f64;
    let shader_context = create_gl_pbr_extension_shader_context((state).clone());
    {
        let mut i = 0.0_f64;
        while (i < (extensions.len() as f64)) {
            let extension = extensions[i as usize].clone();
            if kinds.iter().any(|item| item == &(extension.kind).clone()) {
                issues.push(GlPbrExtensionIssue {
                    __flight_identity: std::sync::Arc::new(()),
                    code: "duplicate-kind".to_owned(),
                    kind: (extension.kind).clone(),
                });
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            {
                let __flight_value = (extension.kind).clone();
                if !kinds.contains(&__flight_value) {
                    kinds.push(__flight_value);
                }
            };
            let registration = get_gl_scene3_d_runtime(state)
                .pbr_extension_registry
                .iter()
                .find(|(key, _)| key == &(extension.kind).clone())
                .map(|(_, value)| value.clone());
            if (registration).is_none() {
                issues.push(GlPbrExtensionIssue {
                    __flight_identity: std::sync::Arc::new(()),
                    code: "missing-registration".to_owned(),
                    kind: (extension.kind).clone(),
                });
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            if (!{
                let __flight_callback = (registration.as_ref().unwrap().is_supported).clone();
                let __flight_result = __flight_callback.lock().unwrap()((extension).clone());
                __flight_result
            }) {
                issues.push(GlPbrExtensionIssue {
                    __flight_identity: std::sync::Arc::new(()),
                    code: "unsupported-extension".to_owned(),
                    kind: (extension.kind).clone(),
                });
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let contribution = {
                let __flight_callback =
                    (registration.as_ref().unwrap().create_shader_contribution).clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    (shader_context).clone(),
                    (extension).clone(),
                );
                __flight_result
            };
            if (contribution.samples_transmission_scene_color) == Some(true) {
                transmission_scene_color_kind = Some((extension.kind).clone());
            }
            texture_count += contribution.texture_count;
            {
                i += 1.0;
                i
            };
        }
    }
    let scene_color = (get_gl_scene3_d_runtime(state).pbr_transmission_scene_color).clone();
    let active_target = (get_gl_render_state_runtime(state)
        .inner
        .lock()
        .unwrap()
        .gl_render_state_runtime
        .current_render_target)
        .clone();
    if (((transmission_scene_color_kind).is_some()) && ((scene_color).is_some()))
        && ({
            let __flight_value = (scene_color.as_ref().unwrap().texture).clone();
            (active_target.as_ref().map(|value| (value.textures).clone()))
                .as_ref()
                .unwrap()
                .iter()
                .any(|item| item == &__flight_value)
        } == true)
    {
        issues.push(GlPbrExtensionIssue {
            __flight_identity: std::sync::Arc::new(()),
            code: "framebuffer-feedback".to_owned(),
            kind: (transmission_scene_color_kind).clone().unwrap(),
        });
    }
    if (texture_count > (get_gl_pbr_extension_texture_units((state.gl).clone()).len() as f64)) {
        issues.push(GlPbrExtensionIssue {
            __flight_identity: std::sync::Arc::new(()),
            code: "texture-unit-exhaustion".to_owned(),
            kind: "ExtendedPbrMaterial".to_owned(),
        });
    }
    return issues;
}

// Source: upstream/packages/scene3d-gl/src/glPbrExtensionRegistry.ts:81 (sha256:3a3ed00f0f6017e12e5c3c3b599c95f9aadb753893dc4717e286dc6fc0fd3cc2)
pub fn get_gl_pbr_extension_registration(
    state: &mut GlRenderState,
    kind: Kind,
) -> Option<GlPbrExtensionRegistration> {
    return get_gl_scene3_d_runtime(state)
        .pbr_extension_registry
        .iter()
        .find(|(key, _)| key == &(kind).clone())
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/scene3d-gl/src/glPbrExtensionRegistry.ts:85 (sha256:187b48b23d68dc73cfc4292520356fee8c49d4fd7e21196df135246b2d73b426)
pub fn register_gl_pbr_extension(
    state: &mut GlRenderState,
    kind: Kind,
    registration: &GlPbrExtensionRegistration,
) -> () {
    let mut runtime = get_gl_scene3_d_runtime(state);
    {
        let __flight_key = (kind).clone();
        let __flight_value = (*registration).clone();
        if let Some((_, value)) = runtime
            .pbr_extension_registry
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            runtime
                .pbr_extension_registry
                .push((__flight_key, __flight_value));
        }
    };
    {
        runtime.pbr_extension_registry_version += 1.0;
        runtime.pbr_extension_registry_version
    };
}

// Source: upstream/packages/scene3d-gl/src/glPbrExtensionRegistry.ts:95 (sha256:07928bd65933cb28cf9050991558102118dceb7f449efd0d019d8381b6db3d8c)
pub fn resolve_gl_pbr_extension_contributions(
    state: &mut GlRenderState,
    extensions: &Vec<PbrExtension>,
) -> Option<Vec<GlPbrExtensionShaderContribution>> {
    if ((explain_gl_pbr_extensions(state, extensions).len() as f64) > 0.0_f64) {
        return None;
    }
    let context = create_gl_pbr_extension_shader_context((state).clone());
    let mut contributions: Vec<GlPbrExtensionShaderContribution> = vec![];
    {
        let mut i = 0.0_f64;
        while (i < (extensions.len() as f64)) {
            contributions.push({
                let __flight_callback = (get_gl_scene3_d_runtime(state)
                    .pbr_extension_registry
                    .iter()
                    .find(|(key, _)| key == &(extensions[i as usize].kind).clone())
                    .map(|(_, value)| value.clone())
                    .as_ref()
                    .unwrap()
                    .create_shader_contribution)
                    .clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    (context).clone(),
                    extensions[i as usize].clone(),
                );
                __flight_result
            });
            {
                i += 1.0;
                i
            };
        }
    }
    return Some((contributions).clone());
}

// Source: upstream/packages/scene3d-gl/src/glPbrExtensionRegistry.ts:109 (sha256:568520c0132c46b190d8b53b2c4434f63eb9fac326da972f57777eed1e233452)
fn create_gl_pbr_extension_shader_context(mut state: GlRenderState) -> GlPbrExtensionShaderContext {
    return GlPbrExtensionShaderContext {
        __flight_identity: std::sync::Arc::new(()),
        has_transmission_scene_color: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut state = state.clone();
            move || -> bool {
                return ((get_gl_scene3_d_runtime(&mut state).pbr_transmission_scene_color)
                    .clone())
                .is_some();
            }
        })
            as Box<dyn FnMut() -> bool + Send + 'static>)),
        is_texture_ready: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut state = state.clone();
            move |texture: Option<Texture>| -> bool {
                return is_gl_texture_ready(&state, ((texture).clone()).clone());
            }
        })
            as Box<dyn FnMut(Option<Texture>) -> bool + Send + 'static>)),
    };
}

// Source: upstream/packages/scene3d-gl/src/glPbrExtensionRegistry.ts:120 (sha256:fb70374fb91b059f7b92236eb6961a16b711683a35a9b46c037fd4dbf6b7a4de)
fn create_gl_pbr_extension_bind_context(
    mut state: GlRenderState,
    program: crate::OpaqueHostValue,
) -> GlPbrExtensionBindContext {
    let gl = (state.gl).clone();
    let texture_units = get_gl_pbr_extension_texture_units((gl).clone());
    let texture_index: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    return GlPbrExtensionBindContext {
        __flight_identity: std::sync::Arc::new(()),
        bind_transmission_scene_color: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut state = state.clone();
            let mut texture_index = texture_index.clone();
            let texture_units = texture_units.clone();
            move |sampler_uniform: String, max_lod_uniform: String| -> bool {
                let scene_color =
                    (get_gl_scene3_d_runtime(&mut state).pbr_transmission_scene_color).clone();
                if (scene_color).is_none() {
                    return false;
                }
                let unit = texture_units[{
                    (*texture_index.lock().unwrap()) += 1.0;
                    (*texture_index.lock().unwrap())
                } as usize]
                    .clone();
                if (unit).is_none() {
                    return false;
                }
                crate::host_value::<()>("host.activeTexture");
                crate::host_value::<()>("host.bindTexture");
                crate::host_value::<()>("host.uniform1i");
                crate::host_value::<()>("host.uniform1f");
                return true;
            }
        })
            as Box<dyn FnMut(String, String) -> bool + Send + 'static>)),
        bind_texture: std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut state = state.clone();
            let mut texture_index = texture_index.clone();
            let texture_units = texture_units.clone();
            move |sampler_uniform: String,
                  uv_set_uniform: String,
                  uv_transform_uniform: String,
                  texture: Option<Texture>,
                  uv_set: PbrUvSet|
                  -> bool {
                if (!is_gl_texture_ready(&state, ((texture).clone()).clone())) {
                    return false;
                }
                let unit = texture_units[{
                    (*texture_index.lock().unwrap()) += 1.0;
                    (*texture_index.lock().unwrap())
                } as usize]
                    .clone();
                if ((unit).is_none()) || ((texture).is_none()) {
                    return false;
                }
                crate::host_value::<()>("host.activeTexture");
                if (resolve_gl_texture(
                    &state,
                    (((texture).clone()).clone().unwrap()).clone(),
                    None,
                    None,
                ))
                .is_none()
                {
                    return false;
                }
                crate::host_value::<()>("host.uniform1i");
                crate::host_value::<()>("host.uniform1i");
                get_texture_uv_matrix(&mut (*SCRATCH_UV_MATRIX.lock().unwrap()), &texture);
                crate::host_value::<()>("host.uniformMatrix3fv");
                return true;
            }
        })
            as Box<
                dyn FnMut(String, String, String, Option<Texture>, PbrUvSet) -> bool
                    + Send
                    + 'static,
            >)),
        set_float: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |uniform: String, value: f64| -> () {
                crate::host_value::<()>("host.uniform1f");
            },
        )
            as Box<dyn FnMut(String, f64) -> () + Send + 'static>)),
        set_linear_color: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |uniform: String, color: f64| -> () {
                unpack_color_to_linear(&mut SCRATCH_RGBA, color);
                crate::host_value::<()>("host.uniform3f");
            },
        )
            as Box<dyn FnMut(String, f64) -> () + Send + 'static>)),
    };
}

// Source: upstream/packages/scene3d-gl/src/glPbrExtensionRegistry.ts:158 (sha256:c7584ecf2e956deb91ca62f3404e4afad34f629e0439d815d9e9e76bf0a503e6)
fn get_gl_pbr_extension_texture_units(gl: crate::OpaqueHostValue) -> Vec<f64> {
    let count = crate::host_value::<f64>("host.getParameter");
    let mut units: Vec<f64> = vec![];
    {
        let mut unit = 6.0_f64;
        while (unit < count) {
            if (unit < 8.0_f64) || (unit > 12.0_f64) {
                units.push(unit);
            }
            {
                unit += 1.0;
                unit
            };
        }
    }
    return units;
}

// Source: upstream/packages/scene3d-gl/src/glPbrExtensionRegistry.ts:167 (sha256:33cbdb0c15208a5943cbbf7c6e6dbffb13fa7a74ea8e21fc4e9ff194027c4ad2)
static SCRATCH_RGBA: std::sync::LazyLock<std::sync::Mutex<LinearColor>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64]));

// Source: upstream/packages/scene3d-gl/src/glPbrExtensionRegistry.ts:168 (sha256:08e68a5a6460b4383316b176e6cb950361a8d3721f2377e804398f54d7c2d328)
static SCRATCH_UV_MATRIX: std::sync::LazyLock<std::sync::Mutex<Matrix3>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix3(
            None, None, None, None, None, None, None, None, None,
        ))
    });
