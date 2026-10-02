// @generated from upstream/packages/types/src/RenderStateOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    CanvasShapeCommand, ColorAdjustmentUnsupportedGuard, EffectPaddingResolver, Kind, NodeRenderer,
    RenderProxy, RenderRootGuard, RenderState, StrokeTessellator,
};

// Source: upstream/packages/types/src/RenderStateOptions.ts:13 (sha256:a66316194781980e873e25062fd26c2d365c577357192ae2efb9414958e0fe5a)
#[derive(Clone, Default)]
pub struct RenderStateOptions {
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
}
impl PartialEq for RenderStateOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
