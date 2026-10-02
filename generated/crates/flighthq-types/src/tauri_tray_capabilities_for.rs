// @generated from upstream/packages/types/src/TauriTrayCapabilitiesFor.ts; do not edit.
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

// Source: upstream/packages/types/src/TauriTrayCapabilitiesFor.ts:4 (sha256:98a86b67f7806acda67350e74d0e6041b094c5ad13c9bf442a7563230c6d92d8)
pub(crate) type TauriCommonTrayCapabilities = HostTrayCapabilities;

// Source: upstream/packages/types/src/TauriTrayCapabilitiesFor.ts:7 (sha256:7d7b019abb368d27b216014d0bea3426d6e2642867739ed1db958f82255f3d66)
#[derive(Clone, Default)]
pub(crate) struct TauriLinuxTrayCapabilities {
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
impl PartialEq for TauriLinuxTrayCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriTrayCapabilitiesFor.ts:8 (sha256:eb5891f9d9692169fdc00adad32f5145f9f96ff91f55ddff86889b9ce95161dc)
#[derive(Clone, Default)]
pub(crate) struct TauriMacosTrayCapabilities {
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
impl PartialEq for TauriMacosTrayCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriTrayCapabilitiesFor.ts:10 (sha256:e7812f7bccaff7c2293be19693dc6f6bf9f80a5564972a4b63e45e4958e1f5a5)
#[derive(Clone, Default)]
pub(crate) struct TauriWindowsTrayCapabilities {
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
impl PartialEq for TauriWindowsTrayCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TauriTrayCapabilitiesFor.ts:13 (sha256:38b7db63bdde6b1a221029afb656b60f7e457220201b7eef071894540792f099)
pub struct TauriTrayCapabilitiesFor<Profile>(
    pub crate::OpaqueHostValue,
    pub core::marker::PhantomData<fn() -> (Profile,)>,
);
impl<Profile> Clone for TauriTrayCapabilitiesFor<Profile> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), core::marker::PhantomData)
    }
}
