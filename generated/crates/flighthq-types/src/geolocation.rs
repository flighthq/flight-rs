// @generated from upstream/packages/types/src/Geolocation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/Geolocation.ts:8 (sha256:0bf54f43ed52b1d5a1436c2d53acf9ac9de2a4b6456ce8b785b8ea3e2c2bd312)
#[derive(Clone, Default)]
pub struct GeolocationPosition {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f64,
    pub altitude: f64,
    pub altitude_accuracy: f64,
    pub floor_level: f64,
    pub heading: f64,
    pub speed: f64,
    pub timestamp: f64,
}
impl PartialEq for GeolocationPosition {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for GeolocationPosition {
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

// Source: upstream/packages/types/src/Geolocation.ts:22 (sha256:b7520bfe18268367c0e715beb88c590516293b3ed21056fc88a3f66c356cc4e4)
pub type GeolocationErrorReason = String;

// Source: upstream/packages/types/src/Geolocation.ts:26 (sha256:97f839ec4d25afae1174396ca465ff3c0410247133885d15513378ac41b7bdcf)
#[derive(Clone, Default)]
pub struct GeolocationPositionResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub position: Option<GeolocationPosition>,
    pub reason: Option<GeolocationErrorReason>,
}
impl PartialEq for GeolocationPositionResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Geolocation.ts:31 (sha256:5f1a61b650bedcbcdbe22934d14b583849c03ba675c429173cd6b734ff9eb020)
#[derive(Clone, Default)]
pub struct GeolocationRequestOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub enable_high_accuracy: Option<bool>,
    pub timeout_ms: Option<f64>,
    pub maximum_age_ms: Option<f64>,
}
impl PartialEq for GeolocationRequestOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Geolocation.ts:52 (sha256:4bc598e6397dc3a454ad668b3d4af2c9878a8d2db989be322ea5f98c41364157)
#[derive(Clone, Default)]
pub struct GeolocationAccessOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for GeolocationAccessOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Geolocation.ts:63 (sha256:0a3035c705e756b3ee2a716c35865ccdabf801cceb3a2d40ba218b7ee007ff1b)
#[derive(Clone)]
pub struct HostGeolocationCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_current_position: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        GeolocationRequestOptions,
                    ) -> crate::FlightTask<Option<GeolocationPosition>>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub get_current_position_result: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(GeolocationRequestOptions) -> crate::FlightTask<GeolocationPositionResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub is_available: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub watch_position: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(GeolocationPosition) -> () + Send + 'static>,
                            >,
                        >,
                        GeolocationRequestOptions,
                        Option<
                            std::sync::Arc<
                                std::sync::Mutex<
                                    Box<dyn FnMut(GeolocationErrorReason) -> () + Send + 'static>,
                                >,
                            >,
                        >,
                    ) -> f64
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub clear_watch: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>,
    pub prompt_for_access: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<GeolocationAccessOutcome> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostGeolocationCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
