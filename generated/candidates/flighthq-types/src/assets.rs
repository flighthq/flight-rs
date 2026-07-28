// @generated from upstream/packages/types/src/Assets.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/Assets.ts:14 (sha256:6850f8b1a9cb591bdd720bae70b19fc946b7e19b1b3e1afc26693097a0c7105b)
pub type AssetType = String;

// Source: upstream/packages/types/src/Assets.ts:26 (sha256:dd363886862bfd9145656d2ac0f515e1b2cb838f6ed4935611d53d851439b12d)
#[derive(Clone, Default)]
pub struct AssetDescriptor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: String,
    pub url: String,
    pub type_: AssetType,
    pub group: Option<String>,
}
impl PartialEq for AssetDescriptor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Assets.ts:35 (sha256:a5c567da30c114b281fdbda5fbeb42d18a9f7c05d55e19e175204e70c35f4158)
pub type AssetManifest = Vec<AssetDescriptor>;

// Source: upstream/packages/types/src/Assets.ts:41 (sha256:aee37f828b27f89953cecce6dba9ac83be8f721de0d5a1efc1824678bd4ae6e6)
#[derive(Clone)]
pub struct AssetLoaderAdapter<T> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub load: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(AssetDescriptor) -> crate::Promise<T> + Send + 'static>>,
    >,
    pub dispose: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(T) -> () + Send + 'static>>>,
}
impl<T> PartialEq for AssetLoaderAdapter<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Assets.ts:49 (sha256:a09ecbdccede6d33c01e404ef6a1e92b86b782f6a5059caf0c931054d57e9471)
#[derive(Clone, Default)]
pub struct AssetEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub value: crate::OpaqueHostValue,
    pub refcount: f64,
    pub load_promise: Option<crate::Promise<crate::OpaqueHostValue>>,
    pub resident: bool,
}
impl PartialEq for AssetEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Assets.ts:59 (sha256:c2975e64f20c3887f99effa54097119b86eff375aef455731671bcfdf7a28162)
#[derive(Clone, Default)]
pub struct AssetLibraryRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub adapters: Vec<(AssetType, AssetLoaderAdapter<crate::OpaqueHostValue>)>,
    pub descriptors: Vec<(String, AssetDescriptor)>,
    pub entries: Vec<(String, AssetEntry)>,
    pub groups: Vec<(String, Vec<String>)>,
}
impl PartialEq for AssetLibraryRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Assets.ts:69 (sha256:df8ec4621d734acf79462ed3e19d1ff9af10d97c7e30e0275f530f18818dcb1c)
#[derive(Clone, Default)]
pub struct AssetLibrary {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub runtime: AssetLibraryRuntime,
}
impl PartialEq for AssetLibrary {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Assets.ts:74 (sha256:facd9146857f33e81e96cc11a9688eafb1b7bc17bb7d9530f52b58af4f4c09a0)
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

// Source: upstream/packages/types/src/Assets.ts:81 (sha256:bad60d78e16ffc949d43a19f33a86531d48ca77ab22b48512322c868b6f61b71)
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
