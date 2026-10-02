// @generated from upstream/packages/types/src/Skeleton2DIkConstraint.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Skeleton2DConstraintKind};
use crate::{Skeleton2DPathPositionMode, Skeleton2DPathRotateMode, Skeleton2DPathSpacingMode};

// Source: upstream/packages/types/src/Skeleton2DIkConstraint.ts:22 (sha256:3c0fee3ae382d16ba3cd1c9fc452d167718acbfee3812c43c4f7942f1c656469)
#[derive(Clone, Default)]
pub struct Skeleton2DIkConstraint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
    pub position: f64,
    pub position_mode: Skeleton2DPathPositionMode,
    pub rotate_mode: Skeleton2DPathRotateMode,
    pub spacing: f64,
    pub spacing_mode: Skeleton2DPathSpacingMode,
    pub target_slot_index: f64,
    pub bend_positive: bool,
    pub compress: bool,
    pub stretch: bool,
}
impl PartialEq for Skeleton2DIkConstraint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Skeleton2DIkConstraint {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}
