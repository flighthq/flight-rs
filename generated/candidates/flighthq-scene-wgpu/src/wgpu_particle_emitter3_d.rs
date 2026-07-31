// @generated from upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_image::has_image_resource_pixels;
use flighthq_node::{get_node_runtime, get_node_world_matrix4};
use flighthq_render::prepare_scene_render;
use flighthq_render_wgpu::{bind_wgpu_image_resource_texture, get_wgpu_render_state_runtime};
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, Camera, ColorTransform, DisplayObjectClipHooks,
    ImageResource, InteractionSignals, Kind, Material, Matrix, Matrix4, MeshGeometryGlData,
    MeshGeometryWgpuData, MeshMorphBindPose, MeshSkinBindPose, Node, NodeAny, NodeInteractionState,
    NodeSignals, NodeTraitsKey, PARTICLE_EMITTER3_D_KIND as particle_emitter3_d_kind_constant,
    ParticleBlendMode, ParticleEmitter3D, Rectangle, RenderProxy, RenderProxy2D,
    RenderProxyAdapter, RenderState, Renderable, Renderer, SceneGraphSyncPolicy, SceneLights,
    SceneNode, Stage, StageSignals, Transform3DNode, WgpuBitmapShader, WgpuClipContourEntry,
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
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ModuleSynthesizedRecord2440807288 {
    pub __flight_identity: std::sync::Arc<()>,
    pub array_stride: f64,
    pub attributes: Vec<ModuleSynthesizedRecord928826179>,
    pub step_mode: String,
}
impl PartialEq for ModuleSynthesizedRecord2440807288 {
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

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:21 (sha256:a898f2879e75da7211abb2adcf948b45613b4b61c68abe25c29f2ba8f13f4516)
const INSTANCE_FLOATS: f64 = 16.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:22 (sha256:158d26939b3bb3d19a69d96db2f96a052f8eab461f3875a14559b034ea66c0ee)
const INSTANCE_STRIDE: f64 = 64.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:24 (sha256:c3dc807b578ac94141dd73c6a0532f43a0d50d26a1aa7884792f64f943d23ca6)
const PARTICLE_TRANSFORM_STRIDE: f64 = 4.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:29 (sha256:37b90f7b7898c8bd3fe082ee9973ccd4894097add98118d13091a442747a31e5)
const FRAME_UNIFORM_BYTES: f64 = 96.0_f64;

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:31 (sha256:343dac28670ced6c1f8cde8e29749cac7980c6e130e5cd10a216d956f2e85dde)
const DEPTH_STENCIL_FORMAT: &'static str = "depth24plus-stencil8";

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:38 (sha256:1a3e5d4c10bf6792a6608e3a7277f422c71d475e88a734f6a435bdf96ed7d53c)
const PARTICLE_3_D_WGSL: &'static str = "\noverride HAS_TEXTURE : f32 = 0.0;\n\nstruct Frame {\n  viewProjection : mat4x4f,\n  cameraRight : vec4f,\n  cameraUp : vec4f,\n};\n\n@group(0) @binding(0) var<uniform> frame : Frame;\n@group(1) @binding(0) var particleTexture : texture_2d<f32>;\n@group(1) @binding(1) var particleSampler : sampler;\n\nstruct VertexOutput {\n  @builtin(position) clipPosition : vec4f,\n  @location(0) uv : vec2f,\n  @location(1) color : vec4f,\n};\n\n@vertex fn vs_main(\n  @location(0) corner : vec2f,\n  @location(1) pos : vec3f,\n  @location(2) cosScale : f32,\n  @location(3) sinScale : f32,\n  @location(4) color : vec4f,\n  @location(5) uvRect : vec4f,\n  @location(6) size : vec2f,\n) -> VertexOutput {\n  var out : VertexOutput;\n  let lx = (corner.x - 0.5) * size.x;\n  let ly = (corner.y - 0.5) * size.y;\n  let rx = cosScale * lx - sinScale * ly;\n  let ry = sinScale * lx + cosScale * ly;\n  let worldPos = pos + frame.cameraRight.xyz * rx + frame.cameraUp.xyz * ry;\n  out.clipPosition = frame.viewProjection * vec4f(worldPos, 1.0);\n  out.uv = mix(uvRect.xy, uvRect.zw, corner);\n  out.color = color;\n  return out;\n}\n\n@fragment fn fs_main(in : VertexOutput) -> @location(0) vec4f {\n  var rgba : vec4f;\n  if (HAS_TEXTURE > 0.5) {\n    let tex = textureSample(particleTexture, particleSampler, in.uv);\n    rgba = vec4f(tex.rgb * in.color.rgb, tex.a) * in.color.a;\n  } else {\n    rgba = vec4f(in.color.rgb * in.color.a, in.color.a);\n  }\n  if (rgba.a <= 0.0) { discard; }\n  return rgba;\n}\n";

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:91 (sha256:d1f8934ae89b809718be781342ab8f53a53e166d2665cc7cc4ddc1ee7fd53862)
#[derive(Clone, Default)]
struct WgpuParticle3DInstanceBuffer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub buffer: crate::OpaqueHostValue,
    pub capacity: f64,
}
impl PartialEq for WgpuParticle3DInstanceBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:96 (sha256:208fa3bca44c9af1b522845db0207f18779511a1829560c81c66700436f8253d)
#[derive(Clone, Default)]
struct WgpuParticle3DResources {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub corner_buffer: crate::OpaqueHostValue,
    pub frame_bind_group: crate::OpaqueHostValue,
    pub frame_buffer: crate::OpaqueHostValue,
    pub frame_layout: crate::OpaqueHostValue,
    pub index_buffer: crate::OpaqueHostValue,
    pub instance_buffers: Vec<(ParticleEmitter3D, WgpuParticle3DInstanceBuffer)>,
    pub instance_data: Vec<f32>,
    pub module: crate::OpaqueHostValue,
    pub pipeline_layout: crate::OpaqueHostValue,
    pub pipelines: Vec<(String, crate::OpaqueHostValue)>,
    pub texture_layout: crate::OpaqueHostValue,
}
impl PartialEq for WgpuParticle3DResources {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:113 (sha256:974bae5843024b82171f7bd9126194b68a94b8221308f28c908472531d571dd0)
#[derive(Clone, Default)]
struct WgpuParticleBlendStateSynthesizedRecord2638137061 {
    __flight_identity: std::sync::Arc<()>,
    dst_factor: crate::OpaqueHostValue,
    operation: String,
    src_factor: crate::OpaqueHostValue,
}
impl PartialEq for WgpuParticleBlendStateSynthesizedRecord2638137061 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct WgpuParticleBlendStateSynthesizedRecord3696714177 {
    __flight_identity: std::sync::Arc<()>,
    alpha: WgpuParticleBlendStateSynthesizedRecord2638137061,
    color: WgpuParticleBlendStateSynthesizedRecord2638137061,
}
impl PartialEq for WgpuParticleBlendStateSynthesizedRecord3696714177 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn wgpu_particle_blend_state(mode: ParticleBlendMode) -> crate::OpaqueHostValue {
    let mut src: crate::OpaqueHostValue;
    let mut dst: crate::OpaqueHostValue;
    {
        let __switch_value = mode;
        let __flight_case = if __switch_value == "add" {
            0_usize
        } else if __switch_value == "multiply" {
            1_usize
        } else if __switch_value == "screen" {
            2_usize
        } else {
            3_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                src = crate::OpaqueHostValue::String("one".to_owned());
                dst = crate::OpaqueHostValue::String("one".to_owned());
                break '__flight_switch;
            }
            if __flight_case <= 1_usize {
                src = crate::OpaqueHostValue::String("dst".to_owned());
                dst = crate::OpaqueHostValue::String("one-minus-src-alpha".to_owned());
                break '__flight_switch;
            }
            if __flight_case <= 2_usize {
                src = crate::OpaqueHostValue::String("one".to_owned());
                dst = crate::OpaqueHostValue::String("one-minus-src".to_owned());
                break '__flight_switch;
            }
            if __flight_case <= 3_usize {
                src = crate::OpaqueHostValue::String("one".to_owned());
                dst = crate::OpaqueHostValue::String("one-minus-src-alpha".to_owned());
                break '__flight_switch;
            }
        }
    }
    let mut component = WgpuParticleBlendStateSynthesizedRecord2638137061 {
        __flight_identity: std::sync::Arc::new(()),
        operation: "add".to_owned(),
        src_factor: (src).clone(),
        dst_factor: (dst).clone(),
    };
    return WgpuParticleBlendStateSynthesizedRecord3696714177 {
        __flight_identity: std::sync::Arc::new(()),
        color: (component).clone(),
        alpha: (component).clone(),
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:138 (sha256:fc4e6d87b88c59699e6605ad8776109903079db2ac9efb3e69412aa7d25329a5)
fn collect_particle_emitter3_d_nodes(node: &NodeAny, out: &mut Vec<ParticleEmitter3D>) -> () {
    if (!node.enabled) {
        return;
    }
    if ((node.kind).clone() == particle_emitter3_d_kind_constant) {
        out.push(node);
    }
    let children = {
        let __flight_slot = get_node_runtime(&{
            let __flight_source = &(node);
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
        })
        .__flight_generic_slot::<crate::NodeRuntimeStorage<Traits>>();
        let __flight_storage = __flight_slot.lock().unwrap();
        (__flight_storage.children).clone()
    };
    if (children).is_some() {
        {
            let mut i = 0.0_f64;
            while (i < (children.as_ref().unwrap().len() as f64)) {
                collect_particle_emitter3_d_nodes(
                    &{
                        let __flight_source = &(children.as_ref().unwrap()[i as usize]);
                        NodeAny {
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
                        }
                    },
                    out,
                );
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:151 (sha256:b1d4792e330bd9aa3ae820a06acca7d4a0781c32f8bfe348200dcb9bbcd19b27)
#[derive(Clone, Default)]
struct EnsureParticle3DResourcesRecord4 {
    __flight_identity: std::sync::Arc<()>,
    type_: String,
}
impl PartialEq for EnsureParticle3DResourcesRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct EnsureParticle3DResourcesRecord5 {
    __flight_identity: std::sync::Arc<()>,
    sample_type: String,
}
impl PartialEq for EnsureParticle3DResourcesRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn ensure_particle3_d_resources(state: &WgpuRenderState) -> WgpuParticle3DResources {
    let mut resources = (*RESOURCE_CACHE.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (resources).is_some() {
        return ((resources.as_mut().unwrap()).clone()).clone();
    }
    let device = (state.device).clone();
    let corner_buffer = crate::host_value::<()>("host.createBuffer");
    crate::host_value::<()>("host.writeBuffer");
    let index_buffer = crate::host_value::<()>("host.createBuffer");
    crate::host_value::<()>("host.writeBuffer");
    let frame_layout = crate::host_value::<()>("host.createBindGroupLayout");
    let texture_layout = crate::host_value::<()>("host.createBindGroupLayout");
    let pipeline_layout = crate::host_value::<()>("host.createPipelineLayout");
    let frame_buffer = crate::host_value::<()>("host.createBuffer");
    let frame_bind_group = crate::host_value::<()>("host.createBindGroup");
    resources = Some(WgpuParticle3DResources {
        __flight_identity: std::sync::Arc::new(()),
        corner_buffer: (corner_buffer).clone(),
        frame_bind_group: (frame_bind_group).clone(),
        frame_buffer: (frame_buffer).clone(),
        frame_layout: (frame_layout).clone(),
        index_buffer: (index_buffer).clone(),
        instance_buffers: Vec::new(),
        instance_data: vec![0.0_f32; (0.0_f64) as usize],
        module: crate::host_value::<crate::OpaqueHostValue>("host.createShaderModule"),
        pipeline_layout: (pipeline_layout).clone(),
        pipelines: Vec::new(),
        texture_layout: (texture_layout).clone(),
    });
    {
        let __flight_key = (*state).clone();
        let __flight_value = (resources).clone().unwrap();
        if let Some((_, value)) = (*RESOURCE_CACHE.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*RESOURCE_CACHE.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    return ((resources).clone().unwrap()).clone();
}

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:209 (sha256:68aed93f8a4d9836d80ec53b9612cf4ea08032877a1853c2446077c75585addc)
#[derive(Clone, Default)]
struct EnsureParticle3DPipelineRecord4 {
    __flight_identity: std::sync::Arc<()>,
    topology: String,
    front_face: String,
    cull_mode: String,
}
impl PartialEq for EnsureParticle3DPipelineRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn ensure_particle3_d_pipeline(
    state: &WgpuRenderState,
    resources: &mut WgpuParticle3DResources,
    mode: ParticleBlendMode,
    has_texture: bool,
) -> crate::OpaqueHostValue {
    let format = ((get_wgpu_render_state_runtime(state)
        .inner
        .lock()
        .unwrap()
        .current_color_format)
        .clone())
    .unwrap_or((state.format).clone());
    let key = format!(
        "{}|{}|{}",
        format,
        mode,
        if has_texture {
            "t".to_owned()
        } else {
            "u".to_owned()
        }
    );
    let mut pipeline = resources
        .pipelines
        .iter()
        .find(|(key, _)| key == &(key).clone())
        .map(|(_, value)| value.clone());
    if (pipeline).is_some() {
        return ((pipeline.as_mut().unwrap()).clone()).clone();
    }
    pipeline = Some(crate::host_value::<crate::OpaqueHostValue>(
        "host.createRenderPipeline",
    ));
    {
        let __flight_key = (key).clone();
        let __flight_value = (pipeline).clone().unwrap();
        if let Some((_, value)) = resources
            .pipelines
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            resources.pipelines.push((__flight_key, __flight_value));
        }
    };
    return ((pipeline).clone().unwrap()).clone();
}

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:240 (sha256:0570a5994808327be69d44c0e911fd09744a290bab55a602eb40b02df2f4ea56)
fn ensure_particle3_d_instance_buffer(
    state: &WgpuRenderState,
    resources: &mut WgpuParticle3DResources,
    emitter: &ParticleEmitter3D,
    count: f64,
) -> crate::OpaqueHostValue {
    let mut entry = resources
        .instance_buffers
        .iter()
        .find(|(key, _)| key == &(*emitter).clone())
        .map(|(_, value)| value.clone());
    if ((entry).is_some()) && (entry.as_ref().unwrap().capacity >= count) {
        return (entry.as_ref().unwrap().buffer).clone();
    }
    if (entry).is_some() {
        crate::host_value::<()>("host.destroy");
    }
    let capacity = (count).max(if (entry).is_some() {
        (entry.as_ref().unwrap().capacity * 2.0_f64)
    } else {
        8.0_f64
    });
    let buffer = crate::host_value::<()>("host.createBuffer");
    {
        let __flight_key = (*emitter).clone();
        let __flight_value = WgpuParticle3DInstanceBuffer {
            __flight_identity: std::sync::Arc::new(()),
            buffer: buffer,
            capacity: capacity,
        };
        if let Some((_, value)) = resources
            .instance_buffers
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            resources
                .instance_buffers
                .push((__flight_key, __flight_value));
        }
    };
    return buffer;
}

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:259 (sha256:c47a1dbd92cab5ee5319cf900cd1647829c49bd5fda56674753dcca21191bf13)
fn draw_particle_emitter3_d_node(
    state: &WgpuRenderState,
    resources: &mut WgpuParticle3DResources,
    pass: crate::OpaqueHostValue,
    emitter: &ParticleEmitter3D,
) -> () {
    let atlas = (emitter.data.atlas).clone();
    let particle_count = emitter.data.particle_count;
    if (particle_count == 0.0_f64) {
        return;
    }
    let needed = (particle_count * INSTANCE_FLOATS);
    if ((resources.instance_data.len() as f64) < needed) {
        resources.instance_data = vec![
            0.0_f32;
            ((needed).max(((resources.instance_data.len() as f64) * 2.0_f64)))
                as usize
        ];
    }
    let has_atlas = (((atlas).is_some()) && (((atlas.as_ref().unwrap().image).clone()).is_some()))
        && (has_image_resource_pixels(atlas.as_ref().unwrap().image.as_ref().unwrap()));
    let regions = if has_atlas {
        Some((atlas.as_ref().unwrap().regions).clone())
    } else {
        None
    };
    let num_regions = if (regions).is_some() {
        (regions.as_ref().unwrap().len() as f64)
    } else {
        0.0_f64
    };
    let iw = if has_atlas {
        (1.0_f64
            / if (atlas.as_ref().unwrap().image.as_ref().unwrap().width) != 0.0_f64 {
                atlas.as_ref().unwrap().image.as_ref().unwrap().width
            } else {
                1.0_f64
            })
    } else {
        0.0_f64
    };
    let ih = if has_atlas {
        (1.0_f64
            / if (atlas.as_ref().unwrap().image.as_ref().unwrap().height) != 0.0_f64 {
                atlas.as_ref().unwrap().image.as_ref().unwrap().height
            } else {
                1.0_f64
            })
    } else {
        0.0_f64
    };
    let world_matrix = {
        let __flight_source = &(get_node_world_matrix4(&{
            let __flight_source = &(emitter);
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
        }));
        Matrix4 {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            m: (__flight_source.m).clone(),
        }
    };
    let world_space = emitter.data.world_space;
    let mut base = 0.0_f64;
    let mut draw_count = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < particle_count) {
            let tt = (i * PARTICLE_TRANSFORM_STRIDE);
            let lx = (emitter.data.transforms[tt as usize] as f64);
            let ly = (emitter.data.transforms[(tt + 1.0_f64) as usize] as f64);
            let rotation = (emitter.data.transforms[(tt + 2.0_f64) as usize] as f64);
            let scale = (emitter.data.transforms[(tt + 3.0_f64) as usize] as f64);
            let lz = (emitter.data.positions_z[i as usize] as f64);
            let wx = if world_space {
                ((lx).clone()) as f32
            } else {
                (((((world_matrix.m[0.0_f64 as usize] as f64) * lx)
                    + ((world_matrix.m[4.0_f64 as usize] as f64) * ly))
                    + ((world_matrix.m[8.0_f64 as usize] as f64) * lz))
                    + (world_matrix.m[12.0_f64 as usize] as f64)) as f32
            };
            let wy = if world_space {
                ((ly).clone()) as f32
            } else {
                (((((world_matrix.m[1.0_f64 as usize] as f64) * lx)
                    + ((world_matrix.m[5.0_f64 as usize] as f64) * ly))
                    + ((world_matrix.m[9.0_f64 as usize] as f64) * lz))
                    + (world_matrix.m[13.0_f64 as usize] as f64)) as f32
            };
            let wz = if world_space {
                ((lz).clone()) as f32
            } else {
                (((((world_matrix.m[2.0_f64 as usize] as f64) * lx)
                    + ((world_matrix.m[6.0_f64 as usize] as f64) * ly))
                    + ((world_matrix.m[10.0_f64 as usize] as f64) * lz))
                    + (world_matrix.m[14.0_f64 as usize] as f64)) as f32
            };
            let cos_r = ((rotation).cos() * scale);
            let sin_r = ((rotation).sin() * scale);
            let ct = (i * 3.0_f64);
            let has_colors = ((emitter.data.colors).is_some())
                && ((emitter.data.colors.len() as f64) > (ct + 2.0_f64));
            let r = if has_colors {
                (emitter.data.colors[ct as usize] as f64) as f32
            } else {
                (1.0_f64) as f32
            };
            let g = if has_colors {
                (emitter.data.colors[(ct + 1.0_f64) as usize] as f64) as f32
            } else {
                (1.0_f64) as f32
            };
            let b = if has_colors {
                (emitter.data.colors[(ct + 2.0_f64) as usize] as f64) as f32
            } else {
                (1.0_f64) as f32
            };
            let mut u0 = 0.0_f64;
            let mut v0 = 0.0_f64;
            let mut u1 = 1.0_f64;
            let mut v1 = 1.0_f64;
            let mut region_w = 1.0_f64;
            let mut region_h = 1.0_f64;
            if (regions).is_some() {
                let id = (emitter.data.ids[i as usize] as f64);
                if (id >= num_regions) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                let region = regions.as_ref().unwrap()[id as usize].clone();
                if (region.width <= 0.0_f64) || (region.height <= 0.0_f64) {
                    {
                        i += 1.0;
                        i
                    };
                    continue;
                }
                u0 = (region.x * iw);
                v0 = (region.y * ih);
                u1 = ((region.x + region.width) * iw);
                v1 = ((region.y + region.height) * ih);
                region_w = region.width;
                region_h = region.height;
            }
            resources.instance_data[base as usize] = ((wx).clone()) as f32;
            resources.instance_data[(base + 1.0_f64) as usize] = ((wy).clone()) as f32;
            resources.instance_data[(base + 2.0_f64) as usize] = ((wz).clone()) as f32;
            resources.instance_data[(base + 3.0_f64) as usize] = (cos_r) as f32;
            resources.instance_data[(base + 4.0_f64) as usize] = (sin_r) as f32;
            resources.instance_data[(base + 5.0_f64) as usize] = ((r).clone()) as f32;
            resources.instance_data[(base + 6.0_f64) as usize] = ((g).clone()) as f32;
            resources.instance_data[(base + 7.0_f64) as usize] = ((b).clone()) as f32;
            resources.instance_data[(base + 8.0_f64) as usize] =
                (emitter.data.alphas[i as usize] as f64) as f32;
            resources.instance_data[(base + 9.0_f64) as usize] = (u0) as f32;
            resources.instance_data[(base + 10.0_f64) as usize] = (v0) as f32;
            resources.instance_data[(base + 11.0_f64) as usize] = (u1) as f32;
            resources.instance_data[(base + 12.0_f64) as usize] = (v1) as f32;
            let max_dim = if (region_w >= region_h) {
                region_w
            } else {
                region_h
            };
            resources.instance_data[(base + 13.0_f64) as usize] = (region_w / max_dim) as f32;
            resources.instance_data[(base + 14.0_f64) as usize] = (region_h / max_dim) as f32;
            resources.instance_data[(base + 15.0_f64) as usize] = (0.0_f64) as f32;
            base += INSTANCE_FLOATS;
            {
                draw_count += 1.0;
                draw_count
            };
            {
                i += 1.0;
                i
            };
        }
    }
    if (draw_count == 0.0_f64) {
        return;
    }
    let instance_buffer =
        ensure_particle3_d_instance_buffer(state, resources, &(*emitter).clone(), draw_count);
    crate::host_value::<()>("host.writeBuffer");
    let runtime = get_wgpu_render_state_runtime(state);
    let texture_view = if has_atlas {
        (bind_wgpu_image_resource_texture(
            state,
            atlas.as_ref().unwrap().image.as_ref().unwrap(),
            None,
        )
        .view)
            .clone()
    } else {
        ensure_dummy_texture_view(state)
    };
    let texture_bind_group = crate::host_value::<()>("host.createBindGroup");
    let pipeline =
        ensure_particle3_d_pipeline(state, resources, (emitter.blend_mode).clone(), has_atlas);
    crate::host_value::<()>("host.setPipeline");
    crate::host_value::<()>("host.setBindGroup");
    crate::host_value::<()>("host.setBindGroup");
    crate::host_value::<()>("host.setVertexBuffer");
    crate::host_value::<()>("host.setVertexBuffer");
    crate::host_value::<()>("host.setIndexBuffer");
    crate::host_value::<()>("host.drawIndexed");
}

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:386 (sha256:9d306e137a3b3ddd060e845754ccd0dd39985a3e51ffdfdf3a76594a39643110)
#[derive(Clone, Default)]
struct EnsureDummyTextureViewRecord4 {
    __flight_identity: std::sync::Arc<()>,
    bytes_per_row: f64,
}
impl PartialEq for EnsureDummyTextureViewRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn ensure_dummy_texture_view(state: &WgpuRenderState) -> crate::OpaqueHostValue {
    let mut view = (*DUMMY_TEXTURE_CACHE.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (view).is_some() {
        return ((view.as_mut().unwrap()).clone()).clone();
    }
    let texture = crate::host_value::<()>("host.createTexture");
    crate::host_value::<()>("host.writeTexture");
    view = Some((texture.create_view)());
    {
        let __flight_key = (*state).clone();
        let __flight_value = (view).clone().unwrap();
        if let Some((_, value)) = (*DUMMY_TEXTURE_CACHE.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*DUMMY_TEXTURE_CACHE.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    return ((view).clone().unwrap()).clone();
}

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:402 (sha256:c17ff596516b98f313ae65cc4d8279a2ba2301b24edef8099e5794eb3846157d)
pub fn destroy_wgpu_particle_emitter3_d_resources(state: &WgpuRenderState) -> () {
    let resources = (*RESOURCE_CACHE.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*state).clone())
        .map(|(_, value)| value.clone());
    if (resources).is_none() {
        return;
    }
    crate::host_value::<()>("host.destroy");
    crate::host_value::<()>("host.destroy");
    crate::host_value::<()>("host.destroy");
    {
        let __flight_key = (*state).clone();
        if let Some(__flight_index) = (*RESOURCE_CACHE.lock().unwrap())
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            (*RESOURCE_CACHE.lock().unwrap()).remove(__flight_index);
            true
        } else {
            false
        }
    };
    {
        let __flight_key = (*state).clone();
        if let Some(__flight_index) = (*DUMMY_TEXTURE_CACHE.lock().unwrap())
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            (*DUMMY_TEXTURE_CACHE.lock().unwrap()).remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:418 (sha256:a87b9d96265d3b339df93719a2c750b51afdf50481432446ffe8a39d5a559ff3)
pub fn draw_wgpu_scene_particle_emitters(
    state: &WgpuRenderState,
    scene: &SceneNode,
    camera: &Camera,
    lights: &SceneLights,
) -> () {
    EMITTER_SCRATCH.lock().unwrap().clear();
    collect_particle_emitter3_d_nodes(
        &{
            let __flight_source = &(scene);
            NodeAny {
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
        &mut EMITTER_SCRATCH,
    );
    if ((EMITTER_SCRATCH.lock().unwrap().len() as f64) == 0.0_f64) {
        return;
    }
    let pass = (get_wgpu_render_state_runtime(state)
        .inner
        .lock()
        .unwrap()
        .render_pass)
        .clone();
    if (pass).is_none() {
        return;
    }
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
    let mut resources = ensure_particle3_d_resources(state);
    {
        let mut i = 0.0_f64;
        while (i < 16.0_f64) {
            (*FRAME_SCRATCH.lock().unwrap())[i as usize] =
                (list.view_projection.m[i as usize] as f64) as f32;
            {
                i += 1.0;
                i
            };
        }
    }
    (*FRAME_SCRATCH.lock().unwrap())[16.0_f64 as usize] =
        (camera.view.m[0.0_f64 as usize] as f64) as f32;
    (*FRAME_SCRATCH.lock().unwrap())[17.0_f64 as usize] =
        (camera.view.m[4.0_f64 as usize] as f64) as f32;
    (*FRAME_SCRATCH.lock().unwrap())[18.0_f64 as usize] =
        (camera.view.m[8.0_f64 as usize] as f64) as f32;
    (*FRAME_SCRATCH.lock().unwrap())[19.0_f64 as usize] = (0.0_f64) as f32;
    (*FRAME_SCRATCH.lock().unwrap())[20.0_f64 as usize] =
        (camera.view.m[1.0_f64 as usize] as f64) as f32;
    (*FRAME_SCRATCH.lock().unwrap())[21.0_f64 as usize] =
        (camera.view.m[5.0_f64 as usize] as f64) as f32;
    (*FRAME_SCRATCH.lock().unwrap())[22.0_f64 as usize] =
        (camera.view.m[9.0_f64 as usize] as f64) as f32;
    (*FRAME_SCRATCH.lock().unwrap())[23.0_f64 as usize] = (0.0_f64) as f32;
    crate::host_value::<()>("host.writeBuffer");
    {
        let mut i = 0.0_f64;
        while (i < (EMITTER_SCRATCH.lock().unwrap().len() as f64)) {
            draw_particle_emitter3_d_node(
                state,
                &mut resources,
                (pass.as_ref().unwrap()).clone(),
                &EMITTER_SCRATCH[i as usize],
            );
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:457 (sha256:c5cc17dcaf5604d14ca30545c99f8d67dd5625cfccc3803f8c98e948cbc453b7)
static VERTEX_BUFFER_LAYOUTS: std::sync::LazyLock<Vec<crate::OpaqueHostValue>> =
    std::sync::LazyLock::new(|| {
        vec![
            ModuleSynthesizedRecord2440807288 {
                __flight_identity: std::sync::Arc::new(()),
                array_stride: 8.0_f64,
                step_mode: "vertex".to_owned(),
                attributes: vec![ModuleSynthesizedRecord928826179 {
                    __flight_identity: std::sync::Arc::new(()),
                    shader_location: 0.0_f64,
                    offset: 0.0_f64,
                    format: "float32x2".to_owned(),
                }],
            },
            ModuleSynthesizedRecord2440807288 {
                __flight_identity: std::sync::Arc::new(()),
                array_stride: INSTANCE_STRIDE,
                step_mode: "instance".to_owned(),
                attributes: vec![
                    ModuleSynthesizedRecord928826179 {
                        __flight_identity: std::sync::Arc::new(()),
                        shader_location: 1.0_f64,
                        offset: 0.0_f64,
                        format: "float32x3".to_owned(),
                    },
                    ModuleSynthesizedRecord928826179 {
                        __flight_identity: std::sync::Arc::new(()),
                        shader_location: 2.0_f64,
                        offset: 12.0_f64,
                        format: "float32".to_owned(),
                    },
                    ModuleSynthesizedRecord928826179 {
                        __flight_identity: std::sync::Arc::new(()),
                        shader_location: 3.0_f64,
                        offset: 16.0_f64,
                        format: "float32".to_owned(),
                    },
                    ModuleSynthesizedRecord928826179 {
                        __flight_identity: std::sync::Arc::new(()),
                        shader_location: 4.0_f64,
                        offset: 20.0_f64,
                        format: "float32x4".to_owned(),
                    },
                    ModuleSynthesizedRecord928826179 {
                        __flight_identity: std::sync::Arc::new(()),
                        shader_location: 5.0_f64,
                        offset: 36.0_f64,
                        format: "float32x4".to_owned(),
                    },
                    ModuleSynthesizedRecord928826179 {
                        __flight_identity: std::sync::Arc::new(()),
                        shader_location: 6.0_f64,
                        offset: 52.0_f64,
                        format: "float32x2".to_owned(),
                    },
                ],
            },
        ]
    });

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:477 (sha256:e3b53a5f8871c52fa8f6288404cd13cce354ed7f90dd9905187ba8fa3183b073)
static WHITE_PIXEL: std::sync::LazyLock<Vec<u8>> = std::sync::LazyLock::new(|| {
    (vec![255.0_f64, 255.0_f64, 255.0_f64, 255.0_f64])
        .iter()
        .map(|value| (*value) as u8)
        .collect()
});

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:478 (sha256:8dd70c86c3ccb149585022e65d72b53233f93fe2903f5bdb3f0f61dc264c3d74)
static EMITTER_SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<ParticleEmitter3D>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:479 (sha256:e9ef136822b18ac97a7bad7ec972136de2e59e66281d0fb2e7a82e74b5367feb)
static FRAME_SCRATCH: std::sync::LazyLock<std::sync::Mutex<Vec<f32>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![0.0_f32; (FRAME_UNIFORM_BYTES / 4.0_f64) as usize])
    });

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:480 (sha256:32e19c3d1ff59d507ce4f1030d939e4ba67cad6ce49abddc0ec93bf369388889)
static RESOURCE_CACHE: std::sync::LazyLock<
    std::sync::Mutex<Vec<(WgpuRenderState, WgpuParticle3DResources)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/scene-wgpu/src/wgpuParticleEmitter3D.ts:481 (sha256:9b3778a9e42b31064f51aa885ad91503eb68212bb4a6499a36144dfa65c7d3af)
static DUMMY_TEXTURE_CACHE: std::sync::LazyLock<
    std::sync::Mutex<Vec<(WgpuRenderState, crate::OpaqueHostValue)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
