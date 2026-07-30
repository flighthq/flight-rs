// @generated from upstream/packages/types/src/GrayscaleAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::AdjustmentKind;

// Source: upstream/packages/types/src/GrayscaleAdjustment.ts:3 (sha256:794de8b5340b9e5406232f11e370145bb1f6a0eb81fb1b77cbfbc53f2662cd8e)
#[derive(Clone, Default)]
pub struct GrayscaleAdjustment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: AdjustmentKind,
    pub color_matrix: Vec<f64>,
    pub intensity: Option<f64>,
}
impl PartialEq for GrayscaleAdjustment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
