// @generated from upstream/packages/types/src/Signal.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/Signal.ts:4 (sha256:9cfd34242ee0b32dab6868e786b3baa89a59b339c4721c86891080056188b0a5)
#[derive(Clone)]
pub struct Signal<T> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub data: Option<SignalData<T>>,
    pub emit: T,
}
impl<T> PartialEq for Signal<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl<T: Clone + Send + Sync + 'static> crate::FlightEntity for Signal<T> {
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

// Source: upstream/packages/types/src/Signal.ts:13 (sha256:45898bb4cf807323cd23e167b00cd00161d902dda9afd3d72c535367e6b5d7e1)
pub struct SignalData<T> {
    #[doc(hidden)]
    pub inner: std::sync::Arc<std::sync::Mutex<SignalDataStorage<T>>>,
}
impl<T> Clone for SignalData<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
#[doc(hidden)]
pub struct SignalDataStorage<T> {
    pub slots: Vec<Option<T>>,
    pub priorities: Vec<f64>,
    pub repeat: Vec<bool>,
    pub cancelled: bool,
    pub depth: f64,
}
impl<T> SignalData<T> {
    pub fn new(
        slots: Vec<Option<T>>,
        priorities: Vec<f64>,
        repeat: Vec<bool>,
        cancelled: bool,
        depth: f64,
    ) -> Self {
        Self {
            inner: std::sync::Arc::new(std::sync::Mutex::new(SignalDataStorage {
                slots,
                priorities,
                repeat,
                cancelled,
                depth,
            })),
        }
    }
}
