// @generated from upstream/packages/types/src/Skeleton2DConstraint.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Skeleton2D};
use crate::{Skeleton2DPathPositionMode, Skeleton2DPathRotateMode, Skeleton2DPathSpacingMode};

// Source: upstream/packages/types/src/Skeleton2DConstraint.ts:13 (sha256:295796a7969ecb43458ac873cc3d0800e54d9eeb6f42a01c15b92e8a2045c2f6)
#[derive(Clone, Default)]
pub struct Skeleton2DConstraint {
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
impl PartialEq for Skeleton2DConstraint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Skeleton2DConstraint {
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

// Source: upstream/packages/types/src/Skeleton2DConstraint.ts:28 (sha256:29937c192eef430fc5b763e210e5a527a7d104175182a52d9db1060713e65f5e)
pub type Skeleton2DConstraintSolver = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Skeleton2D, Skeleton2DConstraint) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Skeleton2DConstraint.ts:34 (sha256:5d231be4e261cce5bb93e856591656d29e37b14166d4adc6935fa55fc0996546)
#[derive(Clone, Default)]
pub struct Skeleton2DConstraintKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ik: String,
    pub path: String,
    pub transform: String,
}
impl PartialEq for Skeleton2DConstraintKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static SKELETON2_D_CONSTRAINT_KIND: std::sync::LazyLock<Skeleton2DConstraintKindValues> =
    std::sync::LazyLock::new(|| Skeleton2DConstraintKindValues {
        __flight_identity: std::sync::Arc::new(()),
        ik: "Skeleton2D.IkConstraint".to_owned(),
        path: "Skeleton2D.PathConstraint".to_owned(),
        transform: "Skeleton2D.TransformConstraint".to_owned(),
    });

// Source: upstream/packages/types/src/Skeleton2DConstraint.ts:40 (sha256:5087086c15ba73483edf1052893cd95ad2c8dedbfb9b862d6505c369b2638bdd)
pub type Skeleton2DConstraintKind = String;
