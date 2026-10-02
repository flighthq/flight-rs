// @generated from upstream/packages/types/src/InputSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    EntityRuntime, InputGamepadAxisData, InputGamepadButtonData, InputGamepadConnectData,
    InputKeyboardData, InputPointerData, InputTextData, Signal,
};

// Source: upstream/packages/types/src/InputSignals.ts:8 (sha256:bb4f15626f6ecd9825bb7ea5cc74c5237571b4174c578e6c67f3f74667623459)
#[derive(Clone)]
pub struct InputSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub on_gamepad_axis_move: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(InputGamepadAxisData) -> () + Send + 'static>>,
        >,
    >,
    pub on_gamepad_button_down: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(InputGamepadButtonData) -> () + Send + 'static>>,
        >,
    >,
    pub on_gamepad_button_up: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(InputGamepadButtonData) -> () + Send + 'static>>,
        >,
    >,
    pub on_gamepad_connect: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(InputGamepadConnectData) -> () + Send + 'static>>,
        >,
    >,
    pub on_gamepad_disconnect: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(InputGamepadConnectData) -> () + Send + 'static>>,
        >,
    >,
    pub on_key_down: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputKeyboardData) -> () + Send + 'static>>>,
    >,
    pub on_key_up: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputKeyboardData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_cancel: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputPointerData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_down: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputPointerData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_move: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputPointerData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_move_relative: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputPointerData) -> () + Send + 'static>>>,
    >,
    pub on_pointer_up: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputPointerData) -> () + Send + 'static>>>,
    >,
    pub on_text_edit: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputTextData) -> () + Send + 'static>>>,
    >,
    pub on_text_input: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputTextData) -> () + Send + 'static>>>,
    >,
    pub on_wheel: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(InputPointerData) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for InputSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for InputSignals {
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
