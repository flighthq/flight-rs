// @generated from upstream/packages/types/src/ContactShadowsEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/ContactShadowsEffect.ts:2 (sha256:18141af08abdac885e63bf9981ac65735c9d041498d778ad59ee7191837ee6f7)
#[derive(Clone, Default)]
pub struct ContactShadowsEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub distance: Option<f64>,
    pub opacity: Option<f64>,
    pub samples: Option<f64>,
    pub smoothness: Option<f64>,
}
impl PartialEq for ContactShadowsEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
