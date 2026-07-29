// @generated from upstream/packages/types/src/Sprite.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, ClipRegion, EntityRuntime, Kind, Material, MaterialData, Rectangle, TextureAtlas,
};

// Source: upstream/packages/types/src/Sprite.ts:5 (sha256:417d847384b11dc6fde6e0c4683e3e4bbc271e82de90a2a6369c16d776657111)
#[derive(Clone, Default)]
pub struct SpriteData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub atlas: Option<TextureAtlas>,
    pub id: f64,
    pub rect: Option<Rectangle>,
}
impl PartialEq for SpriteData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Sprite.ts:11 (sha256:d4a64ca8109b8797b1b25a7b5a47513a6506c9bc3d308365b04da0ef1e38345b)
pub type SpriteRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/Sprite.ts:13 (sha256:ebe1fffa076c256cc6d00a80e625c780e8a0f9b262cab3c8b645b094107d72fb)
#[derive(Clone, Default)]
pub struct Sprite {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: SpriteData,
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
impl PartialEq for Sprite {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Sprite {
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

// Source: upstream/packages/types/src/Sprite.ts:17 (sha256:d0b6f5bb793d24c7169d831d340ac49fa9b89b5f7b247a77f6d736fa6a1847e1)
pub const SPRITE_KIND: &'static str = "Sprite";
