// @generated from upstream/packages/types/src/ImageEncodeFailureExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ImageEncodeFailureExplanation.ts:1 (sha256:a353b5f54be981e256639c30d0b37e3867ccc561548cda84a378024a891747e2)
#[derive(Clone, Default)]
pub struct ImageEncodeFailureExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mime_type: String,
    pub reason: String,
}
impl PartialEq for ImageEncodeFailureExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
