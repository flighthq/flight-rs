// @generated from upstream/packages/types/src/HasBlendMode.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, EntityRuntime, Kind, NodeData};

// Source: upstream/packages/types/src/HasBlendMode.ts:8 (sha256:a36a0f6d15182e4c50c7083d92fe9f435046a2df76762ff7b631e6554c0de724)
#[derive(Clone, Default)]
pub struct HasBlendMode {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for HasBlendMode {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HasBlendMode.ts:12 (sha256:682dc986a960120460ce15424fdc03b0f86e4e790640b3452e845120e89c6535)
#[derive(Clone, Default)]
pub struct BlendModeNode {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for BlendModeNode {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for BlendModeNode {
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
