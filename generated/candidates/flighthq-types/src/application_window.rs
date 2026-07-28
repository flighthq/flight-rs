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
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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
    pub on_activate:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_close: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_close_request:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_deactivate:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_drop_file:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>>,
    pub on_focus_in:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_focus_out:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_fullscreen_changed:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_maximize:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_minimize:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_move: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_orientation_changed:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_render_context_lost:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_render_context_restored:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_resize:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_restore:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for ApplicationWindow {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ApplicationWindow.ts:53 (sha256:f1bd48dc8b90c3350a7eb428b57ee99d2442cdfe213fbca3d6c341b0140541e3)
#[derive(Clone)]
pub struct WindowOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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
impl PartialEq for WindowOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ApplicationWindow.ts:76 (sha256:c4283b158481445c344429a7d156350e5c7de857955bfe2d32de79a101df7ec7)
#[derive(Clone)]
pub struct WindowBounds {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
impl PartialEq for WindowBounds {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ApplicationWindow.ts:87 (sha256:580fbba55277b281a242ded99af05738e9bf5e6b321894643422ed36ece5f18d)
#[derive(Clone)]
pub struct WindowBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub open: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ApplicationWindow, WindowOptions) -> bool + Send + 'static>>,
    >,
    pub close:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>>>,
    pub set_title: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ApplicationWindow, String) -> () + Send + 'static>>,
    >,
    pub set_position: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ApplicationWindow, f64, f64) -> () + Send + 'static>>,
    >,
    pub set_size: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ApplicationWindow, f64, f64) -> () + Send + 'static>>,
    >,
    pub get_bounds: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(ApplicationWindow, WindowBounds) -> WindowBounds + Send + 'static>,
        >,
    >,
    pub minimize:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>>>,
    pub maximize:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>>>,
    pub restore:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>>>,
    pub focus:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>>>,
    pub show:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>>>,
    pub hide:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>>>,
    pub center:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>>>,
    pub set_resizable: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ApplicationWindow, bool) -> () + Send + 'static>>,
    >,
    pub set_always_on_top: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ApplicationWindow, bool) -> () + Send + 'static>>,
    >,
    pub set_minimum_size: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ApplicationWindow, f64, f64) -> () + Send + 'static>>,
    >,
    pub set_maximum_size: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ApplicationWindow, f64, f64) -> () + Send + 'static>>,
    >,
    pub set_fullscreen: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ApplicationWindow, bool) -> () + Send + 'static>>,
    >,
    pub set_icon: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ApplicationWindow, String) -> () + Send + 'static>>,
    >,
    pub set_opacity: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ApplicationWindow, f64) -> () + Send + 'static>>,
    >,
    pub set_skip_taskbar: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ApplicationWindow, bool) -> () + Send + 'static>>,
    >,
    pub set_menu_bar_visible: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ApplicationWindow, bool) -> () + Send + 'static>>,
    >,
    pub set_parent: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(ApplicationWindow, Option<ApplicationWindow>) -> () + Send + 'static>,
        >,
    >,
    pub set_progress: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ApplicationWindow, f64) -> () + Send + 'static>>,
    >,
    pub request_attention: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ApplicationWindow, bool) -> () + Send + 'static>>,
    >,
    pub set_content_protection: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ApplicationWindow, bool) -> () + Send + 'static>>,
    >,
    pub flash_window_frame:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ApplicationWindow) -> () + Send + 'static>>>,
    pub set_has_shadow: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ApplicationWindow, bool) -> () + Send + 'static>>,
    >,
}
impl PartialEq for WindowBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
