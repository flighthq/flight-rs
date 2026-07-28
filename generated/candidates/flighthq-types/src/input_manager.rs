// @generated from upstream/packages/types/src/InputManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/InputManager.ts:3 (sha256:9c46b3880e43cd53f90eb927e6182f0408e9d8f24a051e786fb4992850ce7eb2)
#[derive(Clone)]
pub struct AttachInputOptions {
    pub prevent_default: Option<bool>,
}

// Source: upstream/packages/types/src/InputManager.ts:7 (sha256:acb1ec5a0825eae2955aba234b8019647bfebc6c07a57b04c1ffb62af7cc98bd)
#[derive(Clone)]
pub struct InputManager {
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
    pub enabled: bool,
}
