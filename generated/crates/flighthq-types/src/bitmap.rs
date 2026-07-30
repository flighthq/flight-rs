// @generated from upstream/packages/types/src/Bitmap.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, ClipRegion, EntityRuntime, ImageResource, Kind, Material, MaterialData, Rectangle,
};

// Source: upstream/packages/types/src/Bitmap.ts:5 (sha256:f06281ca6690cce0de80eaf20890025a25b951bbeeb25d8536e41672b6d41da8)
#[derive(Clone, Default)]
pub struct BitmapData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub image: Option<ImageResource>,
    pub smoothing: bool,
    pub source_rectangle: Option<Rectangle>,
}
impl PartialEq for BitmapData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Bitmap.ts:11 (sha256:8a4932663a2d96bc8c292e247b29112eb163e8dad91ad18c4e8c10532d0b8231)
pub type BitmapRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/Bitmap.ts:13 (sha256:2a199713d5d6252c439749551560f64cc8d2917a884a5a78617058dd08cb7e60)
#[derive(Clone, Default)]
pub struct Bitmap {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: BitmapData,
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
impl PartialEq for Bitmap {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Bitmap {
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

// Source: upstream/packages/types/src/Bitmap.ts:17 (sha256:f5761838025219d97b7d64c4f139801e3fa233d1d78ef001f219737bb02f5b24)
pub const BITMAP_KIND: &'static str = "Bitmap";
