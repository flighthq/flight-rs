// @generated from upstream/packages/types/src/CaptureColumnBaseline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/CaptureColumnBaseline.ts:8 (sha256:9fd6872be267c6e77c459b05ad106ec1bb7e74eb82fa73774f8adc21835b35fe)
#[derive(Clone, Default)]
pub struct CaptureColumnBaseline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fingerprint: Option<String>,
    pub source_hash: Option<String>,
    pub sha256: Option<String>,
}
impl PartialEq for CaptureColumnBaseline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
