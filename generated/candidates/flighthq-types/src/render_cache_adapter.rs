// @generated from upstream/packages/types/src/RenderCacheAdapter.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{RenderCache, RenderCacheAdapterSignals, RenderProxy2D, RenderState, Renderable};

// Source: upstream/packages/types/src/RenderCacheAdapter.ts:5 (sha256:4a119dc3d47602709277411466ea5c42ad9aa206c78ced10a3009c2f84c0c600)
#[derive(Clone)]
pub struct RenderCacheAdapter {
    pub adapt: std::sync::Arc<
        dyn Fn(RenderState, Renderable, RenderProxy2D) -> Option<bool> + Send + Sync + 'static,
    >,
    pub cache: Option<RenderCache>,
    pub signals: Option<RenderCacheAdapterSignals>,
}
