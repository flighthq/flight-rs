// @generated from upstream/packages/scene-gl/src/glShadowMap.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GL_SKIN_VERTEX_DECLARATIONS_GLSL as gl_skin_vertex_declarations_glsl_constant, GlMeshProgram,
    GlSceneShadow, compile_gl_program, ensure_gl_mesh_upload, ensure_gl_scene_program,
    ensure_gl_skin_palette, get_gl_scene_runtime,
};
use flighthq_camera::get_camera_view_projection_matrix4;
use flighthq_geometry::create_matrix4;
use flighthq_mesh::has_mesh_geometry_skin;
use flighthq_node::for_each_node_descendant;
use flighthq_render_gl::{create_gl_render_target, upload_gl_skin_palette_texture};
use flighthq_scene::update_mesh_morph;
use flighthq_types::{
    Adjustment, BlendMode, BoundsNodeAny, Camera, ColorTransform, DisplayObjectClipHooks,
    GlBitmapShader, GlBlendRealization, GlColorAdjustmentFold, GlColorTransformInstancedShader,
    GlCompressedTextureDecoder, GlCompressedTextureUploader, GlParticleShader, GlQuadBatchShader,
    GlRenderState, GlRenderTarget, GlShaderLocations, GlShapeMeshColorTransformShader,
    GlUniformColorTransformShader, ImageResource, InteractionSignals, Kind, Material, Matrix,
    Matrix4, MeshGeometryGlData, MeshGeometryWgpuData, MeshMorphBindPose, MeshSkinBindPose, Node,
    NodeInteractionState, NodeSignals, NodeTraitsKey, Rectangle, RenderProxy, RenderProxy2D,
    RenderProxyAdapter, RenderState, RenderTargetDescriptor, Renderable, Renderer, Sampler,
    SceneGraphSyncPolicy, SceneNode, SceneResourceRef, Stage, StageSignals, TextureColorSpace,
    TextureFilter, TextureWrap, Vector2,
};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

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

// Source: upstream/packages/scene-gl/src/glShadowMap.ts:27 (sha256:6693ff51408cac99f2767eb6549b3d66d3dc5d89a20af71bf580747a6860e9f4)
pub fn draw_gl_scene_shadow_map(
    mut state: GlRenderState,
    scene: &SceneNode,
    shadow_camera: &Camera,
) -> () {
    let gl: std::sync::Arc<std::sync::Mutex<crate::OpaqueHostValue>> =
        std::sync::Arc::new(std::sync::Mutex::new((state.gl).clone()));
    let mut runtime = get_gl_scene_runtime(&mut state);
    if ((runtime.shadow_target).clone()).is_none() {
        runtime.shadow_target = Some(create_gl_render_target(
            &state,
            &RenderTargetDescriptor {
                __flight_identity: std::sync::Arc::new(()),
                depth: Some("depth-stencil-sampled".to_owned()),
                height: SHADOW_MAP_SIZE,
                width: SHADOW_MAP_SIZE,
                format: None,
                color_attachments: None,
                color_formats: None,
                sample_count: None,
                color_space: None,
                clear_colors: None,
                clear_depth: None,
            },
        ));
    }
    let target = (runtime.shadow_target).clone();
    let matrix: std::sync::Arc<std::sync::Mutex<Matrix4>> =
        std::sync::Arc::new(std::sync::Mutex::new(
            (runtime.shadow.as_ref().map(|value| (value.matrix).clone())).unwrap_or(
                create_matrix4(
                    None, None, None, None, None, None, None, None, None, None, None, None, None,
                    None, None, None,
                ),
            ),
        ));
    get_camera_view_projection_matrix4(&mut (*matrix.lock().unwrap()), shadow_camera, 1.0_f64);
    let rigid_program = ensure_gl_scene_program(
        &mut state,
        "shadow:depth".to_owned(),
        &mut compile_shadow_depth_program,
    );
    let skinned_program: std::sync::Arc<std::sync::Mutex<Option<GlMeshProgram>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    let prev_framebuffer = crate::host_value::<Option<crate::OpaqueHostValue>>("host.getParameter");
    let prev_viewport = crate::host_value::<Vec<i32>>("host.getParameter");
    crate::host_value::<()>("host.bindFramebuffer");
    crate::host_value::<()>("host.viewport");
    crate::host_value::<()>("host.enable");
    crate::host_value::<()>("host.depthFunc");
    crate::host_value::<()>("host.depthMask");
    crate::host_value::<()>("host.enable");
    crate::host_value::<()>("host.cullFace");
    crate::host_value::<()>("host.clear");
    let bound_program: std::sync::Arc<std::sync::Mutex<Option<GlMeshProgram>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
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
            if ((mesh.morph).clone()).is_some() {
                update_mesh_morph(&mut mesh);
            }
            let skinned =
                (((mesh.skin).clone()).is_some()) && (has_mesh_geometry_skin(&mesh.geometry));
            let program = if skinned {
                {
                    (*skinned_program.lock().unwrap())?? = Some(ensure_gl_scene_program(
                        &mut state,
                        "shadow:depth:skin".to_owned(),
                        &mut compile_shadow_depth_skinned_program,
                    ));
                    (*skinned_program.lock().unwrap())
                }
            } else {
                (rigid_program).clone()
            };
            if (program != (*bound_program.lock().unwrap()).clone()) {
                crate::host_value::<()>("host.useProgram");
                crate::host_value::<()>("host.uniformMatrix4fv");
                (*bound_program.lock().unwrap()) = Some(program);
            }
            crate::host_value::<()>("host.uniformMatrix4fv");
            if skinned {
                crate::host_value::<()>("host.activeTexture");
                upload_gl_skin_palette_texture(
                    (*gl.lock().unwrap()).clone(),
                    &mut ensure_gl_skin_palette(&mut state),
                    &mesh.skin.as_mut().unwrap().skeleton.joint_matrices,
                    (__flight_js_to_i32(
                        ((mesh.skin.as_mut().unwrap().skeleton.joint_matrices.len() as f64)
                            / 16.0_f64),
                    ) | __flight_js_to_i32(0.0_f64)) as f64,
                );
                crate::host_value::<()>("host.uniform1i");
            }
            let upload = ensure_gl_mesh_upload(&mut state, &mut mesh.geometry, Some(skinned));
            crate::host_value::<()>("host.bindVertexArray");
            if ((upload.index_buffer).clone()).is_some() {
                crate::host_value::<()>("host.drawElements");
            } else {
                crate::host_value::<()>("host.drawArrays");
            }
        },
    );
    crate::host_value::<()>("host.activeTexture");
    crate::host_value::<()>("host.bindFramebuffer");
    crate::host_value::<()>("host.viewport");
    crate::host_value::<()>("host.disable");
    crate::host_value::<()>("host.cullFace");
    runtime.shadow = Some(GlSceneShadow {
        __flight_identity: std::sync::Arc::new(()),
        matrix: (*matrix.lock().unwrap()).clone(),
        texture: ((target.as_ref().unwrap().depth_texture).clone()).unwrap(),
    });
}

// Source: upstream/packages/scene-gl/src/glShadowMap.ts:126 (sha256:caf241aba7522f1ee4e6972feba6e17a209eb37361e610d4232ba5dd9ec3a893)
fn compile_shadow_depth_program(gl: crate::OpaqueHostValue) -> GlMeshProgram {
    let program = compile_gl_program(
        (gl).clone(),
        (SHADOW_DEPTH_VERTEX).clone(),
        (SHADOW_DEPTH_FRAGMENT).clone(),
    );
    return GlMeshProgram {
        __flight_identity: std::sync::Arc::new(()),
        loc_model: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_normal_matrix: None,
        loc_view_projection: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        program: (program).clone(),
        loc_object_alpha: None,
        loc_joint_texture: None,
        loc_uv_transform: None,
    };
}

// Source: upstream/packages/scene-gl/src/glShadowMap.ts:139 (sha256:56ff60d588c0d3209cdd1d1b0894f009fe7d12a917aa5985f3c4c9ef1925c48f)
fn compile_shadow_depth_skinned_program(gl: crate::OpaqueHostValue) -> GlMeshProgram {
    let program = compile_gl_program(
        (gl).clone(),
        (SHADOW_DEPTH_SKINNED_VERTEX).clone(),
        (SHADOW_DEPTH_FRAGMENT).clone(),
    );
    return GlMeshProgram {
        __flight_identity: std::sync::Arc::new(()),
        loc_joint_texture: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        loc_model: crate::host_value::<Option<crate::OpaqueHostValue>>("host.getUniformLocation"),
        loc_normal_matrix: None,
        loc_view_projection: crate::host_value::<Option<crate::OpaqueHostValue>>(
            "host.getUniformLocation",
        ),
        program: (program).clone(),
        loc_object_alpha: None,
        loc_uv_transform: None,
    };
}

// Source: upstream/packages/scene-gl/src/glShadowMap.ts:150 (sha256:4b42abc931bc4636bfa4ae19dd3c8d8a3f69e109d3189486fc666c93f6b840da)
const SHADOW_MAP_SIZE: f64 = 1024.0_f64;

// Source: upstream/packages/scene-gl/src/glShadowMap.ts:152 (sha256:4b908dfae7067667aaf297ec418ff95ffb30aae196acb1afcd6a86d4901354ea)
const SHADOW_DEPTH_VERTEX: &'static str = "#version 300 es\nlayout(location = 0) in vec3 a_position;\nuniform mat4 u_viewProjection;\nuniform mat4 u_model;\nvoid main() {\n  gl_Position = u_viewProjection * u_model * vec4(a_position, 1.0);\n}\n";

// Source: upstream/packages/scene-gl/src/glShadowMap.ts:164 (sha256:0e6ef7941f59172549c49098338d5d1c21390e61e831e4272bf63e0960fada04)
static SHADOW_DEPTH_SKINNED_VERTEX: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "#version 300 es\n{}\nlayout(location = 0) in vec3 a_position;\nuniform mat4 u_viewProjection;\nuniform mat4 u_model;\nvoid main() {{\n  gl_Position = u_viewProjection * u_model * skinMatrix() * vec4(a_position, 1.0);\n}}\n",
        gl_skin_vertex_declarations_glsl_constant
    )
});

// Source: upstream/packages/scene-gl/src/glShadowMap.ts:174 (sha256:caadb2b83b29a7a394896a8a7eccd789b2e7af0866325b4a1045105ec5bb6d4b)
const SHADOW_DEPTH_FRAGMENT: &'static str = "#version 300 es\nprecision highp float;\nout vec4 fragColor;\nvoid main() {\n  fragColor = vec4(1.0);\n}\n";
