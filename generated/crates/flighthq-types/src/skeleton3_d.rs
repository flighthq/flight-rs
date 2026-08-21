// @generated from upstream/packages/types/src/Skeleton3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Node3D};

// Source: upstream/packages/types/src/Skeleton3D.ts:13 (sha256:f3df109087ade0de26157b3ee09b2f37ab1e92d4685ace63c1b088c3809b829c)
#[derive(Clone, Default)]
pub struct Skeleton3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub inverse_bind_matrices: Vec<f32>,
    pub joint_matrices: Vec<f32>,
    pub normal_matrices: Vec<f32>,
    pub joints: Vec<Node3D>,
    pub names: Option<Vec<String>>,
}
impl PartialEq for Skeleton3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Skeleton3D {
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
