// @generated from upstream/packages/types/src/App.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{MenuItemTemplate, Signal};

// Source: upstream/packages/types/src/App.ts:7 (sha256:d3f9581fb8f160890538a1b921de3c394e43f6d8c0dffc2a85f2855c87719740)
pub type AppActivationPolicy = String;

// Source: upstream/packages/types/src/App.ts:10 (sha256:0281de8d7faa1c59332d24025a12c05617382864b9ea653e696fc73447d42e38)
#[derive(Clone, Default)]
pub struct AppLoginItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub open_at_login: bool,
    pub open_as_hidden: bool,
    pub path: String,
    pub args: Vec<String>,
}
impl PartialEq for AppLoginItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:22 (sha256:b5fa820102a6212ceff5de2018db5d6130b3cac3032ba564f49a3b3cf978f7ee)
#[derive(Clone, Default)]
pub struct AppLoginItemLike {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub open_at_login: Option<bool>,
    pub open_as_hidden: Option<bool>,
    pub path: Option<String>,
    pub args: Option<Vec<String>>,
}
impl PartialEq for AppLoginItemLike {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:31 (sha256:919ed3b8ac65b8961138f34a70d769b628ba8dc2cfe7c5f269cb7ff76c134806)
pub type AppPathKind = String;

// Source: upstream/packages/types/src/App.ts:34 (sha256:2b502effeb9c15d29cfd3718ca9f8928961bc84ae53136c2e3bfcc40adc34ea9)
#[derive(Clone)]
pub struct App {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_activate:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_all_windows_closed:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_open_file:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>>,
    pub on_quit_request:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_ready: Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_second_instance: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Vec<String>) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for App {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/App.ts:46 (sha256:d93c5577047f7340870e7626537a9ebc6c17d5d687125539dfde5cb72ab58886)
#[derive(Clone)]
pub struct AppBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub add_recent_document:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub bounce_dock: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>>,
    pub cancel_attention:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>,
    pub cancel_dock_bounce:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>,
    pub clear_recent_documents:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub focus: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub get_app_directory_path:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppPathKind) -> String + Send + 'static>>>,
    pub get_app_path: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub get_command_line:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Vec<String> + Send + 'static>>>,
    pub get_executable_path:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub get_locale: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub get_login_item:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> AppLoginItem + Send + 'static>>>,
    pub get_name: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub get_preferred_system_languages:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Vec<String> + Send + 'static>>>,
    pub get_system_locale:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub get_version: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub has_single_instance_lock:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub hide_app: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub is_app_hidden: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub quit: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub relaunch: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub release_single_instance_lock:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub request_attention:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) -> f64 + Send + 'static>>>,
    pub request_single_instance_lock:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub set_activation_policy: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(AppActivationPolicy) -> () + Send + 'static>>,
    >,
    pub set_badge_count:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> bool + Send + 'static>>>,
    pub set_dock_badge:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub set_dock_menu: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Vec<MenuItemTemplate>) -> () + Send + 'static>>,
    >,
    pub set_login_item:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(AppLoginItemLike) -> bool + Send + 'static>>>,
    pub set_name: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub set_user_model_id:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub show_app: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub subscribe_activate: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_all_windows_closed: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_open_file: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_quit_request: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<
                                    dyn FnMut(
                                            std::sync::Arc<
                                                std::sync::Mutex<
                                                    Box<dyn FnMut() -> () + Send + 'static>,
                                                >,
                                            >,
                                        ) -> ()
                                        + Send
                                        + 'static,
                                >,
                            >,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_ready: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_second_instance: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(Vec<String>) -> () + Send + 'static>>,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for AppBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
