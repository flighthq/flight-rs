// @generated from upstream/packages/types/src/CanvasShapeRegistry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CanvasShapeDrawState, ShapeBoundsCommandHandler};

// Source: upstream/packages/types/src/CanvasShapeRegistry.ts:6 (sha256:70412d87857775922177c1a5fde48cd3fb8f8baf66ac245c4b4198e388ecd145)
pub type CanvasShapeHandler = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(
                    crate::OpaqueHostValue,
                    CanvasShapeDrawState,
                    Vec<crate::OpaqueHostValue>,
                    f64,
                ) -> ()
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/CanvasShapeRegistry.ts:13 (sha256:067b8f4a159ea402e90fe4a19f3e9ea03de35867d6f8db9cfad36f7880a73104)
#[derive(Clone)]
pub struct CanvasShapeCommand<K> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub key: K,
    pub fill_bounds: Option<ShapeBoundsCommandHandler>,
    pub stroke_bounds: Option<ShapeBoundsCommandHandler>,
    pub draw: CanvasShapeHandler,
}
impl<K> PartialEq for CanvasShapeCommand<K> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
