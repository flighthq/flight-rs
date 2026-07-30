// @generated from upstream/packages/types/src/BokehDepthOfFieldEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/BokehDepthOfFieldEffect.ts:3 (sha256:16a862ca4f652abfebfe38e52d9f65c9f3949642874a3160d3f6ad43a5d5ac9d)
#[derive(Clone, Default)]
pub struct BokehDepthOfFieldEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub focus_distance: Option<f64>,
    pub focus_range: Option<f64>,
    pub max_blur: Option<f64>,
}
impl PartialEq for BokehDepthOfFieldEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
