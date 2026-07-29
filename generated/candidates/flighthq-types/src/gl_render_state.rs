// @generated from upstream/packages/types/src/GlRenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, ColorTransform, DisplayObjectClipHooks, EntityRuntime, GlBitmapShader,
    GlMaterialRenderer, GlMeshMaterialRenderer, GlShapeMesh, ImageResource, Kind, Matrix,
    RenderProxy2D, SceneGraphSyncPolicy, VideoTexture,
};

// Source: upstream/packages/types/src/GlRenderState.ts:17 (sha256:464eeba3f4bedfaa41e89c4eeecce566a24f5114fdb842604f4ceec1b98c796a)
#[derive(Clone, Default)]
pub struct GlRenderState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
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
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(GlRenderState, Option<BlendMode>) -> () + Send + 'static>,
            >,
        >,
    >,
    pub canvas: crate::OpaqueHostValue,
    pub gl: crate::OpaqueHostValue,
}
impl PartialEq for GlRenderState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for GlRenderState {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:27 (sha256:08d46091d710deac70dc82dd5ba3988c6e74e8161dfcd2028e9f0166f91e02d5)
#[derive(Clone, Default)]
pub struct GlBlendRealization {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub src: GlBlendFactor,
    pub dst: GlBlendFactor,
    pub equation: Option<GlBlendEquation>,
}
impl PartialEq for GlBlendRealization {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:33 (sha256:b12ab248cba7a5676510fa787e945e56b616d16e05eccabd097a7995c7afce8f)
pub type GlBlendFactor = String;

// Source: upstream/packages/types/src/GlRenderState.ts:35 (sha256:8b84dd066ca9a399220d5b710e5f54629408eb1a49b565d2d53ee71c7f6c457b)
pub type GlBlendEquation = String;

// Source: upstream/packages/types/src/GlRenderState.ts:48 (sha256:d985f9387f4eedb986935d1b36db6062b8dbff65da644b4f550b54eec7741cd5)
#[derive(Clone)]
pub struct GlColorAdjustmentFold {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub draw_shape_meshes: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(GlRenderState, RenderProxy2D, Vec<GlShapeMesh>) -> () + Send + 'static>,
        >,
    >,
    pub flush: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(GlRenderState, f64) -> bool + Send + 'static>>,
    >,
    pub record: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(GlRenderStateRuntime, Option<ColorTransform>, f64) -> () + Send + 'static,
            >,
        >,
    >,
}
impl PartialEq for GlColorAdjustmentFold {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:58 (sha256:7f085bde9bee33f86e50ded46dba560b552423df3fb29df4e96cc9721d0b4390)
#[derive(Clone, Default)]
pub struct GlRenderStateRuntimeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub width: f64,
    pub height: f64,
}
impl PartialEq for GlRenderStateRuntimeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GlRenderStateRuntimeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub texture: crate::OpaqueHostValue,
    pub version: f64,
}
impl PartialEq for GlRenderStateRuntimeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct GlRenderStateRuntimeRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub texture: crate::OpaqueHostValue,
    pub uploaded_frame_id: f64,
}
impl PartialEq for GlRenderStateRuntimeRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[doc(hidden)]
#[derive(Default)]
pub struct GlRenderStateRuntimeStorage {
    pub default_bitmap_shader: Option<GlBitmapShader>,
    pub particle_instance_buffer: Option<crate::OpaqueHostValue>,
    pub particle_instance_data: Option<Vec<f32>>,
    pub material_renderer_map: Option<Vec<(Kind, GlMaterialRenderer)>>,
    pub scene_mesh_material_registry: Option<Vec<(Kind, GlMeshMaterialRenderer)>>,
    pub sprite_batch_material_renderer: Option<GlMaterialRenderer>,
    pub current_scissor_rect: Option<GlScissorRect>,
    pub render_target_viewport: Option<GlRenderStateRuntimeRecord1>,
    pub texture_cache: Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>,
    pub image_resource_texture_cache: Vec<(ImageResource, GlRenderStateRuntimeRecord2)>,
    pub video_texture_cache: Option<Vec<(VideoTexture, GlRenderStateRuntimeRecord3)>>,
    pub scissor_stack: Option<Vec<GlScissorRect>>,
}
pub type GlRenderStateRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/GlRenderState.ts:199 (sha256:0da153eab934fe3f87515e82075d31ac520b4f06c9b9f577ab8c9df6deb67b3b)
#[derive(Clone, Default)]
pub struct GlParticleShader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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
impl PartialEq for GlParticleShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:212 (sha256:ded8686177b12513a00a42563ec4137a71485ee36e78558ecc4264002402f2e6)
#[derive(Clone, Default)]
pub struct GlQuadBatchShader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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
impl PartialEq for GlQuadBatchShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:227 (sha256:878007b0f0638b9a8cffca06d9f3695352bacac83938a2bdd17c8531e0284244)
#[derive(Clone, Default)]
pub struct GlColorTransformInstancedShader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub program: crate::OpaqueHostValue,
    pub loc_corner: f64,
    pub loc_world_matrix: crate::OpaqueHostValue,
    pub loc_texture: crate::OpaqueHostValue,
}
impl PartialEq for GlColorTransformInstancedShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:237 (sha256:62f2e01ebd177547173cfce08db7ebca084c85651e7ad11469783a9caf3c435d)
#[derive(Clone, Default)]
pub struct GlUniformColorTransformShader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub program: crate::OpaqueHostValue,
    pub loc_corner: f64,
    pub loc_world_matrix: crate::OpaqueHostValue,
    pub loc_texture: crate::OpaqueHostValue,
    pub loc_color_multiplier: crate::OpaqueHostValue,
    pub loc_color_offset: crate::OpaqueHostValue,
}
impl PartialEq for GlUniformColorTransformShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:250 (sha256:7406eb32184442b43c5c70062e932bc89b3e113b70586bc5d1fc23a1eb54eaae)
#[derive(Clone, Default)]
pub struct GlShapeMeshColorTransformShader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub program: crate::OpaqueHostValue,
    pub position_location: f64,
    pub matrix_location: Option<crate::OpaqueHostValue>,
    pub color_location: Option<crate::OpaqueHostValue>,
    pub color_multiplier_location: Option<crate::OpaqueHostValue>,
    pub color_offset_location: Option<crate::OpaqueHostValue>,
}
impl PartialEq for GlShapeMeshColorTransformShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:259 (sha256:c5eed51656152d130c5bd39967bda2fdec09e68c7666b1789992993ec2ac9b57)
#[derive(Clone, Default)]
pub struct GlScissorRect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for GlScissorRect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
