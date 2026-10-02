// @generated from upstream/packages/types/src/Sensors.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Signal};

// Source: upstream/packages/types/src/Sensors.ts:5 (sha256:0cf7accce78a809f61aff1748412306e8aea94882aed71d3ec66f507a5d009d2)
pub type SensorAccuracy = String;

// Source: upstream/packages/types/src/Sensors.ts:9 (sha256:ad12fb987cd42c73a7cc85d18304e253b02083e21959b51cd6c3fa4031875cd3)
pub type SensorsPermissionState = String;

// Source: upstream/packages/types/src/Sensors.ts:13 (sha256:044a0d9f6a0c93c6f5432e7a36417455ea0eb827d6cf5cee4f59db1e2310e551)
#[derive(Clone, Default)]
pub struct SensorSubscribeOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frequency: Option<f64>,
}
impl PartialEq for SensorSubscribeOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Sensors.ts:19 (sha256:51bd776e4a7a8b10fcbbc699dc76e06d0449bb319ba3fec3622edf9a25ad972d)
#[derive(Clone, Default)]
pub struct SensorReading {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub accuracy: SensorAccuracy,
    pub interval: f64,
    pub timestamp: f64,
}
impl PartialEq for SensorReading {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Sensors.ts:26 (sha256:658bc6b9114fc941382bda4fc975ec33c0a5798914ccb98a338dabed49f09a69)
#[derive(Clone, Default)]
pub struct AmbientLightReading {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub accuracy: SensorAccuracy,
    pub interval: f64,
    pub timestamp: f64,
    pub illuminance: f64,
}
impl PartialEq for AmbientLightReading {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AmbientLightReading {
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

// Source: upstream/packages/types/src/Sensors.ts:32 (sha256:7101bd01ac8d16848c8197e8e6f33ed6a051a95535dfe77e5e0c0fc8d7cb3a3c)
#[derive(Clone, Default)]
pub struct MotionReading {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub accuracy: SensorAccuracy,
    pub interval: f64,
    pub timestamp: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl PartialEq for MotionReading {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for MotionReading {
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

// Source: upstream/packages/types/src/Sensors.ts:39 (sha256:81b11c566fabcdde2bf9af1a8ee4620395b5d2491d463f49b0a8c5c9c6dfc896)
#[derive(Clone, Default)]
pub struct OrientationReading {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub accuracy: SensorAccuracy,
    pub interval: f64,
    pub timestamp: f64,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    pub absolute: bool,
    pub heading: f64,
}
impl PartialEq for OrientationReading {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for OrientationReading {
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

// Source: upstream/packages/types/src/Sensors.ts:50 (sha256:571b9932aee0ab6ce0f3ece34015ffa8f0fb63ff4cb072bc30a8ec7604357da0)
#[derive(Clone, Default)]
pub struct PressureReading {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub accuracy: SensorAccuracy,
    pub interval: f64,
    pub timestamp: f64,
    pub altitude: f64,
    pub pressure: f64,
}
impl PartialEq for PressureReading {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for PressureReading {
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

// Source: upstream/packages/types/src/Sensors.ts:57 (sha256:316c26e9265f1c87fc075d1d02d04256780746cd984d784add2298075bfb6a67)
#[derive(Clone, Default)]
pub struct ProximityReading {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub accuracy: SensorAccuracy,
    pub interval: f64,
    pub timestamp: f64,
    pub distance: f64,
    pub max: f64,
    pub near: bool,
}
impl PartialEq for ProximityReading {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ProximityReading {
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

// Source: upstream/packages/types/src/Sensors.ts:64 (sha256:4a25795b0e6b19ecbc6cb2b5c26a488198f4a750fbf257321889cee214e9cf6f)
#[derive(Clone, Default)]
pub struct QuaternionReading {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub accuracy: SensorAccuracy,
    pub interval: f64,
    pub timestamp: f64,
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl PartialEq for QuaternionReading {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for QuaternionReading {
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

// Source: upstream/packages/types/src/Sensors.ts:72 (sha256:80907b927f2f94f7c1eeec18a12f9f629f15ed0a7a8fef7a95a4d2067804ecee)
#[derive(Clone, Default)]
pub struct RotationRateReading {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub accuracy: SensorAccuracy,
    pub interval: f64,
    pub timestamp: f64,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
}
impl PartialEq for RotationRateReading {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RotationRateReading {
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

// Source: upstream/packages/types/src/Sensors.ts:84 (sha256:edb862797abb98f54ccaeab8582d3f8de3f39e1a4e213c2d55ee2b5810d4e906)
#[derive(Clone)]
pub struct HostSensorsCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_permission_state: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Option<String>) -> crate::FlightTask<SensorsPermissionState>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub is_ambient_light_supported:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub is_barometer_supported:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub is_gravity_supported:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub is_gyroscope_supported:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub is_linear_acceleration_supported:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub is_magnetometer_supported:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub is_motion_supported:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub is_orientation_supported:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub is_proximity_supported:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub request_permission: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<bool> + Send + 'static>>,
    >,
    pub subscribe_absolute_orientation: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(OrientationReading) -> () + Send + 'static>,
                            >,
                        >,
                        Option<SensorSubscribeOptions>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_ambient_light: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(AmbientLightReading) -> () + Send + 'static>,
                            >,
                        >,
                        Option<SensorSubscribeOptions>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_barometer: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(PressureReading) -> () + Send + 'static>,
                            >,
                        >,
                        Option<SensorSubscribeOptions>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_gravity: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(MotionReading) -> () + Send + 'static>>,
                        >,
                        Option<SensorSubscribeOptions>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_linear_acceleration: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(MotionReading) -> () + Send + 'static>>,
                        >,
                        Option<SensorSubscribeOptions>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_magnetometer: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(MotionReading) -> () + Send + 'static>>,
                        >,
                        Option<SensorSubscribeOptions>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_motion: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<
                                    dyn FnMut(MotionReading, RotationRateReading) -> ()
                                        + Send
                                        + 'static,
                                >,
                            >,
                        >,
                        Option<SensorSubscribeOptions>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_orientation: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(OrientationReading) -> () + Send + 'static>,
                            >,
                        >,
                        Option<SensorSubscribeOptions>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_proximity: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(ProximityReading) -> () + Send + 'static>,
                            >,
                        >,
                        Option<SensorSubscribeOptions>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_quaternion: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(QuaternionReading) -> () + Send + 'static>,
                            >,
                        >,
                        Option<SensorSubscribeOptions>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostSensorsCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Sensors.ts:160 (sha256:b9483a63d1915ff1f28100c5918e029a775fa9e8bafc9088bfba64575e6e4b0c)
#[derive(Clone)]
pub struct Sensors {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_absolute_orientation: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(OrientationReading) -> () + Send + 'static>>>,
    >,
    pub on_accelerometer: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(MotionReading) -> () + Send + 'static>>>,
    >,
    pub on_ambient_light: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(AmbientLightReading) -> () + Send + 'static>>,
        >,
    >,
    pub on_barometer: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PressureReading) -> () + Send + 'static>>>,
    >,
    pub on_gravity: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(MotionReading) -> () + Send + 'static>>>,
    >,
    pub on_gyroscope: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RotationRateReading) -> () + Send + 'static>>,
        >,
    >,
    pub on_linear_acceleration: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(MotionReading) -> () + Send + 'static>>>,
    >,
    pub on_magnetometer: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(MotionReading) -> () + Send + 'static>>>,
    >,
    pub on_orientation: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(OrientationReading) -> () + Send + 'static>>>,
    >,
    pub on_proximity: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ProximityReading) -> () + Send + 'static>>>,
    >,
    pub on_quaternion: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(QuaternionReading) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for Sensors {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Sensors {
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
