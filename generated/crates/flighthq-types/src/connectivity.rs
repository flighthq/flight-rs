// @generated from upstream/packages/types/src/Connectivity.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Signal};

// Source: upstream/packages/types/src/Connectivity.ts:4 (sha256:b9bbd32114106672af94b1a445e31e8c67f45986c74b8befb4fe833aac58903f)
pub type ConnectivityConnectionType = String;

// Source: upstream/packages/types/src/Connectivity.ts:17 (sha256:e7e0444ebc69cb31a5ec215ccbe937c6ea19c6a0b3c634fb7af068552740de07)
#[derive(Clone, Default)]
pub struct ConnectivityStatus {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub online: Option<bool>,
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

// Source: upstream/packages/types/src/Connectivity.ts:36 (sha256:4a5a0a696af9982bef1e5f820e58d16adc7bae99cd95ca4ab2e96f8b4102e502)
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

// Source: upstream/packages/types/src/Connectivity.ts:43 (sha256:04eb504c78a104034fcf12550f79a5fa4e3c1cce0567add6b5870616ea27e511)
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

// Source: upstream/packages/types/src/Connectivity.ts:55 (sha256:36130bcbc7f8dedd52c884d9ca54da3421d5d72f13bbb22351b486eb03be8fe9)
#[derive(Clone)]
pub struct HostConnectivityStatusCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_status: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ConnectivityStatus) -> ConnectivityStatus + Send + 'static>>,
    >,
}
impl PartialEq for HostConnectivityStatusCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Connectivity.ts:59 (sha256:692751dd321662bea4c96d952d2fd288a7f6a1eeaa47842dc96af2e6c6ba0bea)
#[derive(Clone)]
pub struct HostConnectivityChangeCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub destroy: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> Option<
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostConnectivityChangeCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Connectivity.ts:67 (sha256:8d66c78a7538f7a25e5afd267dc69bf1b3199fcc96e94dab7094ae856f0dc02b)
#[derive(Clone)]
pub struct HostConnectivityReachabilityCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub detect_reachability: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        ConnectivityReachabilityOptions,
                        ConnectivityReachability,
                    ) -> crate::FlightTask<ConnectivityReachability>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostConnectivityReachabilityCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Connectivity.ts:76 (sha256:2666f7f3c34a1f14e7a24f487ffdf5d60c6511f3168a868313db802eaf9cf4d8)
#[derive(Clone)]
pub struct Connectivity {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
impl crate::FlightEntity for Connectivity {
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
