// @generated from upstream/packages/types/src/StatusBar.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/StatusBar.ts:6 (sha256:1ada9f3d5cd5e30174d9676172afc07caf571b4891f7439a987722af3adaf398)
pub type StatusBarStyle = String;

// Source: upstream/packages/types/src/StatusBar.ts:10 (sha256:06190a39d8776df1d3d892dd1d1b6440fc7dbb71bc4889a3bb37a65932138cab)
pub type StatusBarAnimation = String;

// Source: upstream/packages/types/src/StatusBar.ts:14 (sha256:39a60095237c9cfdaeb731229049618a8b0c81e14de71e4a10e547ee1622284e)
pub type StatusBarStyleEntryHandle = f64;

// Source: upstream/packages/types/src/StatusBar.ts:18 (sha256:baf105696afc3a95b25b0bb94d2213bb3e4ae1f2e12440ec8fe44751dc5f0ce3)
#[derive(Clone, Default)]
pub struct StatusBarInfo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color: f64,
    pub height: f64,
    pub overlays_content: bool,
    pub style: StatusBarStyle,
    pub visible: bool,
}
impl PartialEq for StatusBarInfo {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StatusBar.ts:31 (sha256:cb0442851b0b549082926c288dde8cd8d17ba423dbd107e50192db91817e5273)
#[derive(Clone, Default)]
pub struct StatusBarStyleEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub animation: Option<StatusBarAnimation>,
    pub color: Option<f64>,
    pub overlays_content: Option<bool>,
    pub style: Option<StatusBarStyle>,
    pub visible: Option<bool>,
}
impl PartialEq for StatusBarStyleEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StatusBar.ts:40 (sha256:de21ea49407b9e2dc14f811e96e04ae81d77fdc6684bd6419229c6ef01a738e0)
#[derive(Clone)]
pub struct StatusBarBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_info: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(StatusBarInfo) -> StatusBarInfo + Send + 'static>>,
    >,
    pub set_background_color:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, Option<bool>) -> () + Send + 'static>>>,
    pub set_overlays_content:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) -> () + Send + 'static>>>,
    pub set_style:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(StatusBarStyle) -> () + Send + 'static>>>,
    pub set_visible: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(bool, Option<StatusBarAnimation>) -> () + Send + 'static>>,
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
impl PartialEq for StatusBarBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/StatusBar.ts:54 (sha256:61f104c2472f2b968a4181a566b648aa3abaa0a5b77c24a976f91f2096b07a8f)
#[derive(Clone)]
pub struct StatusBar {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_change: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(StatusBarInfo) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for StatusBar {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
