// @generated from upstream/packages/types/src/CanvasRenderStateOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, CanvasEffectRunner, CanvasQuadMaterialRenderer, CanvasRenderState,
    CanvasShapeCommand, CanvasTextureResolvers, ColorAdjustmentUnsupportedGuard,
    EffectPaddingResolver, HostCanvasCapability, Kind, NodeRenderer, RenderProxy, RenderRootGuard,
    RenderState, Scene3DGraphSyncPolicy, StrokeTessellator,
};

// Source: upstream/packages/types/src/CanvasRenderStateOptions.ts:11 (sha256:b5f20e2422fe621d89d6d26d50f59af02fe2ce1403a2945b22659ff4620350c7)
#[derive(Clone, Default)]
pub struct CanvasRenderStateOptions {
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
    pub image_smoothing_enabled: Option<bool>,
    pub image_smoothing_quality: Option<crate::OpaqueHostValue>,
    pub pixel_ratio: Option<f64>,
    pub round_pixels: Option<bool>,
    pub scene_graph_sync_policy: Option<Scene3DGraphSyncPolicy>,
    pub blend_mode_application: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(CanvasRenderState, Option<BlendMode>) -> () + Send + 'static>,
            >,
        >,
    >,
    pub canvas_host: Option<HostCanvasCapability>,
    pub canvas_texture_resolvers: Option<CanvasTextureResolvers>,
    pub effects: Option<Vec<(Kind, CanvasEffectRunner)>>,
    pub material_renderers: Option<Vec<(Kind, CanvasQuadMaterialRenderer)>>,
}
impl PartialEq for CanvasRenderStateOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
