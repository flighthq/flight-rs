// @generated from upstream/packages/types/src/ShapeRasterizer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{RenderState, ShapeCommandToken};

// Source: upstream/packages/types/src/ShapeRasterizer.ts:15 (sha256:7f2fbaab7c2000eb32472185f7bbfd44e9a9748cdb2ae3723f30d73c9b4296fe)
pub type ShapeRasterizer = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(crate::OpaqueHostValue, Vec<ShapeCommandToken>, RenderState) -> ()
                + Send
                + 'static,
        >,
    >,
>;
