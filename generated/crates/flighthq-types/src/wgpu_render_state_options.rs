// @generated from upstream/packages/types/src/WgpuRenderStateOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    CanvasShapeCommand, ColorAdjustmentUnsupportedGuard, EffectPaddingResolver,
    HostCanvasCapability, HostImageCapability, Kind, NodeRenderer, RenderProxy, RenderRootGuard,
    RenderState, Scene3DGraphSyncPolicy, ShapeRasterizer, StrokeTessellator,
    WgpuColorAdjustmentMaterialFeature, WgpuColorAdjustmentMaterialFeatureGuard,
    WgpuCompressedTextureDecoder, WgpuCompressedTextureUploader, WgpuCustomMaterialShaderSource,
    WgpuEffectRegistration, WgpuMeshMaterialRenderer, WgpuModifierSnippet,
    WgpuQuadMaterialRenderer, WgpuScene3DPass, WgpuSkinningAdapter, WgpuTextureResolver,
    WgpuVelocityWriter,
};

// Source: upstream/packages/types/src/WgpuRenderStateOptions.ts:18 (sha256:9c03fc2c5cded897367ad9704a79a62f79a0f877b7539fd4a9d8adbe840d199b)
#[derive(Clone, Default)]
pub struct WgpuRenderStateOptions {
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
    pub canvas_host: Option<HostCanvasCapability>,
    pub image_host: Option<HostImageCapability>,
    pub format: Option<crate::OpaqueHostValue>,
    pub image_smoothing_enabled: Option<bool>,
    pub pixel_ratio: Option<f64>,
    pub round_pixels: Option<bool>,
    pub scene_graph_sync_policy: Option<Scene3DGraphSyncPolicy>,
    pub color_adjustment_feature: Option<WgpuColorAdjustmentMaterialFeature>,
    pub color_adjustment_feature_guard: Option<WgpuColorAdjustmentMaterialFeatureGuard>,
    pub compressed_texture_decoder: Option<WgpuCompressedTextureDecoder>,
    pub compressed_texture_upload: Option<WgpuCompressedTextureUploader>,
    pub custom_material_shaders: Option<Vec<(Kind, WgpuCustomMaterialShaderSource)>>,
    pub effects: Option<Vec<(Kind, WgpuEffectRegistration)>>,
    pub gpu_skinning: Option<WgpuSkinningAdapter>,
    pub material_renderers: Option<
        Vec<(
            Kind,
            crate::FlightUnion2<WgpuMeshMaterialRenderer, WgpuQuadMaterialRenderer>,
        )>,
    >,
    pub modifier_snippets: Option<Vec<(Kind, WgpuModifierSnippet)>>,
    pub passes: Option<Vec<WgpuScene3DPass>>,
    pub shape_rasterizer: Option<ShapeRasterizer>,
    pub texture_resolvers: Option<Vec<(Kind, WgpuTextureResolver)>>,
    pub velocity_writers: Option<Vec<(Kind, WgpuVelocityWriter)>>,
}
impl PartialEq for WgpuRenderStateOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
