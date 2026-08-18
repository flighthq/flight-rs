// @generated from upstream/packages/types/src/Group.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Kind, NodeData, Quaternion, Vector3};

// Source: upstream/packages/types/src/Group.ts:2 (sha256:36634dedc9bfe0fb5a50b9b59e0968a3a9ba3e5d973878b9aed0f41975152da8)
#[derive(Clone, Default)]
pub struct Group {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha: f64,
    pub visible: bool,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}
impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Group {
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

// Source: upstream/packages/types/src/Group.ts:3 (sha256:97c2dc2e993b3e72b9ed59e96413cb2d738fdfacf9e9b5bf1ec458bab9fd667d)
pub type GroupRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/Group.ts:4 (sha256:abeb23f5f4400780b2f81bdafad57f005072187637585f9ac6aa050ffd49b285)
pub const GROUP_KIND: &'static str = "Group";
