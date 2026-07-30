// @generated from upstream/packages/types/src/GamepadAxisKind.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GamepadAxisKind.ts:1 (sha256:84a9ba443c46b06a538ba92b0c24148ff23ab6841975b789897f9519ed7afbbb)
#[derive(Clone, Default)]
pub struct GamepadAxisKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub stick_left_x: String,
    pub stick_left_y: String,
    pub stick_right_x: String,
    pub stick_right_y: String,
}
impl PartialEq for GamepadAxisKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static GAMEPAD_AXIS_KIND: std::sync::LazyLock<GamepadAxisKindValues> =
    std::sync::LazyLock::new(|| GamepadAxisKindValues {
        __flight_identity: std::sync::Arc::new(()),
        stick_left_x: "StickLeftX".to_owned(),
        stick_left_y: "StickLeftY".to_owned(),
        stick_right_x: "StickRightX".to_owned(),
        stick_right_y: "StickRightY".to_owned(),
    });

// Source: upstream/packages/types/src/GamepadAxisKind.ts:8 (sha256:a595c1acd4375edb2a517b2514b25bfa93ab6713e9ebfd4badbb785be6b21411)
pub type GamepadAxisKind = String;
