// @generated from upstream/packages/types/src/Sprite.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, ClipRegion, EntityRuntime, Kind, Material, MaterialData, Texture};

// Source: upstream/packages/types/src/Sprite.ts:4 (sha256:0a192307a03c8542e477cc8b64353c3b5de08c6cde73bec89ca39694c56943b4)
#[derive(Clone, Default)]
pub struct SpriteData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub texture: Option<Texture>,
}
impl PartialEq for SpriteData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Sprite.ts:8 (sha256:65d02f91307d74bea839eda6dba3a93ed11e89d64e3788d0bce0b9dafd7a2173)
pub type SpriteRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/Sprite.ts:13 (sha256:d7459435d0471453f1a562948d6fa63807ac8d1381c6ea5874b6f8299eb1b9c2)
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
