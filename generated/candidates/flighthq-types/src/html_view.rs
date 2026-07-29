// @generated from upstream/packages/types/src/HtmlView.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, ClipRegion, EntityRuntime, Kind, Material, MaterialData};

// Source: upstream/packages/types/src/HtmlView.ts:3 (sha256:bba194f0689423bf8577d59419ce7b6d25107d76feeea43c14bd296df846fd93)
#[derive(Clone, Default)]
pub struct HtmlViewData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub element: Option<crate::OpaqueHostValue>,
    pub height: f64,
    pub width: f64,
}
impl PartialEq for HtmlViewData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HtmlView.ts:9 (sha256:56cbcce7ac4a7d3949a91bbd7c6ff19e5a5db0a8763036515002d9b3cbc9c661)
pub type HtmlViewRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/HtmlView.ts:11 (sha256:384fc590d337ee16124dcab9b2c796a36cf6ebd5f72e6ecc6530a8d0851df17f)
#[derive(Clone, Default)]
pub struct HtmlView {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: HtmlViewData,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha: f64,
    pub visible: bool,
    pub blend_mode: Option<BlendMode>,
    pub clip: Option<ClipRegion>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub skew_x: f64,
    pub skew_y: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for HtmlView {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for HtmlView {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
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

// Source: upstream/packages/types/src/HtmlView.ts:15 (sha256:1b73ef42e7b0700ee6fae93abed7ad989557e38ed0fcffe229addd777e635b6f)
pub const HTML_VIEW_KIND: &'static str = "HtmlView";
