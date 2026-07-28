// @generated from upstream/packages/types/src/RenderProxyResolver.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{DisplayObject, RenderProxy2D, RenderState};

// Source: upstream/packages/types/src/RenderProxyResolver.ts:5 (sha256:2ad36164449672ef6ac056b03fbb5d5c7760916655c5055bcb09f7ae544c12f5)
#[derive(Clone)]
pub struct RenderProxyResolver {
    pub resolve: std::sync::Arc<
        dyn Fn(RenderState, DisplayObject, RenderProxy2D) -> Option<bool> + Send + Sync + 'static,
    >,
}
