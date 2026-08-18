// @generated from upstream/packages/scene3d-gl/src/transmissionVolumePbrGlExtension.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::register_gl_pbr_extension;
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, CanvasShapeCommand, CanvasTextureResolvers,
    ColorScaleBias, ExternalTexture, GlBitmapShader, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder,
    GlParticleShader, GlPbrExtensionBindContext, GlPbrExtensionRegistration,
    GlPbrExtensionShaderContext, GlPbrExtensionShaderContribution, GlQuadBatchShader,
    GlRenderEffectRunner, GlRenderState, GlRenderTextureEntry, GlRenderTextureGuard,
    GlShaderLocations, GlShapeMeshColorScaleBiasShader, GlTextureResolver,
    GlUniformColorScaleBiasShader, InteractionSignals, Kind, Material, Matrix, Matrix4,
    MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose, MeshSkinBindPose, Node,
    NodeInteractionState, NodeSignals, NodeTraitsKey, Path, PathMesh, PbrExtension, Rectangle,
    RenderEffectPaddingResolver, RenderProxy, RenderProxy2D, RenderProxyAdapter, RenderState,
    RenderTexture, Renderable, Renderer, SamplerLike, Scene2D, Scene2DClipHooks, Scene2DSignals,
    Scene3DGraphSyncPolicy, ShapeRasterizer, StrokeStyle,
    TRANSMISSION_VOLUME_PBR_EXTENSION_KIND as transmission_volume_pbr_extension_kind_constant,
    Texture, TextureFilter, TextureSourceKind, TextureWrap, TintMaterialData,
    TransmissionVolumePbrExtension,
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

// Source: upstream/packages/scene3d-gl/src/transmissionVolumePbrGlExtension.ts:10 (sha256:6f1e318aa7954e52500154dbdaa8998acd17d5409c46c6df060d6116b3390cc7)
pub static TRANSMISSION_VOLUME_PBR_GL_EXTENSION: std::sync::LazyLock<GlPbrExtensionRegistration> =
    std::sync::LazyLock::new(|| GlPbrExtensionRegistration {
        __flight_identity: std::sync::Arc::new(()),
        bind: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |context: GlPbrExtensionBindContext, value: PbrExtension| -> () {
                let extension = {
                    let __flight_source = &((value).clone());
                    TransmissionVolumePbrExtension {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        __flight_entity_runtime: std::sync::Arc::clone(
                            &__flight_source.__flight_entity_runtime,
                        ),
                        kind: (__flight_source.kind).clone(),
                        thickness: __flight_source.thickness,
                        thickness_map: (__flight_source.thickness_map).clone(),
                        thickness_map_uv_set: __flight_source.thickness_map_uv_set,
                        wrapped_diffuse_color: __flight_source.wrapped_diffuse_color,
                        wrapped_diffuse_map: (__flight_source.wrapped_diffuse_map).clone(),
                        wrapped_diffuse_map_uv_set: __flight_source.wrapped_diffuse_map_uv_set,
                        wrapped_diffuse_strength: __flight_source.wrapped_diffuse_strength,
                        attenuation_color: __flight_source.attenuation_color,
                        attenuation_distance: __flight_source.attenuation_distance,
                        ior: __flight_source.ior,
                        transmission: __flight_source.transmission,
                        transmission_map: (__flight_source.transmission_map).clone(),
                        transmission_map_uv_set: __flight_source.transmission_map_uv_set,
                        specular: __flight_source.specular,
                        specular_color: __flight_source.specular_color,
                        specular_color_map: (__flight_source.specular_color_map).clone(),
                        specular_color_map_uv_set: __flight_source.specular_color_map_uv_set,
                        specular_map: (__flight_source.specular_map).clone(),
                        specular_map_uv_set: __flight_source.specular_map_uv_set,
                        sheen_color: __flight_source.sheen_color,
                        sheen_color_map: (__flight_source.sheen_color_map).clone(),
                        sheen_color_map_uv_set: __flight_source.sheen_color_map_uv_set,
                        sheen_roughness: __flight_source.sheen_roughness,
                        sheen_roughness_map: (__flight_source.sheen_roughness_map).clone(),
                        sheen_roughness_map_uv_set: __flight_source.sheen_roughness_map_uv_set,
                        iridescence: __flight_source.iridescence,
                        iridescence_ior: __flight_source.iridescence_ior,
                        iridescence_map: (__flight_source.iridescence_map).clone(),
                        iridescence_map_uv_set: __flight_source.iridescence_map_uv_set,
                        iridescence_thickness_map: (__flight_source.iridescence_thickness_map)
                            .clone(),
                        iridescence_thickness_map_uv_set: __flight_source
                            .iridescence_thickness_map_uv_set,
                        iridescence_thickness_max: __flight_source.iridescence_thickness_max,
                        iridescence_thickness_min: __flight_source.iridescence_thickness_min,
                        clearcoat: __flight_source.clearcoat,
                        clearcoat_map: (__flight_source.clearcoat_map).clone(),
                        clearcoat_map_uv_set: __flight_source.clearcoat_map_uv_set,
                        clearcoat_normal_map: (__flight_source.clearcoat_normal_map).clone(),
                        clearcoat_normal_map_uv_set: __flight_source.clearcoat_normal_map_uv_set,
                        clearcoat_normal_scale: __flight_source.clearcoat_normal_scale,
                        clearcoat_roughness: __flight_source.clearcoat_roughness,
                        clearcoat_roughness_map: (__flight_source.clearcoat_roughness_map).clone(),
                        clearcoat_roughness_map_uv_set: __flight_source
                            .clearcoat_roughness_map_uv_set,
                        anisotropy_map: (__flight_source.anisotropy_map).clone(),
                        anisotropy_map_uv_set: __flight_source.anisotropy_map_uv_set,
                        anisotropy_rotation: __flight_source.anisotropy_rotation,
                        anisotropy_strength: __flight_source.anisotropy_strength,
                        ..Default::default()
                    }
                };
                {
                    let __flight_callback = (context.set_float).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        "u_flightTransmission".to_owned(),
                        extension.transmission,
                    );
                    __flight_result
                };
                {
                    let __flight_callback = (context.set_float).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        "u_flightTransmissionThickness".to_owned(),
                        extension.thickness,
                    );
                    __flight_result
                };
                {
                    let __flight_callback = (context.set_float).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        "u_flightTransmissionIor".to_owned(),
                        extension.ior,
                    );
                    __flight_result
                };
                {
                    let __flight_callback = (context.set_float).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        "u_flightAttenuationDistance".to_owned(),
                        extension.attenuation_distance,
                    );
                    __flight_result
                };
                {
                    let __flight_callback = (context.set_linear_color).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        "u_flightAttenuationColor".to_owned(),
                        extension.attenuation_color,
                    );
                    __flight_result
                };
                {
                    let __flight_callback = (context.bind_texture).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        "u_flightTransmissionMap".to_owned(),
                        "u_flightTransmissionMapUvSet".to_owned(),
                        "u_flightTransmissionMapTransform".to_owned(),
                        (extension.transmission_map).clone(),
                        extension.transmission_map_uv_set,
                    );
                    __flight_result
                };
                {
                    let __flight_callback = (context.bind_texture).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        "u_flightTransmissionThicknessMap".to_owned(),
                        "u_flightTransmissionThicknessMapUvSet".to_owned(),
                        "u_flightTransmissionThicknessMapTransform".to_owned(),
                        (extension.thickness_map).clone(),
                        extension.thickness_map_uv_set,
                    );
                    __flight_result
                };
                {
                    let __flight_callback = (context.bind_transmission_scene_color).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        "u_flightTransmissionSceneColor".to_owned(),
                        "u_flightTransmissionMaxLod".to_owned(),
                    );
                    __flight_result
                };
            },
        )
            as Box<dyn FnMut(GlPbrExtensionBindContext, PbrExtension) -> () + Send + 'static>)),
        create_shader_contribution: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |context: GlPbrExtensionShaderContext,
                  value: PbrExtension|
                  -> GlPbrExtensionShaderContribution {
                let extension = {
                    let __flight_source = &((value).clone());
                    TransmissionVolumePbrExtension {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        __flight_entity_runtime: std::sync::Arc::clone(
                            &__flight_source.__flight_entity_runtime,
                        ),
                        kind: (__flight_source.kind).clone(),
                        thickness: __flight_source.thickness,
                        thickness_map: (__flight_source.thickness_map).clone(),
                        thickness_map_uv_set: __flight_source.thickness_map_uv_set,
                        wrapped_diffuse_color: __flight_source.wrapped_diffuse_color,
                        wrapped_diffuse_map: (__flight_source.wrapped_diffuse_map).clone(),
                        wrapped_diffuse_map_uv_set: __flight_source.wrapped_diffuse_map_uv_set,
                        wrapped_diffuse_strength: __flight_source.wrapped_diffuse_strength,
                        attenuation_color: __flight_source.attenuation_color,
                        attenuation_distance: __flight_source.attenuation_distance,
                        ior: __flight_source.ior,
                        transmission: __flight_source.transmission,
                        transmission_map: (__flight_source.transmission_map).clone(),
                        transmission_map_uv_set: __flight_source.transmission_map_uv_set,
                        specular: __flight_source.specular,
                        specular_color: __flight_source.specular_color,
                        specular_color_map: (__flight_source.specular_color_map).clone(),
                        specular_color_map_uv_set: __flight_source.specular_color_map_uv_set,
                        specular_map: (__flight_source.specular_map).clone(),
                        specular_map_uv_set: __flight_source.specular_map_uv_set,
                        sheen_color: __flight_source.sheen_color,
                        sheen_color_map: (__flight_source.sheen_color_map).clone(),
                        sheen_color_map_uv_set: __flight_source.sheen_color_map_uv_set,
                        sheen_roughness: __flight_source.sheen_roughness,
                        sheen_roughness_map: (__flight_source.sheen_roughness_map).clone(),
                        sheen_roughness_map_uv_set: __flight_source.sheen_roughness_map_uv_set,
                        iridescence: __flight_source.iridescence,
                        iridescence_ior: __flight_source.iridescence_ior,
                        iridescence_map: (__flight_source.iridescence_map).clone(),
                        iridescence_map_uv_set: __flight_source.iridescence_map_uv_set,
                        iridescence_thickness_map: (__flight_source.iridescence_thickness_map)
                            .clone(),
                        iridescence_thickness_map_uv_set: __flight_source
                            .iridescence_thickness_map_uv_set,
                        iridescence_thickness_max: __flight_source.iridescence_thickness_max,
                        iridescence_thickness_min: __flight_source.iridescence_thickness_min,
                        clearcoat: __flight_source.clearcoat,
                        clearcoat_map: (__flight_source.clearcoat_map).clone(),
                        clearcoat_map_uv_set: __flight_source.clearcoat_map_uv_set,
                        clearcoat_normal_map: (__flight_source.clearcoat_normal_map).clone(),
                        clearcoat_normal_map_uv_set: __flight_source.clearcoat_normal_map_uv_set,
                        clearcoat_normal_scale: __flight_source.clearcoat_normal_scale,
                        clearcoat_roughness: __flight_source.clearcoat_roughness,
                        clearcoat_roughness_map: (__flight_source.clearcoat_roughness_map).clone(),
                        clearcoat_roughness_map_uv_set: __flight_source
                            .clearcoat_roughness_map_uv_set,
                        anisotropy_map: (__flight_source.anisotropy_map).clone(),
                        anisotropy_map_uv_set: __flight_source.anisotropy_map_uv_set,
                        anisotropy_rotation: __flight_source.anisotropy_rotation,
                        anisotropy_strength: __flight_source.anisotropy_strength,
                        ..Default::default()
                    }
                };
                let transmission_map = {
                    let __flight_callback = (context.is_texture_ready).clone();
                    let __flight_result =
                        __flight_callback.lock().unwrap()((extension.transmission_map).clone());
                    __flight_result
                };
                let thickness_map = {
                    let __flight_callback = (context.is_texture_ready).clone();
                    let __flight_result =
                        __flight_callback.lock().unwrap()((extension.thickness_map).clone());
                    __flight_result
                };
                let scene_color = {
                    let __flight_callback = (context.has_transmission_scene_color).clone();
                    let __flight_result = __flight_callback.lock().unwrap()();
                    __flight_result
                };
                return GlPbrExtensionShaderContribution {
                    __flight_identity: std::sync::Arc::new(()),
                    apply_surface: "".to_owned(),
                    contribute_ibl: "".to_owned(),
                    contribute_punctual: "".to_owned(),
                    finalize: if scene_color {
                        format!(
                            "\n  float flightTransmissionFactor = clamp(u_flightTransmission * {}, 0.0, 1.0);\n  float flightTransmissionThickness = max(u_flightTransmissionThickness * {}, 0.0);\n  float flightTransmissionEta = 1.0 / max(u_flightTransmissionIor, 1.0);\n  vec3 flightTransmissionRefracted = refract(-viewDir, normal, flightTransmissionEta);\n  vec4 flightTransmissionRefractedClip = u_viewProjection * vec4(\n    v_worldPosition + flightTransmissionRefracted * max(flightTransmissionThickness, 0.01), 1.0);\n  vec2 flightTransmissionScreenUv =\n    flightTransmissionRefractedClip.xy / max(flightTransmissionRefractedClip.w, 1e-5) * 0.5 + 0.5;\n  float flightTransmissionLod = roughness * u_flightTransmissionMaxLod;\n  vec3 flightTransmissionBackground = textureLod(\n    u_flightTransmissionSceneColor, clamp(flightTransmissionScreenUv, vec2(0.0), vec2(1.0)), flightTransmissionLod).rgb;\n  vec3 flightTransmissionAbsorption = u_flightAttenuationDistance > 0.0 && !isinf(u_flightAttenuationDistance)\n    ? pow(max(u_flightAttenuationColor, vec3(1e-4)), vec3(flightTransmissionThickness / u_flightAttenuationDistance))\n    : vec3(1.0);\n  vec3 flightTransmissionFresnel = fresnelSchlick(max(dot(normal, viewDir), 0.0), f0);\n  vec3 flightTransmissionThrough = flightTransmissionBackground * flightTransmissionAbsorption;\n  radiance = mix(radiance, radiance * flightTransmissionFresnel + flightTransmissionThrough * (1.0 - flightTransmissionFresnel), flightTransmissionFactor);",
                            if transmission_map {
                                "texture(u_flightTransmissionMap, flightTransmissionUv()).r"
                                    .to_owned()
                            } else {
                                "1.0".to_owned()
                            },
                            if thickness_map {
                                "texture(u_flightTransmissionThicknessMap, flightTransmissionThicknessUv()).g".to_owned()
                            } else {
                                "1.0".to_owned()
                            }
                        )
                    } else {
                        "".to_owned()
                    },
                    fragment_declarations: format!(
                        "\nuniform float u_flightTransmission;\nuniform float u_flightTransmissionThickness;\nuniform float u_flightTransmissionIor;\nuniform float u_flightAttenuationDistance;\nuniform vec3 u_flightAttenuationColor;\n{}\n{}\n{}\n{}",
                        if scene_color {
                            "uniform sampler2D u_flightTransmissionSceneColor;".to_owned()
                        } else {
                            "".to_owned()
                        },
                        if scene_color {
                            "uniform float u_flightTransmissionMaxLod; uniform mat4 u_viewProjection;".to_owned()
                        } else {
                            "".to_owned()
                        },
                        if transmission_map {
                            "uniform sampler2D u_flightTransmissionMap; uniform int u_flightTransmissionMapUvSet; uniform mat3 u_flightTransmissionMapTransform;".to_owned()
                        } else {
                            "".to_owned()
                        },
                        if thickness_map {
                            "uniform sampler2D u_flightTransmissionThicknessMap; uniform int u_flightTransmissionThicknessMapUvSet; uniform mat3 u_flightTransmissionThicknessMapTransform;".to_owned()
                        } else {
                            "".to_owned()
                        }
                    ),
                    fragment_functions: format!(
                        "\n{}\n{}",
                        if transmission_map {
                            "vec2 flightTransmissionUv() { vec2 uv = u_flightTransmissionMapUvSet == 1 ? v_pbrExtensionUv1 : v_pbrExtensionUv0; return (u_flightTransmissionMapTransform * vec3(uv, 1.0)).xy; }".to_owned()
                        } else {
                            "".to_owned()
                        },
                        if thickness_map {
                            "vec2 flightTransmissionThicknessUv() { vec2 uv = u_flightTransmissionThicknessMapUvSet == 1 ? v_pbrExtensionUv1 : v_pbrExtensionUv0; return (u_flightTransmissionThicknessMapTransform * vec3(uv, 1.0)).xy; }".to_owned()
                        } else {
                            "".to_owned()
                        }
                    ),
                    key: format!(
                        "transmission:{}{}{}",
                        if scene_color {
                            "s".to_owned()
                        } else {
                            "-".to_owned()
                        },
                        if transmission_map {
                            "f".to_owned()
                        } else {
                            "-".to_owned()
                        },
                        if thickness_map {
                            "t".to_owned()
                        } else {
                            "-".to_owned()
                        }
                    ),
                    samples_transmission_scene_color: Some(scene_color),
                    texture_count: ((number(scene_color) + number(transmission_map))
                        + number(thickness_map)),
                };
            },
        )
            as Box<
                dyn FnMut(
                        GlPbrExtensionShaderContext,
                        PbrExtension,
                    ) -> GlPbrExtensionShaderContribution
                    + Send
                    + 'static,
            >)),
        is_supported: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_unused_0: PbrExtension| -> bool {
                return true;
            },
        )
            as Box<dyn FnMut(PbrExtension) -> bool + Send + 'static>)),
    });

// Source: upstream/packages/scene3d-gl/src/transmissionVolumePbrGlExtension.ts:86 (sha256:05564132bd824933efde870c939dc912a499a977c2e785d87a3b6685700ff938)
pub fn register_gl_transmission_volume_pbr_extension(state: &mut GlRenderState) -> () {
    register_gl_pbr_extension(
        state,
        (transmission_volume_pbr_extension_kind_constant).to_owned(),
        &TRANSMISSION_VOLUME_PBR_GL_EXTENSION,
    );
}
