// @generated from upstream/packages/types/src/Geolocation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Geolocation.ts:8 (sha256:5417b3f83e7e0285670398975f0aa6df0c4500a1ad50700de97d2a3358a4321a)
#[derive(Clone)]
pub struct GeoPosition {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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
impl PartialEq for GeoPosition {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Geolocation.ts:22 (sha256:b7520bfe18268367c0e715beb88c590516293b3ed21056fc88a3f66c356cc4e4)
pub type GeolocationErrorReason = String;

// Source: upstream/packages/types/src/Geolocation.ts:25 (sha256:37338ec08a1e76cfdd7ce6120774b503a65523bd95fe775f31f26b8c8d94ff43)
pub type GeolocationPermissionState = String;

// Source: upstream/packages/types/src/Geolocation.ts:29 (sha256:22f866c0ec750b09a4f5ebefce2e17cb9296a23384038fd24a041981784c87ab)
#[derive(Clone)]
pub struct GeoPositionResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub position: Option<GeoPosition>,
    pub reason: Option<GeolocationErrorReason>,
}
impl PartialEq for GeoPositionResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Geolocation.ts:34 (sha256:5f1a61b650bedcbcdbe22934d14b583849c03ba675c429173cd6b734ff9eb020)
#[derive(Clone)]
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

// Source: upstream/packages/types/src/Geolocation.ts:40 (sha256:f714682d4acf1065872d4a54442f2448ca40f3957792a7e56c0f0e5f7419e74a)
#[derive(Clone)]
pub struct GeolocationBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_current_position: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(GeolocationRequestOptions) -> crate::Promise<Option<GeoPosition>>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub get_current_position_result: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(GeolocationRequestOptions) -> crate::Promise<GeoPositionResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub get_permission: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::Promise<GeolocationPermissionState> + Send + 'static>,
        >,
    >,
    pub watch_position: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(GeoPosition) -> () + Send + 'static>>,
                        >,
                        GeolocationRequestOptions,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(GeolocationErrorReason) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> f64
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub clear_watch: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>,
    pub request_permission:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> crate::Promise<bool> + Send + 'static>>>,
    pub subscribe_permission: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(GeolocationPermissionState) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for GeolocationBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
