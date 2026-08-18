// @generated from upstream/packages/types/src/Skeleton2DDeformLengthExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Skeleton2DDeformLengthExplanation.ts:11 (sha256:c1d2d6e5501fbc9788e262ce8b46b2b691df5131cb7b86ea75a734360e360e7e)
#[derive(Clone, Default)]
pub struct Skeleton2DDeformLengthExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub accepted: bool,
    pub addressing: String,
    pub addressed: f64,
    pub offsets: f64,
}
impl PartialEq for Skeleton2DDeformLengthExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
