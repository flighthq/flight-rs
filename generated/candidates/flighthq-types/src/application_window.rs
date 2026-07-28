// @generated from upstream/packages/types/src/ApplicationWindow.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/ApplicationWindow.ts:3 (sha256:6813fda41529b94599e990d8672767e38564535318fc5a3d9555a89c3063ec47)
#[derive(Clone)]
pub struct ApplicationWindow {
    pub title: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub device_pixel_ratio: f64,
    pub minimized: bool,
    pub maximized: bool,
    pub fullscreen: bool,
    pub focused: bool,
    pub visible: bool,
    pub resizable: bool,
    pub always_on_top: bool,
    pub skip_taskbar: bool,
    pub opacity: f64,
    pub icon: String,
    pub min_width: f64,
    pub min_height: f64,
    pub max_width: f64,
    pub max_height: f64,
    pub on_activate: Signal,
    pub on_close: Signal,
    pub on_close_request: Signal,
    pub on_deactivate: Signal,
    pub on_drop_file: Signal,
    pub on_focus_in: Signal,
    pub on_focus_out: Signal,
    pub on_fullscreen_changed: Signal,
    pub on_maximize: Signal,
    pub on_minimize: Signal,
    pub on_move: Signal,
    pub on_orientation_changed: Signal,
    pub on_render_context_lost: Signal,
    pub on_render_context_restored: Signal,
    pub on_resize: Signal,
    pub on_restore: Signal,
}

// Source: upstream/packages/types/src/ApplicationWindow.ts:53 (sha256:f1bd48dc8b90c3350a7eb428b57ee99d2442cdfe213fbca3d6c341b0140541e3)
#[derive(Clone)]
pub struct WindowOptions {
    pub title: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub resizable: Option<bool>,
    pub always_on_top: Option<bool>,
    pub fullscreen: Option<bool>,
    pub minimized: Option<bool>,
    pub maximized: Option<bool>,
    pub visible: Option<bool>,
    pub min_width: Option<f64>,
    pub min_height: Option<f64>,
    pub max_width: Option<f64>,
    pub max_height: Option<f64>,
    pub center: Option<bool>,
    pub frame: Option<bool>,
    pub transparent: Option<bool>,
}

// Source: upstream/packages/types/src/ApplicationWindow.ts:76 (sha256:c4283b158481445c344429a7d156350e5c7de857955bfe2d32de79a101df7ec7)
#[derive(Clone)]
pub struct WindowBounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

// Source: upstream/packages/types/src/ApplicationWindow.ts:87 (sha256:580fbba55277b281a242ded99af05738e9bf5e6b321894643422ed36ece5f18d)
#[derive(Clone)]
pub struct WindowBackend {
    pub open: crate::OpaqueHostValue,
    pub close: crate::OpaqueHostValue,
    pub set_title: crate::OpaqueHostValue,
    pub set_position: crate::OpaqueHostValue,
    pub set_size: crate::OpaqueHostValue,
    pub get_bounds: crate::OpaqueHostValue,
    pub minimize: crate::OpaqueHostValue,
    pub maximize: crate::OpaqueHostValue,
    pub restore: crate::OpaqueHostValue,
    pub focus: crate::OpaqueHostValue,
    pub show: crate::OpaqueHostValue,
    pub hide: crate::OpaqueHostValue,
    pub center: crate::OpaqueHostValue,
    pub set_resizable: crate::OpaqueHostValue,
    pub set_always_on_top: crate::OpaqueHostValue,
    pub set_minimum_size: crate::OpaqueHostValue,
    pub set_maximum_size: crate::OpaqueHostValue,
    pub set_fullscreen: crate::OpaqueHostValue,
    pub set_icon: crate::OpaqueHostValue,
    pub set_opacity: crate::OpaqueHostValue,
    pub set_skip_taskbar: crate::OpaqueHostValue,
    pub set_menu_bar_visible: crate::OpaqueHostValue,
    pub set_parent: crate::OpaqueHostValue,
    pub set_progress: crate::OpaqueHostValue,
    pub request_attention: crate::OpaqueHostValue,
    pub set_content_protection: crate::OpaqueHostValue,
    pub flash_window_frame: crate::OpaqueHostValue,
    pub set_has_shadow: crate::OpaqueHostValue,
}
