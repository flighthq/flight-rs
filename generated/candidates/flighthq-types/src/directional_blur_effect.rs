// @generated from upstream/packages/types/src/DirectionalBlurEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/DirectionalBlurEffect.ts:3 (sha256:3826059319d02a6dce524539dfa04c76d510f7c5a7cdd738f758f7d5caac4b4d)
#[derive(Clone)]
pub struct DirectionalBlurEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub angle: Option<f64>,
    pub length: Option<f64>,
    pub samples: Option<f64>,
}
impl PartialEq for DirectionalBlurEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
