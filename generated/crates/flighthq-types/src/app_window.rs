// @generated from upstream/packages/types/src/AppWindow.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Signal};

// Source: upstream/packages/types/src/AppWindow.ts:4 (sha256:25b033522ad7e81269bb2d71bddf0c9bca3ec9748ff5cd32b5b3a3aec4e3b4f4)
#[derive(Clone)]
pub struct AppWindow {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
impl PartialEq for AppWindow {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for AppWindow {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/AppWindow.ts:54 (sha256:f1bd48dc8b90c3350a7eb428b57ee99d2442cdfe213fbca3d6c341b0140541e3)
#[derive(Clone, Default)]
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

// Source: upstream/packages/types/src/AppWindow.ts:77 (sha256:c4283b158481445c344429a7d156350e5c7de857955bfe2d32de79a101df7ec7)
#[derive(Clone, Default)]
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

// Source: upstream/packages/types/src/AppWindow.ts:87 (sha256:7e0f3c467ced203f9992f2f3aca4c665a5f52f03f0de25ac170f44b4d3db5db0)
pub type NativeWindowHandle = crate::FlightValue;

// Source: upstream/packages/types/src/AppWindow.ts:91 (sha256:cffed2a685aa8b4b47c76714f346eb8358e5d1e40bf6f517811ae87dc7d92f02)
pub type WindowAttachmentOwnership = String;

// Source: upstream/packages/types/src/AppWindow.ts:95 (sha256:891511b01007146a7371ee5420c0a0a34198868289e258a74a8c71ebf1900d92)
#[derive(Clone, Default)]
pub struct WindowResizeTargetHandle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub __brand: String,
}
impl PartialEq for WindowResizeTargetHandle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for WindowResizeTargetHandle {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/AppWindow.ts:97 (sha256:5ed600d04a793bff33652ce0c400a98c42b61bbe1d0e00a9483ab3f47132029e)
#[derive(Clone)]
pub struct HostWindowAppearanceCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_icon: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow, String) -> () + Send + 'static>>>,
    >,
    pub set_opacity: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow, f64) -> () + Send + 'static>>>,
    >,
    pub set_title:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow, String) -> () + Send + 'static>>>,
}
impl PartialEq for HostWindowAppearanceCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppWindow.ts:103 (sha256:c080f2d3d2c8b7e3262cafff5c2108efb27245d65605445b78b2ba0b30698924)
#[derive(Clone)]
pub struct HostWindowAttachCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attach: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(AppWindow, NativeWindowHandle, WindowAttachmentOwnership) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostWindowAttachCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppWindow.ts:107 (sha256:cd2dfd2f8eef169d4b6ce0cc50e12d318086ae09b953200483d312e8b001c60a)
#[derive(Clone)]
pub struct HostWindowAttentionCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub flash_window_frame:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow) -> () + Send + 'static>>>>,
    pub request_attention:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow, bool) -> () + Send + 'static>>>,
}
impl PartialEq for HostWindowAttentionCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppWindow.ts:112 (sha256:136d774748dd7ba446ccc84f9f97faeca5e6afcf8101b23aa670c83561184a74)
#[derive(Clone)]
pub struct HostWindowContentProtectionCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_content_protection:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow, bool) -> () + Send + 'static>>>,
}
impl PartialEq for HostWindowContentProtectionCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppWindow.ts:116 (sha256:aec3520c7f3db3a70c927df4d5a7eaddc55033351139dfae08ca5d3ba2c94584)
#[derive(Clone)]
pub struct HostWindowFocusCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub focus: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow) -> () + Send + 'static>>>,
}
impl PartialEq for HostWindowFocusCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppWindow.ts:120 (sha256:0f056809059e72379045f2b05fc9eb9dd4579c4ade21885b66b8fd3843384dc2)
#[derive(Clone)]
pub struct HostWindowFullscreenCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_fullscreen:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow, bool) -> () + Send + 'static>>>,
}
impl PartialEq for HostWindowFullscreenCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppWindow.ts:124 (sha256:1732725bbf5468f5f51bcfcde80f760e15680bc0db68aa343bfa77635f7b5981)
#[derive(Clone)]
pub struct HostWindowGeometryCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub center:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow) -> () + Send + 'static>>>>,
    pub get_bounds: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(AppWindow, WindowBounds) -> WindowBounds + Send + 'static>>,
    >,
    pub set_position: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(AppWindow, f64, f64) -> () + Send + 'static>>,
    >,
    pub set_size: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(AppWindow, f64, f64) -> () + Send + 'static>>,
    >,
    pub subscribe_move: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub subscribe_resize: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            WindowResizeTargetHandle,
                            std::sync::Arc<
                                std::sync::Mutex<
                                    Box<dyn FnMut(f64, f64, f64) -> () + Send + 'static>,
                                >,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
            >,
        >,
    >,
}
impl PartialEq for HostWindowGeometryCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppWindow.ts:136 (sha256:bda857e9ef5ad3180b98571aa67e78d2c6f4090dfdc2f36d89463f6663b8b17c)
#[derive(Clone)]
pub struct HostWindowHierarchyCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_parent: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(AppWindow, Option<AppWindow>) -> () + Send + 'static>>,
    >,
}
impl PartialEq for HostWindowHierarchyCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppWindow.ts:140 (sha256:46611c5cd1d9bc1e9557eccbfbc399d9e58ba260a3cd93484a9dfc9f4c8dd5c0)
#[derive(Clone)]
pub struct HostWindowLifecycleCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub close: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow) -> () + Send + 'static>>>,
    pub open: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(AppWindow, WindowOptions) -> bool + Send + 'static>>,
    >,
    pub subscribe_close: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>,
                            >,
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
            >,
        >,
    >,
}
impl PartialEq for HostWindowLifecycleCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppWindow.ts:146 (sha256:eeedf3f250ee94d9234319b3999a807a4a504d5529596b259a4106f4d54316ad)
#[derive(Clone)]
pub struct HostWindowProgressCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_progress:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow, f64) -> () + Send + 'static>>>,
}
impl PartialEq for HostWindowProgressCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppWindow.ts:150 (sha256:22d25fab26b4e0cd49e4569a15e27eb00ed77fe62d676bcbfd6a56208d016406)
#[derive(Clone)]
pub struct HostWindowShadowCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_has_shadow:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow, bool) -> () + Send + 'static>>>,
}
impl PartialEq for HostWindowShadowCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppWindow.ts:154 (sha256:f68ee2d42c27869fe2c44c4dbcbde5ca3daf127f1fb7e86e92e8eb03472a73db)
#[derive(Clone, Default)]
pub struct HostWindowShellCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_menu_bar_visible: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow, bool) -> () + Send + 'static>>>,
    >,
    pub set_skip_taskbar: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow, bool) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for HostWindowShellCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppWindow.ts:159 (sha256:2ea961e5a5e44cef8cea9df2957efa7e3f007d7f1e5600e17517d94f92cb8163)
#[derive(Clone)]
pub struct HostWindowSizeConstraintsCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_maximum_size: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(AppWindow, f64, f64) -> () + Send + 'static>>,
    >,
    pub set_minimum_size: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(AppWindow, f64, f64) -> () + Send + 'static>>,
    >,
    pub set_resizable: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow, bool) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for HostWindowSizeConstraintsCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppWindow.ts:167 (sha256:4908cc5187289ca57f8154a90e7a7077cfdec98d8154d770c7f821c0a6af0515)
#[derive(Clone)]
pub struct HostWindowStateCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub maximize:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow) -> () + Send + 'static>>>,
    pub minimize:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow) -> () + Send + 'static>>>,
    pub restore: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow) -> () + Send + 'static>>>,
}
impl PartialEq for HostWindowStateCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppWindow.ts:173 (sha256:d002ddfddf0979872a7f6c4283e4bbcad331c8c3a04a849204bc213425ca171b)
#[derive(Clone)]
pub struct HostWindowVisibilityCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hide: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow) -> () + Send + 'static>>>,
    pub show: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow) -> () + Send + 'static>>>,
    pub subscribe_visibility: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut(bool) -> () + Send + 'static>>,
                            >,
                        ) -> std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                        > + Send
                        + 'static,
                >,
            >,
        >,
    >,
}
impl PartialEq for HostWindowVisibilityCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppWindow.ts:179 (sha256:8358e87c7f039140fc58657897026f0597f4fb031fd08b39a2d2426b034708b4)
#[derive(Clone)]
pub struct HostWindowZOrderCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_always_on_top:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppWindow, bool) -> () + Send + 'static>>>,
}
impl PartialEq for HostWindowZOrderCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
