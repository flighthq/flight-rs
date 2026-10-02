// @generated from upstream/packages/types/src/Screen.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, ScreenChangeEvent, ScreenColorSpace, ScreenOrientation};

// Source: upstream/packages/types/src/Screen.ts:15 (sha256:501f98d2dddd94b2cd03f04b737bfcd654be52f1fc55eabe9dce02ceec2dd453)
#[derive(Clone, Default)]
pub struct ScreenInfo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub id: f64,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub work_width: f64,
    pub work_height: f64,
    pub scale_factor: f64,
    pub is_primary: bool,
    pub rotation: f64,
    pub orientation: ScreenOrientation,
    pub refresh_rate: f64,
    pub color_depth: f64,
    pub pixel_depth: f64,
    pub physical_width: f64,
    pub physical_height: f64,
    pub is_hdr: bool,
    pub color_space: ScreenColorSpace,
    pub max_luminance: f64,
    pub depth_per_component: f64,
    pub dpi: f64,
    pub label: String,
    pub internal: bool,
    pub touch_support: String,
    pub monochrome: bool,
}
impl PartialEq for ScreenInfo {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ScreenInfo {
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

// Source: upstream/packages/types/src/Screen.ts:58 (sha256:7859eb4eac8cd7a442764c5102b4e28e98c894fea3b72e45c2936f17e5f749ae)
pub type ScreenPermissionState = String;

// Source: upstream/packages/types/src/Screen.ts:60 (sha256:709f363f009ebed590aa6af8098257b377b60f9f80b4d406f7dfc851b205bd3d)
#[derive(Clone, Default)]
pub struct HostScreenQueryCapabilityRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for HostScreenQueryCapabilityRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct HostScreenQueryCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub destroy: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub get_screens: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Vec<ScreenInfo>) -> Vec<ScreenInfo> + Send + 'static>>,
    >,
    pub get_primary_screen:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ScreenInfo) -> ScreenInfo + Send + 'static>>>,
    pub get_cursor_position: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(HostScreenQueryCapabilityRecord1) -> HostScreenQueryCapabilityRecord1
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostScreenQueryCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Screen.ts:67 (sha256:7279e4c4e615af81656740fd14c0c800d8cf02b3448308eb5ebf390c2c2f2752)
#[derive(Clone)]
pub struct HostScreenChangeCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(ScreenChangeEvent) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostScreenChangeCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Screen.ts:71 (sha256:ad3c1ef8b547fbfbcca563001115b79fa613ede390fb5165ff7773ae14d8618b)
#[derive(Clone)]
pub struct HostScreenDetailsCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub query_permission: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<ScreenPermissionState> + Send + 'static>,
        >,
    >,
    pub request: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<bool> + Send + 'static>>,
    >,
}
impl PartialEq for HostScreenDetailsCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Screen.ts:76 (sha256:33892de5032accb1b5665c389cdb1c2321db01c62e8060cd1fbcbfc6df0370d7)
#[derive(Clone)]
pub struct HostScreenPermissionChangeCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(ScreenPermissionState) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostScreenPermissionChangeCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
