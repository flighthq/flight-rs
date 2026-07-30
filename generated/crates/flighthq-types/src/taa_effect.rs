// @generated from upstream/packages/types/src/TaaEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/TaaEffect.ts:3 (sha256:df810c7e99092833a66b4f848097a10af3ed4d90817e09382ea1b09debf17c5a)
#[derive(Clone, Default)]
pub struct TaaEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub feedback: Option<f64>,
}
impl PartialEq for TaaEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
