// @generated from upstream/packages/input/src/index.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

pub use crate::{
    apply_gamepad_axis_dead_zone, apply_gamepad_stick_dead_zone, attach_gamepad_input,
    attach_keyboard_input, attach_pointer_input, attach_relative_pointer_input, attach_text_input,
    attach_wheel_input, connect_input_state_to_input_manager, create_input_key_repeat_timer,
    create_input_manager, create_input_state, detach_gamepad_input, detach_keyboard_input,
    detach_pointer_input, detach_relative_pointer_input, detach_text_input, detach_wheel_input,
    end_input_state_frame, exit_input_pointer_lock, get_coalesced_input_pointer_events,
    get_gamepad_axis_name, get_gamepad_button_name, get_input_gamepad_axis,
    get_key_code_from_dom_keyboard_event, get_key_modifier_from_dom_keyboard_event,
    get_mouse_wheel_mode_from_dom_wheel_event, has_input_pointer_lock,
    is_input_gamepad_button_down, is_input_key_down, is_input_pointer_button_down,
    poll_gamepad_input, release_input_pointer_capture, request_input_pointer_lock,
    set_input_pointer_capture, was_input_gamepad_button_pressed, was_input_gamepad_button_released,
    was_input_key_pressed, was_input_key_released,
};
