// @generated from upstream/packages/types/src/ButtonController.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, GuiTransitionDescriptor, Node2D, NodeInteractiveStateBinding, Signal};

// Source: upstream/packages/types/src/ButtonController.ts:9 (sha256:52b462057270de3888dc83b38a7b75d0d07979e1383f66c6891ca0a842a4b87e)
#[derive(Clone, Default)]
pub struct ButtonController {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for ButtonController {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ButtonController {
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

// Source: upstream/packages/types/src/ButtonController.ts:13 (sha256:4620c3539d249002f12d3abe1ec922c9df24ddcbbe96673020698b93191f360d)
#[derive(Clone, Default)]
pub struct ButtonControllerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub transition: Option<GuiTransitionDescriptor>,
    pub disabled: Option<bool>,
    pub down_state: Option<Node2D>,
    pub hit_area: Option<Node2D>,
    pub interactive_state_binding: Option<NodeInteractiveStateBinding>,
    pub over_state: Option<Node2D>,
    pub up_state: Node2D,
}
impl PartialEq for ButtonControllerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ButtonController.ts:24 (sha256:5c9c34c777c5d096ccfa0ee831884100b29c56af043163c412c41178724da7db)
#[derive(Clone)]
pub struct ButtonControllerSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_click: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_press: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_release:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for ButtonControllerSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
