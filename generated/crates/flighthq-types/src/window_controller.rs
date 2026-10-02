// @generated from upstream/packages/types/src/WindowController.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ButtonController, EntityRuntime, GuiTransitionDescriptor, Node2D, Signal};

// Source: upstream/packages/types/src/WindowController.ts:9 (sha256:4b090f8eb1ea48e0b4ab97303b8a7361a5cd3766767415b2dffc0252bc581ad8)
#[derive(Clone, Default)]
pub struct WindowController {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for WindowController {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WindowController {
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

// Source: upstream/packages/types/src/WindowController.ts:13 (sha256:25d93395cd65e6b856984ebd70d48399cf6fa7d00fb0f8c4441635fc8ed98707)
#[derive(Clone, Default)]
pub struct WindowControllerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub transition: Option<GuiTransitionDescriptor>,
    pub close_button: Option<ButtonController>,
    pub content: Option<Node2D>,
    pub draggable: Option<bool>,
    pub frame: Node2D,
    pub minimum_height: Option<f64>,
    pub minimum_width: Option<f64>,
    pub resizable: Option<bool>,
    pub resize_handle: Option<Node2D>,
    pub title_bar: Option<Node2D>,
}
impl PartialEq for WindowControllerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WindowController.ts:25 (sha256:df5fc8a95c513d02919fe67b9d22ab240c6eb98d24b8714c075cb4002d78f926)
#[derive(Clone)]
pub struct WindowControllerSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_close: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_move:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>>,
    pub on_resize:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>>,
}
impl PartialEq for WindowControllerSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
