// @generated from upstream/packages/types/src/RenderCacheRefreshOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/RenderCacheRefreshOptions.ts:1 (sha256:defad2198c51a0a8b548f362dec9f66bc0f1f1c0362f81bbd9272a3d48ae62de)
#[derive(Clone, Default)]
pub struct RenderCacheRefreshOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub padding: Option<f64>,
    pub min_width: Option<f64>,
    pub min_height: Option<f64>,
}
impl PartialEq for RenderCacheRefreshOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
