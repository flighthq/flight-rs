// @generated from upstream/packages/types/src/SepiaAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::AdjustmentKind;

// Source: upstream/packages/types/src/SepiaAdjustment.ts:3 (sha256:a83e972503e43391f11ed9b702ec92cb0573028669784f134f2dcfcc50090790)
#[derive(Clone, Default)]
pub struct SepiaAdjustment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: AdjustmentKind,
    pub color_matrix: Vec<f64>,
    pub intensity: Option<f64>,
}
impl PartialEq for SepiaAdjustment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
