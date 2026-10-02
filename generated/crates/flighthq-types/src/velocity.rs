// @generated from upstream/packages/types/src/Velocity.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Matrix};

// Source: upstream/packages/types/src/Velocity.ts:11 (sha256:9857efd596ffe6f3cd132688ed2264e350ad971fb56bbc6ab0c21e04bf59a1f8)
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

// Source: upstream/packages/types/src/Velocity.ts:20 (sha256:735f8f6b33ae4a5c730243d8695d7b81baf6bb3777af4dd6effa7492f291b1b1)
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

// Source: upstream/packages/types/src/Velocity.ts:31 (sha256:97fa5de5d18a0cc73cdecd77bfb53c56a79a4a5a8e51b13fb37604554e4e3e1b)
#[derive(Clone, Default)]
pub struct VelocityField {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub samples: Vec<(crate::OpaqueHostValue, VelocitySample)>,
    pub frame_id: f64,
}
impl PartialEq for VelocityField {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for VelocityField {
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

// Source: upstream/packages/types/src/Velocity.ts:39 (sha256:fd662aa5d06ea4c451cc5c195fafc6a5338d46daa10aeb1c08b019bd6124167c)
pub type VelocityContributor = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(VelocityField, crate::OpaqueHostValue) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Velocity.ts:41 (sha256:09fdb2bf0dd4c0357b3236ee152246b60350a08b086a3cc412ade9b78a1981ef)
#[derive(Clone, Default)]
pub struct VelocityExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub x: f64,
    pub y: f64,
    pub last_frame_id: f64,
    pub current_frame_id: f64,
    pub explicit: bool,
}
impl PartialEq for VelocityExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
