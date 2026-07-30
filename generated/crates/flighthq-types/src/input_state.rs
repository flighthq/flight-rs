// @generated from upstream/packages/types/src/InputState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/InputState.ts:1 (sha256:216dce6f67c2e578771f19028b5b6df661f640ecf89609634c0f5537d28f30e7)
#[derive(Clone, Default)]
pub struct InputState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub axis_values: Vec<(f64, f64)>,
    pub gamepad_buttons_down: Vec<f64>,
    pub just_pressed_gamepad_buttons: Vec<f64>,
    pub just_pressed_keys: Vec<f64>,
    pub just_released_gamepad_buttons: Vec<f64>,
    pub just_released_keys: Vec<f64>,
    pub keys_down: Vec<f64>,
    pub pointer_buttons_down: Vec<(f64, f64)>,
}
impl PartialEq for InputState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
