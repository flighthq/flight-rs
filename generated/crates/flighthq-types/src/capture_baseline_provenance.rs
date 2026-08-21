// @generated from upstream/packages/types/src/CaptureBaselineProvenance.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/CaptureBaselineProvenance.ts:18 (sha256:34d02d0dff9006fe75f8671a923d7c2ed5e982460c0b170a1ea3356008117eb5)
#[derive(Clone, Default)]
pub struct CaptureBaselineProvenance {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub computation_id: Option<String>,
    pub frames: f64,
    pub source_hash: Option<String>,
    pub target_kind: Option<String>,
    pub verify_published: bool,
    pub warmup_frames: f64,
}
impl PartialEq for CaptureBaselineProvenance {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
