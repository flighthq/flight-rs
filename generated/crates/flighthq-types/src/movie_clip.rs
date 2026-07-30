// @generated from upstream/packages/types/src/MovieClip.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, ClipRegion, EntityRuntime, Kind, Material, MaterialData, Timeline};

// Source: upstream/packages/types/src/MovieClip.ts:5 (sha256:844ebe6fe1ba72c2844947afcc9d7a7e8ed079b6b61270f799b0abc99e5098cf)
#[derive(Clone, Default)]
pub struct MovieClipData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub timeline: Option<Timeline>,
}
impl PartialEq for MovieClipData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MovieClip.ts:9 (sha256:5386662f90c427df70f9b3b82fca86921a3fe1eae777a323d15d43c80087a999)
pub type MovieClipRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/MovieClip.ts:13 (sha256:9cc2d31b0acbcbef484fc61680c640e6d7dc44d735fc62fbf669952d5327347a)
#[derive(Clone, Default)]
pub struct MovieClip {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: MovieClipData,
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
impl PartialEq for MovieClip {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for MovieClip {
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

// Source: upstream/packages/types/src/MovieClip.ts:17 (sha256:04be8325f9b5c0fe0519ffc7b46f601f0e47a74ac2c154d5eeb4ce58648b34b1)
pub const MOVIE_CLIP_KIND: &'static str = "MovieClip";
