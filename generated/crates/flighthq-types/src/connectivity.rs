// @generated from upstream/packages/types/src/Connectivity.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/Connectivity.ts:3 (sha256:b9bbd32114106672af94b1a445e31e8c67f45986c74b8befb4fe833aac58903f)
pub type ConnectivityConnectionType = String;

// Source: upstream/packages/types/src/Connectivity.ts:14 (sha256:5752a05f4ce927af1550ac0ac16f6c72448f9047937cc7c5763607805eda30ac)
#[derive(Clone, Default)]
pub struct ConnectivityStatus {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub online: bool,
    pub type_: ConnectivityConnectionType,
    pub downlink: f64,
    pub downlink_max: f64,
    pub effective_type: String,
    pub rtt: f64,
    pub save_data: bool,
    pub metered: bool,
}
impl PartialEq for ConnectivityStatus {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Connectivity.ts:33 (sha256:4a5a0a696af9982bef1e5f820e58d16adc7bae99cd95ca4ab2e96f8b4102e502)
#[derive(Clone, Default)]
pub struct ConnectivityReachability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reachable: bool,
    pub latency: f64,
}
impl PartialEq for ConnectivityReachability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Connectivity.ts:40 (sha256:04eb504c78a104034fcf12550f79a5fa4e3c1cce0567add6b5870616ea27e511)
#[derive(Clone, Default)]
pub struct ConnectivityReachabilityOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub url: String,
    pub timeout: Option<f64>,
    pub signal: Option<crate::OpaqueHostValue>,
}
impl PartialEq for ConnectivityReachabilityOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Connectivity.ts:51 (sha256:84127482c4a5519250ea7aeb0cf5c4a1509933a2cd6500691c4ae6db765ca634)
#[derive(Clone)]
pub struct ConnectivityBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_status: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ConnectivityStatus) -> ConnectivityStatus + Send + 'static>>,
    >,
    pub detect_reachability: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            ConnectivityReachabilityOptions,
                            ConnectivityReachability,
                        )
                            -> crate::FlightTask<ConnectivityReachability>
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for ConnectivityBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Connectivity.ts:63 (sha256:1e7a7736565afeffa04eb8884a9de690fc2fc2f0b0a08c7c5e4ab07ee486356e)
#[derive(Clone)]
pub struct Connectivity {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_change: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ConnectivityStatus) -> () + Send + 'static>>>,
    >,
    pub on_connection_type_change: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(ConnectivityConnectionType) -> () + Send + 'static>>,
        >,
    >,
    pub on_metered_change:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) -> () + Send + 'static>>>>,
    pub on_online:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_offline:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for Connectivity {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
