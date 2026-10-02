// @generated from upstream/packages/types/src/HasMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Kind, Material2D, MaterialData, NodeData};

// Source: upstream/packages/types/src/HasMaterial.ts:8 (sha256:4a2a91b458f4fed4b5ddb978a7439879f1f6d0ed0f4a5dafe2794485f3a2c47b)
#[derive(Clone, Default)]
pub struct HasMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub material: Option<Material2D>,
    pub material_data: Option<MaterialData>,
}
impl PartialEq for HasMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HasMaterial.ts:15 (sha256:2c0b94d6ae603b5dfd1f3b50bf7d785d985302c0818fe9d0846b13ea2ec1aaa8)
#[derive(Clone, Default)]
pub struct MaterialNode {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub material: Option<Material2D>,
    pub material_data: Option<MaterialData>,
}
impl PartialEq for MaterialNode {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for MaterialNode {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
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
