// @generated from upstream/packages/types/src/Velocity.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Matrix;

// Source: upstream/packages/types/src/Velocity.ts:10 (sha256:9857efd596ffe6f3cd132688ed2264e350ad971fb56bbc6ab0c21e04bf59a1f8)
#[derive(Clone, Default)]
pub struct Velocity2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for Velocity2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Velocity.ts:19 (sha256:735f8f6b33ae4a5c730243d8695d7b81baf6bb3777af4dd6effa7492f291b1b1)
#[derive(Clone, Default)]
pub struct VelocitySample {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub previous_world_transform: Option<Matrix>,
    pub velocity: Velocity2D,
    pub last_frame_id: f64,
    pub explicit_frame_id: f64,
}
impl PartialEq for VelocitySample {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Velocity.ts:30 (sha256:705d00847da60afc2542ab08050acc46b9574d8b5be35d87aab6a6f3d9bfd8cb)
#[derive(Clone, Default)]
pub struct VelocityField {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub samples: Vec<(crate::OpaqueHostValue, VelocitySample)>,
    pub frame_id: f64,
}
impl PartialEq for VelocityField {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Velocity.ts:39 (sha256:fd662aa5d06ea4c451cc5c195fafc6a5338d46daa10aeb1c08b019bd6124167c)
pub type VelocityContributor = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(VelocityField, crate::OpaqueHostValue) -> () + Send + 'static>>,
>;
