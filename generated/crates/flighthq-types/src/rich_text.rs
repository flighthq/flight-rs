// @generated from upstream/packages/types/src/RichText.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, ClipRegion, EntityRuntime, Kind, Material, MaterialData, TextAutoSize, TextFormat,
    TextFormatRange, TextVerticalAlign,
};

// Source: upstream/packages/types/src/RichText.ts:9 (sha256:fa82e08e1863fcc75e3ed9619dc8585f19565703bc84971444398c1df93031eb)
#[derive(Clone, Default)]
pub struct RichTextData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub auto_size: TextAutoSize,
    pub height: f64,
    pub text: String,
    pub text_format: TextFormat,
    pub vertical_align: TextVerticalAlign,
    pub width: f64,
    pub background: bool,
    pub background_color: f64,
    pub border: bool,
    pub border_color: f64,
    pub condense_white: bool,
    pub default_text_format: TextFormat,
    pub max_chars: f64,
    pub mouse_wheel_enabled: bool,
    pub multiline: bool,
    pub scroll_h: f64,
    pub scroll_v: f64,
    pub selectable: bool,
    pub text_color: f64,
    pub text_format_ranges: Vec<TextFormatRange>,
    pub word_wrap: bool,
}
impl PartialEq for RichTextData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RichText.ts:47 (sha256:8366b22af6581d9b3d860205d8d5245e7bb40398342313332aa3c7da2e420aa1)
pub type RichTextRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/RichText.ts:62 (sha256:ede1beea3240687757ee8455992b246d3497476a47de43d9b8e5d02d8b73abe7)
#[derive(Clone, Default)]
pub struct RichText {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: RichTextData,
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
impl PartialEq for RichText {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RichText {
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

// Source: upstream/packages/types/src/RichText.ts:66 (sha256:596b8a1b265ecce1ee0865dbb2e71192fc576e385865362468b050f38fe00952)
pub const RICH_TEXT_KIND: &'static str = "RichText";
