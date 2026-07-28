// @generated from upstream/packages/types/src/Tray.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{RectangleLike, Vector2Like};

// Source: upstream/packages/types/src/Tray.ts:8 (sha256:d39d1ea4692dd41bf5ce452f5c079cea446f0fd228f89bfcf43fcddd3d805fba)
pub type TrayEventType = String;

// Source: upstream/packages/types/src/Tray.ts:17 (sha256:c4ef1c05f76623259a77367cf49aac139141f89a58186212b2b07c8610d9aa4c)
#[derive(Clone)]
pub struct TrayIconOptions {
    pub icon: Option<String>,
    pub icon_template: Option<bool>,
    pub title: Option<String>,
    pub tooltip: Option<String>,
}

// Source: upstream/packages/types/src/Tray.ts:27 (sha256:cde84e659e6cb64716aa79f9915d8eeea1e6f8ea8b24f44d5c1e5f7360052768)
#[derive(Clone)]
pub struct TrayBalloonOptions {
    pub icon: Option<String>,
    pub icon_type: Option<String>,
    pub large_icon: Option<bool>,
    pub no_sound: Option<bool>,
    pub respect_quiet_time: Option<bool>,
    pub text: String,
    pub title: String,
}

// Source: upstream/packages/types/src/Tray.ts:39 (sha256:324c3fc492474e4a425f589501d72ac2cea799bb1972ad63f6e0b56881a052fc)
#[derive(Clone)]
pub struct TrayCapabilities {
    pub balloon: bool,
    pub bounds: bool,
    pub click_events: bool,
    pub drop_files: bool,
    pub pressed_icon: bool,
    pub title: bool,
}

// Source: upstream/packages/types/src/Tray.ts:50 (sha256:49b7e96735e55e9a4cd1b6e5e20192bc01078c2101a25c1fba7b185631e99cc6)
#[derive(Clone)]
pub struct TrayEventData {
    pub alt_key: bool,
    pub bounds: Option<RectangleLike>,
    pub ctrl_key: bool,
    pub drop_files: Option<Vec<String>>,
    pub drop_text: Option<String>,
    pub id: f64,
    pub meta_key: bool,
    pub position: Option<Vector2Like>,
    pub shift_key: bool,
    pub type_: TrayEventType,
}

// Source: upstream/packages/types/src/Tray.ts:63 (sha256:0405b464300168d7dafcea1b461230700cd66bf5675627bd2a5979f7245b49d8)
#[derive(Clone)]
pub struct TrayIcon {
    pub id: f64,
}

// Source: upstream/packages/types/src/Tray.ts:67 (sha256:3ddc26c60494351a4b40d3f0286d9a715b6f7fe37264e95dd15f2f3529545bbb)
#[derive(Clone)]
pub struct TrayBackend {
    pub create: crate::OpaqueHostValue,
    pub destroy: crate::OpaqueHostValue,
    pub display_balloon: crate::OpaqueHostValue,
    pub get_bounds: crate::OpaqueHostValue,
    pub get_capabilities: crate::OpaqueHostValue,
    pub get_title: crate::OpaqueHostValue,
    pub get_tooltip: crate::OpaqueHostValue,
    pub is_destroyed: crate::OpaqueHostValue,
    pub list_ids: crate::OpaqueHostValue,
    pub pop_up_context_menu: crate::OpaqueHostValue,
    pub remove_balloon: crate::OpaqueHostValue,
    pub set_context_menu: crate::OpaqueHostValue,
    pub set_icon: crate::OpaqueHostValue,
    pub set_ignore_double_click_events: crate::OpaqueHostValue,
    pub set_pressed_icon: crate::OpaqueHostValue,
    pub set_template: crate::OpaqueHostValue,
    pub set_title: crate::OpaqueHostValue,
    pub set_tooltip: crate::OpaqueHostValue,
    pub subscribe: crate::OpaqueHostValue,
}
