// @generated from upstream/packages/types/src/InputSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    InputGamepadAxisData, InputGamepadButtonData, InputGamepadConnectData, InputKeyboardData,
    InputPointerData, InputTextData, Signal,
};

// Source: upstream/packages/types/src/InputSignals.ts:7 (sha256:500a9febbfe8630d56eee60244aa862d7e027e1efb840cbd9d5f420b06fc004e)
#[derive(Clone)]
pub struct InputSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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
