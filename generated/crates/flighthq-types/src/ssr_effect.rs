// @generated from upstream/packages/types/src/SsrEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/SsrEffect.ts:3 (sha256:b8cd16061084ca5ebe1d0ff834eeec28042ac12c48f60b14478d08da81560b9a)
#[derive(Clone, Default)]
pub struct SsrEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub max_distance: Option<f64>,
    pub resolution: Option<f64>,
    pub steps: Option<f64>,
}
impl PartialEq for SsrEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
