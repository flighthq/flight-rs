// @generated from upstream/packages/types/src/Protocol.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Signal};

// Source: upstream/packages/types/src/Protocol.ts:6 (sha256:ba077607db282b1c9f6ca90eea8515cebe470d4f5183c598f62a10cbc8ebb69a)
#[derive(Clone, Default)]
pub struct ParsedProtocolUrl {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub scheme: String,
    pub host: String,
    pub path: String,
    pub query: Vec<(String, String)>,
}
impl PartialEq for ParsedProtocolUrl {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Protocol.ts:14 (sha256:24a350ac7241782934d614404f082162f7030c3e4080b5e90ab272b5d1717189)
#[derive(Clone)]
pub struct ProtocolHandler {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_open_url:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>>,
}
impl PartialEq for ProtocolHandler {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ProtocolHandler {
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

// Source: upstream/packages/types/src/Protocol.ts:18 (sha256:e0aceb9b9f250a14d3b973accf9a87505d9c7553bc400664325b5fa6cce4c716)
#[derive(Clone)]
pub struct HostProtocolDefaultCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub is_default:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub remove_as_default:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub set_as_default:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
}
impl PartialEq for HostProtocolDefaultCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Protocol.ts:24 (sha256:b5b6c2f1ead7825c501619caa18b5c357eb5861262ff8236dc13446b637ffebc)
#[derive(Clone)]
pub struct HostProtocolLaunchCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_launch_url:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Option<String> + Send + 'static>>>,
}
impl PartialEq for HostProtocolLaunchCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Protocol.ts:28 (sha256:26cc59517daf88d277a68b59b5be002a6c4f3c9fd3badb59b0f607811d041c4f)
#[derive(Clone)]
pub struct HostProtocolOpenCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostProtocolOpenCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Protocol.ts:32 (sha256:3f930af7f6f6bbfd29ca32a40b49c09cf20a0e0ce72e679c8a83a5fdbea5091c)
#[derive(Clone)]
pub struct HostProtocolRegistrationCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_registered_schemes:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Vec<String> + Send + 'static>>>,
    pub register: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
}
impl PartialEq for HostProtocolRegistrationCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Protocol.ts:37 (sha256:165568affbe994ef0cbd9d0635809d231ed8453ee7f6db60f14ee8a4fb197268)
#[derive(Clone)]
pub struct HostProtocolRegistrationQueryCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub is_registered:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
}
impl PartialEq for HostProtocolRegistrationQueryCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Protocol.ts:41 (sha256:6129b38452ddd17446e27112d4c9b889a199a1e4484204f82078469b3fb9ab25)
#[derive(Clone)]
pub struct HostProtocolUnregistrationCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub unregister:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
}
impl PartialEq for HostProtocolUnregistrationCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
