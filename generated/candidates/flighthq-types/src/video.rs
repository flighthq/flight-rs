// @generated from upstream/packages/types/src/Video.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, ClipRegion, EntityRuntime, Kind, Material, MaterialData, VideoResource};

// Source: upstream/packages/types/src/Video.ts:4 (sha256:8c8f35f7ee5d4f94eb9f0554b9fcc97816a674dda9694357b7a93803162903b6)
#[derive(Clone, Default)]
pub struct VideoData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub smoothing: bool,
    pub source: Option<VideoResource>,
}
impl PartialEq for VideoData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Video.ts:9 (sha256:c24797e3320405c45c0e616f48732af957f98522365bfbb9dd996c941b67e597)
pub type VideoRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/Video.ts:11 (sha256:65a44c3615f4560c8a06e3e136837ccef7693076a3514c1bc2fe482bb1194482)
#[derive(Clone, Default)]
pub struct Video {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: VideoData,
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
impl PartialEq for Video {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Video {
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

// Source: upstream/packages/types/src/Video.ts:15 (sha256:a0a8783a8297b9ded8781f2fbd675af728f0a3cfc4ba322b587b5aeaebfb5d7b)
pub const VIDEO_KIND: &'static str = "Video";
