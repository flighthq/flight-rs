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

// Source: upstream/packages/types/src/Billboard.ts:5 (sha256:bc5e3bf5301b415df32f5c62b40ce38a602a55ed45d7a62a80146efa3c2fbe28)
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

// Source: upstream/packages/types/src/Billboard.ts:10 (sha256:e76d6a325bfa3335e4b0976e84fc133010b59327e50a8a39dab55efa6189fe31)
pub type BillboardRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/Billboard.ts:11 (sha256:2636f900d430b089b49758513c53eac93b09e50d4d1e764bfa809148e33fddc1)
pub const BILLBOARD_KIND: &'static str = "Billboard";
