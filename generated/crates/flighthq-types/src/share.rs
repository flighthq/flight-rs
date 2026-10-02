// @generated from upstream/packages/types/src/Share.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ShareFile;

// Source: upstream/packages/types/src/Share.ts:6 (sha256:10ece21b8f43662d85d80ba68b48e0a4ce28df0fd03f91fa5b20572581cd26d3)
#[derive(Clone, Default)]
pub struct ShareContent {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub text: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
}
impl PartialEq for ShareContent {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Share.ts:11 (sha256:8aa3a36dd9a6b06700ee04c888432414a08e48566695911f68c74326dfdb20fc)
#[derive(Clone, Default)]
pub struct ShareFilesContent {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub files: Vec<ShareFile>,
    pub text: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
}
impl PartialEq for ShareFilesContent {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Share.ts:21 (sha256:4ad57cec0c278f2921a223a6bdec550daf82a2c2bafa0e67d32f8f21150b757b)
#[derive(Clone, Default)]
pub struct ShareResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub completed: bool,
    pub activity_type: Option<String>,
    pub dismissed: bool,
}
impl PartialEq for ShareResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Share.ts:27 (sha256:a43cb8ae84771b3c102dfaf65a9f53eca764350a8782d0c2da46c78430c696e3)
#[derive(Clone)]
pub struct HostShareContentCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub can_share_content:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ShareContent) -> bool + Send + 'static>>>,
    pub share_content: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ShareContent) -> crate::FlightTask<bool> + Send + 'static>>,
    >,
    pub share_content_with_result: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(ShareContent) -> crate::FlightTask<ShareResult> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostShareContentCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Share.ts:33 (sha256:e5709824730b6824978c8ca8cef9c8eeade23a034faac5689af6292c39b5b96d)
#[derive(Clone)]
pub struct HostShareFilesCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub can_share_content: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ShareFilesContent) -> bool + Send + 'static>>,
    >,
    pub share_content: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(ShareFilesContent) -> crate::FlightTask<bool> + Send + 'static>,
        >,
    >,
    pub share_content_with_result: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(ShareFilesContent) -> crate::FlightTask<ShareResult> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostShareFilesCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Share.ts:41 (sha256:d66a7565c7e68caf029d56bb5405ac0be4d9ba98f2a56ca741247a0eeaa17359)
#[derive(Clone, Default)]
pub struct CapacitorShareContentOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub chooser_title: Option<String>,
}
impl PartialEq for CapacitorShareContentOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Share.ts:45 (sha256:f7183e556526e1bfbc3143a44e9cc418ed027d3ebece443285e1004b7d1fec8b)
#[derive(Clone)]
pub struct HostCapacitorShareContentCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub can_share_content:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ShareContent) -> bool + Send + 'static>>>,
    pub share_content: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        ShareContent,
                        Option<CapacitorShareContentOptions>,
                    ) -> crate::FlightTask<bool>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub share_content_with_result: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        ShareContent,
                        Option<CapacitorShareContentOptions>,
                    ) -> crate::FlightTask<ShareResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostCapacitorShareContentCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
