// @generated from upstream/packages/types/src/RenderProxyVisitor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{NodeAny, RenderProxy2D, RenderState};

// Source: upstream/packages/types/src/RenderProxyVisitor.ts:7 (sha256:93b470c60d04784c5d2141e7b1080203d043d2187c83f926ec1cf6e35d7a6968)
pub type RenderProxyVisitor = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(RenderState, NodeAny, RenderProxy2D, Option<RenderProxy2D>) -> ()
                + Send
                + 'static,
        >,
    >,
>;
