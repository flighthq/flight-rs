// @generated from upstream/packages/types/src/CanvasRenderTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/CanvasRenderTarget.ts:3 (sha256:77ecafe9197f64a9e574cc139335cb7aff72a45c8b5efecc742d943d53a49e3a)
#[derive(Clone, Default)]
pub struct CanvasRenderTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub width: f64,
    pub canvas: crate::OpaqueHostValue,
    pub context: crate::OpaqueHostValue,
}
impl PartialEq for CanvasRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
