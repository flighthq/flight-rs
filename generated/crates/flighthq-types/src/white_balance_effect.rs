// @generated from upstream/packages/types/src/WhiteBalanceEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/WhiteBalanceEffect.ts:3 (sha256:46e7afe7400e1dd441ada4a08bd1d9096ceb91e5f08e1de482c8798457b3e22b)
#[derive(Clone, Default)]
pub struct WhiteBalanceEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub temperature: Option<f64>,
    pub tint: Option<f64>,
}
impl PartialEq for WhiteBalanceEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
