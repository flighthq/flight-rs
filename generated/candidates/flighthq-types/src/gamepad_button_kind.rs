// @generated from upstream/packages/types/src/GamepadButtonKind.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GamepadButtonKind.ts:1 (sha256:906685412e512673d8085c9de446922f6e65426abf8ae45aa85d9e6bb2b785a7)
#[derive(Clone, Default)]
pub struct GamepadButtonKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub button_east: String,
    pub button_north: String,
    pub button_south: String,
    pub button_west: String,
    pub dpad_down: String,
    pub dpad_left: String,
    pub dpad_right: String,
    pub dpad_up: String,
    pub home: String,
    pub select: String,
    pub shoulder_left: String,
    pub shoulder_right: String,
    pub start: String,
    pub stick_left: String,
    pub stick_right: String,
    pub touchpad: String,
    pub trigger_left: String,
    pub trigger_right: String,
}
impl PartialEq for GamepadButtonKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static GAMEPAD_BUTTON_KIND: std::sync::LazyLock<GamepadButtonKindValues> =
    std::sync::LazyLock::new(|| GamepadButtonKindValues {
        __flight_identity: std::sync::Arc::new(()),
        button_east: "ButtonEast".to_owned(),
        button_north: "ButtonNorth".to_owned(),
        button_south: "ButtonSouth".to_owned(),
        button_west: "ButtonWest".to_owned(),
        dpad_down: "DpadDown".to_owned(),
        dpad_left: "DpadLeft".to_owned(),
        dpad_right: "DpadRight".to_owned(),
        dpad_up: "DpadUp".to_owned(),
        home: "Home".to_owned(),
        select: "Select".to_owned(),
        shoulder_left: "ShoulderLeft".to_owned(),
        shoulder_right: "ShoulderRight".to_owned(),
        start: "Start".to_owned(),
        stick_left: "StickLeft".to_owned(),
        stick_right: "StickRight".to_owned(),
        touchpad: "Touchpad".to_owned(),
        trigger_left: "TriggerLeft".to_owned(),
        trigger_right: "TriggerRight".to_owned(),
    });

// Source: upstream/packages/types/src/GamepadButtonKind.ts:22 (sha256:534eeaa5acca79a33ea125ce7188185d62c72e99de8c9599cf7f668bbcf14229)
pub type GamepadButtonKind = String;
