// @generated from upstream/packages/types/src/DomRenderOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    CanvasShapeCommand, DomTextureResolver, EffectPaddingResolver, Kind, NodeRenderer,
    Scene3DGraphSyncPolicy, ShapeRasterizer, StrokeTessellator,
};

// Source: upstream/packages/types/src/DomRenderOptions.ts:8 (sha256:1b0eaf263c201b2ff27586a52f9310005fd499f8562d759966dd5f785742ce95)
#[derive(Clone, Default)]
pub struct DomRenderOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub canvas_shape_commands: Option<Vec<(Kind, CanvasShapeCommand<crate::OpaqueHostValue>)>>,
    pub effect_padding_resolvers: Option<Vec<(Kind, EffectPaddingResolver)>>,
    pub node_renderers: Option<Vec<(Kind, NodeRenderer)>>,
    pub shape_rasterizer: Option<ShapeRasterizer>,
    pub stroke_tessellator: Option<StrokeTessellator>,
    pub texture_resolvers: Option<Vec<(Kind, DomTextureResolver)>>,
    pub image_smoothing_enabled: Option<bool>,
    pub pixel_ratio: Option<f64>,
    pub round_pixels: Option<bool>,
    pub scene_graph_sync_policy: Option<Scene3DGraphSyncPolicy>,
}
impl PartialEq for DomRenderOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
