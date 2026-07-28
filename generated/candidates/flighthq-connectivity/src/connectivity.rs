// @generated from upstream/packages/connectivity/src/connectivity.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{
    Connectivity, ConnectivityBackend, ConnectivityConnectionType, ConnectivityReachability,
    ConnectivityReachabilityOptions, ConnectivityStatus,
};

// Source: upstream/packages/connectivity/src/connectivity.ts:14 (sha256:5f8cee79dd1525762b0020a6211ef237fcab2b078a54bf1cb31ef2b1e5f7887e)
pub fn attach_connectivity(net: Connectivity) -> () {
    detach_connectivity(&net);
    let backend: std::sync::Arc<std::sync::Mutex<ConnectivityBackend>> =
        std::sync::Arc::new(std::sync::Mutex::new(get_connectivity_backend()));
    let mut initial = {
        let __flight_callback = ((*backend.lock().unwrap()).get_status).clone();
        let __flight_result = __flight_callback.lock().unwrap()(((*_SCRATCH).clone()).clone());
        __flight_result
    };
    let was_online: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(initial.online));
    let was_type: std::sync::Arc<std::sync::Mutex<ConnectivityConnectionType>> =
        std::sync::Arc::new(std::sync::Mutex::new((initial.type_).clone()));
    let was_metered: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(initial.metered));
    let unsubscribe = {
        let __flight_callback = ((*backend.lock().unwrap()).subscribe).clone();
        let __flight_result = __flight_callback.lock().unwrap()(std::sync::Arc::new(
            std::sync::Mutex::new(Box::new({
                let mut backend = backend.clone();
                let net = net.clone();
                let mut was_metered = was_metered.clone();
                let mut was_online = was_online.clone();
                let mut was_type = was_type.clone();
                move || -> () {
                    let status = {
                        let __flight_callback = ((*backend.lock().unwrap()).get_status).clone();
                        let __flight_result =
                            __flight_callback.lock().unwrap()(((*_SCRATCH).clone()).clone());
                        __flight_result
                    };
                    emit_signal((net.on_change).clone(), ((status).clone(),));
                    if (status.online != (*was_online.lock().unwrap()).clone()) {
                        (*was_online.lock().unwrap()) = status.online;
                        emit_signal(
                            if status.online {
                                (net.on_online).clone()
                            } else {
                                (net.on_offline).clone()
                            },
                            (),
                        );
                    }
                    if ((status.type_).clone() != (*was_type.lock().unwrap()).clone()) {
                        (*was_type.lock().unwrap()) = (status.type_).clone();
                        emit_signal(
                            (net.on_connection_type_change).clone(),
                            ((status.type_).clone(),),
                        );
                    }
                    if (status.metered != (*was_metered.lock().unwrap()).clone()) {
                        (*was_metered.lock().unwrap()) = status.metered;
                        emit_signal((net.on_metered_change).clone(), (status.metered,));
                    }
                }
            }) as Box<dyn FnMut() -> () + Send + 'static>),
        ));
        __flight_result
    };
    {
        let __flight_key = (net).clone();
        let __flight_value = (unsubscribe).clone();
        if let Some((_, value)) = (*_SUBSCRIPTIONS.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_SUBSCRIPTIONS.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/connectivity/src/connectivity.ts:41 (sha256:cd07d0d3a653726d937c379deeb4763548a73cc38fc4a020c461834968830f6d)
pub fn create_connectivity() -> Connectivity {
    return Connectivity {
        __flight_identity: std::sync::Arc::new(()),
        on_change: create_signal(),
        on_connection_type_change: create_signal(),
        on_metered_change: create_signal(),
        on_offline: create_signal(),
        on_online: create_signal(),
    };
}

// Source: upstream/packages/connectivity/src/connectivity.ts:52 (sha256:cca32f407b06e5c7b737e6f2075d6486b63376258f93fcf04c40aef801eb82ed)
#[derive(Clone, Default)]
struct CreateConnectivityStatusRecord1 {
    __flight_identity: std::sync::Arc<()>,
    downlink: f64,
    downlink_max: f64,
    effective_type: String,
    metered: bool,
    online: bool,
    rtt: f64,
    save_data: bool,
    type_: String,
}
impl PartialEq for CreateConnectivityStatusRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_connectivity_status() -> ConnectivityStatus {
    return ConnectivityStatus {
        __flight_identity: std::sync::Arc::new(()),
        downlink: (-1.0_f64),
        downlink_max: (-1.0_f64),
        effective_type: "".to_owned(),
        metered: false,
        online: false,
        rtt: (-1.0_f64),
        save_data: false,
        type_: "unknown".to_owned(),
    };
}

// Source: upstream/packages/connectivity/src/connectivity.ts:67 (sha256:6a3e1126b49f89e6916dbb83bc9c53bc96d8c6d97c49493546614117ae005066)
pub fn create_web_connectivity_backend() -> ConnectivityBackend {
    return ConnectivityBackend {
        __flight_identity: std::sync::Arc::new(()),
        get_status: std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut out: ConnectivityStatus| -> ConnectivityStatus {
                let nav: Option<crate::OpaqueHostValue> = None;
                out.online = (None::<bool>).unwrap_or(true);
                let conn = get_web_connection();
                out.type_ = map_web_connection_type(
                    (conn.as_ref().and_then(|value| (value.type_).clone())).clone(),
                );
                out.downlink = if ((conn.as_ref().and_then(|value| value.downlink))
                    .as_ref()
                    .map_or("undefined", |_| "number")
                    == "number")
                {
                    (conn.as_ref().unwrap().downlink).unwrap()
                } else {
                    (-1.0_f64)
                };
                out.downlink_max = if ((conn.as_ref().and_then(|value| value.downlink_max))
                    .as_ref()
                    .map_or("undefined", |_| "number")
                    == "number")
                {
                    (conn.as_ref().unwrap().downlink_max).unwrap()
                } else {
                    (-1.0_f64)
                };
                out.effective_type = if ((conn
                    .as_ref()
                    .and_then(|value| (value.effective_type).clone()))
                .as_ref()
                .map_or("undefined", |_| "string")
                    == "string")
                {
                    ((conn.as_ref().unwrap().effective_type).clone()).unwrap()
                } else {
                    "".to_owned()
                };
                out.rtt = if ((conn.as_ref().and_then(|value| value.rtt))
                    .as_ref()
                    .map_or("undefined", |_| "number")
                    == "number")
                {
                    (conn.as_ref().unwrap().rtt).unwrap()
                } else {
                    (-1.0_f64)
                };
                out.save_data = (conn.as_ref().and_then(|value| value.save_data)) == Some(true);
                out.metered = (out.save_data) || ((out.type_).clone() == "cellular");
                return out;
            },
        )
            as Box<dyn FnMut(ConnectivityStatus) -> ConnectivityStatus + Send + 'static>)),
        detect_reachability: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |options: ConnectivityReachabilityOptions,
                  mut out: ConnectivityReachability|
                  -> crate::Promise<crate::OpaqueHostValue> {
                {
                    out.reachable = false;
                    out.latency = (-1.0_f64);
                    return out;
                }
            },
        )
            as Box<
                dyn FnMut(
                        ConnectivityReachabilityOptions,
                        ConnectivityReachability,
                    ) -> crate::Promise<ConnectivityReachability>
                    + Send
                    + 'static,
            >))),
        subscribe: std::sync::Arc::new(std::sync::Mutex::new(
            Box::new(
                move |listener: std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                >|
                      -> std::sync::Arc<
                    std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                > {
                    return std::sync::Arc::new(std::sync::Mutex::new(
                        Box::new(move || -> () {}) as Box<dyn FnMut() -> () + Send + 'static>
                    ));
                },
            )
                as Box<
                    dyn FnMut(
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
        )),
    };
}

// Source: upstream/packages/connectivity/src/connectivity.ts:125 (sha256:8e631b9a51f9bf7392a43997e5031142b0cefe08c5cb450f715ef5c2153bdd5c)
pub fn detach_connectivity(net: &Connectivity) -> () {
    let unsubscribe = (*_SUBSCRIPTIONS.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*net).clone())
        .map(|(_, value)| value.clone());
    if (unsubscribe).is_some() {
        {
            let __flight_callback = (unsubscribe.as_ref().unwrap()).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
        {
            let __flight_key = (*net).clone();
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
    }
}

// Source: upstream/packages/connectivity/src/connectivity.ts:138 (sha256:599d1031e281368c595d7c67f9094c5325c5caf880d596a64a3c5d592178e52b)
pub fn detect_connectivity_reachability(
    options: &ConnectivityReachabilityOptions,
    out: &ConnectivityReachability,
) -> crate::Promise<ConnectivityReachability> {
    Default::default()
}

// Source: upstream/packages/connectivity/src/connectivity.ts:159 (sha256:c8bc1fc13bbea1d2ed27b9bb4106f6111e72225de6a4031a984b89cf7a7b004c)
pub fn dispose_connectivity(net: &Connectivity) -> () {
    detach_connectivity(net);
}

// Source: upstream/packages/connectivity/src/connectivity.ts:164 (sha256:3a91ce265ba2b344b69a51259830afb069535c551f13c716fb77e6bcab3040e7)
pub fn get_connectivity_backend() -> ConnectivityBackend {
    if ((*_BACKEND.lock().unwrap()).clone()).is_none() {
        (*_BACKEND.lock().unwrap()) = Some(create_web_connectivity_backend());
    }
    return (((*_BACKEND.lock().unwrap()).clone()).clone().unwrap()).clone();
}

// Source: upstream/packages/connectivity/src/connectivity.ts:170 (sha256:ce3781b01479e8700b40fb399dae490be93cd2a02165fd36dd72a39cb473a4a4)
pub fn get_connectivity_status(out: &ConnectivityStatus) -> ConnectivityStatus {
    return {
        let __flight_callback = (get_connectivity_backend().get_status).clone();
        let __flight_result = __flight_callback.lock().unwrap()((*out).clone());
        __flight_result
    };
}

// Source: upstream/packages/connectivity/src/connectivity.ts:175 (sha256:b6653bbe1b05bfaf01e74396e72ad9e5c9fad9a227c9402edae15b4f6ebf1d5a)
pub fn has_connectivity_status_changed(a: &ConnectivityStatus, b: &ConnectivityStatus) -> bool {
    return (((((((a.online != b.online) || ((a.type_).clone() != (b.type_).clone()))
        || (a.downlink != b.downlink))
        || (a.downlink_max != b.downlink_max))
        || ((a.effective_type).clone() != (b.effective_type).clone()))
        || (a.rtt != b.rtt))
        || (a.save_data != b.save_data))
        || (a.metered != b.metered);
}

// Source: upstream/packages/connectivity/src/connectivity.ts:192 (sha256:030942dfc2ecc55148b9509eec27583ac723912a461d8f2bd2f447c7bd67d14e)
pub fn is_connectivity_metered() -> bool {
    return {
        let __flight_callback = (get_connectivity_backend().get_status).clone();
        let __flight_result = __flight_callback.lock().unwrap()(((*_SCRATCH).clone()).clone());
        __flight_result
    }
    .metered;
}

// Source: upstream/packages/connectivity/src/connectivity.ts:197 (sha256:bcd4e434f72ef86120180391ec4242f7c7a2ffa2e7793ac1d2da059630781a9e)
pub fn is_connectivity_online() -> bool {
    return {
        let __flight_callback = (get_connectivity_backend().get_status).clone();
        let __flight_result = __flight_callback.lock().unwrap()(((*_SCRATCH).clone()).clone());
        __flight_result
    }
    .online;
}

// Source: upstream/packages/connectivity/src/connectivity.ts:202 (sha256:bbbcc82d37784beedc8ea82830cf9e03dbd0824f42041a2f0b395a76a4dc2a09)
pub fn is_connectivity_save_data_enabled() -> bool {
    return {
        let __flight_callback = (get_connectivity_backend().get_status).clone();
        let __flight_result = __flight_callback.lock().unwrap()(((*_SCRATCH).clone()).clone());
        __flight_result
    }
    .save_data;
}

// Source: upstream/packages/connectivity/src/connectivity.ts:207 (sha256:e8deb39aad67323215888a26c9b9248d63fa0739f2e400f2ab48d31bb5096b27)
pub fn set_connectivity_backend(backend: Option<ConnectivityBackend>) -> () {
    (*_BACKEND.lock().unwrap()) = (backend).clone();
}

// Source: upstream/packages/connectivity/src/connectivity.ts:211 (sha256:db828b1b163f0fe838fda0a498c95526e91e05e78f7f2854234f2496ca97a632)
static _BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<ConnectivityBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/connectivity/src/connectivity.ts:212 (sha256:2c012b929115ef9ddcf3ced712b89bb953a7c0c5ebc90caab640d6db82cebe54)
static _SCRATCH: std::sync::LazyLock<ConnectivityStatus> =
    std::sync::LazyLock::new(|| create_connectivity_status());

// Source: upstream/packages/connectivity/src/connectivity.ts:213 (sha256:37f6bcee33c627fe0e07182f73a47155c613de9148ee108332f00e135a2cfaee)
static _SUBSCRIPTIONS: std::sync::LazyLock<
    std::sync::Mutex<
        Vec<(
            Connectivity,
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
        )>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/connectivity/src/connectivity.ts:215 (sha256:57b25d60c9188c8253741d8093cbb24260a108bbc6827163ee285b450c3cd043)
#[derive(Clone, Default)]
struct WebConnectivityConnection {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub type_: Option<String>,
    pub downlink: Option<f64>,
    pub downlink_max: Option<f64>,
    pub effective_type: Option<String>,
    pub rtt: Option<f64>,
    pub save_data: Option<bool>,
    pub add_event_listener: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            String,
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                            >,
                        ) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub remove_event_listener: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            String,
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                            >,
                        ) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
}
impl PartialEq for WebConnectivityConnection {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/connectivity/src/connectivity.ts:227 (sha256:53a721460c566083aaff27454700f9747c180474be954b548da555f42b408a86)
#[derive(Clone, Default)]
struct AnyAbortSignalRecord1 {
    __flight_identity: std::sync::Arc<()>,
    once: bool,
}
impl PartialEq for AnyAbortSignalRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn any_abort_signal(
    a: crate::OpaqueHostValue,
    b: crate::OpaqueHostValue,
) -> crate::OpaqueHostValue {
    let controller = crate::OpaqueHostValue::Object;
    let __flight_recursive_on_abort: std::sync::Arc<
        std::sync::Mutex<
            Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
        >,
    > = std::sync::Arc::new(std::sync::Mutex::new(None));
    let mut on_abort: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let __flight_recursive_on_abort = __flight_recursive_on_abort.clone();
            move || -> () {
                crate::host_value::<()>("host.abort");
                crate::host_value::<()>("host.removeEventListener");
                crate::host_value::<()>("host.removeEventListener");
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    *__flight_recursive_on_abort.lock().unwrap() = Some(on_abort.clone());
    crate::host_value::<()>("host.addEventListener");
    crate::host_value::<()>("host.addEventListener");
    return crate::host_value::<crate::OpaqueHostValue>("host.signal");
}

// Source: upstream/packages/connectivity/src/connectivity.ts:242 (sha256:a4f0abba2e93ebb47824d54f4daff3565e3314823da8f362667cf16cc2a73457)
fn get_web_connection() -> Option<WebConnectivityConnection> {
    return None;
}

// Source: upstream/packages/connectivity/src/connectivity.ts:248 (sha256:a6774e86039c4f4e1a2580d38c398d35c65002e72d7456e6a563c49a956da701)
fn map_web_connection_type(type_: Option<String>) -> ConnectivityConnectionType {
    {
        let __switch_value = type_;
        let __flight_case = if __switch_value == "bluetooth" {
            0_usize
        } else if __switch_value == "cellular" {
            1_usize
        } else if __switch_value == "ethernet" {
            2_usize
        } else if __switch_value == "none" {
            3_usize
        } else if __switch_value == "other" {
            4_usize
        } else if __switch_value == "vpn" {
            5_usize
        } else if __switch_value == "wifi" {
            6_usize
        } else if __switch_value == "wimax" {
            7_usize
        } else {
            8_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                return "bluetooth".to_owned();
            }
            if __flight_case <= 1_usize {
                return "cellular".to_owned();
            }
            if __flight_case <= 2_usize {
                return "ethernet".to_owned();
            }
            if __flight_case <= 3_usize {
                return "none".to_owned();
            }
            if __flight_case <= 4_usize {
                return "other".to_owned();
            }
            if __flight_case <= 5_usize {
                return "vpn".to_owned();
            }
            if __flight_case <= 6_usize {
                return "wifi".to_owned();
            }
            if __flight_case <= 7_usize {
                return "wimax".to_owned();
            }
            if __flight_case <= 8_usize {
                return "unknown".to_owned();
            }
            unreachable!("exhaustive TypeScript switch completed without returning");
        }
    }
}

// Source: upstream/packages/connectivity/src/connectivity.ts:271 (sha256:14bfa601038def0fca1ab44bbfb9645464b47f8ec58f3b14f532fba4218e5087)
static _CACHED_WEB_BACKEND: std::sync::LazyLock<std::sync::Mutex<Option<ConnectivityBackend>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
