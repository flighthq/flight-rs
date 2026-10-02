// @generated from upstream/packages/types/src/TextInputController.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    EntityRuntime, GuiTransitionDescriptor, InputKeyboardData, Node2D, RichText, Signal,
    TextInputManager, TextInputSource,
};

// Source: upstream/packages/types/src/TextInputController.ts:11 (sha256:8dc033b87c1a10f5d91f3e5dffc66653b393ed8edea2c0b1aec51097edcb0289)
#[derive(Clone, Default)]
pub struct TextInputController {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for TextInputController {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for TextInputController {
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

// Source: upstream/packages/types/src/TextInputController.ts:15 (sha256:09dab98f36d3a4fc8b921a1ce96a1d65aad30d016c004797581ff3a622a03331)
#[derive(Clone, Default)]
pub struct TextInputControllerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub transition: Option<GuiTransitionDescriptor>,
    pub background: Option<Node2D>,
    pub caret: Option<Node2D>,
    pub input: Option<TextInputSource>,
    pub manager: Option<TextInputManager>,
    pub text_field: RichText,
}
impl PartialEq for TextInputControllerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextInputController.ts:23 (sha256:9dc2ab84ae2ff14d290f53da07d3bca3725f9aad2e7ce207c412fdad7a3d027d)
#[derive(Clone)]
pub struct TextInputControllerSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_change:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>>,
    pub on_submit:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>>,
}
impl PartialEq for TextInputControllerSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextInputController.ts:28 (sha256:9fd840a1c0817f191b8cea4147597c10f06f33731d4be12a99c44b3b480fd49e)
pub type TextInputControllerKeyboardData = InputKeyboardData;
