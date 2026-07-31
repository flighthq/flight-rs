// @generated from upstream/packages/scene-wgpu/src/wgpuShadowMap.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    WgpuSceneRuntime, WgpuSceneShadow, ensure_wgpu_mesh_upload, get_wgpu_scene_runtime,
    write_wgpu_draw_uniform,
};
use flighthq_camera::get_camera_view_projection_matrix4;
use flighthq_geometry::{create_matrix3, create_matrix4, multiply_matrix4};
use flighthq_node::{for_each_node_descendant, get_node_world_matrix4};
use flighthq_render_wgpu::get_wgpu_render_state_runtime;
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, Camera, ColorTransform, DisplayObjectClipHooks,
    ImageResource, InteractionSignals, Kind, Material, Matrix, Matrix4, Matrix4Like,
    MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose, MeshSkinBindPose, MeshSubset,
    Node, NodeInteractionState, NodeSignals, NodeTraitsKey, Rectangle, RenderProxy, RenderProxy2D,
    RenderProxyAdapter, RenderState, Renderable, Renderer, Sampler, SceneGraphSyncPolicy,
    SceneNode, SceneRenderProxy, SceneResourceRef, Stage, StageSignals, TextureColorSpace,
    TextureFilter, TextureWrap, Transform3DNode, Vector2, WgpuBitmapShader, WgpuClipContourEntry,
    WgpuClipContourPipelines, WgpuColorAdjustmentFold, WgpuRenderState, WgpuSavedPassState,
    WgpuShapeMeshPipeline, WgpuSpriteBatchBufferSlot,
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
pub struct ModuleSynthesizedRecord2902218824 {
    pub __flight_identity: std::sync::Arc<()>,
    pub array_stride: f64,
    pub attributes: Vec<ModuleSynthesizedRecord928826179>,
}
impl PartialEq for ModuleSynthesizedRecord2902218824 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ModuleSynthesizedRecord928826179 {
    pub __flight_identity: std::sync::Arc<()>,
    pub format: String,
    pub offset: f64,
    pub shader_location: f64,
}
impl PartialEq for ModuleSynthesizedRecord928826179 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ModuleSynthesizedRecord58771532 {
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for ModuleSynthesizedRecord58771532 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuShadowMap.ts:26 (sha256:c516a26bec62d668defbb32f59a164a639fd3162441c43ff6d46200aa7e2d73d)
pub fn destroy_wgpu_scene_shadow(state: &mut WgpuRenderState) -> () {
    let mut scene = get_wgpu_scene_runtime(state);
    if ((scene.shadow).clone()).is_some() {
        crate::host_value::<()>("host.destroy");
        scene.shadow = None;
    }
    if ((scene.shadow_dummy_texture).clone()).is_some() {
        crate::host_value::<()>("host.destroy");
        scene.shadow_dummy_texture = None;
        scene.shadow_dummy_view = None;
    }
    if ((scene.shadow_uniform_buffer).clone()).is_some() {
        crate::host_value::<()>("host.destroy");
        scene.shadow_uniform_buffer = None;
    }
    scene.shadow_comparison_sampler = None;
    scene.shadow_depth_pipeline = None;
    scene.shadow_sample_bind_group = None;
    scene.shadow_sample_layout = None;
    scene.shadow_sample_view = None;
    scene.pbr_sample_bind_group = None;
    scene.pbr_sample_shadow_view = None;
}

// Source: upstream/packages/scene-wgpu/src/wgpuShadowMap.ts:65 (sha256:29f5086b1e97a82169894225f6b556b69134cf863792bd1ff61edd096f70f592)
pub fn draw_wgpu_scene_shadow_map(
    mut state: WgpuRenderState,
    scene: &SceneNode,
    shadow_camera: &Camera,
) -> () {
    let runtime = get_wgpu_render_state_runtime(&state);
    let encoder = (runtime.inner.lock().unwrap().command_encoder).clone();
    if (encoder).is_none() {
        return;
    }
    let scene_runtime: std::sync::Arc<std::sync::Mutex<WgpuSceneRuntime>> =
        std::sync::Arc::new(std::sync::Mutex::new(get_wgpu_scene_runtime(&mut state)));
    let mut shadow = ((*scene_runtime.lock().unwrap()).shadow).clone();
    if (shadow).is_none() {
        let depth_texture = crate::host_value::<()>("host.createTexture");
        shadow = Some(WgpuSceneShadow {
            __flight_identity: std::sync::Arc::new(()),
            depth_texture: depth_texture,
            depth_view: (depth_texture.create_view)(),
            matrix: create_matrix4(
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None,
            ),
        });
        (*scene_runtime.lock().unwrap()).shadow = (shadow).clone();
    }
    get_camera_view_projection_matrix4(
        &mut shadow.as_mut().unwrap().matrix,
        shadow_camera,
        1.0_f64,
    );
    let pipeline = ensure_wgpu_shadow_depth_pipeline(&mut state);
    let pass = crate::host_value::<()>("host.beginRenderPass");
    crate::host_value::<()>("host.setViewport");
    crate::host_value::<()>("host.setPipeline");
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
            let upload = ensure_wgpu_mesh_upload(&mut state, &mut mesh.geometry);
            if ((upload).is_none()) || (((upload.as_ref().unwrap().index_buffer).clone()).is_none())
            {
                return;
            }
            let world = {
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
            multiply_matrix4(
                &mut (*_SHADOW_PROXY.lock().unwrap()).world_matrix,
                &{
                    let __flight_source = &(*light_matrix.lock().unwrap());
                    Matrix4Like {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        __flight_entity_runtime: std::sync::Arc::clone(
                            &__flight_source.__flight_entity_runtime,
                        ),
                        m: (__flight_source.m).clone(),
                    }
                },
                &{
                    let __flight_source = &(world);
                    Matrix4Like {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        __flight_entity_runtime: std::sync::Arc::clone(
                            &__flight_source.__flight_entity_runtime,
                        ),
                        m: (__flight_source.m).clone(),
                    }
                },
            );
            let draw_bind_group =
                write_wgpu_draw_uniform(&mut state, &(*_SHADOW_PROXY.lock().unwrap()));
            (*_DYNAMIC_OFFSETS.lock().unwrap())[0.0_f64 as usize] =
                ((*scene_runtime.lock().unwrap()).pending_draw_offset) as u32;
            crate::host_value::<()>("host.setBindGroup");
            crate::host_value::<()>("host.setVertexBuffer");
            crate::host_value::<()>("host.setIndexBuffer");
            crate::host_value::<()>("host.drawIndexed");
        },
    );
    crate::host_value::<()>("host.end");
}

// Source: upstream/packages/scene-wgpu/src/wgpuShadowMap.ts:131 (sha256:165f16c048b1b3faf4fbdb901c698a164a815e41d33e7582c10662c64774f78a)
#[derive(Clone, Default)]
struct EnsureWgpuShadowDepthPipelineRecord6 {
    __flight_identity: std::sync::Arc<()>,
    topology: String,
    front_face: String,
    cull_mode: String,
}
impl PartialEq for EnsureWgpuShadowDepthPipelineRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn ensure_wgpu_shadow_depth_pipeline(state: &mut WgpuRenderState) -> crate::OpaqueHostValue {
    let mut scene = get_wgpu_scene_runtime(state);
    if ((scene.shadow_depth_pipeline).clone()).is_some() {
        return ((scene.shadow_depth_pipeline).clone()).unwrap();
    }
    let device = (state.device).clone();
    let module = crate::host_value::<()>("host.createShaderModule");
    let layout = crate::host_value::<()>("host.createPipelineLayout");
    let pipeline = crate::host_value::<()>("host.createRenderPipeline");
    scene.shadow_depth_pipeline = Some((pipeline).clone());
    return pipeline;
}

// Source: upstream/packages/scene-wgpu/src/wgpuShadowMap.ts:151 (sha256:4b42abc931bc4636bfa4ae19dd3c8d8a3f69e109d3189486fc666c93f6b840da)
const SHADOW_MAP_SIZE: f64 = 1024.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuShadowMap.ts:158 (sha256:09cbf892466b3736f5965777a2157c38f75f763682fccdc952195f372200d0e1)
const SHADOW_DEPTH_WGSL: &'static str = "\nstruct Draw { world : mat4x4f };\n@group(0) @binding(0) var<uniform> draw : Draw;\n\n@vertex fn vs_main(@location(0) position : vec3f) -> @builtin(position) vec4f {\n  var clip = draw.world * vec4f(position, 1.0);\n  clip.z = (clip.z + clip.w) * 0.5;\n  return clip;\n}\n";

// Source: upstream/packages/scene-wgpu/src/wgpuShadowMap.ts:170 (sha256:fac88ba0fbd9df9019741b05088b4c1f4ef32787451e8595b160e866bc3ba70d)
static SHADOW_VERTEX_BUFFER_LAYOUTS: std::sync::LazyLock<Vec<crate::OpaqueHostValue>> =
    std::sync::LazyLock::new(|| {
        vec![ModuleSynthesizedRecord2902218824 {
            __flight_identity: std::sync::Arc::new(()),
            array_stride: 48.0_f64,
            attributes: vec![ModuleSynthesizedRecord928826179 {
                __flight_identity: std::sync::Arc::new(()),
                shader_location: 0.0_f64,
                offset: 0.0_f64,
                format: "float32x3".to_owned(),
            }],
        }]
    });

// Source: upstream/packages/scene-wgpu/src/wgpuShadowMap.ts:176 (sha256:ee3caa98e305ffdb4d3c9eb1aea470ac90843e167ca082715b572e5e7104e27e)
static _SHADOW_PROXY: std::sync::LazyLock<std::sync::Mutex<SceneRenderProxy>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(SceneRenderProxy {
            __flight_identity: std::sync::Arc::new(()),
            material: Material {
                __flight_identity: std::sync::Arc::new(()),
                __flight_entity_runtime: Default::default(),
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

// Source: upstream/packages/scene-wgpu/src/wgpuShadowMap.ts:183 (sha256:58f9b06eb92a298e62fc3daaeeb389221c18a43a05437c9efca88f9e2187010d)
static _DYNAMIC_OFFSETS: std::sync::LazyLock<std::sync::Mutex<Vec<u32>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0_u32; (1.0_f64) as usize]));
