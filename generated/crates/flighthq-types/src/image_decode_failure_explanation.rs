// @generated from upstream/packages/types/src/ImageDecodeFailureExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ImageDecodeFailureExplanation.ts:1 (sha256:1605a6764e422da6d22e3806d800440702badfc30dd3030e0f174c45c46ef22a)
#[derive(Clone)]
pub struct ImageDecodeFailureExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mime_type: crate::FlightUnion2<crate::OpaqueHostValue, String>,
    pub reason: String,
}
impl PartialEq for ImageDecodeFailureExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
