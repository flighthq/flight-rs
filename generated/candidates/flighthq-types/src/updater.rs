// @generated from upstream/packages/types/src/Updater.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/Updater.ts:3 (sha256:1a4227bf38066f4a4499f6a90fd5f83924785eed1ec826d5e0179710b06f66bb)
#[derive(Clone, Default)]
pub struct UpdateInfo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub version: String,
    pub notes: String,
    pub release_date: String,
    pub delta_from_version: Option<String>,
    pub download_size_bytes: f64,
    pub is_mandatory: bool,
    pub minimum_os_version: Option<String>,
    pub sha512: String,
    pub staged_rollout_percent: f64,
}
impl PartialEq for UpdateInfo {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Updater.ts:19 (sha256:11d221573978c7b4e9f7ba9e3c0d1d3c61febc18c1d0183bdbcfd1f43d1dc822)
#[derive(Clone, Default)]
pub struct UpdateProgress {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bytes_per_second: f64,
    pub is_delta: bool,
    pub percent: f64,
    pub total_bytes: f64,
    pub transferred_bytes: f64,
}
impl PartialEq for UpdateProgress {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Updater.ts:29 (sha256:e5ccf43d6259ff948cd1ba965273672b0bb44d11ee4b25cca5a996fb25c9fad6)
#[derive(Clone, Default)]
pub struct UpdaterError {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub message: String,
}
impl PartialEq for UpdaterError {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Updater.ts:36 (sha256:470ed3d4a89e436a0060785a649555cfc186680cbe7ae153237b7e388779f9b9)
#[derive(Clone, Default)]
pub struct UpdaterConfig {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub allow_prerelease: bool,
    pub auto_download: bool,
    pub auto_install_on_app_quit: bool,
}
impl PartialEq for UpdaterConfig {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Updater.ts:43 (sha256:1ac49d69455eaa69b05812978062d74dcd0c76e96b9736868b345f996393ffcb)
#[derive(Clone, Default)]
pub struct UpdaterSignatureConfig {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub algorithm: String,
    pub public_key: String,
}
impl PartialEq for UpdaterSignatureConfig {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Updater.ts:50 (sha256:8bfdf8d59f93c6566661b00cb391700f075cb2b8b9e8ee532937ff376f5dad45)
pub type UpdaterPhase = String;

// Source: upstream/packages/types/src/Updater.ts:54 (sha256:3be040e014fd577cb94613014924a615e10e19a94f95f69af0e4e1573141d271)
#[derive(Clone, Default)]
pub struct UpdaterState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub phase: UpdaterPhase,
    pub info: Option<UpdateInfo>,
    pub progress: Option<UpdateProgress>,
    pub error: Option<UpdaterError>,
}
impl PartialEq for UpdaterState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Updater.ts:62 (sha256:5debc9d457c5f11b9d4984b8450a710726b2717a256b7a1dd758bb9efe7d43d4)
#[derive(Clone)]
pub struct AppUpdater {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_checking:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_update_available:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(UpdateInfo) -> () + Send + 'static>>>>,
    pub on_update_not_available:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_download_progress: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(UpdateProgress) -> () + Send + 'static>>>,
    >,
    pub on_update_downloaded:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(UpdateInfo) -> () + Send + 'static>>>>,
    pub on_error: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(UpdaterError) -> () + Send + 'static>>>,
    >,
    pub on_update_cancelled:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_update_staging:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_update_verified:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_update_rolled_back:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for AppUpdater {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Updater.ts:78 (sha256:a2a545316b2fcd45eae29d8afade5b56671727eadd85ca9f81d42bdfc527ceae)
#[derive(Clone)]
pub struct UpdaterBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cancel_download: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub check_for_updates:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub download_update: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub get_channel: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub get_config:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> UpdaterConfig + Send + 'static>>>,
    pub quit_and_install: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub rollback: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub set_channel:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub set_config:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(UpdaterConfig) -> () + Send + 'static>>>,
    pub set_feed_url:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    pub set_signature_config: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Option<UpdaterSignatureConfig>) -> () + Send + 'static>>,
    >,
    pub subscribe_checking: std::sync::Arc<
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
    pub subscribe_download_progress: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(UpdateProgress) -> () + Send + 'static>>,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_error: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(UpdaterError) -> () + Send + 'static>>,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_update_available: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(UpdateInfo) -> () + Send + 'static>>,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_update_cancelled: std::sync::Arc<
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
    pub subscribe_update_downloaded: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        std::sync::Arc<
                            std::sync::Mutex<Box<dyn FnMut(UpdateInfo) -> () + Send + 'static>>,
                        >,
                    ) -> std::sync::Arc<
                        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub subscribe_update_not_available: std::sync::Arc<
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
    pub subscribe_update_rolled_back: std::sync::Arc<
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
    pub subscribe_update_staging: std::sync::Arc<
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
    pub subscribe_update_verified: std::sync::Arc<
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
}
impl PartialEq for UpdaterBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
