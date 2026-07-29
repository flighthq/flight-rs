// @generated from upstream/packages/types/src/EasingSegment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EasingFunction;

// Source: upstream/packages/types/src/EasingSegment.ts:2 (sha256:55a214f86ca556a427a87e073ef570d97e5ab05bbf370c6958cda32ab9e91697)
#[derive(Clone)]
pub struct EasingSegment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ease: EasingFunction,
    pub weight: Option<f64>,
}
impl PartialEq for EasingSegment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
