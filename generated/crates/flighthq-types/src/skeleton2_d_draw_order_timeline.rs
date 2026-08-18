// @generated from upstream/packages/types/src/Skeleton2DDrawOrderTimeline.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Skeleton2DDrawOrderTimeline.ts:19 (sha256:123632de2b9b7baac75543b2d029fc8ba8df71833030562d322b15773ed008b9)
#[derive(Clone, Default)]
pub struct Skeleton2DDrawOrderTimeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub orderings: Vec<f64>,
    pub times: Vec<f64>,
}
impl PartialEq for Skeleton2DDrawOrderTimeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
