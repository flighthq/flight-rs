// @generated from upstream/packages/types/src/CapacitorApi.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub value: bool,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub path: String,
    pub recursive: Option<bool>,
}
impl PartialEq for SharedStructuralRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub style: String,
}
impl PartialEq for SharedStructuralRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub notifications: Vec<SharedStructuralRecord5>,
}
impl PartialEq for SharedStructuralRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub id: f64,
}
impl PartialEq for SharedStructuralRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub notifications: Vec<CapacitorLocalNotificationSchema>,
}
impl PartialEq for SharedStructuralRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:31 (sha256:b0dc0c96ded1a17737bca7c5890baa1686ceb20716717440a7c6d69ac0b5e6fc)
#[derive(Clone)]
pub struct CapacitorApi {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub app: CapacitorAppPlugin,
    pub clipboard: CapacitorClipboardPlugin,
    pub device: CapacitorDevicePlugin,
    pub dialog: CapacitorDialogPlugin,
    pub filesystem: CapacitorFilesystemPlugin,
    pub geolocation: CapacitorGeolocationPlugin,
    pub haptics: CapacitorHapticsPlugin,
    pub keyboard: CapacitorKeyboardPlugin,
    pub local_notifications: CapacitorLocalNotificationsPlugin,
    pub network: CapacitorNetworkPlugin,
    pub share: CapacitorSharePlugin,
    pub status_bar: CapacitorStatusBarPlugin,
}
impl PartialEq for CapacitorApi {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:48 (sha256:d0a354a3cfc5c14b447b091bf3317dbbcc1a29b5393d7e7cfb1abde4a4435624)
#[derive(Clone)]
pub struct CapacitorPluginListenerHandle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub remove: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
}
impl PartialEq for CapacitorPluginListenerHandle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:54 (sha256:e11411a1666fe330b1f7854ca3b29243a6f5b45d0f66da14bc14415e4b23c0ea)
#[derive(Clone, Default)]
pub struct CapacitorAppPluginRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub url: String,
}
impl PartialEq for CapacitorAppPluginRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct CapacitorAppPlugin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub add_listener: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(CapacitorAppPluginRecord1) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> crate::FlightTask<CapacitorPluginListenerHandle>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub exit_app: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub get_info: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<CapacitorAppInfo> + Send + 'static>>,
    >,
    pub minimize_app: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
}
impl PartialEq for CapacitorAppPlugin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:68 (sha256:2371806e5bb53aeb4c9a888a7f152ff0d64cfd81c40a93fcb1f2de6d290995e5)
#[derive(Clone, Default)]
pub struct CapacitorAppInfo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub build: String,
    pub id: String,
    pub name: String,
    pub version: String,
}
impl PartialEq for CapacitorAppInfo {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:78 (sha256:7f2cff0829ca04e5d65d9c6b2fa0337ba6bbfe58aacf4990f2509e8b12d40601)
#[derive(Clone)]
pub struct CapacitorClipboardPlugin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub read: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<CapacitorClipboardReadResult> + Send + 'static>,
        >,
    >,
    pub write: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(CapacitorClipboardWriteOptions) -> crate::FlightTask<()> + Send + 'static,
            >,
        >,
    >,
}
impl PartialEq for CapacitorClipboardPlugin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:83 (sha256:56fe9130a39f0b883b6a6d24ff731d4bd863e93c3d2465be06698b47adaedee4)
#[derive(Clone, Default)]
pub struct CapacitorClipboardReadResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub type_: String,
    pub value: String,
}
impl PartialEq for CapacitorClipboardReadResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:88 (sha256:c44824706a71dcca1a8f473d2d8af850974271343939f25c728e2d9c62b6508c)
#[derive(Clone, Default)]
pub struct CapacitorClipboardWriteOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub image: Option<String>,
    pub label: Option<String>,
    pub string: Option<String>,
    pub url: Option<String>,
}
impl PartialEq for CapacitorClipboardWriteOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:97 (sha256:a10be4006aadc24421f58af094fd3f7263962b1dea7ac94f803bf23860d43e31)
#[derive(Clone)]
pub struct CapacitorDevicePlugin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_id: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<CapacitorDeviceId> + Send + 'static>>,
    >,
    pub get_info: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<CapacitorDeviceInfo> + Send + 'static>,
        >,
    >,
}
impl PartialEq for CapacitorDevicePlugin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:102 (sha256:2b7439da4b0735328d0e5ea84c105627068466449eb0fd97a89c9f867a0e9c3a)
#[derive(Clone, Default)]
pub struct CapacitorDeviceId {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub identifier: String,
}
impl PartialEq for CapacitorDeviceId {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:106 (sha256:0d43596323d2b6bae32b2a18e48f1b8b9d4e1b111f4befd669e75060b5a088be)
#[derive(Clone, Default)]
pub struct CapacitorDeviceInfo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub is_virtual: bool,
    pub manufacturer: String,
    pub model: String,
    pub name: Option<String>,
    pub operating_system: String,
    pub os_version: String,
    pub platform: String,
    pub web_view_version: String,
}
impl PartialEq for CapacitorDeviceInfo {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:120 (sha256:24ee2e4c6db3c1bf4ebd605d75ee05c511096a14c3c87af2167c0afde35e2eb7)
#[derive(Clone)]
pub struct CapacitorDialogPlugin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alert: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(CapacitorDialogAlertOptions) -> crate::FlightTask<()> + Send + 'static>,
        >,
    >,
    pub confirm: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        CapacitorDialogConfirmOptions,
                    ) -> crate::FlightTask<CapacitorDialogConfirmResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub prompt: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        CapacitorDialogPromptOptions,
                    ) -> crate::FlightTask<CapacitorDialogPromptResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for CapacitorDialogPlugin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:126 (sha256:17596525e344609727b4ba29d55e430cd0b10413b5cc4d397a5e02474b2666bb)
#[derive(Clone, Default)]
pub struct CapacitorDialogAlertOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub button_title: Option<String>,
    pub message: String,
    pub title: Option<String>,
}
impl PartialEq for CapacitorDialogAlertOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:132 (sha256:88c2ea5d7bcca5d4dc28f07980d227fbff477b95e579577de8c035b20763b0d9)
#[derive(Clone, Default)]
pub struct CapacitorDialogConfirmOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cancel_button_title: Option<String>,
    pub message: String,
    pub ok_button_title: Option<String>,
    pub title: Option<String>,
}
impl PartialEq for CapacitorDialogConfirmOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:139 (sha256:70c469bab6cc1443f2e7170e84a1360bd80bdc70dd26f249d7439fdc168ed430)
#[derive(Clone, Default)]
pub struct CapacitorDialogConfirmResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub value: bool,
}
impl PartialEq for CapacitorDialogConfirmResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:143 (sha256:f3f3ca6e74d99e8f3625e5d4414a1b8149f33099f48be51831ed9ccf2ff7c292)
#[derive(Clone, Default)]
pub struct CapacitorDialogPromptOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cancel_button_title: Option<String>,
    pub input_placeholder: Option<String>,
    pub input_text: Option<String>,
    pub message: String,
    pub ok_button_title: Option<String>,
    pub title: Option<String>,
}
impl PartialEq for CapacitorDialogPromptOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:152 (sha256:3a395a5216c1eaf0d58127dfb68a739952977076597077a33a7a2f0662a3a299)
#[derive(Clone, Default)]
pub struct CapacitorDialogPromptResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cancelled: bool,
    pub value: String,
}
impl PartialEq for CapacitorDialogPromptResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:161 (sha256:49a5d2bf812324c8d82d9981a784f21dfa5f69e141bfaf7e3d5003693ec4d761)
#[derive(Clone)]
pub struct CapacitorFilesystemPlugin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub append_file: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(CapacitorFilesystemWriteOptions) -> crate::FlightTask<()>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub copy: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(CapacitorFilesystemCopyOptions) -> crate::FlightTask<()> + Send + 'static,
            >,
        >,
    >,
    pub delete_file: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(CapacitorFilesystemPathOptions) -> crate::FlightTask<()> + Send + 'static,
            >,
        >,
    >,
    pub mkdir: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(CapacitorFilesystemMkdirOptions) -> crate::FlightTask<()>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub read_file: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        CapacitorFilesystemReadOptions,
                    ) -> crate::FlightTask<CapacitorFilesystemReadResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub readdir: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        CapacitorFilesystemPathOptions,
                    )
                        -> crate::FlightTask<CapacitorFilesystemReaddirResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub rename: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(CapacitorFilesystemCopyOptions) -> crate::FlightTask<()> + Send + 'static,
            >,
        >,
    >,
    pub rmdir: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(CapacitorFilesystemRmdirOptions) -> crate::FlightTask<()>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub stat: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        CapacitorFilesystemPathOptions,
                    ) -> crate::FlightTask<CapacitorFilesystemStatResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub write_file: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        CapacitorFilesystemWriteOptions,
                    ) -> crate::FlightTask<CapacitorFilesystemWriteResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for CapacitorFilesystemPlugin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:174 (sha256:05494c13aaab0a8479bb429141b5c655d3d13d20cf4a79adb88ac7609a0dac4d)
#[derive(Clone, Default)]
pub struct CapacitorFilesystemPathOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub path: String,
}
impl PartialEq for CapacitorFilesystemPathOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:178 (sha256:f6edba838fa3717143fac0aa2ef7e41effd7318b5c8940316b7227513fbc4891)
#[derive(Clone, Default)]
pub struct CapacitorFilesystemReadOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub encoding: Option<String>,
    pub path: String,
}
impl PartialEq for CapacitorFilesystemReadOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:183 (sha256:c414137c00e1efb5ef822ff535a9c8587d0463a8363f0f976ccd6832d9b6425e)
#[derive(Clone, Default)]
pub struct CapacitorFilesystemReadResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: String,
}
impl PartialEq for CapacitorFilesystemReadResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:187 (sha256:5ba595437222a72793d640b353c0e85b0748f8d38f74ba7ff0a5f77846135eb3)
#[derive(Clone, Default)]
pub struct CapacitorFilesystemWriteOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: String,
    pub encoding: Option<String>,
    pub path: String,
    pub recursive: Option<bool>,
}
impl PartialEq for CapacitorFilesystemWriteOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:194 (sha256:102664075ce042f52a44ccea8b10392a6ed335b25f99774eaf3a1926b81c3d22)
#[derive(Clone, Default)]
pub struct CapacitorFilesystemWriteResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub uri: String,
}
impl PartialEq for CapacitorFilesystemWriteResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:198 (sha256:3498d4c7d48ba6506394df45c17947d26e7a60789554c8c1d380266685cb0969)
#[derive(Clone, Default)]
pub struct CapacitorFilesystemCopyOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub from: String,
    pub to: String,
}
impl PartialEq for CapacitorFilesystemCopyOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:203 (sha256:5abc3e7357ddc3e85db61386d043edf803c4f169ea7a259f7caf5e6b00e751f9)
#[derive(Clone, Default)]
pub struct CapacitorFilesystemMkdirOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub path: String,
    pub recursive: Option<bool>,
}
impl PartialEq for CapacitorFilesystemMkdirOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:208 (sha256:4003ce362fdf5dd3b78503787ba5e9702746db82f580f450f37cdf19bc605e63)
#[derive(Clone, Default)]
pub struct CapacitorFilesystemRmdirOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub path: String,
    pub recursive: Option<bool>,
}
impl PartialEq for CapacitorFilesystemRmdirOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:213 (sha256:d3e211bb4d93164be18c5a4da40c8cc6e55c259eccfbfda6873ab2379b69929b)
#[derive(Clone, Default)]
pub struct CapacitorFilesystemReaddirResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub files: Vec<CapacitorFileInfo>,
}
impl PartialEq for CapacitorFilesystemReaddirResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:219 (sha256:0eaed0c6692f7f367d9d06cafea605237aaf868b8cbe073f47f8058b3a06bad5)
#[derive(Clone, Default)]
pub struct CapacitorFileInfo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ctime: Option<f64>,
    pub mtime: f64,
    pub name: String,
    pub size: f64,
    pub type_: String,
    pub uri: String,
}
impl PartialEq for CapacitorFileInfo {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:228 (sha256:252f92b2f5e80aa961d9cd529cf8d4e5bfa2e1d1774d95df867dd4f27cff506a)
#[derive(Clone, Default)]
pub struct CapacitorFilesystemStatResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ctime: Option<f64>,
    pub mtime: f64,
    pub size: f64,
    pub type_: String,
    pub uri: String,
}
impl PartialEq for CapacitorFilesystemStatResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:240 (sha256:a86bf654693b542e7ec591600f87d0662786357ba272899149f296e6e277b4a5)
#[derive(Clone, Default)]
pub struct CapacitorGeolocationPluginRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub id: String,
}
impl PartialEq for CapacitorGeolocationPluginRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct CapacitorGeolocationPlugin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub check_permissions: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut() -> crate::FlightTask<CapacitorGeolocationPermissionStatus>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub clear_watch: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(CapacitorGeolocationPluginRecord1) -> crate::FlightTask<()>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub get_current_position: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Option<CapacitorGeolocationOptions>,
                    ) -> crate::FlightTask<CapacitorPosition>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub request_permissions: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Option<CapacitorGeolocationPermissionOptions>,
                    )
                        -> crate::FlightTask<CapacitorGeolocationPermissionStatus>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub watch_position: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        CapacitorGeolocationOptions,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<
                                    dyn FnMut(
                                            Option<CapacitorPosition>,
                                            Option<crate::FlightValue>,
                                        ) -> ()
                                        + Send
                                        + 'static,
                                >,
                            >,
                        >,
                    ) -> crate::FlightTask<String>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for CapacitorGeolocationPlugin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:253 (sha256:f4c9e37f0dbf80514e8c95a9122a0da27e567ac24e96a5dd9685f4c7d6c4d419)
#[derive(Clone, Default)]
pub struct CapacitorGeolocationOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub enable_high_accuracy: Option<bool>,
    pub maximum_age: Option<f64>,
    pub timeout: Option<f64>,
}
impl PartialEq for CapacitorGeolocationOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:259 (sha256:7f0b1fcda9df9b3b4937c320ed110aee8cfac7623c3ce8caf67cf194cb1d0438)
#[derive(Clone, Default)]
pub struct CapacitorGeolocationPermissionOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub permissions: Option<Vec<String>>,
}
impl PartialEq for CapacitorGeolocationPermissionOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:264 (sha256:380f8258259983687d3c982bc8936499d0e2fceee317b74e38f9493ecb652056)
#[derive(Clone, Default)]
pub struct CapacitorGeolocationPermissionStatus {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub coarse_location: Option<String>,
    pub location: String,
}
impl PartialEq for CapacitorGeolocationPermissionStatus {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:269 (sha256:6216f911f8185b30c4fba096cd02d91c987f35f840aaae9669316ce22c9cbf08)
#[derive(Clone, Default)]
pub struct CapacitorPosition {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub coords: CapacitorPositionCoords,
    pub timestamp: f64,
}
impl PartialEq for CapacitorPosition {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:274 (sha256:11198f596ff3955757f04ec897660f68dd0b9247759cae0cd23b8e0f230dd36b)
#[derive(Clone, Default)]
pub struct CapacitorPositionCoords {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub accuracy: f64,
    pub altitude: Option<f64>,
    pub altitude_accuracy: Option<f64>,
    pub heading: Option<f64>,
    pub latitude: f64,
    pub longitude: f64,
    pub speed: Option<f64>,
}
impl PartialEq for CapacitorPositionCoords {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:287 (sha256:ea348912addde09fac9a5e0f72a4001f050b515030ef74d30393168e7cc84bf6)
#[derive(Clone, Default)]
pub struct CapacitorHapticsPluginRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub duration: Option<f64>,
}
impl PartialEq for CapacitorHapticsPluginRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CapacitorHapticsPluginRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub type_: String,
}
impl PartialEq for CapacitorHapticsPluginRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct CapacitorHapticsPlugin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub impact: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(SharedStructuralRecord3) -> crate::FlightTask<()> + Send + 'static>,
        >,
    >,
    pub notification: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(CapacitorHapticsPluginRecord2) -> crate::FlightTask<()> + Send + 'static>,
        >,
    >,
    pub selection_changed: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub selection_end: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub selection_start: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub vibrate: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(CapacitorHapticsPluginRecord1) -> crate::FlightTask<()> + Send + 'static>,
        >,
    >,
}
impl PartialEq for CapacitorHapticsPlugin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:300 (sha256:fff044a83d168a6809a484c061e0684813c82c806faa97a32f7ebace4acf609c)
#[derive(Clone, Default)]
pub struct CapacitorKeyboardPluginRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub is_disabled: bool,
}
impl PartialEq for CapacitorKeyboardPluginRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CapacitorKeyboardPluginRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub mode: String,
}
impl PartialEq for CapacitorKeyboardPluginRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CapacitorKeyboardPluginRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub is_visible: bool,
}
impl PartialEq for CapacitorKeyboardPluginRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct CapacitorKeyboardPlugin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub add_listener: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    ) -> crate::FlightTask<CapacitorPluginListenerHandle>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub hide: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub set_accessory_bar_visible: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(CapacitorKeyboardPluginRecord3) -> crate::FlightTask<()> + Send + 'static,
            >,
        >,
    >,
    pub set_resize_mode: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(CapacitorKeyboardPluginRecord2) -> crate::FlightTask<()> + Send + 'static,
            >,
        >,
    >,
    pub set_scroll: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(CapacitorKeyboardPluginRecord1) -> crate::FlightTask<()> + Send + 'static,
            >,
        >,
    >,
    pub set_style: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(SharedStructuralRecord3) -> crate::FlightTask<()> + Send + 'static>,
        >,
    >,
    pub show: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
}
impl PartialEq for CapacitorKeyboardPlugin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:320 (sha256:72042e1a14a46ed0fb3e8c561cc083f8073b871c8c8d2160c789516f8bfeee9c)
#[derive(Clone)]
pub struct CapacitorLocalNotificationsPlugin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub add_listener: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<
                                    dyn FnMut(CapacitorLocalNotificationAction) -> ()
                                        + Send
                                        + 'static,
                                >,
                            >,
                        >,
                    ) -> crate::FlightTask<CapacitorPluginListenerHandle>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub cancel: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(SharedStructuralRecord4) -> crate::FlightTask<()> + Send + 'static>,
        >,
    >,
    pub check_permissions: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut() -> crate::FlightTask<CapacitorLocalNotificationsPermission>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub get_pending: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut() -> crate::FlightTask<CapacitorLocalNotificationsPending>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub request_permissions: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut() -> crate::FlightTask<CapacitorLocalNotificationsPermission>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub schedule: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        SharedStructuralRecord6,
                    )
                        -> crate::FlightTask<CapacitorLocalNotificationsScheduleResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for CapacitorLocalNotificationsPlugin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:334 (sha256:baa47ad08e9c56c783c2d9b377c8922be7df7c180343834b02b08078127136a7)
#[derive(Clone, Default)]
pub struct CapacitorLocalNotificationSchemaRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub at: Option<crate::OpaqueHostValue>,
}
impl PartialEq for CapacitorLocalNotificationSchemaRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CapacitorLocalNotificationSchema {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body: Option<String>,
    pub id: f64,
    pub schedule: Option<CapacitorLocalNotificationSchemaRecord1>,
    pub title: String,
}
impl PartialEq for CapacitorLocalNotificationSchema {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:341 (sha256:185d7ae530081b28197a75a34ed813876c14e69955604081a06ad2c0d13c6875)
#[derive(Clone, Default)]
pub struct CapacitorLocalNotificationsScheduleResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub notifications: Vec<SharedStructuralRecord5>,
}
impl PartialEq for CapacitorLocalNotificationsScheduleResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:345 (sha256:e8ece6213f06348b287ec0639763800db8df23fe06b916246cbcea165d7c23c2)
#[derive(Clone, Default)]
pub struct CapacitorLocalNotificationsPending {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub notifications: Vec<CapacitorLocalNotificationSchema>,
}
impl PartialEq for CapacitorLocalNotificationsPending {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:349 (sha256:90165052b6afc119968fec24965526b2c7f1ddd13e1fe95c1ba2752612d3c254)
#[derive(Clone, Default)]
pub struct CapacitorLocalNotificationsPermission {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub display: String,
}
impl PartialEq for CapacitorLocalNotificationsPermission {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:353 (sha256:a4e5748e798d53191d87b0db4f18e8bfbe98ad641cf4b6aeb2a8a5f901d66a2b)
#[derive(Clone, Default)]
pub struct CapacitorLocalNotificationAction {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub action_id: String,
    pub notification: SharedStructuralRecord5,
}
impl PartialEq for CapacitorLocalNotificationAction {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:361 (sha256:b3e501196c8c0d37c465e1878c78294312a5016551c48178076adeebe5056a7e)
#[derive(Clone)]
pub struct CapacitorNetworkPlugin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub add_listener: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        String,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(CapacitorConnectionStatus) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> crate::FlightTask<CapacitorPluginListenerHandle>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub get_status: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<CapacitorConnectionStatus> + Send + 'static>,
        >,
    >,
}
impl PartialEq for CapacitorNetworkPlugin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:370 (sha256:bb97f345c6f1dfbf691db41dd1fed96d2d93880f74297edb1ca2ea96f8b83058)
#[derive(Clone, Default)]
pub struct CapacitorConnectionStatus {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub connected: bool,
    pub connection_type: String,
}
impl PartialEq for CapacitorConnectionStatus {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:378 (sha256:f2ce61a1db6f1281889f1e359c55a7cb28de0910e0522fa5bd44de6c2336cafa)
#[derive(Clone)]
pub struct CapacitorSharePlugin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub can_share: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<CapacitorShareCanResult> + Send + 'static>,
        >,
    >,
    pub share: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(CapacitorShareOptions) -> crate::FlightTask<CapacitorShareResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for CapacitorSharePlugin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:383 (sha256:27a6a721ce960730010432e09f2cb76b6cce032ae682e7c1c306a0480a41c6ac)
#[derive(Clone, Default)]
pub struct CapacitorShareOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub dialog_title: Option<String>,
    pub files: Option<Vec<String>>,
    pub text: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
}
impl PartialEq for CapacitorShareOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:391 (sha256:ded884798ec3debea4e76d5cec1b8593522e70ec25c562a88420a9b4bd97d9da)
#[derive(Clone, Default)]
pub struct CapacitorShareCanResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub value: bool,
}
impl PartialEq for CapacitorShareCanResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:395 (sha256:02eaee12bbad5fde4df0ddcfb2c51a7593c26573c4df2eb7c356f15721ed2b92)
#[derive(Clone, Default)]
pub struct CapacitorShareResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub activity_type: Option<String>,
}
impl PartialEq for CapacitorShareResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:402 (sha256:4821d82621c498f3712e2272f121760de426296038da6fda9032644c9d9e8578)
#[derive(Clone, Default)]
pub struct CapacitorStatusBarPluginRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub overlay: bool,
}
impl PartialEq for CapacitorStatusBarPluginRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CapacitorStatusBarPluginRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub color: String,
}
impl PartialEq for CapacitorStatusBarPluginRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct CapacitorStatusBarPlugin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_info: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<CapacitorStatusBarInfoResult> + Send + 'static>,
        >,
    >,
    pub hide: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
    pub set_background_color: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(CapacitorStatusBarPluginRecord2) -> crate::FlightTask<()>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub set_overlays_web_view: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(CapacitorStatusBarPluginRecord1) -> crate::FlightTask<()>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub set_style: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(SharedStructuralRecord3) -> crate::FlightTask<()> + Send + 'static>,
        >,
    >,
    pub show: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<()> + Send + 'static>>,
    >,
}
impl PartialEq for CapacitorStatusBarPlugin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorApi.ts:411 (sha256:6706f6673cc636115f9dede392c9695a41ec647edeaa87ed4612c9f613401819)
#[derive(Clone, Default)]
pub struct CapacitorStatusBarInfoResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color: Option<String>,
    pub overlays: Option<bool>,
    pub style: String,
    pub visible: bool,
}
impl PartialEq for CapacitorStatusBarInfoResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
