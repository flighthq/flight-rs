// @generated from upstream/packages/types/src/HtmlView.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, ClipRegion, EntityRuntime, Kind, Material, MaterialData};

// Source: upstream/packages/types/src/HtmlView.ts:3 (sha256:1f958ef194404e34ddb923ccea83436361f131efc7aceca837b5870b44dd87c7)
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

// Source: upstream/packages/types/src/HtmlView.ts:9 (sha256:de2bc381cbd1d3922360a6a22f4ec3807c4718fa3f359162f20f14d639fd426f)
pub type HtmlViewRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/HtmlView.ts:11 (sha256:adf46dafa5053a528e9580cf1a9889196170d6dda66b663119ffac11fb3b64a4)
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
