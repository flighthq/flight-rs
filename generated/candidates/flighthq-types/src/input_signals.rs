// @generated from upstream/packages/types/src/InputSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/InputSignals.ts:7 (sha256:500a9febbfe8630d56eee60244aa862d7e027e1efb840cbd9d5f420b06fc004e)
#[derive(Clone)]
pub struct InputSignals {
    pub on_gamepad_axis_move: Signal,
    pub on_gamepad_button_down: Signal,
    pub on_gamepad_button_up: Signal,
    pub on_gamepad_connect: Signal,
    pub on_gamepad_disconnect: Signal,
    pub on_key_down: Signal,
    pub on_key_up: Signal,
    pub on_pointer_cancel: Signal,
    pub on_pointer_down: Signal,
    pub on_pointer_move: Signal,
    pub on_pointer_move_relative: Signal,
    pub on_pointer_up: Signal,
    pub on_text_edit: Signal,
    pub on_text_input: Signal,
    pub on_wheel: Signal,
}
