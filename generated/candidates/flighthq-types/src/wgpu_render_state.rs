// @generated from upstream/packages/types/src/WgpuRenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, ColorTransform, DisplayObjectClipHooks, EntityRuntime, ImageResource, Kind, Matrix,
    SceneGraphSyncPolicy, WgpuMaterialRenderer, WgpuMeshMaterialRenderer,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub width: f64,
    pub height: f64,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:12 (sha256:b04573db8382026a9e16cb4953f1dda844ec6cd863bf52f4cb3b0689edf3bda4)
#[derive(Clone, Default)]
pub struct WgpuRenderState {
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
                Box<dyn FnMut(WgpuRenderState, Option<BlendMode>) -> () + Send + 'static>,
            >,
        >,
    >,
    pub canvas: crate::OpaqueHostValue,
    pub context: crate::OpaqueHostValue,
    pub device: crate::OpaqueHostValue,
    pub format: crate::OpaqueHostValue,
}
impl PartialEq for WgpuRenderState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuRenderState {
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:27 (sha256:b977ad13d425db23ab8ebbca26676f92b77376bfbf7fe7ecc3d65e8e918eb83b)
#[derive(Clone)]
pub struct WgpuColorAdjustmentFold {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub record: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(WgpuRenderStateRuntime, Option<ColorTransform>, f64) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub resolve_flush: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(WgpuRenderState, f64) -> Option<WgpuColorAdjustmentFlush>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for WgpuColorAdjustmentFold {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:39 (sha256:5d7cf14380f473bf149be5f06391b3076820daa8a31e504660b3fd6f948b79c7)
#[derive(Clone, Default)]
pub struct WgpuColorAdjustmentFlush {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: Vec<f32>,
    pub floats: f64,
    pub module: crate::OpaqueHostValue,
}
impl PartialEq for WgpuColorAdjustmentFlush {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:49 (sha256:152399c9dc63e027a8e5022e9fef13605940e46fd3c45f04d0d59efc783da472)
#[doc(hidden)]
#[derive(Default)]
pub struct WgpuRenderStateRuntimeStorage {
    pub texture_cache: Vec<(crate::OpaqueHostValue, WgpuTextureEntry)>,
    pub image_resource_texture_cache: Vec<(ImageResource, WgpuImageResourceTextureEntry)>,
    pub default_bitmap_shader: Option<WgpuBitmapShader>,
    pub particle_instance_buffer: Option<crate::OpaqueHostValue>,
    pub particle_instance_data: Option<Vec<f32>>,
    pub sprite_batch_material_renderer: Option<WgpuMaterialRenderer>,
    pub material_renderer_map: Option<Vec<(Kind, WgpuMaterialRenderer)>>,
    pub scene_mesh_material_registry: Option<Vec<(Kind, WgpuMeshMaterialRenderer)>>,
    pub scissor_stack: Vec<WgpuScissorRect>,
    pub current_scissor_rect: Option<WgpuScissorRect>,
    pub render_target_viewport: Option<SharedStructuralRecord1>,
}
pub type WgpuRenderStateRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/WgpuRenderState.ts:228 (sha256:e003cc095073ba6707274c00e75dcf6b990c0b298fb4057aa462e70bf224260d)
#[derive(Clone, Default)]
pub struct WgpuBitmapShaderRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: f64,
}
impl PartialEq for WgpuBitmapShaderRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct WgpuBitmapShader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub pipeline: crate::OpaqueHostValue,
    pub bind: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(WgpuRenderState, WgpuBitmapShaderRecord2) -> () + Send + 'static>,
        >,
    >,
}
impl PartialEq for WgpuBitmapShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:240 (sha256:5fe417094a9800132bc849b19f0360a096f37d1fda60600f603a8eeba76f6676)
#[derive(Clone, Default)]
pub struct WgpuClipContourEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub vertex_buffer: crate::OpaqueHostValue,
    pub vertex_count: f64,
    pub uniform_buffer: crate::OpaqueHostValue,
    pub bind_group: crate::OpaqueHostValue,
    pub depth: f64,
}
impl PartialEq for WgpuClipContourEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:251 (sha256:da157d7dd2aef06c3ff53a1e2cafb130aaa6d7f8d3ae707eac7859094af30f73)
#[derive(Clone, Default)]
pub struct WgpuClipContourPipelines {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub write: crate::OpaqueHostValue,
    pub erase: crate::OpaqueHostValue,
    pub bind_group_layout: crate::OpaqueHostValue,
}
impl PartialEq for WgpuClipContourPipelines {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:259 (sha256:2c70165a98eeef7595432faecc986eeba242241269a5e9ae3b54f559748e316b)
#[derive(Clone, Default)]
pub struct WgpuSavedPassState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub canvas_texture_view: Option<crate::OpaqueHostValue>,
    pub canvas_view_cleared: bool,
    pub depth_stencil_view: Option<crate::OpaqueHostValue>,
    pub render_target_viewport: Option<SharedStructuralRecord1>,
    pub render_transform2_d: Option<Matrix>,
    pub color_format: Option<crate::OpaqueHostValue>,
}
impl PartialEq for WgpuSavedPassState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:270 (sha256:34dfe22efbf1d2f4e16ac9a93fc703b8a54032d9ea689c75c5e61549dc76a3c9)
#[derive(Clone, Default)]
pub struct WgpuScissorRect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for WgpuScissorRect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:281 (sha256:21262fc0044ca6ebf82591068fd13c9e14513b1f95e2d0943a2f51bb474441e0)
#[derive(Clone, Default)]
pub struct WgpuShapeMeshBuffers {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub vertex_buffer: Option<crate::OpaqueHostValue>,
    pub vertex_capacity: f64,
    pub index_buffer: Option<crate::OpaqueHostValue>,
    pub index_capacity: f64,
    pub uniform_buffer: Option<crate::OpaqueHostValue>,
    pub bind_group: Option<crate::OpaqueHostValue>,
}
impl PartialEq for WgpuShapeMeshBuffers {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:294 (sha256:0e94554b02fe046b289bb369e7bc2bf9804ca1d647be647f82be40f65cb53680)
#[derive(Clone, Default)]
pub struct WgpuShapeMeshPipeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub pipeline: crate::OpaqueHostValue,
    pub bind_group_layout: crate::OpaqueHostValue,
}
impl PartialEq for WgpuShapeMeshPipeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:302 (sha256:adc353123663dbd2ed1e80cc22e5e57943326bd6ff1ecaa3c5724791ff9e06e4)
#[derive(Clone, Default)]
pub struct WgpuSpriteBatchBufferSlot {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub instance_buffer: Option<crate::OpaqueHostValue>,
    pub instance_capacity: f64,
    pub material_buffer: Option<crate::OpaqueHostValue>,
    pub material_capacity: f64,
}
impl PartialEq for WgpuSpriteBatchBufferSlot {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:311 (sha256:10e77703426773a02555b8f4a540df49a57af8461c8a87f0f58cd595170e3cfe)
#[derive(Clone, Default)]
pub struct WgpuTextureEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bind_group: crate::OpaqueHostValue,
    pub texture: crate::OpaqueHostValue,
    pub view: crate::OpaqueHostValue,
}
impl PartialEq for WgpuTextureEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:319 (sha256:b112f5e5239daad7dc84cf342ce4b77b177150c08024fa588589d634f0947d6b)
#[derive(Clone, Default)]
pub struct WgpuImageResourceTextureEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bind_group: crate::OpaqueHostValue,
    pub texture: crate::OpaqueHostValue,
    pub view: crate::OpaqueHostValue,
    pub version: f64,
}
impl PartialEq for WgpuImageResourceTextureEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
