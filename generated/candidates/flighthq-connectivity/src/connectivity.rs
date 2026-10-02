// @generated from upstream/packages/connectivity/src/connectivity.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_signals::{clear_signal, create_signal, emit_signal};
use flighthq_types::{
    Connectivity, ConnectivityConnectionType, ConnectivityReachability,
    ConnectivityReachabilityOptions, ConnectivityStatus, EntityConstruction,
    HostConnectivityChangeCapability, HostConnectivityReachabilityCapability,
    HostConnectivityStatusCapability,
};

// Source: upstream/packages/connectivity/src/connectivity.ts:18 (sha256:b65d962fa450617b73adb76e95519f41bd8f9fb96f2ac2df24731c0ba7b10589)
pub fn attach_connectivity(
    host_connectivity_status: &HostConnectivityStatusCapability,
    host_connectivity_change: &HostConnectivityChangeCapability,
    connectivity: Connectivity,
) -> bool {
    detach_connectivity(&connectivity);
    let initial = {
        let __flight_callback = (host_connectivity_status.get_status).clone();
        let __flight_result = __flight_callback.lock().unwrap()(connectivity_status_out());
        __flight_result
    };
    let was_online: std::sync::Arc<std::sync::Mutex<Option<bool>>> =
        std::sync::Arc::new(std::sync::Mutex::new(initial.online));
    let was_type: std::sync::Arc<std::sync::Mutex<ConnectivityConnectionType>> =
        std::sync::Arc::new(std::sync::Mutex::new((initial.type_).clone()));
    let was_metered: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(initial.metered));
    let unsubscribe = {
        let __flight_callback = (host_connectivity_change.subscribe).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let connectivity = connectivity.clone();
                let status_backend = status_backend.clone();
                let mut was_metered = was_metered.clone();
                let mut was_online = was_online.clone();
                let mut was_type = was_type.clone();
                move || -> () {
                    let status = {
                        let __flight_callback = (status_backend.get_status).clone();
                        let __flight_result =
                            __flight_callback.lock().unwrap()(connectivity_status_out());
                        __flight_result
                    };
                    emit_signal((connectivity.on_change).clone(), ((status).clone(),));
                    if (status.online != (*was_online.lock().unwrap()).clone()) {
                        (*was_online.lock().unwrap()) = status.online;
                        if (status.online) == Some(true) {
                            emit_signal((connectivity.on_online).clone(), ());
                        } else {
                            if (status.online) == Some(false) {
                                emit_signal((connectivity.on_offline).clone(), ());
                            }
                        }
                    }
                    if ((status.type_).clone() != (*was_type.lock().unwrap()).clone()) {
                        (*was_type.lock().unwrap()) = (status.type_).clone();
                        emit_signal(
                            (connectivity.on_connection_type_change).clone(),
                            ((status.type_).clone(),),
                        );
                    }
                    if (status.metered != (*was_metered.lock().unwrap()).clone()) {
                        (*was_metered.lock().unwrap()) = status.metered;
                        emit_signal((connectivity.on_metered_change).clone(), (status.metered,));
                    }
                }
            }) as Box<dyn FnMut() -> () + Send + 'static>),
        ));
        __flight_result
    };
    if (unsubscribe).is_none() {
        return false;
    }
    {
        let __flight_key = (connectivity).clone();
        let __flight_value = (unsubscribe.as_ref().unwrap()).clone();
        if let Some((_, value)) = (*_SUBSCRIPTIONS.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_SUBSCRIPTIONS.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    return true;
}

// Source: upstream/packages/connectivity/src/connectivity.ts:51 (sha256:72c291e439e01a9a5fd33dca82369522efeadde1c31ed34445c3d81ae8bf6435)
pub fn create_connectivity() -> Connectivity {
    let mut out = allocate_entity();
    initialize_connectivity((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/connectivity/src/connectivity.ts:59 (sha256:660592e13fd2fe3b91659df2b80062a6891d47e63d8477ad8cb491886cd9250f)
pub fn destroy_connectivity(host_connectivity_change: &HostConnectivityChangeCapability) -> () {
    {
        let __flight_callback = (host_connectivity_change.destroy).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/connectivity/src/connectivity.ts:63 (sha256:bf0ba07dde0d52d7b2685d8a4af368769671a7fcf67c856c76726957299c3bea)
pub fn detach_connectivity(connectivity: &Connectivity) -> () {
    let unsubscribe = (*_SUBSCRIPTIONS.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*connectivity).clone())
        .map(|(_, value)| value.clone());
    if (unsubscribe).is_none() {
        return;
    }
    {
        let __flight_key = (*connectivity).clone();
        if let Some(__flight_index) = (*_SUBSCRIPTIONS.lock().unwrap())
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            (*_SUBSCRIPTIONS.lock().unwrap()).remove(__flight_index);
            true
        } else {
            false
        }
    };
    {
        let __flight_callback = (unsubscribe.as_ref().unwrap()).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
}

// Source: upstream/packages/connectivity/src/connectivity.ts:70 (sha256:ec26e3b79d797e542794dca19688c41af5fb389fe1852a1bb5629b4f34b7b71b)
pub fn detect_connectivity_reachability(
    host_connectivity_reachability: &HostConnectivityReachabilityCapability,
    options: &ConnectivityReachabilityOptions,
    out: &ConnectivityReachability,
) -> crate::FlightTask<ConnectivityReachability> {
    return {
        let __flight_callback = (host_connectivity_reachability.detect_reachability).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*options).clone(), (*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/connectivity/src/connectivity.ts:80 (sha256:8d79c8eb0fa99b4e31aea0c348127832d371014c736a9870a2700b3ce54f974a)
pub fn dispose_connectivity(connectivity: &mut Connectivity) -> () {
    detach_connectivity(connectivity);
    clear_signal(&mut connectivity.on_change);
    clear_signal(&mut connectivity.on_connection_type_change);
    clear_signal(&mut connectivity.on_metered_change);
    clear_signal(&mut connectivity.on_offline);
    clear_signal(&mut connectivity.on_online);
}

// Source: upstream/packages/connectivity/src/connectivity.ts:89 (sha256:afe0d7bdb41170693a29b20b9804df58506300e39daa324e2115875d0a8d682b)
pub fn get_connectivity_online(
    host_connectivity_status: &HostConnectivityStatusCapability,
) -> Option<bool> {
    return {
        let __flight_callback = (host_connectivity_status.get_status).clone();
        let __flight_result = __flight_callback.lock().unwrap()(connectivity_status_out());
        __flight_result
    }
    .online;
}

// Source: upstream/packages/connectivity/src/connectivity.ts:95 (sha256:ff9805c9e419d918936626b8a233d4501ab3538f9741e63251cbf1acb1603b6d)
pub fn get_connectivity_status(
    host_connectivity_status: &HostConnectivityStatusCapability,
    out: &ConnectivityStatus,
) -> ConnectivityStatus {
    return {
        let __flight_callback = (host_connectivity_status.get_status).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/connectivity/src/connectivity.ts:102 (sha256:b6653bbe1b05bfaf01e74396e72ad9e5c9fad9a227c9402edae15b4f6ebf1d5a)
pub fn has_connectivity_status_changed(a: &ConnectivityStatus, b: &ConnectivityStatus) -> bool {
    return (((((((a.online != b.online) || ((a.type_).clone() != (b.type_).clone()))
        || (a.downlink != b.downlink))
        || (a.downlink_max != b.downlink_max))
        || ((a.effective_type).clone() != (b.effective_type).clone()))
        || (a.rtt != b.rtt))
        || (a.save_data != b.save_data))
        || (a.metered != b.metered);
}

// Source: upstream/packages/connectivity/src/connectivity.ts:118 (sha256:05c4eb3131dcc87e93e07f35c6625f0003422f1ee07fb0246f2abb0392e57f26)
pub fn initialize_connectivity(out: EntityConstruction<Connectivity>) -> () {
    crate::host_set("host.onChange", create_signal());
    crate::host_set("host.onConnectionTypeChange", create_signal());
    crate::host_set("host.onMeteredChange", create_signal());
    crate::host_set("host.onOffline", create_signal());
    crate::host_set("host.onOnline", create_signal());
}

// Source: upstream/packages/connectivity/src/connectivity.ts:126 (sha256:9cd43639fb0cd96c3d2544418e2f33d4ae9621c161c2c068d398e4deab5f14a3)
pub fn is_connectivity_metered(
    host_connectivity_status: &HostConnectivityStatusCapability,
) -> bool {
    return {
        let __flight_callback = (host_connectivity_status.get_status).clone();
        let __flight_result = __flight_callback.lock().unwrap()(connectivity_status_out());
        __flight_result
    }
    .metered;
}

// Source: upstream/packages/connectivity/src/connectivity.ts:130 (sha256:25162d01e0f723dd7d61e28a8435b2199d619893735bcaaadce02dee81274f1f)
pub fn is_connectivity_save_data_enabled(
    host_connectivity_status: &HostConnectivityStatusCapability,
) -> bool {
    return {
        let __flight_callback = (host_connectivity_status.get_status).clone();
        let __flight_result = __flight_callback.lock().unwrap()(connectivity_status_out());
        __flight_result
    }
    .save_data;
}

// Source: upstream/packages/connectivity/src/connectivity.ts:138 (sha256:5d68015a53f455893da96c51c62bcf239f741ead9ad0134fbbc9438f6a0e3dda)
fn connectivity_status_out() -> ConnectivityStatus {
    return ConnectivityStatus {
        __flight_identity: std::sync::Arc::new(()),
        downlink: (-1.0_f64),
        downlink_max: (-1.0_f64),
        effective_type: "".to_owned(),
        metered: false,
        online: None,
        rtt: (-1.0_f64),
        save_data: false,
        type_: "unknown".to_owned(),
    };
}

// Source: upstream/packages/connectivity/src/connectivity.ts:153 (sha256:37f6bcee33c627fe0e07182f73a47155c613de9148ee108332f00e135a2cfaee)
static _SUBSCRIPTIONS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            Connectivity,
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
