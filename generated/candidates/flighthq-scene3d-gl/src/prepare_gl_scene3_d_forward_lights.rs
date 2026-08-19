// @generated from upstream/packages/scene3d-gl/src/prepareGlScene3DForwardLights.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::{create_aabb, create_bounding_sphere, set_bounding_sphere_from_aabb};
use flighthq_lighting::select_scene3_d_forward_lights;
use flighthq_node::get_node_world_matrix4;
use flighthq_render::pack_scene3_d_light_block;
use flighthq_scene3d::get_node3_d_world_bounds;
use flighthq_types::{
    Aabb, AabbLike, Adjustment, BlendMode, BoundingSphere, BoundingSphereLike, BoundsNodeAny,
    CanvasShapeCommand, CanvasTextureResolvers, ColorScaleBias, ExternalTexture, GlBitmapShader,
    GlBlendRealization, GlColorAdjustmentMaterialFeature, GlColorScaleBiasInstancedShader,
    GlCompressedTextureDecoder, GlParticleShader, GlQuadBatchShader, GlRenderEffectRunner,
    GlRenderState, GlRenderTextureEntry, GlRenderTextureGuard, GlScene3DForwardLightList,
    GlShaderLocations, GlShapeMeshColorScaleBiasShader, GlTextureResolver,
    GlUniformColorScaleBiasShader, InteractionSignals, Kind, Material, Matrix, Matrix4, Mesh,
    MeshGeometry, MeshGeometryGlData, MeshGeometryWgpuData, MeshMorph, MeshMorphBindPose,
    MeshSkinBindPose, Node, NodeData, NodeInteractionState, NodeSignals, NodeTraitsKey, Path,
    PathMesh, Quaternion, Rectangle, RenderEffectPaddingResolver, RenderProxy, RenderProxy2D,
    RenderProxyAdapter, RenderState, RenderTexture, Renderable, Renderer,
    SCENE_LIGHT_BLOCK_FLOATS as scene_light_block_floats_constant, SamplerLike, Scene2D,
    Scene2DSignals, Scene3DForwardLightSelection, Scene3DLightBlock, Scene3DLightsLike,
    Scene3DRenderList, ShapeRasterizer, Skin, StrokeStyle, Texture, TextureSourceKind,
    TintMaterialData, Transform3DNode, Vector3,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
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
    pub geometry: Option<MeshGeometry>,
    pub materials: Option<Vec<Option<Material>>>,
    pub morph: Option<MeshMorph>,
    pub skin: Option<Skin>,
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
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub position: Option<Vector3>,
    pub rotation: Option<Quaternion>,
    pub scale: Option<Vector3>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene3d-gl/src/prepareGlScene3DForwardLights.ts:26 (sha256:9415859296e744190259ce38e2ae1c42f976708b592c2996d1098e9781a64906)
pub fn prepare_gl_scene3_d_forward_lights(
    state: &GlRenderState,
    scene_render_list: &mut Scene3DRenderList,
    lights: &Scene3DLightsLike,
) -> GlScene3DForwardLightList {
    let mut prepared = ensure_prepared_gl_scene3_d_forward_lights(state);
    prepared.block_count = 0.0_f64;
    prepared
        .list
        .mesh_light_blocks
        .truncate((scene_render_list.mesh_count) as usize);
    prepared.list.mesh_count = scene_render_list.mesh_count;
    {
        let mut mesh_index = 0.0_f64;
        while (mesh_index < scene_render_list.mesh_count) {
            let mut mesh = scene_render_list.visible_meshes[mesh_index as usize].clone();
            set_mesh_world_bounding_sphere(&mut mesh);
            select_scene3_d_forward_lights(&mut prepared.selection, lights, &{
                let __flight_source = &(*SCRATCH_WORLD_SPHERE.lock().unwrap());
                BoundingSphereLike {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    __flight_entity_runtime: std::sync::Arc::clone(
                        &__flight_source.__flight_entity_runtime,
                    ),
                    center: (__flight_source.center).clone(),
                    radius: __flight_source.radius,
                }
            });
            let mut block_index = find_prepared_block(&prepared, &prepared.selection.indices);
            if (block_index < 0.0_f64) {
                block_index = {
                    prepared.block_count += 1.0;
                    prepared.block_count
                };
                let mut block = ensure_prepared_block(&mut prepared, block_index);
                copy_indices(&mut block.indices, &prepared.selection.indices);
                (*SELECTED_LIGHTS.lock().unwrap()).ambient = (lights.ambient).clone();
                (*SELECTED_LIGHTS.lock().unwrap()).directional = (lights.directional).clone();
                (*SELECTED_LIGHTS.lock().unwrap()).hemisphere = (lights.hemisphere).clone();
                (*SELECTED_LIGHTS.lock().unwrap()).point = Some((prepared.selection.point).clone());
                (*SELECTED_LIGHTS.lock().unwrap()).spot = Some((prepared.selection.spot).clone());
                pack_scene3_d_light_block(&mut block.lights, &(*SELECTED_LIGHTS.lock().unwrap()));
            }
            {
                let __flight_index = (mesh_index) as usize;
                let __flight_value = (prepared.blocks[block_index as usize].lights).clone();
                if __flight_index == prepared.list.mesh_light_blocks.len() {
                    prepared.list.mesh_light_blocks.push(__flight_value);
                } else {
                    prepared.list.mesh_light_blocks[__flight_index] = __flight_value;
                }
            };
            {
                mesh_index += 1.0;
                mesh_index
            };
        }
    }
    return prepared.list;
}

// Source: upstream/packages/scene3d-gl/src/prepareGlScene3DForwardLights.ts:60 (sha256:5094df56ca1c1001e81b8fafa6fcc6afc988ef49fd4ef2bcc2657c88964d4391)
fn copy_indices(out: &mut Vec<f64>, indices: &Vec<f64>) -> () {
    out.truncate((indices.len() as f64) as usize);
    {
        let mut i = 0.0_f64;
        while (i < (indices.len() as f64)) {
            {
                let __flight_index = (i) as usize;
                let __flight_value = indices[i as usize].clone();
                if __flight_index == out.len() {
                    out.push(__flight_value);
                } else {
                    out[__flight_index] = __flight_value;
                }
            };
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/scene3d-gl/src/prepareGlScene3DForwardLights.ts:65 (sha256:565a6dcb8742f44864b9ad4b769abe754a7fa23bab4519ce446a590e4c8e831c)
fn create_scene3_d_light_block() -> Scene3DLightBlock {
    return Scene3DLightBlock {
        __flight_identity: std::sync::Arc::new(()),
        ambient_count: 0.0_f64,
        data: vec![0.0_f32; (scene_light_block_floats_constant) as usize],
        directional_count: 0.0_f64,
        hemisphere_count: 0.0_f64,
        point_count: 0.0_f64,
        spot_count: 0.0_f64,
        version: 0.0_f64,
    };
}

// Source: upstream/packages/scene3d-gl/src/prepareGlScene3DForwardLights.ts:77 (sha256:d08750238ce74673b5b5ad0ddc1a8ed352ca455b7c38179e798bdaeea2072d15)
fn ensure_prepared_block(
    prepared: &mut PreparedGlScene3DForwardLights,
    index: f64,
) -> PreparedForwardLightBlock {
    let mut block = prepared.blocks[index as usize].clone();
    if ((block).clone()).is_none() {
        block = PreparedForwardLightBlock {
            __flight_identity: std::sync::Arc::new(()),
            indices: vec![],
            lights: create_scene3_d_light_block(),
        };
        {
            let __flight_index = (index) as usize;
            let __flight_value = (block).clone();
            if __flight_index == prepared.blocks.len() {
                prepared.blocks.push(__flight_value);
            } else {
                prepared.blocks[__flight_index] = __flight_value;
            }
        };
    }
    return block;
}

// Source: upstream/packages/scene3d-gl/src/prepareGlScene3DForwardLights.ts:86 (sha256:d75a908efba7da08924241a010125a146df6910556f117b20efba614e61437e4)
fn ensure_prepared_gl_scene3_d_forward_lights(
    state: &GlRenderState,
) -> PreparedGlScene3DForwardLights {
    let mut prepared = (*PREPARED_GL_SCENE3_D_FORWARD_LIGHTS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if ((prepared).clone()).is_none() {
        prepared = Some(PreparedGlScene3DForwardLights {
            __flight_identity: std::sync::Arc::new(()),
            block_count: 0.0_f64,
            blocks: vec![],
            list: GlScene3DForwardLightList {
                __flight_identity: std::sync::Arc::new(()),
                mesh_count: 0.0_f64,
                mesh_light_blocks: vec![],
            },
            selection: Scene3DForwardLightSelection {
                __flight_identity: std::sync::Arc::new(()),
                indices: vec![],
                point: vec![],
                spot: vec![],
            },
        });
        {
            let __flight_key = (*state).clone();
            let __flight_value = ((prepared).clone()).clone().unwrap();
            if let Some((_, value)) = (*PREPARED_GL_SCENE3_D_FORWARD_LIGHTS.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*PREPARED_GL_SCENE3_D_FORWARD_LIGHTS.lock().unwrap())
                    .push((__flight_key, __flight_value));
            }
        };
    }
    return ((prepared).clone().unwrap()).clone();
}

// Source: upstream/packages/scene3d-gl/src/prepareGlScene3DForwardLights.ts:100 (sha256:1cde745f57ebedd7dafc9b882c08fd1fdb674919ba44fffa2ce609851fdfcb61)
fn find_prepared_block(prepared: &PreparedGlScene3DForwardLights, indices: &Vec<f64>) -> f64 {
    {
        let mut block_index = 0.0_f64;
        while (block_index < prepared.block_count) {
            if ((prepared.blocks[block_index as usize].indices.len() as f64)
                != (indices.len() as f64))
            {
                {
                    block_index += 1.0;
                    block_index
                };
                continue;
            }
            let mut equal = true;
            {
                let mut i = 0.0_f64;
                while (i < (indices.len() as f64)) {
                    if (prepared.blocks[block_index as usize].indices[i as usize].clone()
                        != indices[i as usize].clone())
                    {
                        equal = false;
                        break;
                    }
                    {
                        i += 1.0;
                        i
                    };
                }
            }
            if equal {
                return block_index;
            }
            {
                block_index += 1.0;
                block_index
            };
        }
    }
    return (-1.0_f64);
}

// Source: upstream/packages/scene3d-gl/src/prepareGlScene3DForwardLights.ts:116 (sha256:523001aeb7e31e5c35f0a7854d1143d5d07ff7d8cb14410d245761a7c6c99b1b)
fn set_mesh_world_bounding_sphere(mesh: &mut Mesh) -> () {
    get_node3_d_world_bounds(&mut (*SCRATCH_WORLD_BOUNDS.lock().unwrap()), mesh);
    set_bounding_sphere_from_aabb(&mut (*SCRATCH_WORLD_SPHERE.lock().unwrap()), &{
        let __flight_source = &(*SCRATCH_WORLD_BOUNDS.lock().unwrap());
        AabbLike {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            max: (__flight_source.max).clone(),
            min: (__flight_source.min).clone(),
        }
    });
    if ((*SCRATCH_WORLD_SPHERE.lock().unwrap()).radius < 0.0_f64) {
        (*SCRATCH_WORLD_SPHERE.lock().unwrap()).center.x = (get_node_world_matrix4(&{
            let __flight_source = &(mesh);
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
        })
        .m[12.0_f64 as usize] as f64);
        (*SCRATCH_WORLD_SPHERE.lock().unwrap()).center.y = (get_node_world_matrix4(&{
            let __flight_source = &(mesh);
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
        })
        .m[13.0_f64 as usize] as f64);
        (*SCRATCH_WORLD_SPHERE.lock().unwrap()).center.z = (get_node_world_matrix4(&{
            let __flight_source = &(mesh);
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
        })
        .m[14.0_f64 as usize] as f64);
        (*SCRATCH_WORLD_SPHERE.lock().unwrap()).radius = 0.0_f64;
    }
}

// Source: upstream/packages/scene3d-gl/src/prepareGlScene3DForwardLights.ts:128 (sha256:3bedc8f468ba79b6ad966e33678368e0ea42e3d00811f329d434da56d6cdf3a2)
#[derive(Clone, Default)]
struct PreparedForwardLightBlock {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub indices: Vec<f64>,
    pub lights: Scene3DLightBlock,
}
impl PartialEq for PreparedForwardLightBlock {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene3d-gl/src/prepareGlScene3DForwardLights.ts:133 (sha256:743f9f1dfe8a418a64576ce5bda1649be83a65370d2b83b240212c6ba6b75e29)
#[derive(Clone, Default)]
struct PreparedGlScene3DForwardLights {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub block_count: f64,
    pub blocks: Vec<PreparedForwardLightBlock>,
    pub list: GlScene3DForwardLightList,
    pub selection: Scene3DForwardLightSelection,
}
impl PartialEq for PreparedGlScene3DForwardLights {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene3d-gl/src/prepareGlScene3DForwardLights.ts:140 (sha256:a8b44756da1ec08641706572a05466a58898b8d70fcac24da237a9de310370b0)
static PREPARED_GL_SCENE3_D_FORWARD_LIGHTS: std::sync::LazyLock<
    std::sync::Mutex<Vec<(GlRenderState, PreparedGlScene3DForwardLights)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/scene3d-gl/src/prepareGlScene3DForwardLights.ts:141 (sha256:89ec8208fa89a18ea4ca1dc03e1a2258cfd19857419fb3bb28ed213446a9967b)
static SCRATCH_WORLD_BOUNDS: std::sync::LazyLock<std::sync::Mutex<Aabb>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_aabb(None, None, None, None, None, None))
    });

// Source: upstream/packages/scene3d-gl/src/prepareGlScene3DForwardLights.ts:142 (sha256:f2023c8c5a5a5764c3b3594c19e9406768f3d94035e07e5c963da25c6c9e5ddc)
static SCRATCH_WORLD_SPHERE: std::sync::LazyLock<std::sync::Mutex<BoundingSphere>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_bounding_sphere(None, None, None, None))
    });

// Source: upstream/packages/scene3d-gl/src/prepareGlScene3DForwardLights.ts:143 (sha256:de7c9205b8d9cb686183ce48ff0c26d98fd10d8797a89d310f4c51d42b4b9f44)
static SELECTED_LIGHTS: std::sync::LazyLock<std::sync::Mutex<Scene3DLightsLike>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(Scene3DLightsLike {
            __flight_identity: std::sync::Arc::new(()),
            __flight_entity_runtime: Default::default(),
            ambient: None,
            directional: None,
            hemisphere: Some(vec![]),
            point: Some(vec![]),
            spot: Some(vec![]),
        })
    });
