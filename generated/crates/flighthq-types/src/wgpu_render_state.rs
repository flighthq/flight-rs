// @generated from upstream/packages/types/src/WgpuRenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, ColorScaleBias, EntityRuntime, Image, Kind, Matrix, RenderProxy2D,
    RenderRegistrySignals, Scene2DClipHooks, Scene3DGraphSyncPolicy, TextureSource,
    TintMaterialData, WgpuCompressedTextureUploader, WgpuMaterialRenderer,
    WgpuMeshMaterialRenderer, WgpuRenderTarget, WgpuShapeMesh,
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:26 (sha256:b04573db8382026a9e16cb4953f1dda844ec6cd863bf52f4cb3b0689edf3bda4)
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
    pub display_object_clip_hooks: Option<Scene2DClipHooks>,
    pub pixel_ratio: f64,
    pub render_alpha: f64,
    pub render_blend_mode: Option<BlendMode>,
    pub render_transform2_d: Option<Matrix>,
    pub scene_graph_sync_policy: Scene3DGraphSyncPolicy,
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:41 (sha256:7db70fc400926f7c23e83eca154e3294581b61b582548d2dd1df3969f5bf7edb)
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:66 (sha256:ba949754af13c5bc5e13f170befd56868a323fecc070c8482dbde40c37eb6942)
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:76 (sha256:a2bc23bace382a83f246f14c86f03121db15a25a1f479edc706a6b00dfe0475d)
#[derive(Clone, Default)]
pub struct WgpuRenderStateRuntimeRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub bind_group_layout: crate::OpaqueHostValue,
    pub pipeline: crate::OpaqueHostValue,
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
    pub mipmap_pipeline_cache: Vec<(crate::OpaqueHostValue, WgpuRenderStateRuntimeRecord2)>,
    pub texture_cache: Vec<(crate::OpaqueHostValue, WgpuTextureEntry)>,
    pub texture_source_premultiplied_texture_cache:
        Vec<(TextureSource, WgpuTextureSourceTextureEntry)>,
    pub texture_source_premultiplied_srgb_texture_cache:
        Vec<(TextureSource, WgpuTextureSourceTextureEntry)>,
    pub texture_source_straight_texture_cache: Vec<(TextureSource, WgpuTextureSourceTextureEntry)>,
    pub texture_source_straight_srgb_texture_cache:
        Vec<(TextureSource, WgpuTextureSourceTextureEntry)>,
    pub compressed_texture_upload: Option<WgpuCompressedTextureUploader>,
    pub video_texture_cache: Option<Vec<(Image, WgpuVideoTextureEntry)>>,
    pub video_srgb_texture_cache: Option<Vec<(Image, WgpuVideoTextureEntry)>>,
    pub default_bitmap_shader: Option<WgpuBitmapShader>,
    pub particle_instance_buffer: Option<crate::OpaqueHostValue>,
    pub particle_instance_data: Option<Vec<f32>>,
    pub quad_batch_writer_material_renderer: Option<WgpuMaterialRenderer>,
    pub quad_batch_writer_texture: Option<WgpuTextureEntry>,
    pub material_renderer_map: Option<Vec<(Kind, WgpuMaterialRenderer)>>,
    pub scene_mesh_material_registry: Option<Vec<(Kind, WgpuMeshMaterialRenderer)>>,
    pub scissor_stack: Vec<WgpuScissorRect>,
    pub current_scissor_rect: Option<WgpuScissorRect>,
    pub render_target_viewport: Option<SharedStructuralRecord1>,
    pub current_render_target: Option<WgpuRenderTarget>,
}
impl Default for WgpuRenderStateRuntimeStorage {
    fn default() -> Self {
        Self {
            mipmap_pipeline_cache: Default::default(),
            texture_cache: Default::default(),
            texture_source_premultiplied_texture_cache: Default::default(),
            texture_source_premultiplied_srgb_texture_cache: Default::default(),
            texture_source_straight_texture_cache: Default::default(),
            texture_source_straight_srgb_texture_cache: Default::default(),
            compressed_texture_upload: Default::default(),
            video_texture_cache: Default::default(),
            video_srgb_texture_cache: Default::default(),
            default_bitmap_shader: Default::default(),
            particle_instance_buffer: Default::default(),
            particle_instance_data: Default::default(),
            quad_batch_writer_material_renderer: Default::default(),
            quad_batch_writer_texture: Default::default(),
            material_renderer_map: Default::default(),
            scene_mesh_material_registry: Default::default(),
            scissor_stack: Default::default(),
            current_scissor_rect: Default::default(),
            render_target_viewport: Default::default(),
            current_render_target: Default::default(),
        }
    }
}
pub type WgpuRenderStateRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/WgpuRenderState.ts:299 (sha256:e003cc095073ba6707274c00e75dcf6b990c0b298fb4057aa462e70bf224260d)
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:311 (sha256:5fe417094a9800132bc849b19f0360a096f37d1fda60600f603a8eeba76f6676)
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:322 (sha256:da157d7dd2aef06c3ff53a1e2cafb130aaa6d7f8d3ae707eac7859094af30f73)
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:330 (sha256:22df7e6d3385e1e076ae9715044784097c49ec0109050e4f7f8424f0fb7c93a1)
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
    pub render_target: Option<WgpuRenderTarget>,
}
impl PartialEq for WgpuSavedPassState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:342 (sha256:34dfe22efbf1d2f4e16ac9a93fc703b8a54032d9ea689c75c5e61549dc76a3c9)
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:353 (sha256:f9514088a8f644f0471aa1aa5a043041544b3296d2aa7a9994fa8dfa8ae9e7b8)
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:370 (sha256:0e94554b02fe046b289bb369e7bc2bf9804ca1d647be647f82be40f65cb53680)
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:378 (sha256:0362fdf0b62095db70100964f8f2d188eae552a2513337d7a145648619fd9486)
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

// Source: upstream/packages/types/src/WgpuRenderState.ts:387 (sha256:0b40042b9e4b6832f9579ad30017ae29f1fd11d0339c3000b65e8ff33a50bb29)
#[derive(Clone, Default)]
pub struct WgpuTextureEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bind_group: crate::OpaqueHostValue,
    pub bind_group_linear: Option<crate::OpaqueHostValue>,
    pub bind_group_nearest: Option<crate::OpaqueHostValue>,
    pub straight_alpha: Option<bool>,
    pub texture: crate::OpaqueHostValue,
    pub view: crate::OpaqueHostValue,
}
impl PartialEq for WgpuTextureEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:403 (sha256:89b8cf222fe23605091e257b356350a8d4bf1de8cd89062a08a65cc99128f75a)
#[derive(Clone, Default)]
pub struct WgpuTextureSourceTextureEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bind_group: crate::OpaqueHostValue,
    pub bind_group_linear: Option<crate::OpaqueHostValue>,
    pub bind_group_nearest: Option<crate::OpaqueHostValue>,
    pub straight_alpha: Option<bool>,
    pub texture: crate::OpaqueHostValue,
    pub view: crate::OpaqueHostValue,
    pub version: f64,
}
impl PartialEq for WgpuTextureSourceTextureEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuRenderState.ts:407 (sha256:da0f630196cf440da445e080b9729ba42c3fb30d7645cfdfac1fd789e78c86cd)
#[derive(Clone, Default)]
pub struct WgpuVideoTextureEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bind_group: crate::OpaqueHostValue,
    pub bind_group_linear: Option<crate::OpaqueHostValue>,
    pub bind_group_nearest: Option<crate::OpaqueHostValue>,
    pub straight_alpha: Option<bool>,
    pub texture: crate::OpaqueHostValue,
    pub view: crate::OpaqueHostValue,
    pub height: f64,
    pub sampler: crate::OpaqueHostValue,
    pub uploaded_version: f64,
    pub width: f64,
}
impl PartialEq for WgpuVideoTextureEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
