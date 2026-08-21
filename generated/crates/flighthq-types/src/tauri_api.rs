// @generated from upstream/packages/types/src/TauriApi.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub __construct: Option<crate::OpaqueHostValue>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:25 (sha256:44bcb932c0e88a332e71c3d85aecb67af098ff10aec46f4e8e656678fcc4e7e7)
#[derive(Clone)]
pub struct TauriApi {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub app: TauriAppModule,
    pub clipboard: TauriClipboardManager,
    pub dialog: TauriDialogPlugin,
    pub global_shortcut: TauriGlobalShortcutPlugin,
    pub menu: TauriMenuModule,
    pub notification: TauriNotificationPlugin,
    pub opener: TauriOpenerPlugin,
    pub os: TauriOsModule,
    pub process: TauriProcessPlugin,
    pub tray: TauriTrayModule,
    pub window: TauriWindowModule,
}
impl PartialEq for TauriApi {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:41 (sha256:08ba953ebe17678812ac5f1d64dbc33f14efaf9220a0f1cee1e2230703b4bab2)
#[derive(Clone)]
pub struct TauriAppModule {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_name: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<String> + Send + 'static>>,
    >,
    pub get_version: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<String> + Send + 'static>>,
    >,
    pub hide: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub show: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
}
impl PartialEq for TauriAppModule {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:49 (sha256:35324ce52779be833b631b8d82def6f0ce825a591689017d192cac5714c3be88)
#[derive(Clone)]
pub struct TauriProcessPlugin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub exit: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Option<f64>) -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub relaunch: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
}
impl PartialEq for TauriProcessPlugin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:57 (sha256:bf7fd8824727c8c3f680374dbfe73273055779ffad9068b3ff1d35c44df31671)
#[derive(Clone)]
pub struct TauriClipboardManager {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub clear: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub read_text: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<String> + Send + 'static>>,
    >,
    pub write_text: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<()> + Send + 'static>>,
    >,
}
impl PartialEq for TauriClipboardManager {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:63 (sha256:848104b3231dd7fe86e34964bd7bcb006970d585f8d000e8ce1e6767fb80981e)
#[derive(Clone, Default)]
pub struct TauriDialogFilter {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub extensions: Vec<String>,
    pub name: String,
}
impl PartialEq for TauriDialogFilter {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:68 (sha256:22ed44bcb96b8f67cd39fe84a77893cf46a6ee2cd208e8c70f749556d6500b2d)
#[derive(Clone, Default)]
pub struct TauriDialogOpenOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub default_path: Option<String>,
    pub directory: Option<bool>,
    pub filters: Option<Vec<TauriDialogFilter>>,
    pub multiple: Option<bool>,
    pub title: Option<String>,
}
impl PartialEq for TauriDialogOpenOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:76 (sha256:79b0f0a9802bd9ba8f4b15fd807719090ec911fac7c4ff60102a310c417cfc02)
#[derive(Clone, Default)]
pub struct TauriDialogSaveOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub default_path: Option<String>,
    pub filters: Option<Vec<TauriDialogFilter>>,
    pub title: Option<String>,
}
impl PartialEq for TauriDialogSaveOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:82 (sha256:a5dad1a143a0d23700b36a5760065d93fdc9e08d0bf6dcd3ac27ea699cc67704)
#[derive(Clone, Default)]
pub struct TauriDialogMessageOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cancel_label: Option<String>,
    pub kind: Option<String>,
    pub ok_label: Option<String>,
    pub title: Option<String>,
}
impl PartialEq for TauriDialogMessageOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:91 (sha256:12394dbfbb0cc34c339c5cb83c02aa4d0d0ba5a4026b573f1889e97c0c85bf4e)
#[derive(Clone)]
pub struct TauriDialogPlugin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ask: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String, Option<TauriDialogMessageOptions>) -> crate::FlightTask<bool>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub confirm: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String, Option<TauriDialogMessageOptions>) -> crate::FlightTask<bool>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub message: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String, Option<TauriDialogMessageOptions>) -> crate::FlightTask<()>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub open: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Option<TauriDialogOpenOptions>,
                    )
                        -> crate::FlightTask<Option<crate::FlightUnion2<String, Vec<String>>>>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub save: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Option<TauriDialogSaveOptions>) -> crate::FlightTask<Option<String>>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for TauriDialogPlugin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:99 (sha256:77b9cd06deee3e51f9616500425b499e45c4e8648bd6ee11130d5d4050dceae8)
pub type TauriNotificationPermission = String;

// Source: upstream/packages/types/src/TauriApi.ts:101 (sha256:f2414ca446a1db6aa5010db8fe98cd38e538cf9efe1aa6d4ee6953c682d69374)
#[derive(Clone, Default)]
pub struct TauriNotificationOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body: Option<String>,
    pub icon: Option<String>,
    pub title: String,
}
impl PartialEq for TauriNotificationOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:110 (sha256:83c32c6416a1a0cb682c64a7194b97bd8b2a72f5d2d019e5afbdff503b45f82e)
#[derive(Clone)]
pub struct TauriNotificationPlugin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub is_permission_granted: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<bool> + Send + 'static>>,
    >,
    pub request_permission: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<TauriNotificationPermission> + Send + 'static>,
        >,
    >,
    pub send_notification: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(TauriNotificationOptions) -> () + Send + 'static>>,
    >,
}
impl PartialEq for TauriNotificationPlugin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:118 (sha256:4d6a0b7cef7e70e5ac1f13b54957ce8a0725636c97dd50ddea63064f93e966a3)
#[derive(Clone)]
pub struct TauriOpenerPlugin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub open_path: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String, Option<String>) -> crate::FlightTask<()> + Send + 'static>,
        >,
    >,
    pub open_url: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String, Option<String>) -> crate::FlightTask<()> + Send + 'static>,
        >,
    >,
    pub reveal_item_in_dir: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<()> + Send + 'static>>,
    >,
}
impl PartialEq for TauriOpenerPlugin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:126 (sha256:246e12e848faaf96cde9adad6bf3a31cf7ea15db79020049b2298cfd6afb62ba)
#[derive(Clone)]
pub struct TauriOsModule {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub arch: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub locale:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Option<String> + Send + 'static>>>,
    pub platform: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub version: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
}
impl PartialEq for TauriOsModule {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:135 (sha256:794fb58853246d9ab4a4f300331b775bab0098b139b36d6a444587450d93b560)
#[derive(Clone, Default)]
pub struct TauriShortcutEvent {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub shortcut: String,
    pub state: String,
}
impl PartialEq for TauriShortcutEvent {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:142 (sha256:7c928fec3b33199f0ddb197c838842a9e006f7e39da7dc3598ede1838a9d5bdf)
#[derive(Clone)]
pub struct TauriGlobalShortcutPlugin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub is_registered: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<bool> + Send + 'static>>,
    >,
    pub register: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(TauriShortcutEvent) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> crate::FlightTask<()>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub unregister: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub unregister_all: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
}
impl PartialEq for TauriGlobalShortcutPlugin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:151 (sha256:3559c6a53952ea41b47eb4fc080062a0abdf3584067b8b7add1b33b146cb239f)
#[derive(Clone)]
pub struct TauriMenuModule {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub menu: TauriMenuFactory,
    pub menu_item: TauriMenuItemFactory,
    pub predefined_menu_item: TauriPredefinedMenuItemFactory,
    pub submenu: TauriSubmenuFactory,
}
impl PartialEq for TauriMenuModule {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:160 (sha256:f070520820b61a76ef694c0787550a09dd00349be9c2f453232ba28d195d8d92)
#[derive(Clone)]
pub struct TauriMenuFactory {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub new: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Option<TauriMenuOptions>) -> crate::FlightTask<TauriMenu>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for TauriMenuFactory {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:164 (sha256:90c757a268354e5da4e49b8741a2241193872d41e80d6679842810cf9b36de5c)
#[derive(Clone)]
pub struct TauriMenuItemFactory {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub new: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Option<TauriMenuItemOptions>) -> crate::FlightTask<TauriMenuItemHandle>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for TauriMenuItemFactory {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:168 (sha256:b84446ab6e3888a4e5a2c6e2a27b3fe0d31428f385f687ca390dd088652dd83c)
#[derive(Clone)]
pub struct TauriSubmenuFactory {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub new: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Option<TauriSubmenuOptions>) -> crate::FlightTask<TauriMenuItemHandle>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for TauriSubmenuFactory {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:172 (sha256:83a21eecaaf09c6ac0f9f4583a5255c52a0f2c5df324742ccc22600c61af08e4)
#[derive(Clone)]
pub struct TauriPredefinedMenuItemFactory {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub new: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(TauriPredefinedMenuItemOptions) -> crate::FlightTask<TauriMenuItemHandle>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for TauriPredefinedMenuItemFactory {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:176 (sha256:2f7aaa2534ac26b17b72d89270bbec3aa0a0babab510d5e1df9c33177361f6ab)
#[derive(Clone, Default)]
pub struct TauriMenuOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub items: Option<Vec<TauriMenuItemHandle>>,
}
impl PartialEq for TauriMenuOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:180 (sha256:e22a71cd15e5913f236052f007b50d5cd40773ea611f34a36a5183b6d0cc92e2)
#[derive(Clone, Default)]
pub struct TauriMenuItemOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub accelerator: Option<String>,
    pub action:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>>,
    pub enabled: Option<bool>,
    pub id: Option<String>,
    pub text: Option<String>,
}
impl PartialEq for TauriMenuItemOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:188 (sha256:ad40af76de4a3c6d305c78b3644c5029d04f099160eb4857115de0514b6df4a1)
#[derive(Clone, Default)]
pub struct TauriSubmenuOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub enabled: Option<bool>,
    pub items: Option<Vec<TauriMenuItemHandle>>,
    pub text: Option<String>,
}
impl PartialEq for TauriSubmenuOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:195 (sha256:5311151b387cd5117bcc239053fab7cb73b4e67b1956291d1b54a5fa771ae8cc)
#[derive(Clone, Default)]
pub struct TauriPredefinedMenuItemOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub item: String,
}
impl PartialEq for TauriPredefinedMenuItemOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:199 (sha256:a30d7c3652dd87d91ff673ecc217533b42cc4efc44a3b0c528efcf806a6a876a)
#[derive(Clone, Default)]
pub struct TauriMenuItemHandle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: String,
}
impl PartialEq for TauriMenuItemHandle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:203 (sha256:4373b3864066e880f3ab03c33ec6198de7052fe7e26962861073f4f2e7ee063e)
#[derive(Clone)]
pub struct TauriMenu {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub popup: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Option<TauriPhysicalPositionLike>) -> crate::FlightTask<()>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub set_as_app_menu: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<crate::FlightValue> + Send + 'static>,
        >,
    >,
}
impl PartialEq for TauriMenu {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:210 (sha256:bc0882b4cde6b170e9220b5ccf903aaac10b91bcb817499e678c9ab47dec784a)
#[derive(Clone)]
pub struct TauriTrayModule {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub tray_icon: TauriTrayIconFactory,
}
impl PartialEq for TauriTrayModule {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:215 (sha256:db8cc14622e9cf957721cc052c747b7a71bbad62378bde3e4983c243c5ae52cf)
#[derive(Clone)]
pub struct TauriTrayIconFactory {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub new: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Option<TauriTrayIconOptions>) -> crate::FlightTask<TauriTrayIcon>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for TauriTrayIconFactory {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:219 (sha256:c5089b9fdde398f40991c5fa3f0cf77dc413ec70ff6cb11a0c44a8b97a46a898)
#[derive(Clone, Default)]
pub struct TauriTrayIconOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub action: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(TauriTrayIconEvent) -> () + Send + 'static>>>,
    >,
    pub icon: Option<String>,
    pub menu: Option<TauriMenu>,
    pub title: Option<String>,
    pub tooltip: Option<String>,
}
impl PartialEq for TauriTrayIconOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:229 (sha256:2f7c1d2a94abb4a509f13633fc076210814d0e73374b24db36c4818390d6096b)
#[derive(Clone, Default)]
pub struct TauriTrayIconEvent {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub button: Option<String>,
    pub type_: String,
}
impl PartialEq for TauriTrayIconEvent {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:234 (sha256:d634e07888ec49ab7fd6886972c5fb90807a61c27675345eda326d9cd1718d05)
#[derive(Clone)]
pub struct TauriTrayIcon {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub close: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub set_icon: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Option<String>) -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub set_menu: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Option<TauriMenu>) -> crate::FlightTask<()> + Send + 'static>,
        >,
    >,
    pub set_title: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Option<String>) -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub set_tooltip: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Option<String>) -> crate::FlightTask<()> + Send + 'static>>,
    >,
}
impl PartialEq for TauriTrayIcon {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:245 (sha256:2795a5583f58aeeb92dedc7091398bf1b9d5b35285ca453ae1a70eec0d4e189b)
#[derive(Clone)]
pub struct TauriWindowModule {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_current_window:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> TauriWindow + Send + 'static>>>,
    pub logical_position: TauriLogicalPositionConstructor,
    pub logical_size: TauriLogicalSizeConstructor,
}
impl PartialEq for TauriWindowModule {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:251 (sha256:bec9c6d2442908c970f5172e36d701331a52c6826d28ca8c4092b58dd46bcb97)
#[derive(Clone, Default)]
pub struct TauriLogicalPositionConstructor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub __construct: Option<crate::OpaqueHostValue>,
}
impl PartialEq for TauriLogicalPositionConstructor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:255 (sha256:74872f5d736e4e133d3482f2e11838aeb917e4c6a11938a864668cf17984cd4e)
#[derive(Clone, Default)]
pub struct TauriLogicalSizeConstructor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub __construct: Option<crate::OpaqueHostValue>,
}
impl PartialEq for TauriLogicalSizeConstructor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:261 (sha256:ce3971126702a634cdfc546ae6c1186cd58316ffde5a7f9a05c80cce0c317ed6)
#[derive(Clone, Default)]
pub struct TauriPhysicalPositionLike {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for TauriPhysicalPositionLike {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:266 (sha256:a9b35df6942f65f75ad2e42011b0ea4cc6568491f38c0371dc609e1d68284e44)
#[derive(Clone, Default)]
pub struct TauriLogicalSizeLike {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub width: f64,
}
impl PartialEq for TauriLogicalSizeLike {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:272 (sha256:9fd522b35ed4e4fef8f23d598d958cb5fdc3fcfeb57f1d6f2365280356d9bc9a)
pub type TauriUnlisten = std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/TauriApi.ts:274 (sha256:e14778aebce9fedd52ae029a54e8939101af6bbbe5a6062b86158b5a9bb1a1e5)
#[derive(Clone, Default)]
pub struct TauriWindowRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub payload: TauriLogicalSizeLike,
}
impl PartialEq for TauriWindowRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TauriWindowRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub payload: TauriPhysicalPositionLike,
}
impl PartialEq for TauriWindowRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TauriWindowRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub payload: bool,
}
impl PartialEq for TauriWindowRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct TauriWindow {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub center: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub close: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub hide: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub maximize: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub minimize: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub on_close_requested: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(TauriCloseRequestedEvent) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> crate::FlightTask<TauriUnlisten>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub on_focus_changed: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(TauriWindowRecord3) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> crate::FlightTask<TauriUnlisten>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub on_moved: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(TauriWindowRecord2) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> crate::FlightTask<TauriUnlisten>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub on_resized: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(TauriWindowRecord1) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> crate::FlightTask<TauriUnlisten>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub request_user_attention: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Option<f64>) -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub set_always_on_top: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(bool) -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub set_content_protected: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(bool) -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub set_focus: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub set_fullscreen: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(bool) -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub set_icon: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub set_max_size: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Option<TauriLogicalSizeLike>) -> crate::FlightTask<()> + Send + 'static>,
        >,
    >,
    pub set_min_size: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Option<TauriLogicalSizeLike>) -> crate::FlightTask<()> + Send + 'static>,
        >,
    >,
    pub set_position: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(TauriPhysicalPositionLike) -> crate::FlightTask<()> + Send + 'static>,
        >,
    >,
    pub set_resizable: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(bool) -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub set_shadow: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(bool) -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub set_size: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(TauriLogicalSizeLike) -> crate::FlightTask<()> + Send + 'static>,
        >,
    >,
    pub set_skip_taskbar: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(bool) -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub set_title: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(String) -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub show: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub unmaximize: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
}
impl PartialEq for TauriWindow {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriApi.ts:302 (sha256:02ded392e35ee19c8530acdd378d18caabda7b1ea30daff03c403b69b5086168)
#[derive(Clone)]
pub struct TauriCloseRequestedEvent {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub prevent_default: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for TauriCloseRequestedEvent {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
