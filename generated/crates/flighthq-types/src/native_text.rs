// @generated from upstream/packages/types/src/NativeText.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, ClipRegion, EntityRuntime, Kind, Material, MaterialData, TextAutoSize,
    TextFormatAlign, TextVerticalAlign,
};

// Source: upstream/packages/types/src/NativeText.ts:14 (sha256:f078ff7b5a1fba1d9be2cabd7ecea006f95793b8270319a759825bed6f622e85)
#[derive(Clone, Default)]
pub struct NativeTextStyle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub align: Option<TextFormatAlign>,
    pub bold: Option<bool>,
    pub color: Option<f64>,
    pub font: Option<String>,
    pub italic: Option<bool>,
    pub leading: Option<f64>,
    pub size: Option<f64>,
}
impl PartialEq for NativeTextStyle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/NativeText.ts:27 (sha256:5e7d4b75130bdda69787b9c27ae02b9270e3c086f66849b6aecb864787210fd6)
#[derive(Clone, Default)]
pub struct NativeTextData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub auto_size: TextAutoSize,
    pub height: f64,
    pub style: NativeTextStyle,
    pub text: String,
    pub vertical_align: TextVerticalAlign,
    pub width: f64,
}
impl PartialEq for NativeTextData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/NativeText.ts:40 (sha256:a31ec0e776fefad7311e898f90401d6895f78c493ec129f623f6512f580a2a18)
pub type NativeTextRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/NativeText.ts:52 (sha256:88013e44c2b9873292c3001ef0df176f90cb4974f368d0820db78530ca328431)
#[derive(Clone, Default)]
pub struct NativeText {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: NativeTextData,
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
impl PartialEq for NativeText {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for NativeText {
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

// Source: upstream/packages/types/src/NativeText.ts:56 (sha256:4e663a6e6e8006b1ba071a4432421e8b640bdf963a2bbc72776e4af3a563159e)
pub const NATIVE_TEXT_KIND: &'static str = "NativeText";
