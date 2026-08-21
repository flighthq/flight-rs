// @generated from upstream/packages/types/src/CaptureColumnBaseline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::CaptureBaselineProvenance;

// Source: upstream/packages/types/src/CaptureColumnBaseline.ts:10 (sha256:32b5124851191753577eb94b0003cfcf4ab39cb245fe4b2ed02f27c0a3c88839)
#[derive(Clone, Default)]
pub struct CaptureColumnBaseline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fingerprint: Option<String>,
    pub source_hash: Option<String>,
    pub sha256: Option<String>,
    pub fingerprint_provenance: Option<CaptureBaselineProvenance>,
    pub sha256_provenance: Option<CaptureBaselineProvenance>,
}
impl PartialEq for CaptureColumnBaseline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
