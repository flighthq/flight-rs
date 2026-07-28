// @generated from upstream/packages/types/src/Screen.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ScreenColorSpace, ScreenMode, ScreenOrientation};

// Source: upstream/packages/types/src/Screen.ts:15 (sha256:8c0ce810ee2e74aaea03bb2fbd0ee5e0c2217e6884ce71d709afffa863c7141a)
#[derive(Clone, Default)]
pub struct ScreenInfo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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

// Source: upstream/packages/types/src/Screen.ts:55 (sha256:1cc599f5359d3e14c76c56912e89bb8191f25c1314ca323f59fd3de988da9c97)
#[derive(Clone, Default)]
pub struct ScreenBackendRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for ScreenBackendRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct ScreenBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_screens: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Vec<ScreenInfo>) -> Vec<ScreenInfo> + Send + 'static>>,
    >,
    pub get_primary_screen:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ScreenInfo) -> ScreenInfo + Send + 'static>>>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub get_cursor_position: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(ScreenBackendRecord1) -> ScreenBackendRecord1 + Send + 'static>,
        >,
    >,
    pub get_modes: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(ScreenInfo, Vec<ScreenMode>) -> Vec<ScreenMode> + Send + 'static>,
            >,
        >,
    >,
}
impl PartialEq for ScreenBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
