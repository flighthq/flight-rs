// @generated from upstream/packages/types/src/DeviceDisplayMetrics.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/DeviceDisplayMetrics.ts:4 (sha256:fd5b657fabf51d428cdc82826b34ec7bec63021ebe69311aa6204fa508c6be76)
#[derive(Clone, Default)]
pub struct DeviceDisplayMetrics {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color_depth: f64,
    pub density_dpi: f64,
    pub logical_height: f64,
    pub logical_width: f64,
    pub physical_height: f64,
    pub physical_width: f64,
    pub pixel_ratio: f64,
}
impl PartialEq for DeviceDisplayMetrics {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
