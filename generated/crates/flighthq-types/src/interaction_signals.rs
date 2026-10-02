// @generated from upstream/packages/types/src/InteractionSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, FocusEventData, KeyboardEventData, PointerEventData, Signal};

// Source: upstream/packages/types/src/InteractionSignals.ts:7 (sha256:2e75ed80ab89d58569f72bd47a6f83bbdeab7e87465bbb9b736ca1efdc83b32f)
#[derive(Clone)]
pub struct InteractionSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_click: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_context_menu: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_double_click: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_focus_in: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(FocusEventData) -> () + Send + 'static>>>,
    >,
    pub on_focus_out: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(FocusEventData) -> () + Send + 'static>>>,
    >,
    pub on_key_down: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(KeyboardEventData) -> () + Send + 'static>>>,
    >,
    pub on_key_up: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(KeyboardEventData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_cancel: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_double_click: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_down: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_move: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_out: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_over: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_roll_out: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_roll_over: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_up: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_release_outside: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
    pub on_wheel: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(PointerEventData) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for InteractionSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for InteractionSignals {
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
