// @generated from upstream/packages/types/src/ScreenSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, ScreenChangeEvent, ScreenInfo, ScreenPermissionState, Signal};

// Source: upstream/packages/types/src/ScreenSignals.ts:9 (sha256:2f71aaba62d04618e1026ded04608f1ebf3cb540ed7624698ee3d0afa6fb36fb)
#[derive(Clone)]
pub struct ScreenSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_screen_added:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ScreenInfo) -> () + Send + 'static>>>>,
    pub on_screen_metrics_changed: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ScreenChangeEvent) -> () + Send + 'static>>>,
    >,
    pub on_screen_removed:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ScreenInfo) -> () + Send + 'static>>>>,
}
impl PartialEq for ScreenSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ScreenSignals {
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

// Source: upstream/packages/types/src/ScreenSignals.ts:15 (sha256:58ec38b7510ac3e1c10a58396c63cbd1ed03fc0fab7638ea3eaf0cd837501341)
#[derive(Clone)]
pub struct ScreenPermissionChange {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_change: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(ScreenPermissionState) -> () + Send + 'static>>,
        >,
    >,
}
impl PartialEq for ScreenPermissionChange {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ScreenPermissionChange {
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
