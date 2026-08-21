// @generated from upstream/packages/types/src/GlRenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, CanvasShapeCommand, ColorAdjustmentUnsupportedGuard, ColorScaleBias, EntityRuntime,
    GlBitmapShader, GlCompressedTextureDecoder, GlCompressedTextureUploader,
    GlCustomMaterialShaderSource, GlMaterialRenderer, GlMeshMaterialRenderer, GlModifierSnippet,
    GlPbrExtensionRegistration, GlRenderEffectRegistration, GlRenderTarget, GlShapeMesh,
    GlTextureResolver, GlVelocityWriter, Image, KeyedTable, Matrix, Path, PathMesh,
    RenderEffectPaddingResolver, RenderProxy, RenderProxy2D, RenderRootGuard, RenderState,
    Renderer, Scene2DClipHooks, Scene3DGraphSyncPolicy, ShapeRasterizer, SlotTable, StrokeStyle,
    TextureSource, TintMaterialData,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:30 (sha256:464eeba3f4bedfaa41e89c4eeecce566a24f5114fdb842604f4ceec1b98c796a)
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

// Source: upstream/packages/types/src/GlRenderState.ts:38 (sha256:43740eafc1e1c310207dcce7ac38be6340b29e9a1a67fcec617ff72fef5634ea)
#[derive(Clone, Default)]
pub struct GlRenderRegistries {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub canvas_shape_commands: Option<KeyedTable<CanvasShapeCommand<crate::OpaqueHostValue>>>,
    pub color_adjustments: Option<
        SlotTable<
            std::sync::Arc<
                std::sync::Mutex<
                    Box<
                        dyn FnMut(RenderState, RenderProxy, Option<RenderProxy>) -> ()
                            + Send
                            + 'static,
                    >,
                >,
            >,
        >,
    >,
    pub color_adjustment_unsupported_guard: Option<SlotTable<ColorAdjustmentUnsupportedGuard>>,
    pub effect_padding_resolvers: Option<KeyedTable<RenderEffectPaddingResolver>>,
    pub renderers: KeyedTable<Renderer>,
    pub render_root_guard: Option<SlotTable<RenderRootGuard>>,
    pub stroke_tessellator: SlotTable<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(Path, StrokeStyle, Option<f64>) -> Option<PathMesh> + Send + 'static>,
            >,
        >,
    >,
    pub blend_realizations: KeyedTable<GlBlendRealization>,
    pub color_adjustment_feature: Option<SlotTable<GlColorAdjustmentMaterialFeature>>,
    pub color_adjustment_feature_guard: Option<SlotTable<GlColorAdjustmentMaterialFeatureGuard>>,
    pub compressed_texture_decoder: SlotTable<GlCompressedTextureDecoder>,
    pub compressed_texture_upload: SlotTable<GlCompressedTextureUploader>,
    pub custom_effect_shaders: KeyedTable<String>,
    pub custom_material_shaders: KeyedTable<GlCustomMaterialShaderSource>,
    pub material_renderers: KeyedTable<GlMaterialRenderer>,
    pub mesh_material_renderers: KeyedTable<GlMeshMaterialRenderer>,
    pub modifier_snippets: KeyedTable<GlModifierSnippet>,
    pub modifier_snippet_revision: f64,
    pub pbr_extensions: KeyedTable<GlPbrExtensionRegistration>,
    pub pbr_extension_revision: f64,
    pub render_effects: KeyedTable<GlRenderEffectRegistration>,
    pub shape_rasterizer: SlotTable<ShapeRasterizer>,
    pub texture_resolvers: KeyedTable<GlTextureResolver>,
    pub velocity_writers: KeyedTable<GlVelocityWriter>,
}
impl PartialEq for GlRenderRegistries {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:70 (sha256:08d46091d710deac70dc82dd5ba3988c6e74e8161dfcd2028e9f0166f91e02d5)
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

// Source: upstream/packages/types/src/GlRenderState.ts:76 (sha256:b12ab248cba7a5676510fa787e945e56b616d16e05eccabd097a7995c7afce8f)
pub type GlBlendFactor = String;

// Source: upstream/packages/types/src/GlRenderState.ts:78 (sha256:8b84dd066ca9a399220d5b710e5f54629408eb1a49b565d2d53ee71c7f6c457b)
pub type GlBlendEquation = String;

// Source: upstream/packages/types/src/GlRenderState.ts:91 (sha256:bd98a75c3475e29dfdb842948e8f4a8b85c2e10bc3158734f770d6f037bb7dd8)
#[derive(Clone)]
pub struct GlColorAdjustmentMaterialFeature {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fragment_shader_chunk: String,
    pub matrix_fragment_shader_chunk: String,
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
                dyn FnMut(
                        GlRenderStateRuntime,
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
}
impl PartialEq for GlColorAdjustmentMaterialFeature {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:107 (sha256:38536bde80c2ab230666995653a7acd91d194f205eb13af21b757f795414c96b)
pub type GlColorAdjustmentMaterialFeatureGuard = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(
                    GlRenderState,
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

// Source: upstream/packages/types/src/GlRenderState.ts:116 (sha256:42e9530ec685e1f00cc45e3695ffe475266047164f10002bcb41a8ad935d17e6)
#[derive(Clone, Default)]
pub struct GlRenderStateRuntimeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub texture: crate::OpaqueHostValue,
    pub uploaded_version: f64,
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

#[doc(hidden)]
pub struct GlRenderStateRuntimeStorage {
    pub registries: GlRenderRegistries,
    pub default_bitmap_shader: Option<GlBitmapShader>,
    pub particle_instance_buffer: Option<crate::OpaqueHostValue>,
    pub particle_instance_data: Option<Vec<f32>>,
    pub quad_batch_writer_material_renderer: Option<GlMaterialRenderer>,
    pub quad_batch_writer_texture: Option<crate::OpaqueHostValue>,
    pub current_scissor_rect: Option<GlScissorRect>,
    pub current_render_target: Option<GlRenderTarget>,
    pub render_target_viewport: Option<GlViewportRect>,
    pub texture_cache: Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>,
    pub texture_source_premultiplied_texture_cache:
        Vec<(TextureSource, GlRenderStateRuntimeRecord2)>,
    pub texture_source_premultiplied_srgb_texture_cache:
        Vec<(TextureSource, GlRenderStateRuntimeRecord2)>,
    pub texture_source_straight_texture_cache: Vec<(TextureSource, GlRenderStateRuntimeRecord2)>,
    pub texture_source_straight_srgb_texture_cache:
        Vec<(TextureSource, GlRenderStateRuntimeRecord2)>,
    pub video_texture_cache: Option<Vec<(Image, GlRenderStateRuntimeRecord1)>>,
    pub video_srgb_texture_cache: Option<Vec<(Image, GlRenderStateRuntimeRecord1)>>,
    pub scissor_stack: Option<Vec<GlScissorRect>>,
}
impl Default for GlRenderStateRuntimeStorage {
    fn default() -> Self {
        Self {
            registries: Default::default(),
            default_bitmap_shader: Default::default(),
            particle_instance_buffer: Default::default(),
            particle_instance_data: Default::default(),
            quad_batch_writer_material_renderer: Default::default(),
            quad_batch_writer_texture: Default::default(),
            current_scissor_rect: Default::default(),
            current_render_target: Default::default(),
            render_target_viewport: Default::default(),
            texture_cache: Default::default(),
            texture_source_premultiplied_texture_cache: Default::default(),
            texture_source_premultiplied_srgb_texture_cache: Default::default(),
            texture_source_straight_texture_cache: Default::default(),
            texture_source_straight_srgb_texture_cache: Default::default(),
            video_texture_cache: Default::default(),
            video_srgb_texture_cache: Default::default(),
            scissor_stack: Default::default(),
        }
    }
}
pub type GlRenderStateRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/GlRenderState.ts:256 (sha256:92ef9e960d48ccadf9d840f3dc2863ee3f64c2089ea081effa5c2ecaa9d1a079)
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
    pub loc_straight_texture_alpha: crate::OpaqueHostValue,
}
impl PartialEq for GlParticleShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:270 (sha256:dd54f9f662967291bb053c203ae7ab4ba75c1cbc97000e5a7a27ec51d44b5014)
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
    pub loc_straight_texture_alpha: crate::OpaqueHostValue,
}
impl PartialEq for GlQuadBatchShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:286 (sha256:eb2748590ab9d2f4190685a0e0023dcfbc58ae2fcfa924a12b27f0b5867c273c)
#[derive(Clone, Default)]
pub struct GlColorScaleBiasInstancedShader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub program: crate::OpaqueHostValue,
    pub loc_corner: f64,
    pub loc_world_matrix: crate::OpaqueHostValue,
    pub loc_texture: crate::OpaqueHostValue,
    pub loc_straight_texture_alpha: crate::OpaqueHostValue,
}
impl PartialEq for GlColorScaleBiasInstancedShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:297 (sha256:c041cbbcaaa16bdba25bba01bd230322edf62bd9dc987ec0acae5c410449cbb1)
#[derive(Clone, Default)]
pub struct GlUniformColorScaleBiasShader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub program: crate::OpaqueHostValue,
    pub loc_corner: f64,
    pub loc_world_matrix: crate::OpaqueHostValue,
    pub loc_texture: crate::OpaqueHostValue,
    pub loc_straight_texture_alpha: crate::OpaqueHostValue,
    pub loc_color_scale: crate::OpaqueHostValue,
    pub loc_color_bias: crate::OpaqueHostValue,
}
impl PartialEq for GlUniformColorScaleBiasShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:311 (sha256:df1e98bd12a8d711c970bbc0453b9fbccdcb484b40c72fbb3f426e18442333ed)
#[derive(Clone, Default)]
pub struct GlShapeMeshColorScaleBiasShader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub program: crate::OpaqueHostValue,
    pub position_location: f64,
    pub matrix_location: Option<crate::OpaqueHostValue>,
    pub color_location: Option<crate::OpaqueHostValue>,
    pub color_scale_location: Option<crate::OpaqueHostValue>,
    pub color_bias_location: Option<crate::OpaqueHostValue>,
    pub color_matrix_locations: Option<Vec<Option<crate::OpaqueHostValue>>>,
}
impl PartialEq for GlShapeMeshColorScaleBiasShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:321 (sha256:c5eed51656152d130c5bd39967bda2fdec09e68c7666b1789992993ec2ac9b57)
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

// Source: upstream/packages/types/src/GlRenderState.ts:330 (sha256:b0b1de9b1a624baec9c5e6a1e62ec9c8ebf103c9b1a6779d90493772ef40a693)
#[derive(Clone, Default)]
pub struct GlViewportRect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for GlViewportRect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
