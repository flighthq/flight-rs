// @generated from upstream/packages/types/src/ResourceLoadReport.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ResourceLoadReport.ts:1 (sha256:5d88a5895d063fb1af7f3cf44977166f657bc07086ddc3c5fc5042d01dd48f7f)
#[derive(Clone)]
pub struct ResourceLoadReport {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attempts: f64,
    pub bytes: f64,
    pub elapsed_ms: f64,
    pub group: Option<String>,
    pub key: String,
    pub status: ResourceLoadReportStatus,
}
impl PartialEq for ResourceLoadReport {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ResourceLoadReport.ts:10 (sha256:99ad0f91cc3852b1f2168caa126068050035f28caba92c4d2f8e3f1d58445c82)
pub type ResourceLoadReportStatus = String;
