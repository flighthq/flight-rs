// @generated from upstream/packages/types/src/ElectronAppCapabilitiesFor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    HostAppActivateCapability, HostAppActivationPolicyCapability,
    HostAppAllWindowsClosedCapability, HostAppBadgeCapability, HostAppCapabilities,
    HostAppDockCapability, HostAppExitCapability, HostAppFocusCapability, HostAppHideCapability,
    HostAppLocaleCapability, HostAppLoginItemCapability, HostAppLoopCapability,
    HostAppNameCapability, HostAppNameWriteCapability, HostAppOpenFileCapability,
    HostAppPathCapability, HostAppQuitCapability, HostAppQuitRequestCapability,
    HostAppReadyCapability, HostAppRecentDocumentsCapability, HostAppRelaunchCapability,
    HostAppSecondInstanceCapability, HostAppShowCapability, HostAppSingleInstanceCapability,
    HostAppUserModelIdCapability, HostAppVersionCapability,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub activate: Option<HostAppActivateCapability>,
    pub activation_policy: Option<HostAppActivationPolicyCapability>,
    pub all_windows_closed: Option<HostAppAllWindowsClosedCapability>,
    pub badge: Option<HostAppBadgeCapability>,
    pub dock: Option<HostAppDockCapability>,
    pub exit: Option<HostAppExitCapability>,
    pub focus: Option<HostAppFocusCapability>,
    pub hide: Option<HostAppHideCapability>,
    pub locale: Option<HostAppLocaleCapability>,
    pub login_item: Option<HostAppLoginItemCapability>,
    pub loop_: Option<HostAppLoopCapability>,
    pub name: Option<HostAppNameCapability>,
    pub name_write: Option<HostAppNameWriteCapability>,
    pub open_file: Option<HostAppOpenFileCapability>,
    pub path: Option<HostAppPathCapability>,
    pub quit: Option<HostAppQuitCapability>,
    pub quit_request: Option<HostAppQuitRequestCapability>,
    pub ready: Option<HostAppReadyCapability>,
    pub recent_documents: Option<HostAppRecentDocumentsCapability>,
    pub relaunch: Option<HostAppRelaunchCapability>,
    pub second_instance: Option<HostAppSecondInstanceCapability>,
    pub show: Option<HostAppShowCapability>,
    pub single_instance: Option<HostAppSingleInstanceCapability>,
    pub user_model_id: Option<HostAppUserModelIdCapability>,
    pub version: Option<HostAppVersionCapability>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronAppCapabilitiesFor.ts:4 (sha256:039ff8b46a2fea8eb7fac4b996e3cc6c900d3e5d7ae784ab081697b906b19722)
pub type ElectronCommonAppCapabilities = HostAppCapabilities;

// Source: upstream/packages/types/src/ElectronAppCapabilitiesFor.ts:22 (sha256:0f32b6bd108eda0eed5ae918cd7938ded44ddd8559e88824db8d3742db60e6af)
#[derive(Clone, Default)]
pub struct ElectronMacosAppCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub activate: Option<HostAppActivateCapability>,
    pub activation_policy: Option<HostAppActivationPolicyCapability>,
    pub all_windows_closed: Option<HostAppAllWindowsClosedCapability>,
    pub badge: Option<HostAppBadgeCapability>,
    pub dock: Option<HostAppDockCapability>,
    pub exit: Option<HostAppExitCapability>,
    pub focus: Option<HostAppFocusCapability>,
    pub hide: Option<HostAppHideCapability>,
    pub locale: Option<HostAppLocaleCapability>,
    pub login_item: Option<HostAppLoginItemCapability>,
    pub loop_: Option<HostAppLoopCapability>,
    pub name: Option<HostAppNameCapability>,
    pub name_write: Option<HostAppNameWriteCapability>,
    pub open_file: Option<HostAppOpenFileCapability>,
    pub path: Option<HostAppPathCapability>,
    pub quit: Option<HostAppQuitCapability>,
    pub quit_request: Option<HostAppQuitRequestCapability>,
    pub ready: Option<HostAppReadyCapability>,
    pub recent_documents: Option<HostAppRecentDocumentsCapability>,
    pub relaunch: Option<HostAppRelaunchCapability>,
    pub second_instance: Option<HostAppSecondInstanceCapability>,
    pub show: Option<HostAppShowCapability>,
    pub single_instance: Option<HostAppSingleInstanceCapability>,
    pub user_model_id: Option<HostAppUserModelIdCapability>,
    pub version: Option<HostAppVersionCapability>,
}
impl PartialEq for ElectronMacosAppCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronAppCapabilitiesFor.ts:37 (sha256:fcc87c7979e63f85d2bdcd0a4b86a6929a8dd0802dc05c3b70bf80d70c07fbcf)
#[derive(Clone, Default)]
pub struct ElectronLinuxAppCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub activate: Option<HostAppActivateCapability>,
    pub activation_policy: Option<HostAppActivationPolicyCapability>,
    pub all_windows_closed: Option<HostAppAllWindowsClosedCapability>,
    pub badge: Option<HostAppBadgeCapability>,
    pub dock: Option<HostAppDockCapability>,
    pub exit: Option<HostAppExitCapability>,
    pub focus: Option<HostAppFocusCapability>,
    pub hide: Option<HostAppHideCapability>,
    pub locale: Option<HostAppLocaleCapability>,
    pub login_item: Option<HostAppLoginItemCapability>,
    pub loop_: Option<HostAppLoopCapability>,
    pub name: Option<HostAppNameCapability>,
    pub name_write: Option<HostAppNameWriteCapability>,
    pub open_file: Option<HostAppOpenFileCapability>,
    pub path: Option<HostAppPathCapability>,
    pub quit: Option<HostAppQuitCapability>,
    pub quit_request: Option<HostAppQuitRequestCapability>,
    pub ready: Option<HostAppReadyCapability>,
    pub recent_documents: Option<HostAppRecentDocumentsCapability>,
    pub relaunch: Option<HostAppRelaunchCapability>,
    pub second_instance: Option<HostAppSecondInstanceCapability>,
    pub show: Option<HostAppShowCapability>,
    pub single_instance: Option<HostAppSingleInstanceCapability>,
    pub user_model_id: Option<HostAppUserModelIdCapability>,
    pub version: Option<HostAppVersionCapability>,
}
impl PartialEq for ElectronLinuxAppCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronAppCapabilitiesFor.ts:38 (sha256:a6400139823cc6942a3f25eb3a473e769bbf1aebdfa314e60cddfdbfa071d09f)
#[derive(Clone, Default)]
pub struct ElectronWindowsAppCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub activate: Option<HostAppActivateCapability>,
    pub activation_policy: Option<HostAppActivationPolicyCapability>,
    pub all_windows_closed: Option<HostAppAllWindowsClosedCapability>,
    pub badge: Option<HostAppBadgeCapability>,
    pub dock: Option<HostAppDockCapability>,
    pub exit: Option<HostAppExitCapability>,
    pub focus: Option<HostAppFocusCapability>,
    pub hide: Option<HostAppHideCapability>,
    pub locale: Option<HostAppLocaleCapability>,
    pub login_item: Option<HostAppLoginItemCapability>,
    pub loop_: Option<HostAppLoopCapability>,
    pub name: Option<HostAppNameCapability>,
    pub name_write: Option<HostAppNameWriteCapability>,
    pub open_file: Option<HostAppOpenFileCapability>,
    pub path: Option<HostAppPathCapability>,
    pub quit: Option<HostAppQuitCapability>,
    pub quit_request: Option<HostAppQuitRequestCapability>,
    pub ready: Option<HostAppReadyCapability>,
    pub recent_documents: Option<HostAppRecentDocumentsCapability>,
    pub relaunch: Option<HostAppRelaunchCapability>,
    pub second_instance: Option<HostAppSecondInstanceCapability>,
    pub show: Option<HostAppShowCapability>,
    pub single_instance: Option<HostAppSingleInstanceCapability>,
    pub user_model_id: Option<HostAppUserModelIdCapability>,
    pub version: Option<HostAppVersionCapability>,
}
impl PartialEq for ElectronWindowsAppCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronAppCapabilitiesFor.ts:41 (sha256:ae349dc50d4eee4ec840b5a2fed6ac6551c83b2654581be2108cb78ac0e457af)
pub struct ElectronAppCapabilitiesFor<Profile>(
    pub crate::OpaqueHostValue,
    pub core::marker::PhantomData<fn() -> (Profile,)>,
);
impl<Profile> Clone for ElectronAppCapabilitiesFor<Profile> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), core::marker::PhantomData)
    }
}
