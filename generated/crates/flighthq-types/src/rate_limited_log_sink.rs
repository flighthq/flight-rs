// @generated from upstream/packages/types/src/RateLimitedLogSink.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::LogSink;

// Source: upstream/packages/types/src/RateLimitedLogSink.ts:4 (sha256:28f6576dee2c4486f22c4526d9be537d37c3ca1ea84db7d606a3b7f3e0897f1c)
#[derive(Clone)]
pub struct RateLimitedLogSink {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub sink: LogSink,
}
impl PartialEq for RateLimitedLogSink {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
