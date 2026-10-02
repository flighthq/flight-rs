// @generated from upstream/packages/types/src/ElectronApi.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub name: String,
    pub extensions: Vec<String>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for SharedStructuralRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
impl PartialEq for SharedStructuralRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord1135797624 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}
impl PartialEq for FlightPartialRecord1135797624 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:20 (sha256:f54a1342b6e20a6877114b8a64de522c5dd432abe1db067f1d64cf56da7987a5)
#[derive(Clone)]
pub struct ElectronApi {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub app: ElectronApp,
    pub clipboard: ElectronClipboard,
    pub fs: ElectronFs,
    pub shell: ElectronShell,
    pub dialog: ElectronDialog,
    pub global_shortcut: ElectronGlobalShortcut,
    pub screen: ElectronScreen,
    pub power_monitor: ElectronPowerMonitor,
    pub power_save_blocker: ElectronPowerSaveBlocker,
    pub native_image: ElectronNativeImageModule,
    pub ipc_main: ElectronIpcMain,
    pub auto_updater: ElectronAutoUpdater,
    pub browser_window: ElectronBrowserWindowConstructor,
    pub menu: ElectronMenuConstructor,
    pub tray: ElectronTrayConstructor,
    pub notification: ElectronNotificationConstructor,
}
impl PartialEq for ElectronApi {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:42 (sha256:75e11af963ee408526561a32d477e76f81a3bd1c2dcdc94d787dbe2feda068f1)
#[derive(Clone)]
pub struct ElectronFs {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub exists_sync:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub read_file_sync:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String, String) -> String + Send + 'static>>>,
    pub rename_sync:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String, String) -> () + Send + 'static>>>,
    pub unlink_sync:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub write_file_sync:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String, String) -> () + Send + 'static>>>,
}
impl PartialEq for ElectronFs {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:50 (sha256:3e46ec257e59382a6f444397f6b4f1c64be91afd4a4dc30c32ef08331e514e76)
#[derive(Clone)]
pub struct ElectronApp {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_name: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub get_version: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub get_locale: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub get_system_locale:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub get_preferred_system_languages:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Vec<String> + Send + 'static>>>,
    pub get_app_path: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub get_path:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> String + Send + 'static>>>,
    pub set_name: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub set_app_user_model_id:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub set_activation_policy:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub hide: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub show: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub is_hidden: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub add_recent_document:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub clear_recent_documents:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub get_login_item_settings: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> ElectronLoginItemSettings + Send + 'static>>,
    >,
    pub set_login_item_settings: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ElectronLoginItemSettingsLike) -> () + Send + 'static>>,
    >,
    pub quit: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub exit: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Option<f64>) -> () + Send + 'static>>>,
    pub relaunch: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub focus: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub request_single_instance_lock:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub has_single_instance_lock:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub release_single_instance_lock:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub set_badge_count:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> bool + Send + 'static>>>,
    pub set_as_default_protocol_client:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub remove_as_default_protocol_client:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub is_default_protocol_client:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub on: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(Vec<crate::FlightValue>) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub remove_listener: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(Vec<crate::FlightValue>) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub dock: Option<ElectronDock>,
}
impl PartialEq for ElectronApp {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:85 (sha256:f16dca8a64510b52937957ae11ce1e642946e1b3e7b8a9b9cc8ac5004885887f)
#[derive(Clone, Default)]
pub struct ElectronLoginItemSettings {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub open_at_login: bool,
    pub open_as_hidden: bool,
    pub executable_will_launch_at_login: Option<bool>,
}
impl PartialEq for ElectronLoginItemSettings {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:91 (sha256:c976436373f06063fc0a589624ab0ca6b6278a3844b3eb9f38d0d3464054d989)
#[derive(Clone, Default)]
pub struct ElectronLoginItemSettingsLike {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub open_at_login: Option<bool>,
    pub open_as_hidden: Option<bool>,
    pub path: Option<String>,
    pub args: Option<Vec<String>>,
}
impl PartialEq for ElectronLoginItemSettingsLike {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:98 (sha256:1cf5766e007f376ec3c137bd8eed5b25ca5447cd65b7ffc48d3d8d09390f80d6)
#[derive(Clone)]
pub struct ElectronDock {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bounce:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Option<String>) -> f64 + Send + 'static>>>,
    pub cancel_bounce: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>,
    pub set_badge: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub set_menu:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ElectronMenu) -> () + Send + 'static>>>,
}
impl PartialEq for ElectronDock {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:105 (sha256:c2d10e88459de0e61a174f937a8e8a2afa1c839ca60a902b78d3dfffae2000bf)
#[derive(Clone, Default)]
pub struct ElectronClipboardRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub title: String,
    pub url: String,
}
impl PartialEq for ElectronClipboardRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct ElectronClipboard {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub read_text: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub write_text: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub read_html: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub write_html: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub read_rtf: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub write_rtf: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub read_bookmark: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> ElectronClipboardRecord1 + Send + 'static>>,
    >,
    pub write_bookmark:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String, String) -> () + Send + 'static>>>,
    pub read_image:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> ElectronNativeImage + Send + 'static>>>,
    pub write_image: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ElectronNativeImage) -> () + Send + 'static>>,
    >,
    pub read: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> String + Send + 'static>>>,
    pub write: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ElectronClipboardData) -> () + Send + 'static>>,
    >,
    pub has: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub available_formats:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Vec<String> + Send + 'static>>>,
    pub clear: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for ElectronClipboard {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:123 (sha256:55cc8e0cb47173522baec789cddbc9f5940e031cec263b7b3f9f218d0c0b509b)
#[derive(Clone, Default)]
pub struct ElectronClipboardData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub text: Option<String>,
    pub html: Option<String>,
    pub rtf: Option<String>,
    pub bookmark: Option<String>,
    pub image: Option<ElectronNativeImage>,
}
impl PartialEq for ElectronClipboardData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:131 (sha256:1c7c8b442be860f2d165cdcd41313ebdfd3b0f018846d39f38fcbe968dcec719)
#[derive(Clone)]
pub struct ElectronShell {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub open_external: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub open_path: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<String> + Send + 'static>>,
    >,
    pub show_item_in_folder:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub trash_item: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub beep: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub read_shortcut_link: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> ElectronShortcutDetails + Send + 'static>>,
    >,
    pub write_shortcut_link: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String, String, ElectronShortcutDetails) -> bool + Send + 'static>,
        >,
    >,
}
impl PartialEq for ElectronShell {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:145 (sha256:87f8b1e86de98aa0ff5e21e3aae28d222fd0ef695acd5861eaf9325a83487aaa)
#[derive(Clone, Default)]
pub struct ElectronShortcutDetails {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub target: String,
    pub app_user_model_id: Option<String>,
    pub args: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub icon_index: Option<f64>,
    pub cwd: Option<String>,
}
impl PartialEq for ElectronShortcutDetails {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:155 (sha256:95a800bbf1d57a449fccd7abfeb872c00f6159899cf1252f6edcd7aab5ac2e34)
#[derive(Clone, Default)]
pub struct ElectronDialogRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub response: f64,
    pub checkbox_checked: bool,
}
impl PartialEq for ElectronDialogRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ElectronDialogRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub canceled: bool,
    pub file_path: Option<String>,
}
impl PartialEq for ElectronDialogRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ElectronDialogRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub canceled: bool,
    pub file_paths: Vec<String>,
}
impl PartialEq for ElectronDialogRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct ElectronDialog {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub show_open_dialog: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Option<ElectronBrowserWindow>,
                        ElectronOpenDialogOptions,
                    ) -> crate::FlightTask<ElectronDialogRecord3>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub show_save_dialog: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Option<ElectronBrowserWindow>,
                        ElectronSaveDialogOptions,
                    ) -> crate::FlightTask<ElectronDialogRecord2>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub show_message_box: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Option<ElectronBrowserWindow>,
                        ElectronMessageBoxOptions,
                    ) -> crate::FlightTask<ElectronDialogRecord1>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for ElectronDialog {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:170 (sha256:6679fd8531da5f74cf89ad8c4433b53c012d682b1672324d6550619e615acfa6)
#[derive(Clone, Default)]
pub struct ElectronOpenDialogOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub title: Option<String>,
    pub default_path: Option<String>,
    pub filters: Option<Vec<SharedStructuralRecord1>>,
    pub properties: Option<Vec<String>>,
}
impl PartialEq for ElectronOpenDialogOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:177 (sha256:26df83cb5755d4915f53e38aca497142a72f2516cc32367319248c98b1c6615a)
#[derive(Clone, Default)]
pub struct ElectronSaveDialogOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub title: Option<String>,
    pub default_path: Option<String>,
    pub filters: Option<Vec<SharedStructuralRecord1>>,
}
impl PartialEq for ElectronSaveDialogOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:183 (sha256:736f6b3d3c430fb3b2e2278465497b81f7692f7ab104b34e46cb9513bca1aaab)
#[derive(Clone, Default)]
pub struct ElectronMessageBoxOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub type_: Option<String>,
    pub title: Option<String>,
    pub message: String,
    pub detail: Option<String>,
    pub buttons: Option<Vec<String>>,
    pub default_id: Option<f64>,
    pub cancel_id: Option<f64>,
    pub signal: Option<crate::OpaqueHostValue>,
    pub checkbox_label: Option<String>,
    pub checkbox_checked: Option<bool>,
}
impl PartialEq for ElectronMessageBoxOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:196 (sha256:f2f22fb5e843a2376a6bcc876d42857d7431881c9d648e5b546ba23fdacd36fa)
#[derive(Clone)]
pub struct ElectronGlobalShortcut {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub register: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub unregister: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub unregister_all: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub is_registered:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
}
impl PartialEq for ElectronGlobalShortcut {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:203 (sha256:fc210c01a2d8820717d5d48026d38d2445adc1a96bb518822d5546549e37ceef)
#[derive(Clone)]
pub struct ElectronScreen {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_primary_display:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> ElectronDisplay + Send + 'static>>>,
    pub get_all_displays:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Vec<ElectronDisplay> + Send + 'static>>>,
    pub get_cursor_screen_point: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> SharedStructuralRecord2 + Send + 'static>>,
    >,
    pub on: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(Vec<crate::FlightValue>) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub remove_listener: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(Vec<crate::FlightValue>) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for ElectronScreen {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:211 (sha256:021cb40f60ced0ae90a2bcc5ab9547ce29c493b006f71521ee347cd68bfc34fa)
#[derive(Clone, Default)]
pub struct ElectronDisplay {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: f64,
    pub bounds: SharedStructuralRecord3,
    pub work_area: SharedStructuralRecord3,
    pub scale_factor: f64,
    pub color_depth: Option<f64>,
    pub color_space: Option<String>,
    pub display_frequency: Option<f64>,
    pub internal: Option<bool>,
    pub label: Option<String>,
    pub monochrome: Option<bool>,
    pub rotation: Option<f64>,
    pub touch_support: Option<String>,
}
impl PartialEq for ElectronDisplay {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:229 (sha256:3c7d0794ea10a03e51df6a0f1b58278a037193a10b0bb9bbef4002e0b0b86be3)
pub type ElectronPowerMonitorEvent = String;

// Source: upstream/packages/types/src/ElectronApi.ts:238 (sha256:60d0b0fa2f62f7c0d2ce2fc2f048646bb44f7e3da830df9fc502f5d756b1bc4e)
#[derive(Clone)]
pub struct ElectronPowerMonitor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        ElectronPowerMonitorEvent,
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub remove_listener: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        ElectronPowerMonitorEvent,
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub get_current_thermal_state:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>>,
    pub get_system_idle_state:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> String + Send + 'static>>>,
    pub get_system_idle_time:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>>,
    pub on_battery_power: Option<bool>,
}
impl PartialEq for ElectronPowerMonitor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:249 (sha256:7cbeef78e93c210affc14d15c20591cf956207433994019a1a4d9eb83acdc83c)
#[derive(Clone)]
pub struct ElectronPowerSaveBlocker {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub start: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> f64 + Send + 'static>>>,
    pub stop: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>,
    pub is_started: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> bool + Send + 'static>>>,
}
impl PartialEq for ElectronPowerSaveBlocker {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:255 (sha256:165f23f609737eb54b17d93f6a8c6f5d075a548b9302fd464d751c951ebbcd89)
#[derive(Clone)]
pub struct ElectronNativeImageModule {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub create_from_data_url: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> ElectronNativeImage + Send + 'static>>,
    >,
    pub create_from_path: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> ElectronNativeImage + Send + 'static>>,
    >,
}
impl PartialEq for ElectronNativeImageModule {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:260 (sha256:a04938d5f3d8792ac42960f0fbfa3ec49b27dd03fd8daa286e22e10ed791f04d)
#[derive(Clone)]
pub struct ElectronNativeImage {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub to_data_url: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub is_empty: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub set_template_image:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) -> () + Send + 'static>>>,
}
impl PartialEq for ElectronNativeImage {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:266 (sha256:a3e022b01f8ff3799798939fa9dfe3c8b45a3b3b6d0226dc27609e95b5a8ebba)
#[derive(Clone)]
pub struct ElectronIpcMain {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<
                                    dyn FnMut(crate::FlightValue, Vec<crate::FlightValue>) -> ()
                                        + Send
                                        + 'static,
                                >,
                            >,
                        >,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub remove_listener: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<
                                    dyn FnMut(crate::FlightValue, Vec<crate::FlightValue>) -> ()
                                        + Send
                                        + 'static,
                                >,
                            >,
                        >,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub handle: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<
                                    dyn FnMut(
                                            crate::FlightValue,
                                            Vec<crate::FlightValue>,
                                        )
                                            -> crate::FlightValue
                                        + Send
                                        + 'static,
                                >,
                            >,
                        >,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub remove_handler:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
}
impl PartialEq for ElectronIpcMain {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:275 (sha256:77df5d688ba61522868f14ee6de3e5e19974117868dfeb559225065105220ac4)
#[derive(Clone)]
pub struct ElectronIpcRenderer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub invoke: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String, Vec<crate::FlightValue>) -> crate::FlightTask<crate::FlightValue>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub send: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String, Vec<crate::FlightValue>) -> () + Send + 'static>>,
    >,
}
impl PartialEq for ElectronIpcRenderer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:282 (sha256:f6629ad22fd41a9e066d157981181b900df1648cb92f41875acb1bd50f99b488)
#[derive(Clone)]
pub struct ElectronIpcTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub send: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String, Vec<crate::FlightValue>) -> () + Send + 'static>>,
    >,
}
impl PartialEq for ElectronIpcTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:286 (sha256:af9b44b45fd162436246f44f47fbd690e6070ac46912849ddeb93bdb4b92ed0c)
#[derive(Clone, Default)]
pub struct ElectronAutoUpdaterRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub url: String,
}
impl PartialEq for ElectronAutoUpdaterRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct ElectronAutoUpdater {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_feed_url: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ElectronAutoUpdaterRecord1) -> () + Send + 'static>>,
    >,
    pub check_for_updates:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub quit_and_install: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub on: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(Vec<crate::FlightValue>) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub remove_listener: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(Vec<crate::FlightValue>) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for ElectronAutoUpdater {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:294 (sha256:81548540680a9eb50124904aca9e5de3bf1f57a38972d526b4c36b1481df7b74)
#[derive(Clone)]
pub struct ElectronBrowserWindowConstructor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub __construct: Option<crate::OpaqueHostValue>,
    pub get_all_windows: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> Vec<ElectronBrowserWindow> + Send + 'static>>,
    >,
    pub from_id: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64) -> Option<ElectronBrowserWindow> + Send + 'static>>,
    >,
}
impl PartialEq for ElectronBrowserWindowConstructor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:300 (sha256:4ee853ac9175adb5aa59626ef098bbe401ea50a9aa6232495be4cc12f08b6759)
#[derive(Clone, Default)]
pub struct ElectronBrowserWindowOptions {
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
    pub show: Option<bool>,
    pub min_width: Option<f64>,
    pub min_height: Option<f64>,
    pub max_width: Option<f64>,
    pub max_height: Option<f64>,
    pub frame: Option<bool>,
    pub transparent: Option<bool>,
}
impl PartialEq for ElectronBrowserWindowOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:318 (sha256:b6d1c737450f66fa221301095025d2d5cc6e89f73afe45f736ec72dc3c1b39eb)
#[derive(Clone, Default)]
pub struct ElectronRectangle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
impl PartialEq for ElectronRectangle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:325 (sha256:bead4386486ae947f8d57991302cc1e8ab46a26de23a0f85335490c4f0b6677c)
#[derive(Clone)]
pub struct ElectronBrowserWindow {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: f64,
    pub load_url: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub load_file: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub set_title: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub get_title: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub set_position:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>,
    pub set_size: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>,
    pub get_bounds:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> ElectronRectangle + Send + 'static>>>,
    pub set_bounds: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(FlightPartialRecord1135797624) -> () + Send + 'static>>,
    >,
    pub minimize: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub maximize: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub unmaximize: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub restore: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub is_minimized: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub is_maximized: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub focus: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub show: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub hide: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub center: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub set_resizable:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) -> () + Send + 'static>>>,
    pub set_always_on_top:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) -> () + Send + 'static>>>,
    pub set_minimum_size:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>,
    pub set_maximum_size:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>,
    pub set_full_screen:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) -> () + Send + 'static>>>,
    pub is_full_screen: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub set_icon: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(crate::FlightUnion2<String, ElectronNativeImage>) -> () + Send + 'static>,
        >,
    >,
    pub set_opacity: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>,
    pub set_progress_bar:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>,
    pub flash_frame: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) -> () + Send + 'static>>>,
    pub set_content_protection:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) -> () + Send + 'static>>>,
    pub set_has_shadow:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) -> () + Send + 'static>>>,
    pub set_skip_taskbar:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) -> () + Send + 'static>>>,
    pub set_menu_bar_visibility:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) -> () + Send + 'static>>>,
    pub set_parent_window: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Option<ElectronBrowserWindow>) -> () + Send + 'static>>,
    >,
    pub close: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub destroy: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub is_destroyed: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub on: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(Vec<crate::FlightValue>) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub off: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(Vec<crate::FlightValue>) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub remove_all_listeners:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Option<String>) -> () + Send + 'static>>>,
}
impl PartialEq for ElectronBrowserWindow {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:370 (sha256:15d49d7434da77722a9aefa0f8a302076546eb87294860bc811bebf51aa3beb9)
#[derive(Clone)]
pub struct ElectronMenuConstructor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub __construct: Option<crate::OpaqueHostValue>,
    pub build_from_template: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Vec<ElectronMenuItemOptions>) -> ElectronMenu + Send + 'static>,
        >,
    >,
    pub set_application_menu: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Option<ElectronMenu>) -> () + Send + 'static>>,
    >,
}
impl PartialEq for ElectronMenuConstructor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:376 (sha256:0274aea886bef8b32ee3250ce1253c4c6bd44762e30ea3b78d96754109f0d941)
#[derive(Clone, Default)]
pub struct ElectronMenuRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}
impl PartialEq for ElectronMenuRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct ElectronMenu {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub popup: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Option<ElectronMenuRecord1>) -> () + Send + 'static>>,
    >,
}
impl PartialEq for ElectronMenu {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:380 (sha256:525b6aa03f442c444cfbf86f1e5344387e15e10ff940082398b36053c63502a4)
#[derive(Clone, Default)]
pub struct ElectronMenuItemOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: Option<String>,
    pub label: Option<String>,
    pub type_: Option<String>,
    pub role: Option<ElectronMenuItemRole>,
    pub accelerator: Option<String>,
    pub enabled: Option<bool>,
    pub checked: Option<bool>,
    pub click: Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub submenu: Option<Vec<ElectronMenuItemOptions>>,
}
impl PartialEq for ElectronMenuItemOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:392 (sha256:02e836b781698a458feb57bc7601e2d9faad867b10aa6c2ebb72a019eb60bb83)
pub type ElectronMenuItemRole = String;

// Source: upstream/packages/types/src/ElectronApi.ts:438 (sha256:2d53e9b7bc654216df32ecd30d6dc22c6655e51f7821e164c6c7a0928f21488b)
#[derive(Clone, Default)]
pub struct ElectronTrayConstructor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub __construct: Option<crate::OpaqueHostValue>,
}
impl PartialEq for ElectronTrayConstructor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:442 (sha256:533efb026d41c8a9b40da9e3989dcd5238c20fbc55c1118832f961aefe6e50b8)
#[derive(Clone)]
pub struct ElectronTray {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_tool_tip:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub set_title: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub set_image: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(crate::FlightUnion2<String, ElectronNativeImage>) -> () + Send + 'static>,
        >,
    >,
    pub set_pressed_image: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(crate::FlightUnion2<String, ElectronNativeImage>) -> () + Send + 'static>,
        >,
    >,
    pub set_context_menu: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Option<ElectronMenu>) -> () + Send + 'static>>,
    >,
    pub pop_up_context_menu: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Option<ElectronMenu>, Option<SharedStructuralRecord2>) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub set_ignore_double_click_events:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(bool) -> () + Send + 'static>>>,
    pub display_balloon: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(ElectronTrayBalloonOptions) -> () + Send + 'static>>,
    >,
    pub remove_balloon: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub get_bounds:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> ElectronRectangle + Send + 'static>>>,
    pub is_destroyed: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
    pub on: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(Vec<crate::FlightValue>) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub remove_listener: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(Vec<crate::FlightValue>) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub destroy: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for ElectronTray {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:459 (sha256:0d2f0683c620c6b7ef32add363313d448601146da2949cb8a59e01ddc7d41c10)
#[derive(Clone, Default)]
pub struct ElectronTrayBalloonOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub icon: Option<crate::FlightUnion2<String, ElectronNativeImage>>,
    pub icon_type: Option<String>,
    pub title: String,
    pub content: String,
    pub large_icon: Option<bool>,
    pub no_sound: Option<bool>,
    pub respect_quiet_time: Option<bool>,
}
impl PartialEq for ElectronTrayBalloonOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:469 (sha256:95eaa8c7e991c382c61a92921f11e61f6cfac916d055e3c2a7ade0d98b37bd62)
#[derive(Clone)]
pub struct ElectronNotificationConstructor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub __construct: Option<crate::OpaqueHostValue>,
    pub is_supported: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> bool + Send + 'static>>>,
}
impl PartialEq for ElectronNotificationConstructor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:474 (sha256:bc4df506aa878ff940de4e0e443a9e61273e6469794799381c148329b66e83b6)
#[derive(Clone, Default)]
pub struct ElectronNotificationOptionsRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub text: String,
    pub type_: String,
}
impl PartialEq for ElectronNotificationOptionsRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct ElectronNotificationOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub title: String,
    pub body: Option<String>,
    pub icon: Option<String>,
    pub silent: Option<bool>,
    pub actions: Option<Vec<ElectronNotificationOptionsRecord1>>,
    pub has_reply: Option<bool>,
}
impl PartialEq for ElectronNotificationOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronApi.ts:483 (sha256:0d989ba938f51f702d3804b0618a355aeddc70312302547941a2fc803158af34)
#[derive(Clone)]
pub struct ElectronNotification {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub show: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub close: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub on: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(Vec<crate::FlightValue>) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for ElectronNotification {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
