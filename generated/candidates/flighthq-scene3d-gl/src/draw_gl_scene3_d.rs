// @generated from upstream/packages/scene3d-gl/src/drawGlScene3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    draw_gl_scene3_d_particle_emitter3_ds, get_gl_scene3_d_runtime,
    get_gl_scene3_d_viewport_aspect, resolve_gl_mesh_material_renderer,
};
use flighthq_geometry::{create_matrix3, create_matrix4, set_matrix3_normal_from_matrix4};
use flighthq_mesh::has_mesh_geometry_skin;
use flighthq_node::get_node_world_matrix4;
use flighthq_render::prepare_scene3_d_render;
use flighthq_render_gl::{
    declare_gl_render_target_color_space, enable_gl_blend_mode_support,
    get_gl_render_state_runtime, invalidate_gl_render_state_cache,
};
use flighthq_scene3d::{get_node3_d_runtime, get_node3_d_world_alpha};
use flighthq_types::{
    Adjustment, BLEND_MODE as blend_mode_constant, BoundsNodeAny, Camera3D, CanvasShapeCommand,
    CanvasTextureResolvers, ColorScaleBias, ExternalTexture, GlBitmapShader, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader, GlCompressedTextureDecoder,
    GlMeshMaterialRenderer, GlParticleShader, GlQuadBatchShader, GlRenderEffectRunner,
    GlRenderState, GlRenderTextureEntry, GlRenderTextureGuard, GlScene3DDrawEntry,
    GlScene3DForwardLightList, GlShaderLocations, GlShapeMeshColorScaleBiasShader,
    GlTextureResolver, GlUniformColorScaleBiasShader, InteractionSignals, Kind,
    MAX_FORWARD_LIGHTS as max_forward_lights_constant, Material, Matrix, Matrix3, Matrix4,
    Matrix4Like, Mesh, MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose,
    MeshSkinBindPose, MeshSubset, Node, Node3D, NodeData, NodeInteractionState, NodeSignals,
    NodeTraitsKey, Path, PathMesh, Quaternion, Rectangle, RenderEffectPaddingResolver, RenderProxy,
    RenderProxy2D, RenderProxyAdapter, RenderState, RenderTexture, Renderable, Renderer,
    STANDARD_MATERIAL_KIND as standard_material_kind_constant, SamplerLike, Scene2D,
    Scene2DClipHooks, Scene2DSignals, Scene3DGraphSyncPolicy, Scene3DLightBlock, Scene3DLightsLike,
    Scene3DRenderProxy, ShapeRasterizer, StrokeStyle, SurfaceMaterial, Texture, TextureFilter,
    TextureSourceKind, TextureWrap, TintMaterialData, Transform3DNode, Vector3,
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
    pub data: Option<NodeData>,
    pub enabled: Option<bool>,
    pub kind: Option<Kind>,
    pub name: Option<String>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
    pub position: Option<Vector3>,
    pub rotation: Option<Quaternion>,
    pub scale: Option<Vector3>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3 {
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
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub anisotropy: Option<f64>,
    pub mag_filter: Option<TextureFilter>,
    pub min_filter: Option<TextureFilter>,
    pub mipmaps: Option<bool>,
    pub wrap_u: Option<TextureWrap>,
    pub wrap_v: Option<TextureWrap>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub position: Option<Vector3>,
    pub rotation: Option<Quaternion>,
    pub scale: Option<Vector3>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ModuleSynthesizedRecord1581943931 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: crate::OpaqueHostValue,
}
impl PartialEq for ModuleSynthesizedRecord1581943931 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene3d-gl/src/drawGlScene3D.ts:42 (sha256:d316a3033910c43ecff8210be187afcdc95c3ed252fe4d5ab0cffc90dd82f57a)
fn is_gpu_skinned_draw(mesh: &Mesh) -> bool {
    return (((mesh.skin).clone()).is_some()) && (has_mesh_geometry_skin(&mesh.geometry));
}

// Source: upstream/packages/scene3d-gl/src/drawGlScene3D.ts:69 (sha256:1f2601d3bcf30370e8780bc2fb27203a15ac646afac177aced27832bd99f6202)
pub fn draw_gl_scene3_d(
    state: &mut GlRenderState,
    scene: &mut Node3D,
    camera: &Camera3D,
    lights: &Scene3DLightsLike,
    forward_lights: Option<GlScene3DForwardLightList>,
) -> () {
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
    let mut runtime = get_gl_scene3_d_runtime(state);
    let has_prepared_forward_lights = ((forward_lights).is_some())
        && (forward_lights.as_ref().unwrap().mesh_count == list.mesh_count);
    if (!has_prepared_forward_lights) && (has_excess_forward_lights(lights)) {
        {
            let __flight_callback = (runtime.forward_light_selection_guard).clone();
            __flight_callback
                .as_ref()
                .map(|callback| callback.lock().unwrap()((*lights).clone()))
        };
    }
    if (!declare_gl_render_target_color_space(state, "linear".to_owned())) {
        {
            let __flight_callback = (runtime.color_space_guard).clone();
            __flight_callback
                .as_ref()
                .map(|callback| callback.lock().unwrap()())
        };
    }
    recycle_draw_entries(&mut runtime.opaque_draw_list, &mut runtime.opaque_pool);
    recycle_draw_entries(&mut runtime.blended_draw_list, &mut runtime.blended_pool);
    let deform_guard = (runtime.deform_guard).clone();
    {
        let mut m = 0.0_f64;
        while (m < list.mesh_count) {
            let mesh = list.visible_meshes[m as usize].clone();
            if (deform_guard).is_some() {
                {
                    let __flight_callback = (deform_guard.as_ref().unwrap()).clone();
                    let __flight_result = __flight_callback.lock().unwrap()((mesh).clone());
                    __flight_result
                };
            }
            let world_matrix = {
                let __flight_source = &(get_node_world_matrix4(&{
                    let __flight_source = &(mesh);
                    Transform3DNode {
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
            let wx = (world_matrix.m[12.0_f64 as usize] as f64);
            let wy = (world_matrix.m[13.0_f64 as usize] as f64);
            let wz = (world_matrix.m[14.0_f64 as usize] as f64);
            let clip_w = (((((list.view_projection.m[3.0_f64 as usize] as f64) * wx)
                + ((list.view_projection.m[7.0_f64 as usize] as f64) * wy))
                + ((list.view_projection.m[11.0_f64 as usize] as f64) * wz))
                + (list.view_projection.m[15.0_f64 as usize] as f64));
            let object_alpha = get_node3_d_world_alpha(&{
                let __flight_source = &(mesh);
                Node3D {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    __flight_entity_runtime: std::sync::Arc::clone(
                        &__flight_source.__flight_entity_runtime,
                    ),
                    data: (__flight_source.data).clone(),
                    enabled: __flight_source.enabled,
                    kind: (__flight_source.kind).clone(),
                    name: (__flight_source.name).clone(),
                    alpha: __flight_source.alpha,
                    visible: __flight_source.visible,
                    position: (__flight_source.position).clone(),
                    rotation: (__flight_source.rotation).clone(),
                    scale: (__flight_source.scale).clone(),
                }
            });
            let node_runtime = get_node3_d_runtime(&{
                let __flight_source = &(mesh);
                Node3D {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    __flight_entity_runtime: std::sync::Arc::clone(
                        &__flight_source.__flight_entity_runtime,
                    ),
                    data: (__flight_source.data).clone(),
                    enabled: __flight_source.enabled,
                    kind: (__flight_source.kind).clone(),
                    name: (__flight_source.name).clone(),
                    alpha: __flight_source.alpha,
                    visible: __flight_source.visible,
                    position: (__flight_source.position).clone(),
                    rotation: (__flight_source.rotation).clone(),
                    scale: (__flight_source.scale).clone(),
                }
            });
            let color_scale_bias =
                (node_runtime.inner.lock().unwrap().resolved_color_scale_bias).clone();
            let color_matrix = (node_runtime.inner.lock().unwrap().resolved_color_matrix).clone();
            {
                let mut s = 0.0_f64;
                while (s < (mesh.geometry.subsets.len() as f64)) {
                    let material = resolve_subset_material(&mesh, s);
                    let renderer =
                        resolve_gl_mesh_material_renderer(state, ((material).clone()).clone());
                    if ((renderer).clone()).is_none() {
                        {
                            s += 1.0;
                            s
                        };
                        continue;
                    }
                    let resolved_material =
                        ((material).clone()).unwrap_or((*DEFAULT_MATERIAL).clone());
                    let is_blended =
                        (is_blended_material(&resolved_material)) || (object_alpha < 1.0_f64);
                    let mut entry = acquire_draw_entry(&mut if is_blended {
                        (runtime.blended_pool).clone()
                    } else {
                        (runtime.opaque_pool).clone()
                    });
                    entry.alpha = object_alpha;
                    entry.clip_w = clip_w;
                    entry.color_matrix = (color_matrix).clone();
                    entry.color_scale_bias = (color_scale_bias).clone();
                    entry.light_block = if has_prepared_forward_lights {
                        forward_lights.as_ref().unwrap().mesh_light_blocks[m as usize].clone()
                    } else {
                        (list.lights).clone()
                    };
                    entry.mesh = (mesh).clone();
                    entry.material = (resolved_material).clone();
                    entry.renderer = {
                        let __flight_portable_source = (renderer).clone();
                        match (&__flight_portable_source).as_ref() {
                            Some(value) => crate::FlightValue::Record({
                                let mut __flight_record = Vec::new();
                                __flight_record
                                    .push(("bind".to_owned(), crate::FlightValue::Function));
                                __flight_record
                                    .push(("draw".to_owned(), crate::FlightValue::Function));
                                __flight_record
                            }),
                            None => crate::FlightValue::Null,
                        }
                    };
                    entry.subset = {
                        let __flight_portable_source = mesh.geometry.subsets[s as usize].clone();
                        crate::FlightValue::Record({
                            let mut __flight_record = Vec::new();
                            __flight_record.push((
                                "indexCount".to_owned(),
                                crate::FlightValue::Number(
                                    *(&((&__flight_portable_source).index_count)) as f64,
                                ),
                            ));
                            __flight_record.push((
                                "indexOffset".to_owned(),
                                crate::FlightValue::Number(
                                    *(&((&__flight_portable_source).index_offset)) as f64,
                                ),
                            ));
                            __flight_record
                        })
                    };
                    entry.world_matrix = {
                        let __flight_portable_source = (world_matrix).clone();
                        crate::FlightValue::Record({
                            let mut __flight_record = Vec::new();
                            __flight_record.push((
                                "m".to_owned(),
                                crate::FlightValue::Array(
                                    (&((&__flight_portable_source).m))
                                        .iter()
                                        .map(|value| crate::FlightValue::Number((*value) as f64))
                                        .collect(),
                                ),
                            ));
                            __flight_record
                        })
                    };
                    if is_blended {
                        runtime.blended_draw_list.push(((entry).clone()).clone());
                    } else {
                        runtime.opaque_draw_list.push(((entry).clone()).clone());
                    }
                    {
                        s += 1.0;
                        s
                    };
                }
            }
            {
                m += 1.0;
                m
            };
        }
    }
    let mut bound_material: Option<Material> = None;
    let mut bound_light_block: Option<Scene3DLightBlock> = None;
    let mut bound_renderer: Option<GlMeshMaterialRenderer> = None;
    let mut bound_skinned: Option<bool> = None;
    let mut bound_color_adjustment: Option<bool> = None;
    let mut bound_color_matrix: Option<bool> = None;
    let color_adjustment_feature_enabled = ((get_gl_render_state_runtime(state)
        .inner
        .lock()
        .unwrap()
        .gl_color_adjustment_material_feature)
        .clone())
    .is_some();
    {
        let mut i = 0.0_f64;
        while (i < (runtime.opaque_draw_list.len() as f64)) {
            let mut entry = runtime.opaque_draw_list[i as usize].clone();
            let world_matrix = (entry.world_matrix).clone();
            set_matrix3_normal_from_matrix4(&mut (*SCRATCH_NORMAL_MATRIX.lock().unwrap()), &{
                let __flight_source = &(world_matrix);
                Matrix4Like {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    __flight_entity_runtime: std::sync::Arc::clone(
                        &__flight_source.__flight_entity_runtime,
                    ),
                    m: (__flight_source.m).clone(),
                }
            });
            let skinned = is_gpu_skinned_draw(&entry.mesh);
            let color_adjusted = (color_adjustment_feature_enabled)
                && ((((entry.color_matrix).clone()).is_some())
                    || (((entry.color_scale_bias).clone()).is_some()));
            let color_matrix = (color_adjusted) && (((entry.color_matrix).clone()).is_some());
            if ((((((entry.renderer).clone() != bound_renderer)
                || ((entry.material).clone() != bound_material))
                || ((entry.light_block).clone() != bound_light_block))
                || (skinned != bound_skinned))
                || (color_adjusted != bound_color_adjustment))
                || (color_matrix != bound_color_matrix)
            {
                runtime.active_color_adjustment_run = color_adjusted;
                runtime.active_color_matrix_run = color_matrix;
                runtime.active_skinned_run = skinned;
                {
                    let __flight_callback = (entry.renderer.bind).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        (*state).clone(),
                        Some((entry.material).clone()),
                        (entry.light_block).clone(),
                        (*camera).clone(),
                    );
                    __flight_result
                };
                bound_renderer = Some((entry.renderer).clone());
                bound_material = Some((entry.material).clone());
                bound_light_block = Some((entry.light_block).clone());
                bound_skinned = Some(skinned);
                bound_color_adjustment = Some(color_adjusted);
                bound_color_matrix = Some(color_matrix);
            }
            (*PROXY.lock().unwrap()).alpha = Some(entry.alpha);
            (*PROXY.lock().unwrap()).color_scale_bias = if color_adjusted {
                (entry.color_scale_bias).clone()
            } else {
                None
            };
            (*PROXY.lock().unwrap()).color_matrix = if color_adjusted {
                (entry.color_matrix).clone()
            } else {
                None
            };
            (*PROXY.lock().unwrap()).joint_matrices = if skinned {
                Some((entry.mesh.skin.as_mut().unwrap().skeleton.joint_matrices).clone())
            } else {
                None
            };
            (*PROXY.lock().unwrap()).material = (entry.material).clone();
            (*PROXY.lock().unwrap()).normal_matrix =
                (*SCRATCH_NORMAL_MATRIX.lock().unwrap()).clone();
            (*PROXY.lock().unwrap()).subset = (entry.subset).clone();
            (*PROXY.lock().unwrap()).world_matrix = (world_matrix).clone();
            {
                let __flight_callback = (entry.renderer.draw).clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    (*state).clone(),
                    (*PROXY.lock().unwrap()).clone(),
                    (entry.mesh.geometry).clone(),
                );
                __flight_result
            };
            {
                i += 1.0;
                i
            };
        }
    }
    if ((runtime.blended_draw_list.len() as f64) > 0.0_f64) {
        {
            let mut __flight_values = runtime.blended_draw_list;
            __flight_values.sort_by(|left, right| {
                left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal)
            });
            __flight_values
        };
        let gl = (state.gl).clone();
        crate::host_value::<()>("host.enable");
        bound_material = None;
        bound_light_block = None;
        bound_renderer = None;
        bound_skinned = None;
        bound_color_adjustment = None;
        bound_color_matrix = None;
        {
            let mut i = 0.0_f64;
            while (i < (runtime.blended_draw_list.len() as f64)) {
                let mut entry = runtime.blended_draw_list[i as usize].clone();
                let world_matrix = (entry.world_matrix).clone();
                set_matrix3_normal_from_matrix4(&mut (*SCRATCH_NORMAL_MATRIX.lock().unwrap()), &{
                    let __flight_source = &(world_matrix);
                    Matrix4Like {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        __flight_entity_runtime: std::sync::Arc::clone(
                            &__flight_source.__flight_entity_runtime,
                        ),
                        m: (__flight_source.m).clone(),
                    }
                });
                let skinned = is_gpu_skinned_draw(&entry.mesh);
                let color_adjusted = (color_adjustment_feature_enabled)
                    && ((((entry.color_matrix).clone()).is_some())
                        || (((entry.color_scale_bias).clone()).is_some()));
                let color_matrix = (color_adjusted) && (((entry.color_matrix).clone()).is_some());
                if ((((((entry.renderer).clone() != bound_renderer)
                    || ((entry.material).clone() != bound_material))
                    || ((entry.light_block).clone() != bound_light_block))
                    || (skinned != bound_skinned))
                    || (color_adjusted != bound_color_adjustment))
                    || (color_matrix != bound_color_matrix)
                {
                    apply_gl_surface_blend_mode(state, &entry.material);
                    runtime.active_color_adjustment_run = color_adjusted;
                    runtime.active_color_matrix_run = color_matrix;
                    runtime.active_skinned_run = skinned;
                    {
                        let __flight_callback = (entry.renderer.bind).clone();
                        let __flight_result = __flight_callback.lock().unwrap()(
                            (*state).clone(),
                            Some((entry.material).clone()),
                            (entry.light_block).clone(),
                            (*camera).clone(),
                        );
                        __flight_result
                    };
                    bound_renderer = Some((entry.renderer).clone());
                    bound_material = Some((entry.material).clone());
                    bound_light_block = Some((entry.light_block).clone());
                    bound_skinned = Some(skinned);
                    bound_color_adjustment = Some(color_adjusted);
                    bound_color_matrix = Some(color_matrix);
                }
                (*PROXY.lock().unwrap()).alpha = Some(entry.alpha);
                (*PROXY.lock().unwrap()).color_scale_bias = if color_adjusted {
                    (entry.color_scale_bias).clone()
                } else {
                    None
                };
                (*PROXY.lock().unwrap()).color_matrix = if color_adjusted {
                    (entry.color_matrix).clone()
                } else {
                    None
                };
                (*PROXY.lock().unwrap()).joint_matrices = if skinned {
                    Some((entry.mesh.skin.as_mut().unwrap().skeleton.joint_matrices).clone())
                } else {
                    None
                };
                (*PROXY.lock().unwrap()).material = (entry.material).clone();
                (*PROXY.lock().unwrap()).normal_matrix =
                    (*SCRATCH_NORMAL_MATRIX.lock().unwrap()).clone();
                (*PROXY.lock().unwrap()).subset = (entry.subset).clone();
                (*PROXY.lock().unwrap()).world_matrix = (world_matrix).clone();
                {
                    let __flight_callback = (entry.renderer.draw).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        (*state).clone(),
                        (*PROXY.lock().unwrap()).clone(),
                        (entry.mesh.geometry).clone(),
                    );
                    __flight_result
                };
                {
                    i += 1.0;
                    i
                };
            }
        }
        crate::host_value::<()>("host.disable");
    }
    draw_gl_scene3_d_particle_emitter3_ds(state, scene, camera, lights);
    invalidate_gl_render_state_cache(state);
}

// Source: upstream/packages/scene3d-gl/src/drawGlScene3D.ts:283 (sha256:6d216f5c4f7ae5a1e896bfa3eb43ae5780a1d36198d6acf818e66b13d24211a3)
fn is_blended_material(material: &Material) -> bool {
    return (({
        let __flight_source = &((*material).clone());
        SurfaceMaterial {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            kind: (__flight_source.kind).clone(),
            name: (__flight_source.name).clone(),
            alpha_cutoff: __flight_source.alpha_cutoff,
            alpha_mode: (__flight_source.alpha_mode).clone(),
            blend_mode: (__flight_source.blend_mode).clone(),
            double_sided: __flight_source.double_sided,
            extensions: (__flight_source.extensions).clone(),
            standard: (__flight_source.standard).clone(),
            shader_key: (__flight_source.shader_key).clone(),
            textures: (__flight_source.textures).clone(),
            uniforms: (__flight_source.uniforms).clone(),
            ..Default::default()
        }
    }
    .alpha_mode)
        .clone()
        == "blend");
}

// Source: upstream/packages/scene3d-gl/src/drawGlScene3D.ts:294 (sha256:1e28623d99ae6630ad6362be0bdd5a0f4907e4660209afcf3791ff953e1669c1)
fn apply_gl_surface_blend_mode(state: &mut GlRenderState, material: &Material) -> () {
    let surface = {
        let __flight_source = &((*material).clone());
        SurfaceMaterial {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            kind: (__flight_source.kind).clone(),
            name: (__flight_source.name).clone(),
            alpha_cutoff: __flight_source.alpha_cutoff,
            alpha_mode: (__flight_source.alpha_mode).clone(),
            blend_mode: (__flight_source.blend_mode).clone(),
            double_sided: __flight_source.double_sided,
            extensions: (__flight_source.extensions).clone(),
            standard: (__flight_source.standard).clone(),
            shader_key: (__flight_source.shader_key).clone(),
            textures: (__flight_source.textures).clone(),
            uniforms: (__flight_source.uniforms).clone(),
            ..Default::default()
        }
    };
    let blend_mode = if ((surface.alpha_mode).clone() == "blend") && ("string" == "string") {
        (surface.blend_mode).clone()
    } else {
        (blend_mode_constant.normal).clone()
    };
    if ((state.apply_blend_mode).clone()).is_none() {
        enable_gl_blend_mode_support(state);
    }
    {
        let __flight_callback = state.apply_blend_mode.as_ref().unwrap().clone();
        let __flight_result =
            __flight_callback.lock().unwrap()((*state).clone(), Some((blend_mode).clone()));
        __flight_result
    };
}

// Source: upstream/packages/scene3d-gl/src/drawGlScene3D.ts:302 (sha256:aba2248f760532972c4fc2529ee87b8e3b36582620b16385592ba95ec5ce5d81)
fn has_excess_forward_lights(lights: &Scene3DLightsLike) -> bool {
    return ((lights.point.as_ref().map(|value| value.len() as f64)).unwrap_or(0.0_f64)
        > max_forward_lights_constant)
        || ((lights.spot.as_ref().map(|value| value.len() as f64)).unwrap_or(0.0_f64)
            > max_forward_lights_constant);
}

// Source: upstream/packages/scene3d-gl/src/drawGlScene3D.ts:308 (sha256:6fca77ac5ec010a83c405358d6ea1d659af69ce7eeb876a2e824d84f089db5bc)
fn resolve_subset_material(mesh: &Mesh, subset_index: f64) -> Option<Material> {
    return if (subset_index < (mesh.materials.len() as f64)) {
        mesh.materials[subset_index as usize].clone()
    } else {
        None
    };
}

// Source: upstream/packages/scene3d-gl/src/drawGlScene3D.ts:314 (sha256:16c8d1a87ac683e88e541694bcb76cc890caf737fdf31c5bcb4876df6bb496b5)
fn compare_blended_entries_descending(a: &GlScene3DDrawEntry, b: &GlScene3DDrawEntry) -> f64 {
    return (b.clip_w - a.clip_w);
}

// Source: upstream/packages/scene3d-gl/src/drawGlScene3D.ts:320 (sha256:0ca20e9a5696a59e16a03a202983111b3f8f2e6c2b322acd684d11b527875a72)
#[derive(Clone)]
struct DrawEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: f64,
    pub clip_w: f64,
    pub color_matrix: Option<Vec<f64>>,
    pub color_scale_bias: Option<ColorScaleBias>,
    pub light_block: Scene3DLightBlock,
    pub material: Material,
    pub mesh: Mesh,
    pub renderer: GlMeshMaterialRenderer,
    pub subset: MeshSubset,
    pub world_matrix: Matrix4,
}
impl PartialEq for DrawEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene3d-gl/src/drawGlScene3D.ts:334 (sha256:5ad90aabf877911a5dd7222cd71aa450ec951a81568884248d96827c0015391f)
fn acquire_draw_entry(pool: &mut Vec<GlScene3DDrawEntry>) -> GlScene3DDrawEntry {
    if ((pool.len() as f64) > 0.0_f64) {
        return pool.pop().expect("TypeScript Array.pop returned undefined");
    }
    return create_draw_entry();
}

// Source: upstream/packages/scene3d-gl/src/drawGlScene3D.ts:342 (sha256:b3ea257296105682dec1e067ab3c1cfe64f1054ab0a11f0e31b66673dd49131f)
fn recycle_draw_entries(
    entries: &mut Vec<GlScene3DDrawEntry>,
    pool: &mut Vec<GlScene3DDrawEntry>,
) -> () {
    while ((entries.len() as f64) > 0.0_f64) {
        pool.push(
            entries
                .pop()
                .expect("TypeScript Array.pop returned undefined"),
        );
    }
}

// Source: upstream/packages/scene3d-gl/src/drawGlScene3D.ts:346 (sha256:029c8419ae668c6c1025939cf56cf00523f6a94fc0f37012bf0a300a4d44ba71)
#[derive(Clone, Default)]
struct CreateDrawEntryRecord7 {
    __flight_identity: std::sync::Arc<()>,
    index_count: f64,
    index_offset: f64,
}
impl PartialEq for CreateDrawEntryRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn create_draw_entry() -> GlScene3DDrawEntry {
    return GlScene3DDrawEntry {
        __flight_identity: std::sync::Arc::new(()),
        alpha: 1.0_f64,
        clip_w: 0.0_f64,
        color_matrix: None,
        color_scale_bias: None,
        light_block: None,
        material: {
            let __flight_portable_source = ((*DEFAULT_MATERIAL).clone()).clone();
            crate::FlightValue::Record({
                let mut __flight_record = Vec::new();
                __flight_record.push((
                    "kind".to_owned(),
                    (&((&__flight_portable_source).kind)).clone(),
                ));
                __flight_record
            })
        },
        mesh: crate::FlightValue::Null,
        renderer: crate::FlightValue::Null,
        subset: crate::FlightValue::Record({
            let mut __flight_record = Vec::new();
            let __flight_key_0 = "indexCount".to_owned();
            let __flight_value_0 = {
                let __flight_portable_source = 0.0_f64;
                crate::FlightValue::Number(*(&__flight_portable_source) as f64)
            };
            if let Some((_, __flight_existing)) = __flight_record
                .iter_mut()
                .find(|(existing, _)| existing == &__flight_key_0)
            {
                *__flight_existing = __flight_value_0;
            } else {
                __flight_record.push((__flight_key_0, __flight_value_0));
            }
            let __flight_key_1 = "indexOffset".to_owned();
            let __flight_value_1 = {
                let __flight_portable_source = 0.0_f64;
                crate::FlightValue::Number(*(&__flight_portable_source) as f64)
            };
            if let Some((_, __flight_existing)) = __flight_record
                .iter_mut()
                .find(|(existing, _)| existing == &__flight_key_1)
            {
                *__flight_existing = __flight_value_1;
            } else {
                __flight_record.push((__flight_key_1, __flight_value_1));
            }
            __flight_record
        }),
        world_matrix: {
            let __flight_portable_source = create_matrix4(
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None,
            );
            crate::FlightValue::Record({
                let mut __flight_record = Vec::new();
                __flight_record.push((
                    "m".to_owned(),
                    crate::FlightValue::Array(
                        (&((&__flight_portable_source).m))
                            .iter()
                            .map(|value| crate::FlightValue::Number((*value) as f64))
                            .collect(),
                    ),
                ));
                __flight_record
            })
        },
    };
}

// Source: upstream/packages/scene3d-gl/src/drawGlScene3D.ts:363 (sha256:8781c2525cd16efb0856160b6ea4132ec62e971815fec5bda8d4d3da4a70b27a)
static PROXY: std::sync::LazyLock<std::sync::Mutex<Scene3DRenderProxy>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(Scene3DRenderProxy {
            __flight_identity: std::sync::Arc::new(()),
            color_matrix: None,
            color_scale_bias: None,
            joint_matrices: None,
            material: Material {
                __flight_identity: std::sync::Arc::new(()),
                __flight_entity_runtime: Default::default(),
                kind: (standard_material_kind_constant).to_owned(),
                name: None,
                ..Default::default()
            },
            normal_matrix: create_matrix3(None, None, None, None, None, None, None, None, None),
            subset: MeshSubset {
                __flight_identity: std::sync::Arc::new(()),
                index_count: 0.0_f64,
                index_offset: 0.0_f64,
            },
            world_matrix: create_matrix4(
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None,
            ),
            alpha: None,
        })
    });

// Source: upstream/packages/scene3d-gl/src/drawGlScene3D.ts:375 (sha256:c6eaa59fd2a158ddc09a3fecd1817fe824326df73cc3e1339b0566eab2c77550)
static DEFAULT_MATERIAL: std::sync::LazyLock<Material> = std::sync::LazyLock::new(|| Material {
    __flight_identity: std::sync::Arc::new(()),
    __flight_entity_runtime: Default::default(),
    kind: (standard_material_kind_constant).to_owned(),
    name: None,
    ..Default::default()
});

// Source: upstream/packages/scene3d-gl/src/drawGlScene3D.ts:377 (sha256:590e41ce5748e09578c54b48aa738f2cb9712864e3096ae578ed46a44c4cea6f)
static SCRATCH_NORMAL_MATRIX: std::sync::LazyLock<std::sync::Mutex<Matrix3>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix3(
            None, None, None, None, None, None, None, None, None,
        ))
    });
