// @generated from upstream/packages/types/src/Shape.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    BlendMode, ClipRegion, EntityRuntime, Kind, Material, MaterialData, ShapeCommandToken,
};

// Source: upstream/packages/types/src/Shape.ts:4 (sha256:013919eaba4c67e9fda790d47ff75aaf2af424e067fec2b548ba5faa6781d20f)
#[derive(Clone, Default)]
pub struct ShapeData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub commands: Vec<ShapeCommandToken>,
}
impl PartialEq for ShapeData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Shape.ts:8 (sha256:3254d9457243cbf5dd1751f0b33f4abfd00c422ee45a896fe0d6d0042b410639)
pub type ShapeRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/Shape.ts:10 (sha256:ddef9a6d87db4f5c30a8e3f6b7a3c913911ebbaf2f83aff9861ea80582c0e02f)
#[derive(Clone, Default)]
pub struct Shape {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: ShapeData,
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
impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Shape {
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

// Source: upstream/packages/types/src/Shape.ts:14 (sha256:84306ce1e997b8bc43d4762866739ca1062638686e16a02d87424e476b9e376e)
pub const SHAPE_KIND: &'static str = "Shape";
