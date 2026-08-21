// @generated from upstream/packages/types/src/Node3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Kind, NodeData, Quaternion, Vector3};

// Source: upstream/packages/types/src/Node3D.ts:4 (sha256:5db4edf62f432f403aeb010b26221bf472f8d809139e56043f277df3b26d3fd3)
pub const NODE3_D_KIND: &'static str = "Node3D";

// Source: upstream/packages/types/src/Node3D.ts:5 (sha256:3e3d0a1c44ce498238a9db4e196862dba545b8acdb4add0c8b704a3c87187cd9)
#[derive(Clone, Default)]
pub struct Node3DTraits {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: f64,
    pub visible: bool,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}
impl PartialEq for Node3DTraits {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Node3D.ts:6 (sha256:74b705ed2a9103bffb12537ab771b8235ef22d8cc8ce35bc418600fdc23d857f)
#[derive(Clone, Default)]
pub struct Node3D {
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
    pub alpha: f64,
    pub visible: bool,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}
impl PartialEq for Node3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Node3D {
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

// Source: upstream/packages/types/src/Node3D.ts:7 (sha256:8d342a5a28d13b8a1ae7971924f01179551729ab49520619e728656dff662d33)
pub type Node3DRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/Node3D.ts:8 (sha256:01db40a52051d1e037d4bbcc1f78e0425f5e2038a7b4a8be9845db3063f3b507)
pub static NODE3_D_TRAITS_KEY: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());
