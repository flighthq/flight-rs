// @generated from upstream/packages/types/src/StrokePathTessellationExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::StrokePathTessellationReason;

// Source: upstream/packages/types/src/StrokePathTessellationExplanation.ts:5 (sha256:880687c84929606b6ea63073003a6ca7b025b7c6acd7c0ba13dad5b65bb004eb)
#[derive(Clone, Default)]
pub struct StrokePathTessellationExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: StrokePathTessellationReason,
    pub subpath: Option<f64>,
    pub supported: bool,
}
impl PartialEq for StrokePathTessellationExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
