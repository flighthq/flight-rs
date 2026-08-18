// @generated from upstream/packages/types/src/Skeleton2DTransformConstraint.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Skeleton2DConstraintKind;

// Source: upstream/packages/types/src/Skeleton2DTransformConstraint.ts:21 (sha256:7ed2b99d05ce368354bb2e67e4a595cb4863a929c6755cfcdab9902470c39959)
#[derive(Clone, Default)]
pub struct Skeleton2DTransformConstraint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Skeleton2DConstraintKind,
    pub mix: f64,
    pub bone_indices: Vec<f64>,
    pub mix_rotate: f64,
    pub mix_scale_x: f64,
    pub mix_scale_y: f64,
    pub mix_shear_y: f64,
    pub mix_x: f64,
    pub mix_y: f64,
    pub offset_rotation: f64,
    pub offset_scale_x: f64,
    pub offset_scale_y: f64,
    pub offset_shear_y: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub target_bone_index: f64,
}
impl PartialEq for Skeleton2DTransformConstraint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
