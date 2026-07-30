// @generated from upstream/packages/types/src/Billboard.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Kind, Material, MeshGeometry, NodeData, Quaternion, Vector3};

// Source: upstream/packages/types/src/Billboard.ts:4 (sha256:a15a0412bb657fe1fb7e730884d6d4c20e87a6006c40750c3c691fec885b2055)
pub type BillboardMode = String;

// Source: upstream/packages/types/src/Billboard.ts:5 (sha256:359335bd551f53c00326360bc7d1ddf91f2cb3315a7438c5021d70df7bda9ea8)
#[derive(Clone, Default)]
pub struct Billboard {
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
    pub geometry: MeshGeometry,
    pub materials: Vec<Option<Material>>,
    pub mode: BillboardMode,
}
impl PartialEq for Billboard {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Billboard {
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

// Source: upstream/packages/types/src/Billboard.ts:10 (sha256:d063cc883c96929c07ac1a387813cf27933448fee3c315f77d57621c75606a8a)
pub type BillboardRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/Billboard.ts:11 (sha256:2636f900d430b089b49758513c53eac93b09e50d4d1e764bfa809148e33fddc1)
pub const BILLBOARD_KIND: &'static str = "Billboard";
