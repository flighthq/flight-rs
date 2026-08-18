// @generated from upstream/packages/types/src/Skeleton2DIkConstraint.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Skeleton2DConstraintKind;

// Source: upstream/packages/types/src/Skeleton2DIkConstraint.ts:22 (sha256:3c0fee3ae382d16ba3cd1c9fc452d167718acbfee3812c43c4f7942f1c656469)
#[derive(Clone, Default)]
pub struct Skeleton2DIkConstraint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Skeleton2DConstraintKind,
    pub mix: f64,
    pub bend_positive: bool,
    pub bone_indices: Vec<f64>,
    pub compress: bool,
    pub stretch: bool,
    pub target_bone_index: f64,
}
impl PartialEq for Skeleton2DIkConstraint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
