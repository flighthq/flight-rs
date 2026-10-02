// @generated from upstream/packages/types/src/GlRenderStateOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    CanvasShapeCommand, ColorAdjustmentUnsupportedGuard, EffectPaddingResolver, GlBlendRealization,
    GlColorAdjustmentMaterialFeature, GlColorAdjustmentMaterialFeatureGuard,
    GlCompressedTextureDecoder, GlCompressedTextureUploader, GlCustomMaterialShaderSource,
    GlEffectRegistration, GlMeshMaterialRenderer, GlModifierSnippet, GlPbrExtensionRegistration,
    GlQuadMaterialRenderer, GlScene3DPass, GlTextureResolver, GlVelocityWriter,
    HostCanvasCapability, HostImageCapability, Kind, NodeRenderer, RenderProxy, RenderRootGuard,
    RenderState, Scene3DGraphSyncPolicy, ShapeRasterizer, StrokeTessellator,
};

// Source: upstream/packages/types/src/GlRenderStateOptions.ts:22 (sha256:e86898a2f95e04c5a24efdf97c4d373985c6741accc1e3bdb0c624c7d4c7411d)
#[derive(Clone, Default)]
pub struct GlRenderStateOptions {
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
    pub node_renderers: Option<Vec<(Kind, NodeRenderer)>>,
    pub render_root_guard: Option<RenderRootGuard>,
    pub stroke_tessellator: Option<StrokeTessellator>,
    pub allow_smoothing: Option<bool>,
    pub canvas_host: Option<HostCanvasCapability>,
    pub image_host: Option<HostImageCapability>,
    pub image_smoothing_enabled: Option<bool>,
    pub pixel_ratio: Option<f64>,
    pub round_pixels: Option<bool>,
    pub scene_graph_sync_policy: Option<Scene3DGraphSyncPolicy>,
    pub blend_realizations: Option<Vec<(Kind, GlBlendRealization)>>,
    pub color_adjustment_feature: Option<GlColorAdjustmentMaterialFeature>,
    pub color_adjustment_feature_guard: Option<GlColorAdjustmentMaterialFeatureGuard>,
    pub compressed_texture_decoder: Option<GlCompressedTextureDecoder>,
    pub compressed_texture_upload: Option<GlCompressedTextureUploader>,
    pub custom_effect_shaders: Option<Vec<(Kind, String)>>,
    pub custom_material_shaders: Option<Vec<(Kind, GlCustomMaterialShaderSource)>>,
    pub effects: Option<Vec<(Kind, GlEffectRegistration)>>,
    pub material_renderers: Option<
        Vec<(
            Kind,
            crate::FlightUnion2<GlMeshMaterialRenderer, GlQuadMaterialRenderer>,
        )>,
    >,
    pub modifier_snippets: Option<Vec<(Kind, GlModifierSnippet)>>,
    pub passes: Option<Vec<GlScene3DPass>>,
    pub pbr_extensions: Option<Vec<(Kind, GlPbrExtensionRegistration)>>,
    pub shape_rasterizer: Option<ShapeRasterizer>,
    pub texture_resolvers: Option<Vec<(Kind, GlTextureResolver)>>,
    pub velocity_writers: Option<Vec<(Kind, GlVelocityWriter)>>,
}
impl PartialEq for GlRenderStateOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
