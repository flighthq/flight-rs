// @generated from upstream/packages/types/src/CanvasShapeRegistry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/CanvasShapeRegistry.ts:5 (sha256:70412d87857775922177c1a5fde48cd3fb8f8baf66ac245c4b4198e388ecd145)
pub type CanvasShapeHandler = std::sync::Arc<
    dyn Fn(crate::OpaqueHostValue, crate::OpaqueHostValue, Vec<crate::OpaqueHostValue>, f64) -> ()
        + Send
        + Sync
        + 'static,
>;

// Source: upstream/packages/types/src/CanvasShapeRegistry.ts:12 (sha256:85643107e6e0a515f3f4527f9352d69ab02c0c94bca638c2701c988039d4ed23)
#[derive(Clone)]
pub struct CanvasShapeCommand {
    pub key: crate::OpaqueHostValue,
    pub draw: crate::OpaqueHostValue,
}
