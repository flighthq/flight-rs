// @generated from upstream/packages/types/src/RenderView.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, ClipRegion, EntityRuntime, Kind, Material, MaterialData};

// Source: upstream/packages/types/src/RenderView.ts:3 (sha256:191ad99f18f0b60a0eea5ff2c1d904816fb161d14b81f5cac08ebfe30bc2db33)
#[derive(Clone)]
pub struct RenderViewRenderer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub canvas: crate::OpaqueHostValue,
    pub render: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for RenderViewRenderer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderView.ts:8 (sha256:556e68ccb2b4d2da3c6e6fa1b0f8ef10383a12cfba0251775f192815485a287a)
#[derive(Clone, Default)]
pub struct RenderViewData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub renderer: Option<RenderViewRenderer>,
    pub width: f64,
}
impl PartialEq for RenderViewData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderView.ts:14 (sha256:5176743925d802e0d089da23620726143b90e5c6737b7c2fd726e35f909ed4f9)
pub type RenderViewRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/RenderView.ts:16 (sha256:9e068323ff1e912e4e842c84bad910c8656c7863fbc4ef2b8a47ba90d85cac26)
#[derive(Clone, Default)]
pub struct RenderView {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: RenderViewData,
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
impl PartialEq for RenderView {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RenderView {
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

// Source: upstream/packages/types/src/RenderView.ts:20 (sha256:91bad5d61274706ecac312220b43d67017eb9f216aaaca347bb54d3bb4576f8b)
pub const RENDER_VIEW_KIND: &'static str = "RenderView";
