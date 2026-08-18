// @generated from upstream/packages/types/src/Tilemap.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, ClipRegion, EntityRuntime, Kind, Material, MaterialData, TextureAtlas};

// Source: upstream/packages/types/src/Tilemap.ts:5 (sha256:24320b83bfd5874be2f12540bc06d3b54f1f6d2611c4c7652b684095843ad56b)
#[derive(Clone, Default)]
pub struct TilemapData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub atlas: Option<TextureAtlas>,
    pub columns: f64,
    pub rows: f64,
    pub tile_height: f64,
    pub tile_width: f64,
    pub tiles: Vec<i16>,
    pub material_data: Option<Vec<Option<MaterialData>>>,
}
impl PartialEq for TilemapData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tilemap.ts:17 (sha256:cd698df65d0c63e2e3ec329ca3f6e690cd93310a356a89f93983b2ce3ecb5bf3)
pub type TilemapRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/Tilemap.ts:19 (sha256:baaa0bd15356d53492d909bb22e420d309e45d951731b185dddd284a4bfe42b1)
#[derive(Clone, Default)]
pub struct Tilemap {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: TilemapData,
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
impl PartialEq for Tilemap {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Tilemap {
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

// Source: upstream/packages/types/src/Tilemap.ts:23 (sha256:62481eb694aea171dfe962d6b52ba7db3a8a9d130002f7e9f0f7acc217dc7cd3)
pub const TILEMAP_KIND: &'static str = "Tilemap";
