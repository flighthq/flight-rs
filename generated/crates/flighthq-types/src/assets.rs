// @generated from upstream/packages/types/src/Assets.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Signal};

// Source: upstream/packages/types/src/Assets.ts:15 (sha256:6850f8b1a9cb591bdd720bae70b19fc946b7e19b1b3e1afc26693097a0c7105b)
pub type AssetType = String;

// Source: upstream/packages/types/src/Assets.ts:29 (sha256:80321952d70f70c18603fb043429354c5e01095a5ad39f115adb64997ea9e798)
#[derive(Clone, Default)]
pub struct AssetDescriptor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: String,
    pub url: String,
    pub type_: AssetType,
    pub groups: Option<Vec<String>>,
}
impl PartialEq for AssetDescriptor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Assets.ts:38 (sha256:a5c567da30c114b281fdbda5fbeb42d18a9f7c05d55e19e175204e70c35f4158)
pub type AssetManifest = Vec<AssetDescriptor>;

// Source: upstream/packages/types/src/Assets.ts:44 (sha256:aee37f828b27f89953cecce6dba9ac83be8f721de0d5a1efc1824678bd4ae6e6)
#[derive(Clone)]
pub struct AssetLoaderAdapter<T = crate::FlightValue> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub load: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(AssetDescriptor) -> crate::FlightTask<T> + Send + 'static>>,
    >,
    pub dispose: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(T) -> () + Send + 'static>>>,
}
impl<T> PartialEq for AssetLoaderAdapter<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Assets.ts:52 (sha256:a09ecbdccede6d33c01e404ef6a1e92b86b782f6a5059caf0c931054d57e9471)
#[derive(Clone, Default)]
pub struct AssetEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub value: crate::FlightValue,
    pub refcount: f64,
    pub load_promise: Option<crate::FlightTask<crate::FlightValue>>,
    pub resident: bool,
}
impl PartialEq for AssetEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Assets.ts:62 (sha256:4d1c67ddfa9bf2e4a0fc94bf6281cf2afaa7e8fd39f271d0431c5dbb8b604d99)
#[derive(Clone, Default)]
pub struct AssetLibraryRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub acquire_guard: Option<AssetAcquireGuard>,
    pub adapters: Vec<(AssetType, AssetLoaderAdapter<crate::OpaqueHostValue>)>,
    pub descriptors: Vec<(String, AssetDescriptor)>,
    pub entries: Vec<(String, AssetEntry)>,
    pub freed_ids: Vec<String>,
    pub groups: Vec<(String, Vec<String>)>,
}
impl PartialEq for AssetLibraryRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Assets.ts:74 (sha256:58c9483a88bdac8b21c2d324a2c4644f7afe91caf8a33fa807f008f2abe392da)
#[derive(Clone, Default)]
pub struct AssetLibrary {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub runtime: AssetLibraryRuntime,
}
impl PartialEq for AssetLibrary {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AssetLibrary {
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

// Source: upstream/packages/types/src/Assets.ts:80 (sha256:f0058fa729f46f3cc9962a21dc7bb5ebbf86529e167fb307074a94ccb3bcc201)
#[derive(Clone, Default)]
pub struct AssetLoadExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: String,
    pub ref_count: f64,
    pub status: String,
    pub type_: Option<AssetType>,
}
impl PartialEq for AssetLoadExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Assets.ts:89 (sha256:8e8f233b81481f462f8ae5141a73e6f44d197846f6fd74133a7dd1684d2b1a60)
pub type AssetAcquireGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(AssetLibrary, AssetLoadExplanation) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Assets.ts:92 (sha256:facd9146857f33e81e96cc11a9688eafb1b7bc17bb7d9530f52b58af4f4c09a0)
#[derive(Clone, Default)]
pub struct AssetLoadProgress {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loaded: f64,
    pub total: f64,
}
impl PartialEq for AssetLoadProgress {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Assets.ts:99 (sha256:bad60d78e16ffc949d43a19f33a86531d48ca77ab22b48512322c868b6f61b71)
#[derive(Clone, Default)]
pub struct AssetGroupLoadOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub progress: Option<
        Signal<
            std::sync::Arc<
                std::sync::Mutex<Box<dyn FnMut(AssetLoadProgress) -> () + Send + 'static>>,
            >,
        >,
    >,
}
impl PartialEq for AssetGroupLoadOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
