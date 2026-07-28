// @generated from upstream/packages/types/src/GlRenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, ColorTransform, DisplayObjectClipHooks, GlBitmapShader, GlCompressedTextureDecoder,
    GlCompressedTextureUploader, GlMaterialRenderer, GlRenderTarget, GlShaderLocations,
    ImageResource, Material, Matrix, RenderProxy2D, RenderState, Renderable, SceneGraphSyncPolicy,
};

// Source: upstream/packages/types/src/GlRenderState.ts:17 (sha256:464eeba3f4bedfaa41e89c4eeecce566a24f5114fdb842604f4ceec1b98c796a)
#[derive(Clone)]
pub struct GlRenderState {
    pub allow_smoothing: bool,
    pub background_color: f64,
    pub background_color_rgba: Vec<f64>,
    pub background_color_string: String,
    pub current_clip_depth: f64,
    pub display_object_clip_hooks: Option<DisplayObjectClipHooks>,
    pub pixel_ratio: f64,
    pub render_alpha: f64,
    pub render_blend_mode: Option<BlendMode>,
    pub render_transform2_d: Option<Matrix>,
    pub scene_graph_sync_policy: SceneGraphSyncPolicy,
    pub round_pixels: bool,
    pub apply_blend_mode: Option<
        std::sync::Arc<dyn Fn(GlRenderState, Option<BlendMode>) -> () + Send + Sync + 'static>,
    >,
    pub canvas: crate::OpaqueHostValue,
    pub gl: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/GlRenderState.ts:27 (sha256:08d46091d710deac70dc82dd5ba3988c6e74e8161dfcd2028e9f0166f91e02d5)
#[derive(Clone)]
pub struct GlBlendRealization {
    pub src: GlBlendFactor,
    pub dst: GlBlendFactor,
    pub equation: Option<GlBlendEquation>,
}

// Source: upstream/packages/types/src/GlRenderState.ts:33 (sha256:b12ab248cba7a5676510fa787e945e56b616d16e05eccabd097a7995c7afce8f)
pub type GlBlendFactor = String;

// Source: upstream/packages/types/src/GlRenderState.ts:35 (sha256:8b84dd066ca9a399220d5b710e5f54629408eb1a49b565d2d53ee71c7f6c457b)
pub type GlBlendEquation = String;

// Source: upstream/packages/types/src/GlRenderState.ts:48 (sha256:d985f9387f4eedb986935d1b36db6062b8dbff65da644b4f550b54eec7741cd5)
#[derive(Clone)]
pub struct GlColorAdjustmentFold {
    pub draw_shape_meshes: crate::OpaqueHostValue,
    pub flush: crate::OpaqueHostValue,
    pub record: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/GlRenderState.ts:58 (sha256:7f085bde9bee33f86e50ded46dba560b552423df3fb29df4e96cc9721d0b4390)
#[derive(Clone)]
pub struct GlRenderStateRuntimeRecord1 {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone)]
pub struct GlRenderStateRuntime {
    pub binding: Option<crate::OpaqueHostValue>,
    pub color_adjustment_channel_mixing_guard:
        Option<std::sync::Arc<dyn Fn(RenderState, Renderable) -> () + Send + Sync + 'static>>,
    pub current_frame_id: f64,
    pub render_adapt_hook: Option<
        std::sync::Arc<
            dyn Fn(RenderState, Renderable, RenderProxy2D) -> () + Send + Sync + 'static,
        >,
    >,
    pub render_proxy_adapter_map: crate::OpaqueHostValue,
    pub render_proxy_map: crate::OpaqueHostValue,
    pub renderer_map: crate::OpaqueHostValue,
    pub renderer_map_id: f64,
    pub temp_stack: Vec<Renderable>,
    pub current_blend_mode: Option<BlendMode>,
    pub current_program: Option<crate::OpaqueHostValue>,
    pub current_texture: Option<crate::OpaqueHostValue>,
    pub gl_blend_mode_registry: Option<Option<crate::OpaqueHostValue>>,
    pub default_bitmap_shader: GlBitmapShader,
    pub particle_shader: Option<GlParticleShader>,
    pub particle_corner_buffer: Option<crate::OpaqueHostValue>,
    pub particle_instance_buffer: Option<crate::OpaqueHostValue>,
    pub particle_instance_data: Option<Vec<f32>>,
    pub quad_batch_shader: Option<GlQuadBatchShader>,
    pub quad_batch_corner_buffer: Option<crate::OpaqueHostValue>,
    pub color_transform_instanced_shader: Option<GlColorTransformInstancedShader>,
    pub uniform_color_transform_shader: Option<GlUniformColorTransformShader>,
    pub shape_mesh_color_transform_shader: Option<GlShapeMeshColorTransformShader>,
    pub gl_color_adjustment_fold: Option<Option<GlColorAdjustmentFold>>,
    pub gl_color_adjustment_guard: Option<
        Option<std::sync::Arc<dyn Fn(GlRenderState, ColorTransform) -> () + Send + Sync + 'static>>,
    >,
    pub material_renderer_map: Option<crate::OpaqueHostValue>,
    pub scene_mesh_material_registry: Option<Option<crate::OpaqueHostValue>>,
    pub scene_mesh_upload_cache: Option<Option<crate::OpaqueHostValue>>,
    pub material_bitmap_shader_map: Option<crate::OpaqueHostValue>,
    pub webgl_shader_binding_resolver: Option<
        std::sync::Arc<dyn Fn(RenderProxy2D) -> Option<GlBitmapShader> + Send + Sync + 'static>,
    >,
    pub sprite_batch_blend_mode: Option<BlendMode>,
    pub sprite_batch_material: Option<Material>,
    pub sprite_batch_material_renderer: Option<GlMaterialRenderer>,
    pub sprite_batch_material_floats: f64,
    pub sprite_batch_material_data: Vec<f32>,
    pub sprite_batch_material_buffer: Option<crate::OpaqueHostValue>,
    pub sprite_batch_count: f64,
    pub sprite_batch_instance_buffer: Option<crate::OpaqueHostValue>,
    pub sprite_batch_instance_data: Vec<f32>,
    pub sprite_batch_texture: Option<ImageResource>,
    pub sprite_batch_color_transform_mode: f64,
    pub sprite_batch_uniform_color_transform: Option<ColorTransform>,
    pub sprite_batch_color_transform_data: Vec<f32>,
    pub sprite_batch_color_transform_buffer: Option<crate::OpaqueHostValue>,
    pub clip_forms: Vec<String>,
    pub current_mask_depth: Option<f64>,
    pub current_scissor_rect: Option<Option<GlScissorRect>>,
    pub current_framebuffer: Option<crate::OpaqueHostValue>,
    pub current_render_target: Option<Option<GlRenderTarget>>,
    pub render_target_viewport: Option<GlRenderStateRuntimeRecord1>,
    pub shader_loc: GlShaderLocations,
    pub texture_cache: crate::OpaqueHostValue,
    pub image_resource_texture_cache: crate::OpaqueHostValue,
    pub compressed_texture_decoder: Option<Option<GlCompressedTextureDecoder>>,
    pub compressed_texture_upload: Option<Option<GlCompressedTextureUploader>>,
    pub video_texture_cache: Option<crate::OpaqueHostValue>,
    pub mipmapped_textures: Option<crate::OpaqueHostValue>,
    pub anisotropy_ext: Option<Option<crate::OpaqueHostValue>>,
    pub max_anisotropy: Option<f64>,
    pub quad_vertex_buffer: crate::OpaqueHostValue,
    pub quad_index_buffer: crate::OpaqueHostValue,
    pub quad_vertex_data: Vec<f32>,
    pub matrix_array: Vec<f32>,
    pub scissor_stack: Option<Vec<GlScissorRect>>,
}

// Source: upstream/packages/types/src/GlRenderState.ts:199 (sha256:0da153eab934fe3f87515e82075d31ac520b4f06c9b9f577ab8c9df6deb67b3b)
#[derive(Clone)]
pub struct GlParticleShader {
    pub program: crate::OpaqueHostValue,
    pub loc_corner: f64,
    pub loc_pos: f64,
    pub loc_cos_scale: f64,
    pub loc_sin_scale: f64,
    pub loc_color: f64,
    pub loc_uv_rect: f64,
    pub loc_size: f64,
    pub loc_world_matrix: crate::OpaqueHostValue,
    pub loc_texture: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/GlRenderState.ts:212 (sha256:ded8686177b12513a00a42563ec4137a71485ee36e78558ecc4264002402f2e6)
#[derive(Clone)]
pub struct GlQuadBatchShader {
    pub program: crate::OpaqueHostValue,
    pub loc_corner: f64,
    pub loc_mat_ab: f64,
    pub loc_mat_cd: f64,
    pub loc_mat_txty: f64,
    pub loc_size: f64,
    pub loc_uv_rect: f64,
    pub loc_alpha: f64,
    pub loc_world_matrix: crate::OpaqueHostValue,
    pub loc_texture: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/GlRenderState.ts:227 (sha256:878007b0f0638b9a8cffca06d9f3695352bacac83938a2bdd17c8531e0284244)
#[derive(Clone)]
pub struct GlColorTransformInstancedShader {
    pub program: crate::OpaqueHostValue,
    pub loc_corner: f64,
    pub loc_world_matrix: crate::OpaqueHostValue,
    pub loc_texture: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/GlRenderState.ts:237 (sha256:62f2e01ebd177547173cfce08db7ebca084c85651e7ad11469783a9caf3c435d)
#[derive(Clone)]
pub struct GlUniformColorTransformShader {
    pub program: crate::OpaqueHostValue,
    pub loc_corner: f64,
    pub loc_world_matrix: crate::OpaqueHostValue,
    pub loc_texture: crate::OpaqueHostValue,
    pub loc_color_multiplier: crate::OpaqueHostValue,
    pub loc_color_offset: crate::OpaqueHostValue,
}

// Source: upstream/packages/types/src/GlRenderState.ts:250 (sha256:7406eb32184442b43c5c70062e932bc89b3e113b70586bc5d1fc23a1eb54eaae)
#[derive(Clone)]
pub struct GlShapeMeshColorTransformShader {
    pub program: crate::OpaqueHostValue,
    pub position_location: f64,
    pub matrix_location: Option<crate::OpaqueHostValue>,
    pub color_location: Option<crate::OpaqueHostValue>,
    pub color_multiplier_location: Option<crate::OpaqueHostValue>,
    pub color_offset_location: Option<crate::OpaqueHostValue>,
}

// Source: upstream/packages/types/src/GlRenderState.ts:259 (sha256:c5eed51656152d130c5bd39967bda2fdec09e68c7666b1789992993ec2ac9b57)
#[derive(Clone)]
pub struct GlScissorRect {
    pub height: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
