// @generated from upstream/packages/types/src/ElectronTrayCapabilitiesFor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    HostTrayBalloonCapability, HostTrayBalloonEventsCapability, HostTrayBoundsCapability,
    HostTrayCapabilities, HostTrayDoubleClickPolicyCapability, HostTrayDropEventsCapability,
    HostTrayImageCapability, HostTrayInteractionEventsCapability, HostTrayLifecycleCapability,
    HostTrayMenuCapability, HostTrayMenuSelectionEventsCapability, HostTrayPopupMenuCapability,
    HostTrayPressedImageCapability, HostTrayTemplateImageCapability, HostTrayTitleCapability,
    HostTrayTooltipCapability,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub balloon: Option<HostTrayBalloonCapability>,
    pub balloon_events: Option<HostTrayBalloonEventsCapability>,
    pub bounds: Option<HostTrayBoundsCapability>,
    pub double_click_policy: Option<HostTrayDoubleClickPolicyCapability>,
    pub drop_events: Option<HostTrayDropEventsCapability>,
    pub image: Option<HostTrayImageCapability>,
    pub interaction_events: Option<HostTrayInteractionEventsCapability>,
    pub lifecycle: Option<HostTrayLifecycleCapability>,
    pub menu: Option<HostTrayMenuCapability>,
    pub menu_selection_events: Option<HostTrayMenuSelectionEventsCapability>,
    pub popup_menu: Option<HostTrayPopupMenuCapability>,
    pub pressed_image: Option<HostTrayPressedImageCapability>,
    pub template_image: Option<HostTrayTemplateImageCapability>,
    pub title: Option<HostTrayTitleCapability>,
    pub tooltip: Option<HostTrayTooltipCapability>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronTrayCapabilitiesFor.ts:4 (sha256:fe1f2a747486fa1e86f60a03f5660ba9e06be2b0ccffa387d256371db2678a31)
pub(crate) type ElectronCommonTrayCapabilities = HostTrayCapabilities;

// Source: upstream/packages/types/src/ElectronTrayCapabilitiesFor.ts:10 (sha256:6fe3cecf20028f0f9aad47cba1d5f33ad8e98a727722ca762193b74d428544ab)
#[derive(Clone, Default)]
pub(crate) struct ElectronMacosTrayCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub balloon: Option<HostTrayBalloonCapability>,
    pub balloon_events: Option<HostTrayBalloonEventsCapability>,
    pub bounds: Option<HostTrayBoundsCapability>,
    pub double_click_policy: Option<HostTrayDoubleClickPolicyCapability>,
    pub drop_events: Option<HostTrayDropEventsCapability>,
    pub image: Option<HostTrayImageCapability>,
    pub interaction_events: Option<HostTrayInteractionEventsCapability>,
    pub lifecycle: Option<HostTrayLifecycleCapability>,
    pub menu: Option<HostTrayMenuCapability>,
    pub menu_selection_events: Option<HostTrayMenuSelectionEventsCapability>,
    pub popup_menu: Option<HostTrayPopupMenuCapability>,
    pub pressed_image: Option<HostTrayPressedImageCapability>,
    pub template_image: Option<HostTrayTemplateImageCapability>,
    pub title: Option<HostTrayTitleCapability>,
    pub tooltip: Option<HostTrayTooltipCapability>,
}
impl PartialEq for ElectronMacosTrayCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronTrayCapabilitiesFor.ts:12 (sha256:7a223616fcb57d33dd73b09b80578548116ad46b2d689b0c4bacc5f2e08db405)
#[derive(Clone, Default)]
pub(crate) struct ElectronWindowsTrayCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub balloon: Option<HostTrayBalloonCapability>,
    pub balloon_events: Option<HostTrayBalloonEventsCapability>,
    pub bounds: Option<HostTrayBoundsCapability>,
    pub double_click_policy: Option<HostTrayDoubleClickPolicyCapability>,
    pub drop_events: Option<HostTrayDropEventsCapability>,
    pub image: Option<HostTrayImageCapability>,
    pub interaction_events: Option<HostTrayInteractionEventsCapability>,
    pub lifecycle: Option<HostTrayLifecycleCapability>,
    pub menu: Option<HostTrayMenuCapability>,
    pub menu_selection_events: Option<HostTrayMenuSelectionEventsCapability>,
    pub popup_menu: Option<HostTrayPopupMenuCapability>,
    pub pressed_image: Option<HostTrayPressedImageCapability>,
    pub template_image: Option<HostTrayTemplateImageCapability>,
    pub title: Option<HostTrayTitleCapability>,
    pub tooltip: Option<HostTrayTooltipCapability>,
}
impl PartialEq for ElectronWindowsTrayCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronTrayCapabilitiesFor.ts:15 (sha256:bc304cab58935fc7398b3c50385ac166bbd382573482a7c91793a7e7bd191ae8)
pub struct ElectronTrayCapabilitiesFor<Profile>(
    pub crate::OpaqueHostValue,
    pub core::marker::PhantomData<fn() -> (Profile,)>,
);
impl<Profile> Clone for ElectronTrayCapabilitiesFor<Profile> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), core::marker::PhantomData)
    }
}
