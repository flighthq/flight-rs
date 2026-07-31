// @generated from upstream/packages/scene-wgpu/src/drawWgpuScene.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{draw_wgpu_scene_particle_emitters, resolve_wgpu_mesh_material_renderer};
use flighthq_geometry::{create_matrix3, create_matrix4, set_matrix3_normal_from_matrix4};
use flighthq_node::get_node_world_matrix4;
use flighthq_render::prepare_scene_render;
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, Camera, ColorTransform,
    DEFAULT_MATERIAL_KIND as default_material_kind_constant, DisplayObjectClipHooks, ImageResource,
    InteractionSignals, Kind, Material, Matrix, Matrix3, Matrix4, Matrix4Like, Mesh,
    MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose, MeshSkinBindPose, MeshSubset,
    Node, NodeInteractionState, NodeSignals, NodeTraitsKey, Rectangle, RenderProxy, RenderProxy2D,
    RenderProxyAdapter, RenderState, Renderable, Renderer, Sampler, SceneGraphSyncPolicy,
    SceneLights, SceneNode, SceneRenderProxy, SceneResourceRef, Stage, StageSignals,
    TextureColorSpace, TextureFilter, TextureWrap, Transform3DNode, Vector2, WgpuBitmapShader,
    WgpuClipContourEntry, WgpuClipContourPipelines, WgpuColorAdjustmentFold, WgpuRenderState,
    WgpuSavedPassState, WgpuShapeMeshPipeline, WgpuSpriteBatchBufferSlot,
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
    pub uv_offset: Option<Vector2>,
    pub uv_rotation: Option<f64>,
    pub uv_scale: Option<Vector2>,
    pub color_space: Option<TextureColorSpace>,
    pub image: Option<ImageResource>,
    pub resource: Option<SceneResourceRef>,
    pub sampler: Option<Sampler>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord5 {
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

// Source: upstream/packages/scene-wgpu/src/drawWgpuScene.ts:33 (sha256:ae38fdf831133a71473be51dedd40a3c491279306eb0fad84d62b945f1d3ac45)
pub fn draw_wgpu_scene(
    state: &mut WgpuRenderState,
    scene: &SceneNode,
    camera: &Camera,
    lights: &SceneLights,
) -> () {
    let list = prepare_scene_render(
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
    let mut bound_material: Option<Material> = None;
    let mut bound_renderer: Option<crate::OpaqueHostValue> = None;
    {
        let mut m = 0.0_f64;
        while (m < list.mesh_count) {
            let mesh = list.visible_meshes[m as usize].clone();
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
            {
                let mut s = 0.0_f64;
                while (s < (mesh.geometry.subsets.len() as f64)) {
                    let material = resolve_subset_material(&mesh, s);
                    let renderer =
                        resolve_wgpu_mesh_material_renderer(state, ((material).clone()).clone());
                    if (renderer).is_none() {
                        {
                            s += 1.0;
                            s
                        };
                        continue;
                    }
                    if (renderer != bound_renderer) || (material != bound_material) {
                        {
                            let __flight_callback = (renderer.as_ref().unwrap().bind).clone();
                            let __flight_result = __flight_callback.lock().unwrap()(
                                (*state).clone(),
                                (material).clone(),
                                (list.lights).clone(),
                                (*camera).clone(),
                            );
                            __flight_result
                        };
                        bound_renderer = renderer;
                        bound_material = (material).clone();
                    }
                    (*PROXY.lock().unwrap()).material =
                        (material).unwrap_or((*DEFAULT_MATERIAL).clone());
                    (*PROXY.lock().unwrap()).normal_matrix =
                        (*SCRATCH_NORMAL_MATRIX.lock().unwrap()).clone();
                    (*PROXY.lock().unwrap()).subset = mesh.geometry.subsets[s as usize].clone();
                    (*PROXY.lock().unwrap()).world_matrix = (world_matrix).clone();
                    {
                        let __flight_callback = (renderer.as_ref().unwrap().draw).clone();
                        let __flight_result = __flight_callback.lock().unwrap()(
                            (*state).clone(),
                            (*PROXY.lock().unwrap()).clone(),
                            (mesh.geometry).clone(),
                        );
                        __flight_result
                    };
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
    draw_wgpu_scene_particle_emitters(state, scene, camera, lights);
}

// Source: upstream/packages/scene-wgpu/src/drawWgpuScene.ts:83 (sha256:6fca77ac5ec010a83c405358d6ea1d659af69ce7eeb876a2e824d84f089db5bc)
fn resolve_subset_material(mesh: &Mesh, subset_index: f64) -> Option<Material> {
    return if (subset_index < (mesh.materials.len() as f64)) {
        mesh.materials[subset_index as usize].clone()
    } else {
        None
    };
}

// Source: upstream/packages/scene-wgpu/src/drawWgpuScene.ts:90 (sha256:9579dcdc0698149dd8606f40a038569489f459ce022f79e9fcdb3991aff72e89)
static PROXY: std::sync::LazyLock<std::sync::Mutex<SceneRenderProxy>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(SceneRenderProxy {
            __flight_identity: std::sync::Arc::new(()),
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
            joint_matrices: None,
        })
    });

// Source: upstream/packages/scene-wgpu/src/drawWgpuScene.ts:99 (sha256:8db138954744b37f13b6de8e07b798befcd977f4ef6adc006e52c5beb8faf5d3)
static DEFAULT_MATERIAL: std::sync::LazyLock<Material> = std::sync::LazyLock::new(|| Material {
    __flight_identity: std::sync::Arc::new(()),
    __flight_entity_runtime: Default::default(),
    kind: (default_material_kind_constant).to_owned(),
    name: None,
    ..Default::default()
});

// Source: upstream/packages/scene-wgpu/src/drawWgpuScene.ts:101 (sha256:590e41ce5748e09578c54b48aa738f2cb9712864e3096ae578ed46a44c4cea6f)
static SCRATCH_NORMAL_MATRIX: std::sync::LazyLock<std::sync::Mutex<Matrix3>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix3(
            None, None, None, None, None, None, None, None, None,
        ))
    });
