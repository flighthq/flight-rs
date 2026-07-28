// @generated from upstream/packages/types/src/PixelateEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/PixelateEffect.ts:3 (sha256:e535204e92fcc062d7a617a060d25a2fd20bb8fe4db3da62078fc61d20995cdd)
#[derive(Clone)]
pub struct PixelateEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub size: Option<f64>,
}
impl PartialEq for PixelateEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
