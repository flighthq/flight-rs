// @generated from upstream/packages/types/src/DisplayObject.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, ClipRegion, EntityRuntime, Kind, Material, MaterialData, NodeDataFactory,
    NodeRuntimeFactory,
};

// Source: upstream/packages/types/src/DisplayObject.ts:9 (sha256:6a8c06bc00834ebd8b57775a5f59f472af7b5ecf7f98b1939235b0cca1a66dc4)
#[derive(Clone, Default)]
pub struct DisplayObject {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: Option<DisplayObjectData>,
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
impl PartialEq for DisplayObject {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for DisplayObject {
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

// Source: upstream/packages/types/src/DisplayObject.ts:13 (sha256:ede92710ddf9f1e1a1e8a29eaca2c42e699bd220631531bdcb4363f81ef9a95b)
#[derive(Clone, Default)]
pub struct DisplayObjectTraits {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: Option<DisplayObjectData>,
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
impl PartialEq for DisplayObjectTraits {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for DisplayObjectTraits {
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

// Source: upstream/packages/types/src/DisplayObject.ts:17 (sha256:af6b3e6bcc2bd2d24e7d294305d34cfb623314545885e763f3205e2be1eabe46)
#[derive(Clone, Default)]
pub struct DisplayObjectData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for DisplayObjectData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/DisplayObject.ts:18 (sha256:c916f06b9b3fbab4190d2fe99dcf38db092f10037c67a399dbb9c0b5a8343a6e)
pub const DISPLAY_OBJECT_KIND: &'static str = "DisplayObject";

// Source: upstream/packages/types/src/DisplayObject.ts:19 (sha256:b410fd498bf7cb937ae2ccefbf1693a8bc26b731832cbac2eff9ecce1aff3ea5)
pub static DISPLAY_OBJECT_TRAITS_KEY: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/types/src/DisplayObject.ts:22 (sha256:d48b6602d9ab7bac3e71da19493106c092f4a67899e296ec88436aaa4a539220)
pub type DisplayObjectRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/DisplayObject.ts:25 (sha256:ae86d59cbfc9737e919b7cb4873d1df87027e4321704d93c75ed2acaa8700149)
pub type DisplayObjectDataFactory = NodeDataFactory<DisplayObjectData>;

// Source: upstream/packages/types/src/DisplayObject.ts:26 (sha256:bba4e0042ba61259518104d9ab6eea77fc1d7a233dc336980b70241c5ab0beee)
pub type DisplayObjectRuntimeFactory<R> = NodeRuntimeFactory<R>;
