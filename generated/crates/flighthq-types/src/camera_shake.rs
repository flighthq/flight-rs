// @generated from upstream/packages/types/src/CameraShake.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/CameraShake.ts:10 (sha256:e5b7f6c731c9401f319212ee81d1f88af6e575e14a96eb18de103863902f8fa5)
#[derive(Clone, Default)]
pub struct CameraShake {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub trauma: f64,
    pub decay: f64,
    pub frequency: f64,
    pub translation_amplitude: f64,
    pub rotation_amplitude: f64,
    pub time: f64,
}
impl PartialEq for CameraShake {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for CameraShake {
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

// Source: upstream/packages/types/src/CameraShake.ts:25 (sha256:42b3116293cb4680346d2b3aae31626385efb1e514088bed70cc2a59cc3dc525)
#[derive(Clone, Default)]
pub struct CameraShakeOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub decay: Option<f64>,
    pub frequency: Option<f64>,
    pub rotation_amplitude: Option<f64>,
    pub translation_amplitude: Option<f64>,
}
impl PartialEq for CameraShakeOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CameraShake.ts:33 (sha256:1351d7d6d4168f3228f0fe1c90ee71099539cb2e5676a07a4305dd5b65d93793)
#[derive(Clone, Default)]
pub struct CameraShakeOffset {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub rotation_x: f64,
    pub rotation_y: f64,
    pub rotation_z: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl PartialEq for CameraShakeOffset {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for CameraShakeOffset {
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
