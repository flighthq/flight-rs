// @generated from upstream/packages/types/src/CapacitorAppCapabilitiesFor.ts; do not edit.
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

// Source: upstream/packages/types/src/CapacitorAppCapabilitiesFor.ts:4 (sha256:2de85b09561db5b9cc4a838509c6290f67575d4fce3f0b50741581849eafe6ca)
pub type CapacitorCommonAppCapabilities = HostAppCapabilities;

// Source: upstream/packages/types/src/CapacitorAppCapabilitiesFor.ts:5 (sha256:7c65235579692578092b594e299547dfdf4ac74f547a78feee3d5e4f522066c4)
#[derive(Clone, Default)]
pub struct CapacitorAndroidAppCapabilities {
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
impl PartialEq for CapacitorAndroidAppCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorAppCapabilitiesFor.ts:8 (sha256:e64366e9386be98bdae7e2fe54af7fdf81b998e9bc1d8a3a489d9e94d4db3854)
pub struct CapacitorAppCapabilitiesFor<Profile>(
    pub crate::OpaqueHostValue,
    pub core::marker::PhantomData<fn() -> (Profile,)>,
);
impl<Profile> Clone for CapacitorAppCapabilitiesFor<Profile> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), core::marker::PhantomData)
    }
}
