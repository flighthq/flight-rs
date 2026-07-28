// @generated from upstream/packages/types/src/InputGamepadData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/InputGamepadData.ts:1 (sha256:f6f3872e38d26fd5c0c360caad23d753be822f63cb968e3c284ab16d66ff612e)
#[derive(Clone, Default)]
pub struct InputGamepadAxisData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub axis: f64,
    pub gamepad: f64,
    pub time_stamp: f64,
    pub value: f64,
}
impl PartialEq for InputGamepadAxisData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/InputGamepadData.ts:8 (sha256:67611d7cfb4735a3bd32f9efeacecff3a61b78eab1a7a05b8eeac35d932b0944)
#[derive(Clone, Default)]
pub struct InputGamepadButtonData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub button: f64,
    pub gamepad: f64,
    pub time_stamp: f64,
    pub value: f64,
}
impl PartialEq for InputGamepadButtonData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/InputGamepadData.ts:15 (sha256:4177d3531b0c1218bdfe2219f99275e7b1be1971b99912f52f8debe397599417)
#[derive(Clone, Default)]
pub struct InputGamepadConnectData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub gamepad: f64,
    pub id: String,
    pub mapping: GamepadMapping,
}
impl PartialEq for InputGamepadConnectData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/InputGamepadData.ts:21 (sha256:e03d0e5c7135d71e4cd878b67319d48dc775adcd63dd513a5f20ac1e28c5aff2)
pub type GamepadMapping = String;
