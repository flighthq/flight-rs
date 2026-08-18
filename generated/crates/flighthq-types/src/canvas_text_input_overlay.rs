// @generated from upstream/packages/types/src/CanvasTextInputOverlay.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CanvasRenderState, RenderProxy2D};

// Source: upstream/packages/types/src/CanvasTextInputOverlay.ts:4 (sha256:e5e9bfb5f12fcd54b4d20ad9621d1b416ab5513c108fa1ac3837a52fb18e7863)
pub type CanvasTextInputOverlay = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(CanvasRenderState, RenderProxy2D) -> () + Send + 'static>>,
>;
