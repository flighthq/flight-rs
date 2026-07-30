// @generated from upstream/packages/types/src/RenderViewport2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/RenderViewport2D.ts:1 (sha256:b46ca770066f9cfd6944ad07364c70f769bac42b2d14045f5d9152dc011f6276)
#[derive(Clone, Default)]
pub struct RenderViewport2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for RenderViewport2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
