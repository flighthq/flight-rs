// @generated from upstream/packages/types/src/GlRenderState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, CanvasShapeCommand, ColorAdjustmentUnsupportedGuard, ColorScaleBias,
    EffectPaddingResolver, EntityRuntime, GlBitmapShader, GlCompressedTextureDecoder,
    GlCompressedTextureUploader, GlContext, GlContextRuntime, GlContextState, GlCubeRenderTarget,
    GlCustomMaterialShaderSource, GlEffectRegistration, GlMeshMaterialRenderer, GlModifierSnippet,
    GlPbrExtensionRegistration, GlQuadMaterialRenderer, GlRenderTarget, GlScene3DPass,
    GlShaderLocations, GlShapeMesh, GlTextureResolver, GlVelocityWriter, HostCanvasCapability,
    HostImageCapability, Kind, NodeRenderer, RenderProxy, RenderProxy2D, RenderRegistrySignals,
    RenderRootGuard, RenderState, Scene2DClipHooks, Scene3DGraphSyncPolicy, ShapeRasterizer,
    StrokeTessellator, TintMaterialData,
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

// Source: upstream/packages/types/src/GlRenderState.ts:31 (sha256:be17d8a9f6f9185792c587c052d93d550fd9808be208fab4e64311145b0ae91d)
#[derive(Clone, Default)]
pub struct GlRenderState {
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
                Box<dyn FnMut(GlRenderState, Option<BlendMode>) -> () + Send + 'static>,
            >,
        >,
    >,
    pub context_state: GlContextState,
    pub gl: GlContext,
    pub registries: GlRenderRegistries,
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

// Source: upstream/packages/types/src/GlRenderState.ts:40 (sha256:a509f3a88a179edf52296a0ae7e8ff26cad241f5b48d05f69c402186a2ff5aec)
#[derive(Clone, Default)]
pub struct GlRenderRegistries {
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
    pub blend_realizations: Vec<(Kind, GlBlendRealization)>,
    pub color_adjustment_feature: Option<GlColorAdjustmentMaterialFeature>,
    pub color_adjustment_feature_guard: Option<GlColorAdjustmentMaterialFeatureGuard>,
    pub compressed_texture_decoder: Option<GlCompressedTextureDecoder>,
    pub compressed_texture_upload: Option<GlCompressedTextureUploader>,
    pub custom_effect_shaders: Vec<(Kind, String)>,
    pub custom_material_shaders: Vec<(Kind, GlCustomMaterialShaderSource)>,
    pub material_renderers: Vec<(
        Kind,
        crate::FlightUnion2<GlMeshMaterialRenderer, GlQuadMaterialRenderer>,
    )>,
    pub modifier_snippets: Vec<(Kind, GlModifierSnippet)>,
    pub pbr_extensions: Vec<(Kind, GlPbrExtensionRegistration)>,
    pub effects: Vec<(Kind, GlEffectRegistration)>,
    pub passes: Option<Vec<GlScene3DPass>>,
    pub shape_rasterizer: Option<ShapeRasterizer>,
    pub texture_resolvers: Vec<(Kind, GlTextureResolver)>,
    pub velocity_writers: Vec<(Kind, GlVelocityWriter)>,
}
impl PartialEq for GlRenderRegistries {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:66 (sha256:08d46091d710deac70dc82dd5ba3988c6e74e8161dfcd2028e9f0166f91e02d5)
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

// Source: upstream/packages/types/src/GlRenderState.ts:74 (sha256:8d220ee0534489bc29a78e722d9aceee9d7e2fb0f9ececf43df5dcc16d047e3a)
#[derive(Clone, Default)]
pub struct GlBlendSignature {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub dst: f64,
    pub equation: f64,
    pub src: f64,
}
impl PartialEq for GlBlendSignature {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:83 (sha256:47cb66d99fe4424f5173f83d93556fe967f733d2a3dd7792e8d54fe119727d65)
#[derive(Clone, Default)]
pub struct GlBoundShader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub locations: Option<GlShaderLocations>,
    pub program: crate::OpaqueHostValue,
}
impl PartialEq for GlBoundShader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlRenderState.ts:88 (sha256:b12ab248cba7a5676510fa787e945e56b616d16e05eccabd097a7995c7afce8f)
pub type GlBlendFactor = String;

// Source: upstream/packages/types/src/GlRenderState.ts:90 (sha256:8b84dd066ca9a399220d5b710e5f54629408eb1a49b565d2d53ee71c7f6c457b)
pub type GlBlendEquation = String;

// Source: upstream/packages/types/src/GlRenderState.ts:103 (sha256:bd98a75c3475e29dfdb842948e8f4a8b85c2e10bc3158734f770d6f037bb7dd8)
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

// Source: upstream/packages/types/src/GlRenderState.ts:119 (sha256:38536bde80c2ab230666995653a7acd91d194f205eb13af21b757f795414c96b)
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

// Source: upstream/packages/types/src/GlRenderState.ts:128 (sha256:96187ad87ad59de001df157154e36c5c58757972267963a65545c8fd0e22b69a)
#[derive(Clone)]
pub struct GlRenderStateRuntimeRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clear: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub signals: RenderRegistrySignals,
}
impl PartialEq for GlRenderStateRuntimeRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[doc(hidden)]
pub struct GlRenderStateRuntimeStorage {
    pub context: GlContextRuntime,
    pub registries: GlRenderRegistries,
    pub teardowns:
        Vec<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(GlRenderState) -> () + Send + 'static>>>>,
    pub default_bitmap_shader: Option<GlBitmapShader>,
    pub particle_instance_data: Option<Vec<f32>>,
    pub quad_batch_writer_material_renderer: Option<GlQuadMaterialRenderer>,
    pub quad_batch_writer_texture: Option<crate::OpaqueHostValue>,
    pub current_scissor_rect: Option<GlScissorRect>,
    pub current_render_target: Option<crate::FlightUnion2<GlCubeRenderTarget, GlRenderTarget>>,
    pub render_target_viewport: Option<GlViewportRect>,
    pub scissor_stack: Option<Vec<GlScissorRect>>,
}
impl Default for GlRenderStateRuntimeStorage {
    fn default() -> Self {
        Self {
            context: Default::default(),
            registries: Default::default(),
            teardowns: Default::default(),
            default_bitmap_shader: Default::default(),
            particle_instance_data: Default::default(),
            quad_batch_writer_material_renderer: Default::default(),
            quad_batch_writer_texture: Default::default(),
            current_scissor_rect: Default::default(),
            current_render_target: Default::default(),
            render_target_viewport: Default::default(),
            scissor_stack: Default::default(),
        }
    }
}
pub type GlRenderStateRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/GlRenderState.ts:189 (sha256:92ef9e960d48ccadf9d840f3dc2863ee3f64c2089ea081effa5c2ecaa9d1a079)
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

// Source: upstream/packages/types/src/GlRenderState.ts:203 (sha256:33409fa963fc52e0f9ff8f12bb277256e385fba6fc9ef7b776f4848ad194e2ff)
#[derive(Clone, Default)]
pub struct GlQuadBatchShader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub program: crate::OpaqueHostValue,
    pub loc_corner: f64,
    pub loc_mat_ab: f64,
    pub loc_mat_cd: f64,
    pub loc_mat_txty: f64,
    pub loc_uv_origin_axis_u: f64,
    pub loc_uv_axis_v: f64,
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

// Source: upstream/packages/types/src/GlRenderState.ts:219 (sha256:eb2748590ab9d2f4190685a0e0023dcfbc58ae2fcfa924a12b27f0b5867c273c)
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

// Source: upstream/packages/types/src/GlRenderState.ts:230 (sha256:c041cbbcaaa16bdba25bba01bd230322edf62bd9dc987ec0acae5c410449cbb1)
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

// Source: upstream/packages/types/src/GlRenderState.ts:244 (sha256:df1e98bd12a8d711c970bbc0453b9fbccdcb484b40c72fbb3f426e18442333ed)
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

// Source: upstream/packages/types/src/GlRenderState.ts:254 (sha256:c5eed51656152d130c5bd39967bda2fdec09e68c7666b1789992993ec2ac9b57)
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

// Source: upstream/packages/types/src/GlRenderState.ts:263 (sha256:b0b1de9b1a624baec9c5e6a1e62ec9c8ebf103c9b1a6779d90493772ef40a693)
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
