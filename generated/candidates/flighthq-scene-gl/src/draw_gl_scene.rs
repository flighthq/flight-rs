// @generated from upstream/packages/scene-gl/src/drawGlScene.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GlSceneDrawEntry, draw_gl_scene_particle_emitters, get_gl_scene_runtime,
    resolve_gl_mesh_material_renderer,
};
use flighthq_geometry::{create_matrix3, create_matrix4, set_matrix3_normal_from_matrix4};
use flighthq_mesh::has_mesh_geometry_skin;
use flighthq_node::get_node_world_matrix4;
use flighthq_render::prepare_scene_render;
use flighthq_render_gl::{declare_gl_render_target_color_space, invalidate_gl_render_state_cache};
use flighthq_scene::{get_scene_node_world_alpha, update_mesh_morph};
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, Camera, ColorTransform,
    DEFAULT_MATERIAL_KIND as default_material_kind_constant, DisplayObjectClipHooks,
    GlBitmapShader, GlBlendRealization, GlColorAdjustmentFold, GlColorTransformInstancedShader,
    GlCompressedTextureDecoder, GlCompressedTextureUploader, GlMeshMaterialRenderer,
    GlParticleShader, GlQuadBatchShader, GlRenderState, GlRenderTarget, GlShaderLocations,
    GlShapeMeshColorTransformShader, GlUniformColorTransformShader, ImageResource,
    InteractionSignals, Kind, Material, Matrix, Matrix3, Matrix4, Matrix4Like, Mesh,
    MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose, MeshSkinBindPose, MeshSubset,
    Node, NodeData, NodeInteractionState, NodeSignals, NodeTraitsKey, Quaternion, Rectangle,
    RenderProxy, RenderProxy2D, RenderProxyAdapter, RenderState, Renderable, Renderer, Sampler,
    SceneGraphSyncPolicy, SceneLights, SceneNode, SceneRenderProxy, SceneResourceRef, Stage,
    StageSignals, SurfaceMaterial, TextureColorSpace, TextureFilter, TextureWrap, Transform3DNode,
    Vector2, Vector3,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub allow_smoothing: Option<bool>,
    pub background_color: Option<f64>,
    pub background_color_rgba: Option<Vec<f64>>,
    pub background_color_string: Option<String>,
    pub current_clip_depth: Option<f64>,
    pub display_object_clip_hooks: Option<DisplayObjectClipHooks>,
    pub pixel_ratio: Option<f64>,
    pub render_alpha: Option<f64>,
    pub render_blend_mode: Option<BlendMode>,
    pub render_transform2_d: Option<Matrix>,
    pub scene_graph_sync_policy: Option<SceneGraphSyncPolicy>,
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
    pub binding: Option<crate::OpaqueHostValue>,
    pub bounds_rectangle: Option<Rectangle>,
    pub bounds_using_local_bounds_id: Option<f64>,
    pub bounds_using_local_transform_id: Option<f64>,
    pub build_text_layout_params: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(TextLabel, TextMeasureFunction) -> TextLayoutParams + Send + 'static>,
            >,
        >,
    >,
    pub canvas_texture_view: Option<crate::OpaqueHostValue>,
    pub canvas_view_cleared: Option<bool>,
    pub clip_contour_pipelines: Option<Vec<(crate::OpaqueHostValue, WgpuClipContourPipelines)>>,
    pub clip_contour_stack: Option<Vec<WgpuClipContourEntry>>,
    pub clip_forms: Option<Vec<String>>,
    pub color_adjustment_channel_mixing_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RenderState, Renderable) -> () + Send + 'static>>,
        >,
    >,
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub color_adjustments_channel_mixing: Option<bool>,
    pub color_transform_instanced_shader: Option<GlColorTransformInstancedShader>,
    pub command_encoder: Option<crate::OpaqueHostValue>,
    pub compressed_texture_decoder: Option<GlCompressedTextureDecoder>,
    pub compressed_texture_upload: Option<GlCompressedTextureUploader>,
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
    pub current_render_target: Option<GlRenderTarget>,
    pub current_texture: Option<crate::OpaqueHostValue>,
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
    pub element: Option<crate::OpaqueHostValue>,
    pub frame_capture_buffer: Option<crate::OpaqueHostValue>,
    pub frame_capture_bytes_per_row: Option<f64>,
    pub frame_capture_enabled: Option<bool>,
    pub frame_capture_height: Option<f64>,
    pub frame_capture_texture: Option<crate::OpaqueHostValue>,
    pub frame_capture_width: Option<f64>,
    pub gl_blend_mode_registry: Option<Vec<(BlendMode, GlBlendRealization)>>,
    pub gl_color_adjustment_fold: Option<GlColorAdjustmentFold>,
    pub gl_color_adjustment_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(GlRenderState, ColorTransform) -> () + Send + 'static>>,
        >,
    >,
    pub image_smoothing_enabled: Option<bool>,
    pub image_smoothing_quality: Option<crate::OpaqueHostValue>,
    pub input: Option<TextInputState>,
    pub instance_velocities: Option<Vec<f32>>,
    pub interaction_signals: Option<InteractionSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub linear_sampler: Option<crate::OpaqueHostValue>,
    pub local_bounds_id: Option<f64>,
    pub local_bounds_rectangle: Option<Rectangle>,
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
    pub mipmap_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub mipmapped_textures: Option<Vec<crate::OpaqueHostValue>>,
    pub mipmap_pipeline: Option<crate::OpaqueHostValue>,
    pub morph_bind_pose: Option<MeshMorphBindPose>,
    pub movie_clip_signals: Option<MovieClipSignals>,
    pub nearest_sampler: Option<crate::OpaqueHostValue>,
    pub node_signals: Option<NodeSignals>,
    pub particle_corner_buffer: Option<crate::OpaqueHostValue>,
    pub particle_instance_capacity: Option<f64>,
    pub particle_shader: Option<GlParticleShader>,
    pub pipeline_cache: Option<Vec<(String, crate::OpaqueHostValue)>>,
    pub quad_batch_corner_buffer: Option<crate::OpaqueHostValue>,
    pub quad_batches: Option<Vec<QuadBatch>>,
    pub quad_batch_shader: Option<GlQuadBatchShader>,
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
    pub renderer_map: Option<Vec<(Kind, Renderer)>>,
    pub renderer_map_id: Option<f64>,
    pub render_pass: Option<crate::OpaqueHostValue>,
    pub render_proxy_adapter_map: Option<Vec<(Renderable, RenderProxyAdapter)>>,
    pub render_proxy_map: Option<Vec<(Renderable, RenderProxy)>>,
    pub render_target_stack: Option<Vec<WgpuSavedPassState>>,
    pub resolved_color_transform: Option<ColorTransform>,
    pub retired_buffers: Option<Vec<crate::OpaqueHostValue>>,
    pub rich_text_content: Option<RichTextContent>,
    pub rotation_angle: Option<f64>,
    pub rotation_cosine: Option<f64>,
    pub rotation_sine: Option<f64>,
    pub sampler_cache: Option<Vec<(String, crate::OpaqueHostValue)>>,
    pub scene_mesh_upload_cache: Option<Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>>,
    pub selection_begin_index: Option<f64>,
    pub selection_end_index: Option<f64>,
    pub shader_loc: Option<GlShaderLocations>,
    pub shape_mesh_color_transform_shader: Option<GlShapeMeshColorTransformShader>,
    pub shape_mesh_pipelines: Option<Vec<(crate::OpaqueHostValue, WgpuShapeMeshPipeline)>>,
    pub skin_bind_pose: Option<MeshSkinBindPose>,
    pub sprite_batch_blend_mode: Option<BlendMode>,
    pub sprite_batch_buffer_cursor: Option<f64>,
    pub sprite_batch_buffer_pool: Option<Vec<WgpuSpriteBatchBufferSlot>>,
    pub sprite_batch_color_transform_buffer: Option<crate::OpaqueHostValue>,
    pub sprite_batch_color_transform_data: Option<Vec<f32>>,
    pub sprite_batch_color_transform_mode: Option<f64>,
    pub sprite_batch_count: Option<f64>,
    pub sprite_batch_instance_buffer: Option<crate::OpaqueHostValue>,
    pub sprite_batch_instance_data: Option<Vec<f32>>,
    pub sprite_batch_material: Option<Material>,
    pub sprite_batch_material_buffer: Option<crate::OpaqueHostValue>,
    pub sprite_batch_material_data: Option<Vec<f32>>,
    pub sprite_batch_material_floats: Option<f64>,
    pub sprite_batch_texture: Option<ImageResource>,
    pub sprite_batch_uniform_color_transform: Option<ColorTransform>,
    pub stage: Option<Stage>,
    pub stage_signals: Option<StageSignals>,
    pub temp_stack: Option<Vec<Renderable>>,
    pub text_field_signals: Option<TextFieldSignals>,
    pub text_layout: Option<TextLayoutResult>,
    pub text_layout_using_content_id: Option<f64>,
    pub texture_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub uniform_bind_group: Option<crate::OpaqueHostValue>,
    pub uniform_bind_group_layout: Option<crate::OpaqueHostValue>,
    pub uniform_buffer: Option<crate::OpaqueHostValue>,
    pub uniform_color_transform_shader: Option<GlUniformColorTransformShader>,
    pub uniform_data: Option<Vec<f32>>,
    pub uniform_data_u32: Option<Vec<u32>>,
    pub uniform_offset: Option<f64>,
    pub uniform_stride: Option<f64>,
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
    pub wgpu_color_adjustment_fold: Option<WgpuColorAdjustmentFold>,
    pub wgpu_color_adjustment_guard: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(WgpuRenderState, ColorTransform) -> () + Send + 'static>,
            >,
        >,
    >,
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

#[derive(Clone, Default)]
pub struct FlightPartialRecord4 {
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
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub uv_offset: Option<Vector2>,
    pub uv_rotation: Option<f64>,
    pub uv_scale: Option<Vector2>,
    pub color_space: Option<TextureColorSpace>,
    pub image: Option<ImageResource>,
    pub resource: Option<SceneResourceRef>,
    pub sampler: Option<Sampler>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord7 {
    pub __flight_identity: std::sync::Arc<()>,
    pub position: Option<Vector3>,
    pub rotation: Option<Quaternion>,
    pub scale: Option<Vector3>,
}
impl PartialEq for FlightPartialRecord7 {
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

// Source: upstream/packages/scene-gl/src/drawGlScene.ts:33 (sha256:d316a3033910c43ecff8210be187afcdc95c3ed252fe4d5ab0cffc90dd82f57a)
fn is_gpu_skinned_draw(mesh: &Mesh) -> bool {
    return (((mesh.skin).clone()).is_some()) && (has_mesh_geometry_skin(&mesh.geometry));
}

// Source: upstream/packages/scene-gl/src/drawGlScene.ts:60 (sha256:23e09aaf364735a637a7b56f94b3bbac36337aad65b9041242155ef9c06a6bcc)
pub fn draw_gl_scene(
    state: &mut GlRenderState,
    scene: &SceneNode,
    camera: &Camera,
    lights: &SceneLights,
) -> () {
    let mut list = prepare_scene_render(
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
    );
    let mut runtime = get_gl_scene_runtime(state);
    if (!declare_gl_render_target_color_space(state, "linear".to_owned())) {
        {
            let __flight_callback = (runtime.color_space_guard).clone();
            __flight_callback
                .as_ref()
                .map(|callback| callback.lock().unwrap()())
        };
    }
    runtime.opaque_draw_list.clear();
    runtime.blended_draw_list.clear();
    {
        let mut m = 0.0_f64;
        while (m < list.mesh_count) {
            let mut mesh = list.visible_meshes[m as usize].clone();
            if ((mesh.morph).clone()).is_some() {
                update_mesh_morph(&mut mesh);
            }
            {
                m += 1.0;
                m
            };
        }
    }
    {
        let mut m = 0.0_f64;
        while (m < list.mesh_count) {
            let mut mesh = list.visible_meshes[m as usize].clone();
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
            let object_alpha = get_scene_node_world_alpha(&{
                let __flight_source = &(mesh);
                SceneNode {
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
            {
                let mut s = 0.0_f64;
                while (s < (mesh.geometry.subsets.len() as f64)) {
                    let material = resolve_subset_material(&mesh, s);
                    let renderer =
                        resolve_gl_mesh_material_renderer(state, ((material).clone()).clone());
                    if (renderer).is_none() {
                        {
                            s += 1.0;
                            s
                        };
                        continue;
                    }
                    let resolved_material = (material).unwrap_or((*DEFAULT_MATERIAL).clone());
                    let is_blended =
                        (is_blended_material(&resolved_material)) || (object_alpha < 1.0_f64);
                    let mut entry = if is_blended {
                        acquire_blended_entry(&mut runtime.blended_pool)
                    } else {
                        acquire_opaque_entry(&mut runtime.opaque_pool)
                    };
                    entry.alpha = object_alpha;
                    entry.clip_w = clip_w;
                    entry.mesh = mesh;
                    entry.material = resolved_material;
                    entry.normal_matrix = world_matrix;
                    entry.renderer = (renderer).clone().unwrap();
                    entry.subset = mesh.geometry.subsets[s as usize].clone();
                    entry.world_matrix = world_matrix;
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
    let mut bound_renderer: Option<GlMeshMaterialRenderer> = None;
    let mut bound_skinned: Option<bool> = None;
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
            if (((entry.renderer).clone() != bound_renderer)
                || ((entry.material).clone() != bound_material))
                || (skinned != bound_skinned)
            {
                runtime.active_skinned_run = skinned;
                {
                    let __flight_callback = (entry.renderer.bind).clone();
                    let __flight_result = __flight_callback.lock().unwrap()(
                        (*state).clone(),
                        Some((entry.material).clone()),
                        (list.lights).clone(),
                        (*camera).clone(),
                    );
                    __flight_result
                };
                bound_renderer = Some((entry.renderer).clone());
                bound_material = Some((entry.material).clone());
                bound_skinned = Some(skinned);
            }
            (*PROXY.lock().unwrap()).alpha = Some(entry.alpha);
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
        crate::host_value::<()>("host.blendFunc");
        bound_material = None;
        bound_renderer = None;
        bound_skinned = None;
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
                if (((entry.renderer).clone() != bound_renderer)
                    || ((entry.material).clone() != bound_material))
                    || (skinned != bound_skinned)
                {
                    runtime.active_skinned_run = skinned;
                    {
                        let __flight_callback = (entry.renderer.bind).clone();
                        let __flight_result = __flight_callback.lock().unwrap()(
                            (*state).clone(),
                            Some((entry.material).clone()),
                            (list.lights).clone(),
                            (*camera).clone(),
                        );
                        __flight_result
                    };
                    bound_renderer = Some((entry.renderer).clone());
                    bound_material = Some((entry.material).clone());
                    bound_skinned = Some(skinned);
                }
                (*PROXY.lock().unwrap()).alpha = Some(entry.alpha);
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
    draw_gl_scene_particle_emitters(state, scene, camera, lights);
    invalidate_gl_render_state_cache(state);
}

// Source: upstream/packages/scene-gl/src/drawGlScene.ts:227 (sha256:6d216f5c4f7ae5a1e896bfa3eb43ae5780a1d36198d6acf818e66b13d24211a3)
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
            alpha_type: (__flight_source.alpha_type).clone(),
            blend_mode: (__flight_source.blend_mode).clone(),
            double_sided: __flight_source.double_sided,
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

// Source: upstream/packages/scene-gl/src/drawGlScene.ts:233 (sha256:6fca77ac5ec010a83c405358d6ea1d659af69ce7eeb876a2e824d84f089db5bc)
fn resolve_subset_material(mesh: &Mesh, subset_index: f64) -> Option<Material> {
    return if (subset_index < (mesh.materials.len() as f64)) {
        mesh.materials[subset_index as usize].clone()
    } else {
        None
    };
}

// Source: upstream/packages/scene-gl/src/drawGlScene.ts:239 (sha256:020f2dbc582dcaf9ed4c05d1f7369590b7860c3a74fa7650316bf1d14e93790c)
fn compare_blended_entries_descending(a: &GlSceneDrawEntry, b: &GlSceneDrawEntry) -> f64 {
    return (b.clip_w - a.clip_w);
}

// Source: upstream/packages/scene-gl/src/drawGlScene.ts:245 (sha256:273ccc2260ed2538bfb9e73f9cc7535ba9f832369063ca1b13972f667a2ea6c5)
#[derive(Clone)]
struct DrawEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: f64,
    pub clip_w: f64,
    pub material: Material,
    pub mesh: Mesh,
    pub normal_matrix: Matrix4,
    pub renderer: GlMeshMaterialRenderer,
    pub subset: MeshSubset,
    pub world_matrix: Matrix4,
}
impl PartialEq for DrawEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-gl/src/drawGlScene.ts:257 (sha256:b1905de104ae7d121bf8c2f453569c96ba42d9b47abd0a7e0005ed9168d2c139)
fn acquire_opaque_entry(pool: &mut Vec<GlSceneDrawEntry>) -> GlSceneDrawEntry {
    if ((pool.len() as f64) > 0.0_f64) {
        return pool.pop().expect("TypeScript Array.pop returned undefined");
    }
    return create_draw_entry();
}

// Source: upstream/packages/scene-gl/src/drawGlScene.ts:262 (sha256:af5b8cf3d2e638c3fdab905a4d8d0695e8058f63b80c20abdf9218652c3b9c02)
fn acquire_blended_entry(pool: &mut Vec<GlSceneDrawEntry>) -> GlSceneDrawEntry {
    if ((pool.len() as f64) > 0.0_f64) {
        return pool.pop().expect("TypeScript Array.pop returned undefined");
    }
    return create_draw_entry();
}

// Source: upstream/packages/scene-gl/src/drawGlScene.ts:267 (sha256:5a55a79586f24eaec2c2f1ef9d7ca368e76c5cdefcf6121761f37f3e949dad7a)
#[derive(Clone, Default)]
struct CreateDrawEntryRecord8 {
    __flight_identity: std::sync::Arc<()>,
    index_count: f64,
    index_offset: f64,
}
impl PartialEq for CreateDrawEntryRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn create_draw_entry() -> GlSceneDrawEntry {
    return GlSceneDrawEntry {
        __flight_identity: std::sync::Arc::new(()),
        alpha: 1.0_f64,
        clip_w: 0.0_f64,
        material: (*DEFAULT_MATERIAL).clone(),
        mesh: crate::OpaqueHostValue::Null,
        normal_matrix: create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ),
        renderer: crate::OpaqueHostValue::Null,
        subset: MeshSubset {
            __flight_identity: std::sync::Arc::new(()),
            index_count: 0.0_f64,
            index_offset: 0.0_f64,
        },
        world_matrix: create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ),
    };
}

// Source: upstream/packages/scene-gl/src/drawGlScene.ts:282 (sha256:ffa8fb73c97a70eb1f4177029b1983ed7b69c9ad757b2d905f3ecfb720ec47b7)
static PROXY: std::sync::LazyLock<std::sync::Mutex<SceneRenderProxy>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(SceneRenderProxy {
            __flight_identity: std::sync::Arc::new(()),
            joint_matrices: None,
            material: Material {
                __flight_identity: std::sync::Arc::new(()),
                __flight_entity_runtime: Default::default(),
                kind: (default_material_kind_constant).to_owned(),
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

// Source: upstream/packages/scene-gl/src/drawGlScene.ts:292 (sha256:8db138954744b37f13b6de8e07b798befcd977f4ef6adc006e52c5beb8faf5d3)
static DEFAULT_MATERIAL: std::sync::LazyLock<Material> = std::sync::LazyLock::new(|| Material {
    __flight_identity: std::sync::Arc::new(()),
    __flight_entity_runtime: Default::default(),
    kind: (default_material_kind_constant).to_owned(),
    name: None,
    ..Default::default()
});

// Source: upstream/packages/scene-gl/src/drawGlScene.ts:294 (sha256:590e41ce5748e09578c54b48aa738f2cb9712864e3096ae578ed46a44c4cea6f)
static SCRATCH_NORMAL_MATRIX: std::sync::LazyLock<std::sync::Mutex<Matrix3>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix3(
            None, None, None, None, None, None, None, None, None,
        ))
    });
