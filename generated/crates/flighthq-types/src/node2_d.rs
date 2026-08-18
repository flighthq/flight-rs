// @generated from upstream/packages/types/src/Node2D.ts; do not edit.
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

// Source: upstream/packages/types/src/Node2D.ts:9 (sha256:4f148d8cf9aaff3a8798edcf7f8509330963a70e7fdb27c925ea34469a9a0862)
#[derive(Clone, Default)]
pub struct Node2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: Option<Node2DData>,
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
impl PartialEq for Node2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Node2D {
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

// Source: upstream/packages/types/src/Node2D.ts:13 (sha256:a7d910f0f1079755c0e1a71771fa4665fc8944a7b4e9a1d803d9ce81deeb1f29)
#[derive(Clone, Default)]
pub struct Node2DTraits {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: Option<Node2DData>,
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
impl PartialEq for Node2DTraits {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Node2DTraits {
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

// Source: upstream/packages/types/src/Node2D.ts:17 (sha256:c34481b4654dd2a8efb6b3f019fe6606d88ff989f64ee21ddd34897ebf34bf0a)
#[derive(Clone, Default)]
pub struct Node2DData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for Node2DData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Node2D.ts:18 (sha256:c916f06b9b3fbab4190d2fe99dcf38db092f10037c67a399dbb9c0b5a8343a6e)
pub const DISPLAY_OBJECT_KIND: &'static str = "DisplayObject";

// Source: upstream/packages/types/src/Node2D.ts:19 (sha256:da13f2191d98eab76fd48b55c9a8cf2a488c97e2326f71426a135bef1d6fc4ca)
pub static NODE2_D_TRAITS_KEY: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/types/src/Node2D.ts:22 (sha256:7381345a7afd9c6557ad169752ed3366f892462da9be113e230ac2828c2c82be)
pub type Node2DRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/Node2D.ts:25 (sha256:349e58c2345c42daceec731365684d55df3f4c9fac5edc7d3f8429ebc9fce039)
pub type Node2DDataFactory = NodeDataFactory<Node2DData>;

// Source: upstream/packages/types/src/Node2D.ts:26 (sha256:fe8acb7996de1b6837ddd87d054cd3733a15a0b2fb40c7d51ce9a98ed5635535)
pub type Node2DRuntimeFactory<R> = NodeRuntimeFactory<R>;
