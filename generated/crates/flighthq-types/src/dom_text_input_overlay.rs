// @generated from upstream/packages/types/src/DomTextInputOverlay.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{DomRenderState, RenderProxy2D};

// Source: upstream/packages/types/src/DomTextInputOverlay.ts:4 (sha256:05caf2a6f5887a7ff8b80bbea05c4e07b7f9e070d45242b288098ab87374347f)
pub type DomTextInputOverlay = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(DomRenderState, RenderProxy2D) -> () + Send + 'static>>,
>;
