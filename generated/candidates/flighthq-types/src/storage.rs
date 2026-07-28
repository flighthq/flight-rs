// @generated from upstream/packages/types/src/Storage.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/Storage.ts:7 (sha256:c33a09b6448a34364744537c372f980cfed7536946d2cd2a3bc2fb1186e03165)
#[derive(Clone)]
pub struct StorageBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_item:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> Option<String> + Send + 'static>>>,
    pub set_item:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String, String) -> bool + Send + 'static>>>,
    pub remove_item:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub clear: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub keys: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Vec<String> + Send + 'static>>>,
    pub byte_size:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>>>,
    pub subscribe_changes: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            std::sync::Arc<
                                std::sync::Mutex<
                                    Box<dyn FnMut(StorageChange) -> () + Send + 'static>,
                                >,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
            >,
        >,
    >,
}
impl PartialEq for StorageBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:22 (sha256:02a0e1a37682e44dd655893df32418d5d3b9e6f907cfd7590a289256a8b7bae8)
#[derive(Clone)]
pub struct StorageChange {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub key: Option<String>,
    pub new_value: Option<String>,
    pub old_value: Option<String>,
}
impl PartialEq for StorageChange {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:31 (sha256:96fa8963461679463dc0bfa2f65819759acab569e05f544e2a3527e40a2ea2cd)
#[derive(Clone)]
pub struct StorageMigration {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub migrate:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Option<String>) -> () + Send + 'static>>>,
    pub version: f64,
}
impl PartialEq for StorageMigration {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:37 (sha256:7a96a1d7dd351cf083f20376c7f9e44aa41c631acd3d360ed8b32b29b141895d)
#[derive(Clone)]
pub struct StorageNamespace {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub prefix: String,
}
impl PartialEq for StorageNamespace {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:42 (sha256:1d2171ddf6a3843fac1e2d252f811586bcc5cea0a4a63ff26dca808185bf3ad4)
#[derive(Clone)]
pub struct StorageQuota {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub available: f64,
    pub used: f64,
}
impl PartialEq for StorageQuota {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:48 (sha256:1fec9342f58e0a9e66498a7f10e0a57c0e229ef09ca01942053260aef72aa6c7)
#[derive(Clone)]
pub struct StorageSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_change: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(StorageChange) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for StorageSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
