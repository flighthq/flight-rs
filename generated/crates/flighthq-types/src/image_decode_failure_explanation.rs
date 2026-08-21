// @generated from upstream/packages/types/src/ImageDecodeFailureExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

#[derive(Clone, Default)]
pub struct ImageDecodeFailureExplanationRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub mime_type: String,
    pub reason: String,
}
impl PartialEq for ImageDecodeFailureExplanationRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ImageDecodeFailureExplanationRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub mime_type: crate::OpaqueHostValue,
    pub reason: String,
}
impl PartialEq for ImageDecodeFailureExplanationRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ImageDecodeFailureExplanation.ts:1 (sha256:1605a6764e422da6d22e3806d800440702badfc30dd3030e0f174c45c46ef22a)
pub type ImageDecodeFailureExplanation =
    crate::FlightUnion2<ImageDecodeFailureExplanationRecord2, ImageDecodeFailureExplanationRecord1>;
