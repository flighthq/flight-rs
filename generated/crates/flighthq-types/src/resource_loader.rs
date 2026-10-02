// @generated from upstream/packages/types/src/ResourceLoader.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, ResourceLoadReport, Signal};

// Source: upstream/packages/types/src/ResourceLoader.ts:5 (sha256:3f7fabd10c13efdf341000471b13bfa0f5184682b3a2ee2e883589dac3c22a6b)
#[derive(Clone)]
pub struct ResourceLoader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_cancel:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_complete: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Vec<ResourceLoadReport>) -> () + Send + 'static>>,
        >,
    >,
    pub on_error: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(crate::FlightValue, String) -> () + Send + 'static>>,
        >,
    >,
    pub on_pause: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_progress:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
    pub on_resume:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for ResourceLoader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ResourceLoader {
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
