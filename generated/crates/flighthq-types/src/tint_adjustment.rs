// @generated from upstream/packages/types/src/TintAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::AdjustmentKind;

// Source: upstream/packages/types/src/TintAdjustment.ts:8 (sha256:9d84bdda46ac7af3d56560b1a55cd8a561d06c2af1c2dd407c94b8b2609c6d91)
#[derive(Clone, Default)]
pub struct TintAdjustment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: AdjustmentKind,
    pub color_matrix: Vec<f64>,
}
impl PartialEq for TintAdjustment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
