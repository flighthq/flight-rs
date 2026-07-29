// @generated from upstream/packages/types/src/TextLabel.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, ClipRegion, EntityRuntime, Kind, Material, MaterialData, TextAutoSize, TextFormat,
    TextVerticalAlign,
};

// Source: upstream/packages/types/src/TextLabel.ts:7 (sha256:3f6682b750d0898585b3453f5d4b9d32cacb0721f0e3e4ce087020840c0ef768)
#[derive(Clone, Default)]
pub struct TextLabelData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub auto_size: TextAutoSize,
    pub height: f64,
    pub text: String,
    pub text_format: TextFormat,
    pub vertical_align: TextVerticalAlign,
    pub width: f64,
}
impl PartialEq for TextLabelData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextLabel.ts:18 (sha256:6795acda2aa36d88e003498810c2d72cf123d5046fbeab982770ec4810321cc6)
pub type TextLabelRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/TextLabel.ts:31 (sha256:6e6f193698f819105eae685200bdded28eb20248352584c28b2b560109d65e09)
#[derive(Clone, Default)]
pub struct TextLabel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: TextLabelData,
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
impl PartialEq for TextLabel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for TextLabel {
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

// Source: upstream/packages/types/src/TextLabel.ts:35 (sha256:800f6cacad6f11058247fb3a2fe6ad16ab7f1ea94d9d9a986dfb4599742ef0e2)
pub const TEXT_LABEL_KIND: &'static str = "TextLabel";
