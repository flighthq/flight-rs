// @generated from upstream/packages/types/src/TweenStaggerOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EasingFunction;

// Source: upstream/packages/types/src/TweenStaggerOptions.ts:3 (sha256:ba960751ea24df55f05f1205d33acaf88f17c7d0c860e0cc564d667cd2f97448)
#[derive(Clone, Default)]
pub struct TweenStaggerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub each: Option<f64>,
    pub from: Option<crate::FlightUnion2<String, f64>>,
    pub stagger_ease: Option<EasingFunction>,
}
impl PartialEq for TweenStaggerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
