// @generated from upstream/packages/types/src/Updater.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/Updater.ts:3 (sha256:1a4227bf38066f4a4499f6a90fd5f83924785eed1ec826d5e0179710b06f66bb)
#[derive(Clone)]
pub struct UpdateInfo {
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

// Source: upstream/packages/types/src/Updater.ts:19 (sha256:11d221573978c7b4e9f7ba9e3c0d1d3c61febc18c1d0183bdbcfd1f43d1dc822)
#[derive(Clone)]
pub struct UpdateProgress {
    pub bytes_per_second: f64,
    pub is_delta: bool,
    pub percent: f64,
    pub total_bytes: f64,
    pub transferred_bytes: f64,
}

// Source: upstream/packages/types/src/Updater.ts:29 (sha256:e5ccf43d6259ff948cd1ba965273672b0bb44d11ee4b25cca5a996fb25c9fad6)
#[derive(Clone)]
pub struct UpdaterError {
    pub kind: String,
    pub message: String,
}

// Source: upstream/packages/types/src/Updater.ts:36 (sha256:470ed3d4a89e436a0060785a649555cfc186680cbe7ae153237b7e388779f9b9)
#[derive(Clone)]
pub struct UpdaterConfig {
    pub allow_prerelease: bool,
    pub auto_download: bool,
    pub auto_install_on_app_quit: bool,
}

// Source: upstream/packages/types/src/Updater.ts:43 (sha256:1ac49d69455eaa69b05812978062d74dcd0c76e96b9736868b345f996393ffcb)
#[derive(Clone)]
pub struct UpdaterSignatureConfig {
    pub algorithm: String,
    pub public_key: String,
}

// Source: upstream/packages/types/src/Updater.ts:50 (sha256:8bfdf8d59f93c6566661b00cb391700f075cb2b8b9e8ee532937ff376f5dad45)
pub type UpdaterPhase = String;

// Source: upstream/packages/types/src/Updater.ts:54 (sha256:3be040e014fd577cb94613014924a615e10e19a94f95f69af0e4e1573141d271)
#[derive(Clone)]
pub struct UpdaterState {
    pub phase: UpdaterPhase,
    pub info: Option<UpdateInfo>,
    pub progress: Option<UpdateProgress>,
    pub error: Option<UpdaterError>,
}

// Source: upstream/packages/types/src/Updater.ts:62 (sha256:5debc9d457c5f11b9d4984b8450a710726b2717a256b7a1dd758bb9efe7d43d4)
#[derive(Clone)]
pub struct AppUpdater {
    pub on_checking: Signal,
    pub on_update_available: Signal,
    pub on_update_not_available: Signal,
    pub on_download_progress: Signal,
    pub on_update_downloaded: Signal,
    pub on_error: Signal,
    pub on_update_cancelled: Signal,
    pub on_update_staging: Signal,
    pub on_update_verified: Signal,
    pub on_update_rolled_back: Signal,
}

// Source: upstream/packages/types/src/Updater.ts:78 (sha256:a2a545316b2fcd45eae29d8afade5b56671727eadd85ca9f81d42bdfc527ceae)
#[derive(Clone)]
pub struct UpdaterBackend {
    pub cancel_download: crate::OpaqueHostValue,
    pub check_for_updates: crate::OpaqueHostValue,
    pub download_update: crate::OpaqueHostValue,
    pub get_channel: crate::OpaqueHostValue,
    pub get_config: crate::OpaqueHostValue,
    pub quit_and_install: crate::OpaqueHostValue,
    pub rollback: crate::OpaqueHostValue,
    pub set_channel: crate::OpaqueHostValue,
    pub set_config: crate::OpaqueHostValue,
    pub set_feed_url: crate::OpaqueHostValue,
    pub set_signature_config: crate::OpaqueHostValue,
    pub subscribe_checking: crate::OpaqueHostValue,
    pub subscribe_download_progress: crate::OpaqueHostValue,
    pub subscribe_error: crate::OpaqueHostValue,
    pub subscribe_update_available: crate::OpaqueHostValue,
    pub subscribe_update_cancelled: crate::OpaqueHostValue,
    pub subscribe_update_downloaded: crate::OpaqueHostValue,
    pub subscribe_update_not_available: crate::OpaqueHostValue,
    pub subscribe_update_rolled_back: crate::OpaqueHostValue,
    pub subscribe_update_staging: crate::OpaqueHostValue,
    pub subscribe_update_verified: crate::OpaqueHostValue,
}
