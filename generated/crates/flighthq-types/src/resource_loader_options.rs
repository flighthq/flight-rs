// @generated from upstream/packages/types/src/ResourceLoaderOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ResourceLoaderOptions.ts:1 (sha256:0dd43a3c03529479515b41775d119298f6b8fef74dc6c89fdafd16cc4c5417cc)
#[derive(Clone, Default)]
pub struct ResourceLoaderOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub dedupe: Option<bool>,
    pub error_policy: Option<String>,
    pub max_bytes_per_second: Option<f64>,
    pub max_concurrent: Option<f64>,
    pub retries: Option<f64>,
    pub retry_backoff: Option<String>,
    pub retry_base_delay_ms: Option<f64>,
    pub retry_max_delay_ms: Option<f64>,
    pub streaming: Option<bool>,
    pub timeout_ms: Option<f64>,
}
impl PartialEq for ResourceLoaderOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
