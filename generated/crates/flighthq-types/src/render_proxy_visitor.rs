// @generated from upstream/packages/types/src/RenderProxyVisitor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{RenderProxy2D, RenderState, Renderable};

// Source: upstream/packages/types/src/RenderProxyVisitor.ts:7 (sha256:2cae7bbf9ab7e85e6310dd599b38f294d288a397f60aef8addf3bfae1db346b0)
pub type RenderProxyVisitor = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(RenderState, Renderable, RenderProxy2D, Option<RenderProxy2D>) -> ()
                + Send
                + 'static,
        >,
    >,
>;
