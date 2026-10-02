// @generated from upstream/packages/types/src/ComboBoxController.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    ButtonController, EntityRuntime, GuiTransitionDescriptor, ListController, Node2D, Signal,
};

// Source: upstream/packages/types/src/ComboBoxController.ts:10 (sha256:9ad766975b886dbecaaa704b67c1fc5762be0dd6cae9a9f4863d670890cff4d3)
#[derive(Clone, Default)]
pub struct ComboBoxController {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for ComboBoxController {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for ComboBoxController {
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

// Source: upstream/packages/types/src/ComboBoxController.ts:14 (sha256:1fed4a5792f9da60acb366b36507c2740274d25b967bde77358ff2c1eb23467f)
#[derive(Clone, Default)]
pub struct ComboBoxControllerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub transition: Option<GuiTransitionDescriptor>,
    pub button: ButtonController,
    pub display: Option<Node2D>,
    pub list: ListController,
    pub open: Option<bool>,
}
impl PartialEq for ComboBoxControllerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ComboBoxController.ts:21 (sha256:a97ac3bff33901afb3b0357548897b24cac03ded997dd201d5e116c390df1242)
#[derive(Clone)]
pub struct ComboBoxControllerSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_change:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
    pub on_open_change:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) -> () + Send + 'static>>>>,
}
impl PartialEq for ComboBoxControllerSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
