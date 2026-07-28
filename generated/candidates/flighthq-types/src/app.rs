// @generated from upstream/packages/types/src/App.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/App.ts:7 (sha256:d3f9581fb8f160890538a1b921de3c394e43f6d8c0dffc2a85f2855c87719740)
pub type AppActivationPolicy = String;

// Source: upstream/packages/types/src/App.ts:10 (sha256:0281de8d7faa1c59332d24025a12c05617382864b9ea653e696fc73447d42e38)
#[derive(Clone)]
pub struct AppLoginItem {
    pub open_at_login: bool,
    pub open_as_hidden: bool,
    pub path: String,
    pub args: Vec<String>,
}

// Source: upstream/packages/types/src/App.ts:22 (sha256:b5fa820102a6212ceff5de2018db5d6130b3cac3032ba564f49a3b3cf978f7ee)
#[derive(Clone)]
pub struct AppLoginItemLike {
    pub open_at_login: Option<bool>,
    pub open_as_hidden: Option<bool>,
    pub path: Option<String>,
    pub args: Option<Vec<String>>,
}

// Source: upstream/packages/types/src/App.ts:31 (sha256:919ed3b8ac65b8961138f34a70d769b628ba8dc2cfe7c5f269cb7ff76c134806)
pub type AppPathKind = String;

// Source: upstream/packages/types/src/App.ts:34 (sha256:2b502effeb9c15d29cfd3718ca9f8928961bc84ae53136c2e3bfcc40adc34ea9)
#[derive(Clone)]
pub struct App {
    pub on_activate: Signal,
    pub on_all_windows_closed: Signal,
    pub on_open_file: Signal,
    pub on_quit_request: Signal,
    pub on_ready: Signal,
    pub on_second_instance: Signal,
}

// Source: upstream/packages/types/src/App.ts:46 (sha256:d93c5577047f7340870e7626537a9ebc6c17d5d687125539dfde5cb72ab58886)
#[derive(Clone)]
pub struct AppBackend {
    pub add_recent_document: crate::OpaqueHostValue,
    pub bounce_dock: crate::OpaqueHostValue,
    pub cancel_attention: crate::OpaqueHostValue,
    pub cancel_dock_bounce: crate::OpaqueHostValue,
    pub clear_recent_documents: crate::OpaqueHostValue,
    pub focus: crate::OpaqueHostValue,
    pub get_app_directory_path: crate::OpaqueHostValue,
    pub get_app_path: crate::OpaqueHostValue,
    pub get_command_line: crate::OpaqueHostValue,
    pub get_executable_path: crate::OpaqueHostValue,
    pub get_locale: crate::OpaqueHostValue,
    pub get_login_item: crate::OpaqueHostValue,
    pub get_name: crate::OpaqueHostValue,
    pub get_preferred_system_languages: crate::OpaqueHostValue,
    pub get_system_locale: crate::OpaqueHostValue,
    pub get_version: crate::OpaqueHostValue,
    pub has_single_instance_lock: crate::OpaqueHostValue,
    pub hide_app: crate::OpaqueHostValue,
    pub is_app_hidden: crate::OpaqueHostValue,
    pub quit: crate::OpaqueHostValue,
    pub relaunch: crate::OpaqueHostValue,
    pub release_single_instance_lock: crate::OpaqueHostValue,
    pub request_attention: crate::OpaqueHostValue,
    pub request_single_instance_lock: crate::OpaqueHostValue,
    pub set_activation_policy: crate::OpaqueHostValue,
    pub set_badge_count: crate::OpaqueHostValue,
    pub set_dock_badge: crate::OpaqueHostValue,
    pub set_dock_menu: crate::OpaqueHostValue,
    pub set_login_item: crate::OpaqueHostValue,
    pub set_name: crate::OpaqueHostValue,
    pub set_user_model_id: crate::OpaqueHostValue,
    pub show_app: crate::OpaqueHostValue,
    pub subscribe_activate: crate::OpaqueHostValue,
    pub subscribe_all_windows_closed: crate::OpaqueHostValue,
    pub subscribe_open_file: crate::OpaqueHostValue,
    pub subscribe_quit_request: crate::OpaqueHostValue,
    pub subscribe_ready: crate::OpaqueHostValue,
    pub subscribe_second_instance: crate::OpaqueHostValue,
}
