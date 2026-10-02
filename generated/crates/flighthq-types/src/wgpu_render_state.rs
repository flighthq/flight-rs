// @generated from upstream/packages/types/src/WgpuRenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, CanvasShapeCommand, ColorAdjustmentUnsupportedGuard, ColorScaleBias,
    EffectPaddingResolver, EntityRuntime, HostCanvasCapability, HostImageCapability, ImageResource,
    Kind, NodeRenderer, RenderProxy, RenderProxy2D, RenderRegistrySignals, RenderRootGuard,
    RenderState, Scene2DClipHooks, Scene3DGraphSyncPolicy, ShapeRasterizer, StrokeTessellator,
    TextureSource, TintMaterialData, WgpuCompressedTextureDecoder, WgpuCompressedTextureUploader,
    WgpuCustomMaterialShaderSource, WgpuDeviceRuntime, WgpuDeviceState, WgpuEffectRegistration,
    WgpuMeshMaterialRenderer, WgpuModifierSnippet, WgpuParticleResources, WgpuQuadBatchResources,
    WgpuQuadMaterialRenderer, WgpuRenderPass, WgpuRenderPassViewport, WgpuRenderTarget,
    WgpuScene3DPass, WgpuShapeMesh, WgpuSkinningAdapter, WgpuTextureResolver, WgpuVelocityWriter,
};

// Source: upstream/packages/types/src/WgpuRenderState.ts:34 (sha256:4a4eda3e55c94215c62f6013a1f6fc6dc4e51d4d298b35eb6325857677512fa5)
#[derive(Clone, Default)]
pub struct WgpuRenderState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub allow_smoothing: bool,
    pub current_clip_depth: f64,
    pub display_object_clip_hooks: Option<Scene2DClipHooks>,
    pub pixel_ratio: f64,
    pub canvas_host: Option<HostCanvasCapability>,
    pub image_host: Option<HostImageCapability>,
    pub render_alpha: f64,
    pub render_blend_mode: Option<BlendMode>,
    pub scene_graph_sync_policy: Scene3DGraphSyncPolicy,
    pub round_pixels: bool,
    pub apply_blend_mode: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(WgpuRenderState, Option<BlendMode>) -> () + Send + 'static>,
            >,
        >,
    >,
    pub device_state: WgpuDeviceState,
    pub device: crate::OpaqueHostValue,
    pub format: crate::OpaqueHostValue,
    pub registries: WgpuRenderRegistries,
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
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:50 (sha256:d697a237e1b90f97d65d5b19091e77601e8011e7e537e5591acffaa9604243e3)
#[derive(Clone, Default)]
pub struct WgpuOffscreenRenderStateResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub state: WgpuRenderState,
}
impl PartialEq for WgpuOffscreenRenderStateResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct WgpuOffscreenRenderStateResultRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub info: crate::OpaqueHostValue,
}
impl PartialEq for WgpuOffscreenRenderStateResultRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct WgpuOffscreenRenderStateResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for WgpuOffscreenRenderStateResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuOffscreenRenderStateResult {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:58 (sha256:5b16c5eddca6e7f3f23a5a9459d1e349145f36cfacf1a1ead88d6d6ea4f5f911)
#[derive(Clone, Default)]
pub struct WgpuRenderRegistries {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub canvas_shape_commands: Option<Vec<(Kind, CanvasShapeCommand<crate::OpaqueHostValue>)>>,
    pub color_adjustments: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(RenderState, RenderProxy, Option<RenderProxy>) -> () + Send + 'static,
                >,
            >,
        >,
    >,
    pub color_adjustment_unsupported_guard: Option<ColorAdjustmentUnsupportedGuard>,
    pub effect_padding_resolvers: Option<Vec<(Kind, EffectPaddingResolver)>>,
    pub node_renderers: Vec<(Kind, NodeRenderer)>,
    pub render_root_guard: Option<RenderRootGuard>,
    pub stroke_tessellator: Option<StrokeTessellator>,
    pub color_adjustment_feature: Option<WgpuColorAdjustmentMaterialFeature>,
    pub color_adjustment_feature_guard: Option<WgpuColorAdjustmentMaterialFeatureGuard>,
    pub compressed_texture_decoder: Option<WgpuCompressedTextureDecoder>,
    pub compressed_texture_upload: Option<WgpuCompressedTextureUploader>,
    pub custom_material_shaders: Vec<(Kind, WgpuCustomMaterialShaderSource)>,
    pub gpu_skinning: Option<WgpuSkinningAdapter>,
    pub material_renderers: Vec<(
        Kind,
        crate::FlightUnion2<WgpuMeshMaterialRenderer, WgpuQuadMaterialRenderer>,
    )>,
    pub modifier_snippets: Vec<(Kind, WgpuModifierSnippet)>,
    pub effects: Vec<(Kind, WgpuEffectRegistration)>,
    pub passes: Option<Vec<WgpuScene3DPass>>,
    pub shape_rasterizer: Option<ShapeRasterizer>,
    pub texture_resolvers: Vec<(Kind, WgpuTextureResolver)>,
    pub velocity_writers: Vec<(Kind, WgpuVelocityWriter)>,
}
impl PartialEq for WgpuRenderRegistries {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:85 (sha256:7db70fc400926f7c23e83eca154e3294581b61b582548d2dd1df3969f5bf7edb)
#[derive(Clone)]
pub struct WgpuColorAdjustmentMaterialFeature {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fragment_shader_chunk: String,
    pub matrix_fragment_shader_chunk: String,
    pub draw_shape_meshes: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            WgpuRenderState,
                            RenderProxy2D,
                            Vec<WgpuShapeMesh>,
                            WgpuShapeMeshBuffers,
                        ) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub record: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        WgpuRenderStateRuntime,
                        Option<
                            crate::FlightUnion2<
                                ColorScaleBias,
                                crate::FlightUnion2<TintMaterialData, Vec<f64>>,
                            >,
                        >,
                        f64,
                    ) -> ()
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
impl PartialEq for WgpuColorAdjustmentMaterialFeature {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:107 (sha256:8f369dce49c12d3b007a0eed13140c4bed3f1ba43b05a4a336f8c657c5f8a2ee)
pub type WgpuColorAdjustmentMaterialFeatureGuard = std::sync::Arc<
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
>;

// Source: upstream/packages/types/src/WgpuRenderState.ts:115 (sha256:ba949754af13c5bc5e13f170befd56868a323fecc070c8482dbde40c37eb6942)
#[derive(Clone)]
pub struct WgpuColorAdjustmentFlush {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: crate::FlightUnion2<Vec<f32>, Vec<u32>>,
    pub floats: f64,
    pub module: crate::OpaqueHostValue,
}
impl PartialEq for WgpuColorAdjustmentFlush {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:125 (sha256:c4b20c045bea80317f0e6b7459cb2d354f4a6a18a5912aaf2ac35ef517fc42de)
#[derive(Clone, Default)]
pub struct WgpuBindGroupLayouts {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub texture_bind_group_layout: crate::OpaqueHostValue,
    pub uniform_bind_group_layout: crate::OpaqueHostValue,
}
impl PartialEq for WgpuBindGroupLayouts {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuBindGroupLayouts {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:130 (sha256:31af0fe536b3c8f515b646cdbfa905363830b8dc115575511a7923929d166fe4)
#[derive(Clone, Default)]
pub struct WgpuRenderStateRuntimeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub bind_group_layout: crate::OpaqueHostValue,
    pub pipeline: crate::OpaqueHostValue,
}
impl PartialEq for WgpuRenderStateRuntimeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct WgpuRenderStateRuntimeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub width: f64,
}
impl PartialEq for WgpuRenderStateRuntimeRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct WgpuRenderStateRuntimeRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clear: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub signals: RenderRegistrySignals,
}
impl PartialEq for WgpuRenderStateRuntimeRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[doc(hidden)]
pub struct WgpuRenderStateRuntimeStorage {
    pub context: WgpuDeviceRuntime,
    pub registries: WgpuRenderRegistries,
    pub teardowns: Vec<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(WgpuRenderState) -> () + Send + 'static>>>,
    >,
    pub borrowed_surface_extent: Option<WgpuRenderStateRuntimeRecord2>,
    pub mipmap_pipeline_cache: Vec<(crate::OpaqueHostValue, WgpuRenderStateRuntimeRecord1)>,
    pub texture_cache: Vec<(crate::OpaqueHostValue, WgpuTextureEntry)>,
    pub texture_source_premultiplied_texture_cache:
        Vec<(TextureSource, WgpuTextureSourceTextureEntry)>,
    pub texture_source_premultiplied_srgb_texture_cache:
        Vec<(TextureSource, WgpuTextureSourceTextureEntry)>,
    pub texture_source_straight_texture_cache: Vec<(TextureSource, WgpuTextureSourceTextureEntry)>,
    pub texture_source_straight_srgb_texture_cache:
        Vec<(TextureSource, WgpuTextureSourceTextureEntry)>,
    pub video_texture_cache: Option<Vec<(ImageResource, WgpuVideoTextureEntry)>>,
    pub video_srgb_texture_cache: Option<Vec<(ImageResource, WgpuVideoTextureEntry)>>,
    pub default_bitmap_shader: Option<WgpuBitmapShader>,
    pub particle_instance_data: Option<Vec<f32>>,
    pub quad_batch_writer_material_renderer: Option<WgpuQuadMaterialRenderer>,
    pub quad_batch_writer_texture: Option<WgpuTextureEntry>,
    pub particle_resources: Option<WgpuParticleResources>,
    pub quad_batch_resources: Option<WgpuQuadBatchResources>,
    pub scissor_stack: Vec<WgpuScissorRect>,
    pub current_scissor_rect: Option<WgpuScissorRect>,
    pub render_target_viewport: Option<WgpuRenderPassViewport>,
    pub current_render_target: Option<WgpuRenderTarget>,
    pub pass_stack: Vec<WgpuRenderPass>,
}
impl Default for WgpuRenderStateRuntimeStorage {
    fn default() -> Self {
        Self {
            context: Default::default(),
            registries: Default::default(),
            teardowns: Default::default(),
            borrowed_surface_extent: Default::default(),
            mipmap_pipeline_cache: Default::default(),
            texture_cache: Default::default(),
            texture_source_premultiplied_texture_cache: Default::default(),
            texture_source_premultiplied_srgb_texture_cache: Default::default(),
            texture_source_straight_texture_cache: Default::default(),
            texture_source_straight_srgb_texture_cache: Default::default(),
            video_texture_cache: Default::default(),
            video_srgb_texture_cache: Default::default(),
            default_bitmap_shader: Default::default(),
            particle_instance_data: Default::default(),
            quad_batch_writer_material_renderer: Default::default(),
            quad_batch_writer_texture: Default::default(),
            particle_resources: Default::default(),
            quad_batch_resources: Default::default(),
            scissor_stack: Default::default(),
            current_scissor_rect: Default::default(),
            render_target_viewport: Default::default(),
            current_render_target: Default::default(),
            pass_stack: Default::default(),
        }
    }
}
pub type WgpuRenderStateRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/WgpuRenderState.ts:352 (sha256:e003cc095073ba6707274c00e75dcf6b990c0b298fb4057aa462e70bf224260d)
#[derive(Clone, Default)]
pub struct WgpuBitmapShaderRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: f64,
}
impl PartialEq for WgpuBitmapShaderRecord1 {
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
            Box<dyn FnMut(WgpuRenderState, WgpuBitmapShaderRecord1) -> () + Send + 'static>,
        >,
    >,
}
impl PartialEq for WgpuBitmapShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:364 (sha256:5fe417094a9800132bc849b19f0360a096f37d1fda60600f603a8eeba76f6676)
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:375 (sha256:da157d7dd2aef06c3ff53a1e2cafb130aaa6d7f8d3ae707eac7859094af30f73)
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:383 (sha256:34dfe22efbf1d2f4e16ac9a93fc703b8a54032d9ea689c75c5e61549dc76a3c9)
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:394 (sha256:f9514088a8f644f0471aa1aa5a043041544b3296d2aa7a9994fa8dfa8ae9e7b8)
#[derive(Clone, Default)]
pub struct WgpuShapeMeshBuffers {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub vertex_buffers: Vec<crate::OpaqueHostValue>,
    pub vertex_capacities: Vec<f64>,
    pub index_buffers: Vec<crate::OpaqueHostValue>,
    pub index_capacities: Vec<f64>,
    pub uniform_buffers: Vec<crate::OpaqueHostValue>,
    pub bind_groups: Vec<crate::OpaqueHostValue>,
    pub color_scale_bias_uniform_buffers: Vec<crate::OpaqueHostValue>,
    pub color_scale_bias_bind_groups: Vec<crate::OpaqueHostValue>,
}
impl PartialEq for WgpuShapeMeshBuffers {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:411 (sha256:0e94554b02fe046b289bb369e7bc2bf9804ca1d647be647f82be40f65cb53680)
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:419 (sha256:0362fdf0b62095db70100964f8f2d188eae552a2513337d7a145648619fd9486)
#[derive(Clone, Default)]
pub struct WgpuQuadBatchWriterBufferSlot {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub instance_buffer: Option<crate::OpaqueHostValue>,
    pub instance_capacity: f64,
    pub material_buffer: Option<crate::OpaqueHostValue>,
    pub material_capacity: f64,
}
impl PartialEq for WgpuQuadBatchWriterBufferSlot {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:429 (sha256:de75dddc8a4430a300122c6cd1a46ad6f035d28980c6a7f325573f6b42cf3735)
#[derive(Clone, Default)]
pub struct WgpuMeshInstanceBufferSlot {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub buffer: Option<crate::OpaqueHostValue>,
    pub capacity: f64,
}
impl PartialEq for WgpuMeshInstanceBufferSlot {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:440 (sha256:e106c7c0c814c31f6b624e8052b44416f90bcef516bfe9fa575a5d95e731ab82)
#[derive(Clone, Default)]
pub struct WgpuTextureResource {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mip_level_count: f64,
    pub straight_alpha: Option<bool>,
    pub texture: crate::OpaqueHostValue,
    pub view: crate::OpaqueHostValue,
}
impl PartialEq for WgpuTextureResource {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:459 (sha256:595b8ee17219343815c9ecc36e96d789f4a49aa3027eca7db075c56a3aa19e56)
pub type WgpuTextureBindings = Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>;

// Source: upstream/packages/types/src/WgpuRenderState.ts:463 (sha256:be5e97897eb02dcf05630647c7b870234dcc09f81a50e9317339b252e9aa2f25)
#[derive(Clone, Default)]
pub struct WgpuTextureEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub mip_level_count: f64,
    pub straight_alpha: Option<bool>,
    pub texture: crate::OpaqueHostValue,
    pub view: crate::OpaqueHostValue,
    pub bindings: WgpuTextureBindings,
    pub sampler: Option<crate::OpaqueHostValue>,
}
impl PartialEq for WgpuTextureEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuTextureEntry {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:472 (sha256:89b8cf222fe23605091e257b356350a8d4bf1de8cd89062a08a65cc99128f75a)
#[derive(Clone, Default)]
pub struct WgpuTextureSourceTextureEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub mip_level_count: f64,
    pub straight_alpha: Option<bool>,
    pub texture: crate::OpaqueHostValue,
    pub view: crate::OpaqueHostValue,
    pub bindings: WgpuTextureBindings,
    pub sampler: Option<crate::OpaqueHostValue>,
    pub version: f64,
}
impl PartialEq for WgpuTextureSourceTextureEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuTextureSourceTextureEntry {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:476 (sha256:da0f630196cf440da445e080b9729ba42c3fb30d7645cfdfac1fd789e78c86cd)
#[derive(Clone, Default)]
pub struct WgpuVideoTextureEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub mip_level_count: f64,
    pub straight_alpha: Option<bool>,
    pub texture: crate::OpaqueHostValue,
    pub view: crate::OpaqueHostValue,
    pub bindings: WgpuTextureBindings,
    pub sampler: crate::OpaqueHostValue,
    pub height: f64,
    pub uploaded_version: f64,
    pub width: f64,
}
impl PartialEq for WgpuVideoTextureEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WgpuVideoTextureEntry {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
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
