// @generated from upstream/packages/types/src/CaptureColumnBaseline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/CaptureColumnBaseline.ts:7 (sha256:2ba66ba5544ade33f1481f2b55dbcbfd334b40d738ce9c1e056325d2f0def0eb)
#[derive(Clone)]
pub struct CaptureColumnBaseline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fingerprint: Option<String>,
    pub sha256: Option<String>,
}
impl PartialEq for CaptureColumnBaseline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
