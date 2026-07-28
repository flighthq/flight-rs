// @generated from upstream/packages/types/src/Share.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ShareFile;

// Source: upstream/packages/types/src/Share.ts:8 (sha256:899bb63cf41c77e3ac2f6eb0b8bb40a741677907ea1d9a313634713185401ef3)
#[derive(Clone)]
pub struct ShareContent {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub title: Option<String>,
    pub text: Option<String>,
    pub url: Option<String>,
    pub files: Option<Vec<ShareFile>>,
}
impl PartialEq for ShareContent {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Share.ts:18 (sha256:a0ca18e785ad84e681b4558039731aab27c833c90dae6585aae3e9336c25e4e9)
#[derive(Clone)]
pub struct ShareOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub chooser_title: Option<String>,
    pub excluded_activity_types: Option<Vec<String>>,
}
impl PartialEq for ShareOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Share.ts:28 (sha256:4ad57cec0c278f2921a223a6bdec550daf82a2c2bafa0e67d32f8f21150b757b)
#[derive(Clone)]
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

// Source: upstream/packages/types/src/Share.ts:34 (sha256:00d172569cb2b46c400978e64d87164d0b7ec57265a4771ab5c6523dc7091f8a)
#[derive(Clone)]
pub struct ShareBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub is_available: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub can_share:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ShareContent) -> bool + Send + 'static>>>,
    pub share: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(ShareContent, ShareOptions) -> crate::Promise<bool> + Send + 'static>,
        >,
    >,
    pub share_with_result: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(ShareContent, ShareOptions) -> crate::Promise<ShareResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for ShareBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
