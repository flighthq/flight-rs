// @generated from upstream/packages/types/src/Device.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{DeviceCapabilities, DeviceDisplayMetrics, DeviceFormFactor};

// Source: upstream/packages/types/src/Device.ts:8 (sha256:31bfcf037e151d46ad28b7e075466f3fa81d5b3e63f429ed4920acbb3569d836)
#[derive(Clone, Default)]
pub struct DeviceInfo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub arch: String,
    pub available_memory: f64,
    pub board_name: String,
    pub color_gamut: String,
    pub cpu_cores: f64,
    pub font_scale: f64,
    pub form_factor: DeviceFormFactor,
    pub gpu_renderer: String,
    pub gpu_vendor: String,
    pub is_hdr: bool,
    pub is_jailbroken: bool,
    pub is_low_end_device: bool,
    pub is_rooted: bool,
    pub is_virtual: bool,
    pub manufacturer: String,
    pub marketing_name: String,
    pub model: String,
    pub os_build: String,
    pub os_name: String,
    pub os_version: String,
    pub platform_string: String,
    pub product_name: String,
    pub supported_abis: Vec<String>,
    pub total_memory: f64,
    pub web_view_version: String,
}
impl PartialEq for DeviceInfo {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Device.ts:39 (sha256:ab825aced7d8446357b7a42ff71ba8a55b52a120ccefd1b52fe6ee5a22b7d15b)
#[derive(Clone, Default)]
pub struct SafeAreaInsets {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}
impl PartialEq for SafeAreaInsets {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Device.ts:48 (sha256:d9234d0fef6bd22728456865cec0d55d8c5ad54e516c9d2687070fec558bb782)
#[derive(Clone)]
pub struct DeviceBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_capabilities: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(DeviceCapabilities) -> DeviceCapabilities + Send + 'static>>,
    >,
    pub get_display_metrics: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(DeviceDisplayMetrics) -> DeviceDisplayMetrics + Send + 'static>,
        >,
    >,
    pub get_id: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub get_info:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(DeviceInfo) -> DeviceInfo + Send + 'static>>>,
    pub get_safe_area_insets: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(SafeAreaInsets) -> SafeAreaInsets + Send + 'static>>,
    >,
}
impl PartialEq for DeviceBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
