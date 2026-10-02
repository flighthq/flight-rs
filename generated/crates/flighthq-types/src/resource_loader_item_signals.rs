// @generated from upstream/packages/types/src/ResourceLoaderItemSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Signal};

// Source: upstream/packages/types/src/ResourceLoaderItemSignals.ts:4 (sha256:a133fe33b3545e38d955a452aa211550ae417b46ffdac0c968e4d7bfc264289b)
#[derive(Clone)]
pub struct ResourceLoaderItemSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_item_complete: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(String, crate::FlightValue) -> () + Send + 'static>>,
        >,
    >,
    pub on_item_error: Signal<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(String, crate::FlightValue, f64) -> () + Send + 'static>,
            >,
        >,
    >,
    pub on_item_retry: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String, f64, f64) -> () + Send + 'static>>>,
    >,
    pub on_item_start:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>>,
}
impl PartialEq for ResourceLoaderItemSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ResourceLoaderItemSignals {
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
