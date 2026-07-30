// @generated from upstream/packages/types/src/WebcamCapabilityRange.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WebcamCapabilityRange.ts:1 (sha256:b090602e1f5f4f15751ef80de6e8102f57f2a4f8d13ca40071c522e7ee7cb23d)
#[derive(Clone, Default)]
pub struct WebcamCapabilityRange {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub max: f64,
    pub min: f64,
    pub step: f64,
}
impl PartialEq for WebcamCapabilityRange {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
