// @generated from upstream/packages/types/src/InstancedMeshSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Signal};

// Source: upstream/packages/types/src/InstancedMeshSignals.ts:13 (sha256:39672a719fb3e0286b5f66d0092ed836aa7ca6b379483dc0f1641e849cb7d255)
#[derive(Clone)]
pub struct InstancedMeshSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_cleared:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_instance_appended:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
    pub on_instance_removed:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>>,
}
impl PartialEq for InstancedMeshSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for InstancedMeshSignals {
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
