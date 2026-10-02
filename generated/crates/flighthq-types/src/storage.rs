// @generated from upstream/packages/types/src/Storage.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, PermissionState, Signal};

// Source: upstream/packages/types/src/Storage.ts:5 (sha256:de0bdae5a38188320ffb556cf1b0b1e9bcb729bcdfdb6b88bdb2e7a56715ea5d)
pub type StorageClearFailureReason = String;

// Source: upstream/packages/types/src/Storage.ts:7 (sha256:b1eee40c1ff95cc3c31356ed6db6aa7143e1bcdc19ffdc47322d224789e6f85a)
pub type StorageGetItemFailureReason = String;

// Source: upstream/packages/types/src/Storage.ts:13 (sha256:487fa96192930dd0e7c485093ccbc1c7cd8c9a050ee65cbad80df4c9b5290dec)
pub type StorageKeysFailureReason = StorageGetItemFailureReason;

// Source: upstream/packages/types/src/Storage.ts:15 (sha256:a0dc94c21e2648ecc778ba45f8a319803e33543efebf4279d8fdc2001431a986)
pub type StorageRemoveItemFailureReason = crate::FlightUnion2<StorageGetItemFailureReason, String>;

// Source: upstream/packages/types/src/Storage.ts:17 (sha256:67fffbc8e28c09e1a0c8e2ebe0b8ae8f3a80ff5dac6d35590fba3e331b1e04d4)
pub type StorageSetItemFailureReason = crate::FlightUnion2<StorageGetItemFailureReason, String>;

// Source: upstream/packages/types/src/Storage.ts:19 (sha256:09276adf93d8623513cdf4bb6da33776efcc38237cc95963a1a90b352e19fd0d)
pub type StorageReadFailureReason = crate::FlightUnion2<StorageGetItemFailureReason, String>;

// Source: upstream/packages/types/src/Storage.ts:20 (sha256:5dfd43d84243df71855151ba345615129e4c867d6d60e83d5c7522cd4aa13107)
pub type StorageWriteFailureReason = crate::FlightUnion2<StorageSetItemFailureReason, String>;

// Source: upstream/packages/types/src/Storage.ts:25 (sha256:6fc39a5868fbaa479b0b78217a91a811f3857eb853205e840cd879109056e134)
#[derive(Clone)]
pub struct StorageMutationOutcome<FailureReason> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: crate::FlightUnion2<String, FailureReason>,
}
impl<FailureReason> PartialEq for StorageMutationOutcome<FailureReason> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:29 (sha256:ff11fea079d4c47a4c017626ffb0b2c8b151f504bf3d37c4d785eac083ec26f9)
pub type StorageClearResult = StorageMutationOutcome<StorageClearFailureReason>;

// Source: upstream/packages/types/src/Storage.ts:30 (sha256:8e0225651e4742be606a8f4f2f53c8342da25e0f028ef7d356413d59def2516c)
pub type StorageRemoveItemResult = StorageMutationOutcome<StorageRemoveItemFailureReason>;

// Source: upstream/packages/types/src/Storage.ts:31 (sha256:9bbd3721e90af4f411dd18f1c9bba5d50bb68e90f76011efc9be4f9b0bffb454)
pub type StorageSetItemResult = StorageMutationOutcome<StorageSetItemFailureReason>;

// Source: upstream/packages/types/src/Storage.ts:32 (sha256:ebed89c216a71f49e0b19233295e2f86c991f05752474ca3fa73d128b2beff94)
pub type StorageJsonWriteResult = StorageMutationOutcome<StorageWriteFailureReason>;

// Source: upstream/packages/types/src/Storage.ts:36 (sha256:216c4c3f9021507e2ab33351781fbfcdad943bc115fa1733e0a23d93f6f64acb)
#[derive(Clone)]
pub struct StorageValueOutcome<Value, FailureReason> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: crate::FlightUnion2<String, FailureReason>,
    pub value: crate::FlightUnion2<Value, crate::OpaqueHostValue>,
}
impl<Value, FailureReason> PartialEq for StorageValueOutcome<Value, FailureReason> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:40 (sha256:8d40d3384aa039d9f4b6b58173d70a8c51429feb5e8880810c9ad6911c11d33a)
pub type StorageGetItemResult = StorageValueOutcome<Option<String>, StorageGetItemFailureReason>;

// Source: upstream/packages/types/src/Storage.ts:41 (sha256:f040959128d27d57de1eb7d47050a8334af6cc5fa0f9a15358cb16373971f40c)
pub type StorageKeysResult = StorageValueOutcome<Vec<String>, StorageKeysFailureReason>;

// Source: upstream/packages/types/src/Storage.ts:42 (sha256:50b816f20fee892564d8e00ba0c65f8c5f6b7292352395a9f8634b7154470f42)
pub type StorageBooleanResult = StorageValueOutcome<Option<bool>, StorageReadFailureReason>;

// Source: upstream/packages/types/src/Storage.ts:43 (sha256:8b780e33cda23049db467e60ea6af9a358591adc9023d83dc4f9ec54772218ef)
pub type StorageItemCountResult = StorageValueOutcome<f64, StorageKeysFailureReason>;

// Source: upstream/packages/types/src/Storage.ts:44 (sha256:629ab97e6c0accc23135c845031f4f1f7f77023ca0fcfb826f583c9126720c68)
pub type StorageItemOrResult = StorageValueOutcome<String, StorageGetItemFailureReason>;

// Source: upstream/packages/types/src/Storage.ts:45 (sha256:479c2a01c7f1745b470cc4e70274c6f1a819d44e9020ae04edb113c906e8e1bd)
pub type StorageNumberResult = StorageValueOutcome<Option<f64>, StorageReadFailureReason>;

// Source: upstream/packages/types/src/Storage.ts:46 (sha256:669978962c2238956b0c67090cd0a65f2ff06c011361526aa7f8879a96114249)
pub type StoragePresenceResult = StorageValueOutcome<bool, StorageGetItemFailureReason>;

// Source: upstream/packages/types/src/Storage.ts:50 (sha256:fe0fcb639ee0e9a23cbe4f487a79ff636e9f1c64a8ac999befc4d2baec2df14b)
#[derive(Clone)]
pub struct StorageFallbackOutcome<Value> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: crate::FlightUnion2<String, StorageGetItemFailureReason>,
    pub value: crate::FlightUnion2<Value, crate::OpaqueHostValue>,
}
impl<Value> PartialEq for StorageFallbackOutcome<Value> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:54 (sha256:9c71f00473a3dec364e82438b344a1487dd6e155d9db4fe6f8365ee51aa876f5)
pub type StorageBooleanOrResult = StorageFallbackOutcome<bool>;

// Source: upstream/packages/types/src/Storage.ts:55 (sha256:2a6027669d2b23fe4b36e0587ebed993acfb8eecce19babca0ea660b750739e6)
pub type StorageJsonOrResult<Value> = StorageFallbackOutcome<Option<Value>>;

// Source: upstream/packages/types/src/Storage.ts:56 (sha256:fbfcc82c98a5edadf9b6d56f72b0b31af035d3b4ba30ccf828fb383ac7fb58c0)
pub type StorageNumberOrResult = StorageFallbackOutcome<f64>;

// Source: upstream/packages/types/src/Storage.ts:58 (sha256:05023de56497ae02b7e942017aa242b0d6aa551f833cab6f13e81c635a2090cb)
pub type StorageJsonResult<Value> = StorageValueOutcome<Option<Value>, StorageReadFailureReason>;

// Source: upstream/packages/types/src/Storage.ts:63 (sha256:a8e9f6960f8353394e996941c0b6af32e3405ae5fbba3f91ca16f8d962f4b56a)
#[derive(Clone, Default)]
pub struct StoragePersistenceOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
    pub permission_state: Option<PermissionState>,
}
impl PartialEq for StoragePersistenceOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:68 (sha256:d9246bcb23eadc517dbfa49edf600813c8ae5e18f54cc121969041c1ed0b788f)
#[derive(Clone)]
pub struct HostPreferencesPersistenceQueryCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_persistence: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<StoragePersistenceOutcome> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostPreferencesPersistenceQueryCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:72 (sha256:08d813c4b495d21ae6197385e12ecadb40968642e00181bbb88b47d48f94d08a)
#[derive(Clone)]
pub struct HostPreferencesPersistenceRequestCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub request_persistence: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<StoragePersistenceOutcome> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostPreferencesPersistenceRequestCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:78 (sha256:0969fd5f311c3d815e3139d6692b564ae2e2c73b1b07e552fdede86e46aed180)
#[derive(Clone)]
pub struct WebWorkerStoragePersistenceApi {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_permission_state: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<PermissionState> + Send + 'static>>,
    >,
    pub persisted: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<bool> + Send + 'static>>,
    >,
}
impl PartialEq for WebWorkerStoragePersistenceApi {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:83 (sha256:6fc02cb40eb378546901f5fa3ed86971b36a72c48b16b1d2e5ad29ecee639553)
#[derive(Clone)]
pub struct WebWindowStoragePersistenceApi {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_permission_state: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<PermissionState> + Send + 'static>>,
    >,
    pub persisted: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<bool> + Send + 'static>>,
    >,
    pub persist: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<bool> + Send + 'static>>,
    >,
}
impl PartialEq for WebWindowStoragePersistenceApi {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:87 (sha256:b27f5681c267f522773a69f32960923c649cfd45ddcd870a5bc734677902fc51)
#[derive(Clone)]
pub struct WebWorkerStoragePersistenceCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub persistence_query: HostPreferencesPersistenceQueryCapability,
}
impl PartialEq for WebWorkerStoragePersistenceCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:91 (sha256:6b22546f8cce394a90b56774e335a0b8ccc0051673ed36eb3af11bc03c37d352)
#[derive(Clone)]
pub struct WebWindowStoragePersistenceCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub persistence_query: HostPreferencesPersistenceQueryCapability,
    pub persistence_request: HostPreferencesPersistenceRequestCapability,
}
impl PartialEq for WebWindowStoragePersistenceCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:99 (sha256:860132ea319aeb13f95c27844a0c1d64a19ac915763d2f1336f8747fd262a214)
#[derive(Clone)]
pub struct StorageQueryOutcome<Value> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub failed_key: crate::FlightUnion2<crate::OpaqueHostValue, Option<String>>,
    pub reason: crate::FlightUnion2<String, StorageGetItemFailureReason>,
    pub value: crate::FlightUnion2<Value, crate::OpaqueHostValue>,
}
impl<Value> PartialEq for StorageQueryOutcome<Value> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:107 (sha256:27f147850df6f4e8d48e45dba2f1e697ba0bf62208fe841d265c56c2af70eb57)
pub type StorageByteSizeResult = StorageQueryOutcome<f64>;

// Source: upstream/packages/types/src/Storage.ts:108 (sha256:f87fea9131d3a6d49cc5c312425d893d2cf7514656ded9eec55a797a4e8bb001)
pub type StorageEntriesResult = StorageQueryOutcome<Vec<Vec<String>>>;

// Source: upstream/packages/types/src/Storage.ts:109 (sha256:bd23bd3ec766211ff1a97f16a9b09ae9ffa465896eaaac456b300738e547c690)
pub type StorageItemsResult = StorageQueryOutcome<Vec<Option<String>>>;

// Source: upstream/packages/types/src/Storage.ts:113 (sha256:41287b101fd93117b578e0b36c4629436294d297614faf686d7c5018e4d25dcb)
#[derive(Clone)]
pub struct StorageBatchMutationOutcome<FailureReason> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub completed: f64,
    pub failed_key: crate::FlightUnion2<crate::OpaqueHostValue, Option<String>>,
    pub reason: crate::FlightUnion2<String, FailureReason>,
}
impl<FailureReason> PartialEq for StorageBatchMutationOutcome<FailureReason> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:117 (sha256:9fff8618754697115fb2f60bdea17eab4257d4736ca4c1a0dfd92b8fd49c76ab)
pub type StorageClearNamespaceResult = StorageBatchMutationOutcome<
    crate::FlightUnion2<StorageKeysFailureReason, StorageRemoveItemFailureReason>,
>;

// Source: upstream/packages/types/src/Storage.ts:120 (sha256:b3a852d95fbd507ea8c8e5db31a0f56649c6695a97549aa1f5b19ecd3649ba48)
pub type StorageRemoveItemsResult = StorageBatchMutationOutcome<StorageRemoveItemFailureReason>;

// Source: upstream/packages/types/src/Storage.ts:121 (sha256:53d7cc3299be8ce01f5ba872944bc104bf6d57142dcc122bf98c2775c0a45c50)
pub type StorageSetItemsResult = StorageBatchMutationOutcome<StorageSetItemFailureReason>;

// Source: upstream/packages/types/src/Storage.ts:125 (sha256:4fbf9dd83dcdf190717ff6bda0f7278c8c58809c395bf409bd79542a50f31afd)
#[derive(Clone)]
pub struct HostPreferencesCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub clear:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> StorageClearResult + Send + 'static>>>,
    pub get_item: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> StorageGetItemResult + Send + 'static>>,
    >,
    pub keys:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> StorageKeysResult + Send + 'static>>>,
    pub remove_item: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> StorageRemoveItemResult + Send + 'static>>,
    >,
    pub set_item: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String, String) -> StorageSetItemResult + Send + 'static>>,
    >,
}
impl PartialEq for HostPreferencesCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:135 (sha256:ee76c80959a5d061ce20328398f3f9b4d88f8a7843853b05621be3374b862b99)
#[derive(Clone)]
pub struct HostPreferencesChangeCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub destroy: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(StorageChange) -> () + Send + 'static>>,
                        >,
                    ) -> Option<
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostPreferencesChangeCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:141 (sha256:02a0e1a37682e44dd655893df32418d5d3b9e6f907cfd7590a289256a8b7bae8)
#[derive(Clone, Default)]
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

// Source: upstream/packages/types/src/Storage.ts:150 (sha256:96fa8963461679463dc0bfa2f65819759acab569e05f544e2a3527e40a2ea2cd)
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

// Source: upstream/packages/types/src/Storage.ts:155 (sha256:26690eb1956bde7bf2a97e47d8f0b1dec4dc2e98d2cae17db77de95fbfe4091b)
#[derive(Clone)]
pub struct StorageMigrationOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub failed_version: crate::FlightUnion2<crate::OpaqueHostValue, f64>,
    pub reason: crate::FlightUnion2<
        String,
        crate::FlightUnion2<StorageGetItemFailureReason, StorageSetItemFailureReason>,
    >,
    pub stage: crate::FlightUnion2<crate::OpaqueHostValue, String>,
    pub version: crate::FlightUnion2<f64, crate::OpaqueHostValue>,
}
impl PartialEq for StorageMigrationOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Storage.ts:172 (sha256:7a96a1d7dd351cf083f20376c7f9e44aa41c631acd3d360ed8b32b29b141895d)
#[derive(Clone, Default)]
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

// Source: upstream/packages/types/src/Storage.ts:178 (sha256:8bb9d312a4378c25e4aeb9260b00575705535bb5a2eae1fd913f8e182f2eccf9)
#[derive(Clone)]
pub struct StorageSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_change: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(StorageChange) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for StorageSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for StorageSignals {
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
