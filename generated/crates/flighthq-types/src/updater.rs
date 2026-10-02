// @generated from upstream/packages/types/src/Updater.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EntityRuntime;

// Source: upstream/packages/types/src/Updater.ts:5 (sha256:e02dd9442a787001b830ad5717a08233eead547090b754a7b49a8312bb8f949a)
#[derive(Clone, Default)]
pub struct UpdateInfo {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub download_size_bytes: Option<f64>,
    pub is_mandatory: Option<bool>,
    pub minimum_os_version: Option<String>,
    pub notes: Option<String>,
    pub release_date: Option<String>,
    pub sha512: Option<String>,
    pub version: Option<String>,
}
impl PartialEq for UpdateInfo {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Updater.ts:17 (sha256:9f2a76dbab6ae60936bae03305880138056196b89dde44dd6457b0ab1d871e10)
#[derive(Clone, Default)]
pub struct DownloadedUpdate {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub info: UpdateInfo,
}
impl PartialEq for DownloadedUpdate {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for DownloadedUpdate {
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

// Source: upstream/packages/types/src/Updater.ts:21 (sha256:b88fbd757ed34ff128760eca8717a690ab60df85fe4dfb243d50887c7f2bb4eb)
#[derive(Clone, Default)]
pub struct AppUpdateCheckOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
    pub update: Option<DownloadedUpdate>,
}
impl PartialEq for AppUpdateCheckOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Updater.ts:25 (sha256:d785af3edaa24416f92e50204cc420810308b33164a5e9a22626845ebcf07750)
#[derive(Clone, Default)]
pub struct AppUpdateInstallOutcome {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: String,
}
impl PartialEq for AppUpdateInstallOutcome {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Updater.ts:31 (sha256:1f3714b2e9287ec65895ec3ede16c08673fcc9965d59a975d01c6bf60f9d9e8e)
#[derive(Clone)]
pub struct HostUpdaterCommandCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub check: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> crate::FlightTask<AppUpdateCheckOutcome> + Send + 'static>,
        >,
    >,
    pub destroy: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub install: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(DownloadedUpdate) -> crate::FlightTask<AppUpdateInstallOutcome>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostUpdaterCommandCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
