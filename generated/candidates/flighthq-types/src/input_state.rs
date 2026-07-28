// @generated from upstream/packages/types/src/InputState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/InputState.ts:1 (sha256:216dce6f67c2e578771f19028b5b6df661f640ecf89609634c0f5537d28f30e7)
#[derive(Clone)]
pub struct InputState {
    pub axis_values: crate::OpaqueHostValue,
    pub gamepad_buttons_down: crate::OpaqueHostValue,
    pub just_pressed_gamepad_buttons: crate::OpaqueHostValue,
    pub just_pressed_keys: crate::OpaqueHostValue,
    pub just_released_gamepad_buttons: crate::OpaqueHostValue,
    pub just_released_keys: crate::OpaqueHostValue,
    pub keys_down: crate::OpaqueHostValue,
    pub pointer_buttons_down: crate::OpaqueHostValue,
}
