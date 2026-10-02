// @generated from upstream/packages/types/src/BitmapText.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, ClipRegion, EntityRuntime, GlyphSource, Kind, Material2D, MaterialData, TextureAtlas,
};

// Source: upstream/packages/types/src/BitmapText.ts:9 (sha256:fb814b59647f58d51fc3eb28845ee76447c5c5b70f0cd7733f4325a6e519913f)
pub type BitmapTextAlign = String;

// Source: upstream/packages/types/src/BitmapText.ts:17 (sha256:c50c687cca4faf9794433738dcb9ea0e566e6cc7c2c5091a1060cff6bf1447de)
#[derive(Clone, Default)]
pub struct BitmapTextData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub align: BitmapTextAlign,
    pub ellipsis: String,
    pub glyph_source: Option<GlyphSource>,
    pub letter_spacing: f64,
    pub line_height: f64,
    pub max_lines: Option<f64>,
    pub text: String,
    pub wrap_width: Option<f64>,
}
impl PartialEq for BitmapTextData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for BitmapTextData {
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

// Source: upstream/packages/types/src/BitmapText.ts:45 (sha256:d2115dbabb239acfc6288812800f14051a58f3141d2cff821ddea724af316ba8)
#[derive(Clone, Default)]
pub struct BitmapTextPage {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub atlas: TextureAtlas,
    pub ids: Vec<u16>,
    pub instance_count: f64,
    pub transforms: Vec<f32>,
}
impl PartialEq for BitmapTextPage {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BitmapText.ts:52 (sha256:0f193bd524f785b02262756a52f0a312514878ee209638d6097c4874125b71ca)
pub type BitmapTextRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/BitmapText.ts:74 (sha256:9d0dd7c810abb4bb0279f61cd201ee7648448b026b33fd765828f8e9f89f242a)
#[derive(Clone, Default)]
pub struct BitmapText {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub data: BitmapTextData,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha: f64,
    pub visible: bool,
    pub blend_mode: Option<BlendMode>,
    pub clip: Option<ClipRegion>,
    pub material: Option<Material2D>,
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
impl PartialEq for BitmapText {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for BitmapText {
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

// Source: upstream/packages/types/src/BitmapText.ts:80 (sha256:8780ad95ee1d0640aa77f266ae1cfc56f27ab811360733551edd9a3c9540b034)
#[derive(Clone, Default)]
pub struct BitmapTextOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub align: Option<BitmapTextAlign>,
    pub ellipsis: Option<String>,
    pub letter_spacing: Option<f64>,
    pub line_height: Option<f64>,
    pub max_lines: Option<f64>,
    pub text: Option<String>,
    pub wrap_width: Option<f64>,
}
impl PartialEq for BitmapTextOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BitmapText.ts:90 (sha256:0407460c41722f239ace0d39d723460b0ccd154dcc53002ed4852bda8f9a8fcb)
pub const BITMAP_TEXT_KIND: &'static str = "BitmapText";
