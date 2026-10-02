// @generated from upstream/packages/geolocation/src/geolocation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    EntityConstruction, GeolocationErrorReason, GeolocationPosition as FlightGeolocationPosition,
    GeolocationPositionResult, GeolocationRequestOptions, HostGeolocationCapability,
};

// Source: upstream/packages/geolocation/src/geolocation.ts:11 (sha256:5147ef4b4aecc36a5e4b039a87bf857b32440d13c54dd1e8814c414e7fd0e334)
pub fn clear_geolocation_watch(host_geolocation: &HostGeolocationCapability, id: f64) -> () {
    {
        let __flight_callback = (host_geolocation.clear_watch).clone();
        let __flight_result = __flight_callback.lock().unwrap()(id);
        __flight_result
    };
}

// Source: upstream/packages/geolocation/src/geolocation.ts:15 (sha256:b582aefbf7af5d874760b1d2988927141701f7658cf891dac408f1d59acb644a)
pub fn create_geolocation_position() -> FlightGeolocationPosition {
    let mut out = allocate_entity();
    initialize_geolocation_position((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/geolocation/src/geolocation.ts:21 (sha256:8dce09d3e0ff69cfa04102c1631f90b08020f83366c868f40473f99bfb6dd8a9)
pub fn get_current_geolocation_position(
    host_geolocation: &HostGeolocationCapability,
    options: Option<GeolocationRequestOptions>,
) -> crate::FlightTask<Option<FlightGeolocationPosition>> {
    return {
        let __flight_callback = (host_geolocation.get_current_position).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (options).unwrap_or(((*_EMPTY_OPTIONS).clone()).clone()),
        );
        __flight_result
    };
}

// Source: upstream/packages/geolocation/src/geolocation.ts:28 (sha256:ca205eee05bbbdd46ccb0bffe57cf41cd44e722a31bf9cc85e01ddbed2fbb593)
pub fn get_current_geolocation_position_result(
    host_geolocation: &HostGeolocationCapability,
    options: Option<GeolocationRequestOptions>,
) -> crate::FlightTask<GeolocationPositionResult> {
    return {
        let __flight_callback = (host_geolocation.get_current_position_result).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (options).unwrap_or(((*_EMPTY_OPTIONS).clone()).clone()),
        );
        __flight_result
    };
}

// Source: upstream/packages/geolocation/src/geolocation.ts:35 (sha256:426b52349485b358ffa272cfb120d434d4e5f0ea08aeb899d766ca313aadbad3)
pub fn initialize_geolocation_position(out: EntityConstruction<FlightGeolocationPosition>) -> () {
    crate::host_set("host.accuracy", 0.0_f64);
    crate::host_set("host.altitude", 0.0_f64);
    crate::host_set("host.altitudeAccuracy", 0.0_f64);
    crate::host_set("host.floorLevel", 0.0_f64);
    crate::host_set("host.heading", 0.0_f64);
    crate::host_set("host.latitude", 0.0_f64);
    crate::host_set("host.longitude", 0.0_f64);
    crate::host_set("host.speed", 0.0_f64);
    crate::host_set("host.timestamp", 0.0_f64);
}

// Source: upstream/packages/geolocation/src/geolocation.ts:47 (sha256:ce1dbb0d594a686968afe3544b7d61fe0157bc13f5feed121854473205e2ac5d)
pub fn is_geolocation_available(host_geolocation: &HostGeolocationCapability) -> bool {
    return {
        let __flight_callback = (host_geolocation.is_available).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/geolocation/src/geolocation.ts:51 (sha256:ca039432a0fff0ebdb59c540c493bd7029182656e363244b81ace9619944b817)
pub fn watch_geolocation_position(
    host_geolocation: &HostGeolocationCapability,
    handler: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(FlightGeolocationPosition) -> () + Send + 'static>>,
    >,
    options: Option<GeolocationRequestOptions>,
    on_error: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(GeolocationErrorReason) -> () + Send + 'static>>,
        >,
    >,
) -> f64 {
    return {
        let __flight_callback = (host_geolocation.watch_position).clone();
        let __flight_result = __flight_callback.lock().unwrap()(
            (handler).clone(),
            (options).unwrap_or(((*_EMPTY_OPTIONS).clone()).clone()),
            (on_error).clone(),
        );
        __flight_result
    };
}

// Source: upstream/packages/geolocation/src/geolocation.ts:60 (sha256:4d22a64d5cd1376e132bb4cb6b886f7f08f48dbd01b084cc5aeac53952aec1b1)
static _EMPTY_OPTIONS: std::sync::LazyLock<GeolocationRequestOptions> =
    std::sync::LazyLock::new(|| GeolocationRequestOptions {
        __flight_identity: std::sync::Arc::new(()),
        enable_high_accuracy: None,
        timeout_ms: None,
        maximum_age_ms: None,
    });
