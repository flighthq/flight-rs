// @generated from upstream/packages/types/src/Protocol.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/Protocol.ts:5 (sha256:ba077607db282b1c9f6ca90eea8515cebe470d4f5183c598f62a10cbc8ebb69a)
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

// Source: upstream/packages/types/src/Protocol.ts:13 (sha256:2d288a88ead2d63a4a3e411b5af491389dcf8c94533a2e10283bbccb3130eeb3)
#[derive(Clone)]
pub struct ProtocolHandler {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_open_url:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>>,
}
impl PartialEq for ProtocolHandler {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Protocol.ts:19 (sha256:86af5604996e632e93cc2ec18f11e0d76a6c0f3f1fd33e62174b2646b1e89d79)
#[derive(Clone)]
pub struct ProtocolBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub register: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub unregister:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub is_registered:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub get_registered_schemes:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Vec<String> + Send + 'static>>>,
    pub set_as_default:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub is_default:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub remove_as_default:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub get_launch_url:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Option<String> + Send + 'static>>>,
    pub drain_pending_urls:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Vec<String> + Send + 'static>>>,
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
impl PartialEq for ProtocolBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
