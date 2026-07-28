// @generated from upstream/packages/types/src/CanvasRenderTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/CanvasRenderTarget.ts:1 (sha256:1e5ab0465ab575b13ff2e877fd79882ab53d5418e70294f49d8e22b6a20fbc93)
#[derive(Clone)]
pub struct CanvasRenderTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub canvas: crate::OpaqueHostValue,
    pub context: crate::OpaqueHostValue,
    pub width: f64,
    pub height: f64,
}
impl PartialEq for CanvasRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
