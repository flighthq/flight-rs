// @generated from upstream/packages/input/src/inputManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_host_signals::create_signal;
use flighthq_types::{InputManager, InputSignals};

// Source: upstream/packages/input/src/inputManager.ts:427 (sha256:0aa752968160d69181a564725ef4c7db5551e0f7e5230cbdc0104a5d2081bd0f)
pub fn create_input_manager() -> InputManager {
    return {
        let __flight_spread_0 = create_input_signals();
        InputManager {
            __flight_identity: std::sync::Arc::new(()),
            on_gamepad_axis_move: (__flight_spread_0.on_gamepad_axis_move).clone(),
            on_gamepad_button_down: (__flight_spread_0.on_gamepad_button_down).clone(),
            on_gamepad_button_up: (__flight_spread_0.on_gamepad_button_up).clone(),
            on_gamepad_connect: (__flight_spread_0.on_gamepad_connect).clone(),
            on_gamepad_disconnect: (__flight_spread_0.on_gamepad_disconnect).clone(),
            on_key_down: (__flight_spread_0.on_key_down).clone(),
            on_key_up: (__flight_spread_0.on_key_up).clone(),
            on_pointer_cancel: (__flight_spread_0.on_pointer_cancel).clone(),
            on_pointer_down: (__flight_spread_0.on_pointer_down).clone(),
            on_pointer_move: (__flight_spread_0.on_pointer_move).clone(),
            on_pointer_move_relative: (__flight_spread_0.on_pointer_move_relative).clone(),
            on_pointer_up: (__flight_spread_0.on_pointer_up).clone(),
            on_text_edit: (__flight_spread_0.on_text_edit).clone(),
            on_text_input: (__flight_spread_0.on_text_input).clone(),
            on_wheel: (__flight_spread_0.on_wheel).clone(),
            enabled: true,
        }
    };
}

// Source: upstream/packages/input/src/inputManager.ts:434 (sha256:28fd446928eacd2fe3821deb30d07e652b9a1bf7539a9c5964dddcad0f5dfa6a)
pub fn create_input_signals() -> InputSignals {
    return InputSignals {
        __flight_identity: std::sync::Arc::new(()),
        on_gamepad_axis_move: create_signal(),
        on_gamepad_button_down: create_signal(),
        on_gamepad_button_up: create_signal(),
        on_gamepad_connect: create_signal(),
        on_gamepad_disconnect: create_signal(),
        on_key_down: create_signal(),
        on_key_up: create_signal(),
        on_pointer_cancel: create_signal(),
        on_pointer_down: create_signal(),
        on_pointer_move: create_signal(),
        on_pointer_move_relative: create_signal(),
        on_pointer_up: create_signal(),
        on_text_edit: create_signal(),
        on_text_input: create_signal(),
        on_wheel: create_signal(),
    };
}
