// @generated from upstream/packages/types/src/Keyboard.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/Keyboard.ts:2 (sha256:42d3279c2cdb20aff9d3a5d35242e92c0714de035a50fa2de7972dfe72d49e7f)
pub type SoftKeyboardResizeMode = String;

// Source: upstream/packages/types/src/Keyboard.ts:3 (sha256:ad2d01451b6eafd97a83382393efea599d0cfba42550fdd2a9b1f8c645fee583)
pub const SOFT_KEYBOARD_RESIZE_NONE_KIND: &'static str = "None";

// Source: upstream/packages/types/src/Keyboard.ts:4 (sha256:69d3297dec8fe1ac076005d02500ee846c3a7b06b44e314f1012f9b4b2a9ac9d)
pub const SOFT_KEYBOARD_RESIZE_BODY_KIND: &'static str = "Body";

// Source: upstream/packages/types/src/Keyboard.ts:5 (sha256:61cb285d309b01b744f3fc7c7c67202dd25225a54b714756e650a6aad24f131a)
pub type SoftKeyboardStyleKind = String;

// Source: upstream/packages/types/src/Keyboard.ts:6 (sha256:91265f26ace88abf4092596612cf0a84584af6337641c53aeae75be49b0095b6)
pub const SOFT_KEYBOARD_STYLE_DEFAULT_KIND: &'static str = "Default";

// Source: upstream/packages/types/src/Keyboard.ts:7 (sha256:9266face5c7ff8b54abd7086eaae3137e8c8c1a028707c1996470fd2c52e8f21)
pub const SOFT_KEYBOARD_STYLE_DARK_KIND: &'static str = "Dark";

// Source: upstream/packages/types/src/Keyboard.ts:8 (sha256:f3168b4b70a464cd6c6b2c4d75c70b68fef0b0dd36db13ca4449f5a352ede602)
pub type SoftKeyboardPhase = String;

// Source: upstream/packages/types/src/Keyboard.ts:9 (sha256:cec66cf7178370dadaafb2920ac8407220e3651bcad96a192a18d499ad6420f7)
#[derive(Clone)]
pub struct SoftKeyboardTransition {
    pub duration_seconds: f64,
    pub height: f64,
}

// Source: upstream/packages/types/src/Keyboard.ts:13 (sha256:0d37ab980102fd6c29da9c33e3ff69749aa8fc3fceed59fed424bf51c17c3ca4)
#[derive(Clone)]
pub struct SoftKeyboardInfo {
    pub visible: bool,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub width: f64,
}

// Source: upstream/packages/types/src/Keyboard.ts:20 (sha256:22e83e68acafce6d735f550f7047836739fa15be5f62ceccf0e907510f69496f)
#[derive(Clone)]
pub struct SoftKeyboardBackend {
    pub get_info: crate::OpaqueHostValue,
    pub subscribe: crate::OpaqueHostValue,
    pub show: crate::OpaqueHostValue,
    pub hide: crate::OpaqueHostValue,
    pub get_resize_mode: Option<crate::OpaqueHostValue>,
    pub set_resize_mode: Option<crate::OpaqueHostValue>,
    pub get_accessory_bar_visible: Option<crate::OpaqueHostValue>,
    pub set_accessory_bar_visible: Option<crate::OpaqueHostValue>,
    pub get_scroll_assist_enabled: Option<crate::OpaqueHostValue>,
    pub set_scroll_assist_enabled: Option<crate::OpaqueHostValue>,
    pub set_style: Option<crate::OpaqueHostValue>,
}

// Source: upstream/packages/types/src/Keyboard.ts:33 (sha256:b30795d80dbe254f4e9c948d7cb48d343af06663ca1574e18a0ba9a23e6d8a3e)
#[derive(Clone)]
pub struct SoftKeyboard {
    pub on_show: Signal,
    pub on_hide: Signal,
    pub on_resize: Signal,
    pub on_will_show: Signal,
    pub on_will_hide: Signal,
    pub on_will_resize: Signal,
    pub on_did_show: Signal,
    pub on_did_hide: Signal,
    pub on_did_resize: Signal,
}
