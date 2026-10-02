// @generated from upstream/packages/types/src/Tray.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, MenuItemTemplate, RectangleLike, Signal, Vector2Like};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
}
impl PartialEq for SharedStructuralRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
}
impl PartialEq for SharedStructuralRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayDropEventRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub text: String,
    pub type_: String,
}
impl PartialEq for TrayDropEventRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayDropEventRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub files: Vec<String>,
    pub type_: String,
}
impl PartialEq for TrayDropEventRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayCreateCapabilityResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayCreateCapabilityResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayCreateCapabilityResultRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayCreateCapabilityResultRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayCreateCapabilityResultRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
}
impl PartialEq for TrayCreateCapabilityResultRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayCreateCapabilityResultRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
}
impl PartialEq for TrayCreateCapabilityResultRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayDestroyCapabilityResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub failures: Vec<TrayDestroyFailure>,
    pub outcome: String,
}
impl PartialEq for TrayDestroyCapabilityResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayDestroyCapabilityResultRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
}
impl PartialEq for TrayDestroyCapabilityResultRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayDestroyResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
}
impl PartialEq for TrayDestroyResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayImageUpdateResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayImageUpdateResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayTitleUpdateResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayTitleUpdateResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayTooltipUpdateResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayTooltipUpdateResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayTemplateImageUpdateResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayTemplateImageUpdateResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayPressedImageUpdateResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayPressedImageUpdateResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayDoubleClickPolicyUpdateResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayDoubleClickPolicyUpdateResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayMenuUpdateResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayMenuUpdateResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayMenuUpdateResultRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayMenuUpdateResultRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayTitleReadResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayTitleReadResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayTitleReadResultRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
    pub title: String,
}
impl PartialEq for TrayTitleReadResultRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayTooltipReadResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayTooltipReadResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayTooltipReadResultRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
    pub tooltip: String,
}
impl PartialEq for TrayTooltipReadResultRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayBoundsResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayBoundsResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayBoundsResultRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub bounds: RectangleLike,
    pub outcome: String,
}
impl PartialEq for TrayBoundsResultRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayPopupMenuResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayPopupMenuResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayPopupMenuResultRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
}
impl PartialEq for TrayPopupMenuResultRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayPopupMenuResultRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
}
impl PartialEq for TrayPopupMenuResultRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayBalloonDisplayResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayBalloonDisplayResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayBalloonDisplayResultRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
}
impl PartialEq for TrayBalloonDisplayResultRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayBalloonRemoveResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayBalloonRemoveResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayBalloonRemoveResultRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
}
impl PartialEq for TrayBalloonRemoveResultRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayBalloonRemoveResultRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
}
impl PartialEq for TrayBalloonRemoveResultRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayReleaseResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayReleaseResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayReleaseResultRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
}
impl PartialEq for TrayReleaseResultRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayReleaseResultRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
}
impl PartialEq for TrayReleaseResultRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayEventAttachResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub outcome: String,
}
impl PartialEq for TrayEventAttachResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct TrayEventAttachResultRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
    pub release: TrayEventRelease,
}
impl PartialEq for TrayEventAttachResultRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayAnimationStartResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
}
impl PartialEq for TrayAnimationStartResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct TrayAnimationStartResultRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
    pub release: TrayEventRelease,
}
impl PartialEq for TrayAnimationStartResultRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayAnimationStopResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
}
impl PartialEq for TrayAnimationStopResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct TrayAnimationStopResultRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: String,
}
impl PartialEq for TrayAnimationStopResultRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:9 (sha256:3bd8cd4d6881e73799ebe7989c4ced709422048827c91ab4d7a1f4c3f38542d1)
pub type DesktopOsProfile = String;

// Source: upstream/packages/types/src/Tray.ts:13 (sha256:ed00696b16d8941708d8cec33b7d2134fc5d7bc23aacc9937e1ffb69b12f0a69)
pub type TrayIconSource = String;

// Source: upstream/packages/types/src/Tray.ts:15 (sha256:cc546524dc987b3aca2ae42436439af96e47f6e556161e6b957b996e6eb34830)
#[derive(Clone, Default)]
pub struct TrayIconOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub icon: Option<TrayIconSource>,
    pub icon_template: Option<bool>,
    pub signal: Option<crate::OpaqueHostValue>,
    pub title: Option<String>,
    pub tooltip: Option<String>,
}
impl PartialEq for TrayIconOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:23 (sha256:4dbea4d2f402511db7ffeb327cea223e5f6e76634af735ce39b5c50bfd41dd2c)
#[derive(Clone, Default)]
pub struct TrayBalloonOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub icon: Option<TrayIconSource>,
    pub icon_type: Option<String>,
    pub large_icon: Option<bool>,
    pub no_sound: Option<bool>,
    pub respect_quiet_time: Option<bool>,
    pub text: String,
    pub title: String,
}
impl PartialEq for TrayBalloonOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:34 (sha256:583c45b9aff3d02b0fc8e93108b9b45e0685dca04a6d9441ddc4e9ee268b13db)
#[derive(Clone, Default)]
pub struct TrayIcon {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for TrayIcon {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for TrayIcon {
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

// Source: upstream/packages/types/src/Tray.ts:36 (sha256:a711d8bd963fe1081e215b89292f66e8a2dde1bb86bf059469f178eef36f6c20)
#[derive(Clone, Default)]
pub struct TrayInteractionEvent {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alt_key: bool,
    pub bounds: Option<RectangleLike>,
    pub ctrl_key: bool,
    pub meta_key: bool,
    pub position: Option<Vector2Like>,
    pub shift_key: bool,
    pub type_: String,
}
impl PartialEq for TrayInteractionEvent {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:46 (sha256:5818942f3b3bebb8ebee5cd02ba09161fadec833206f38591edf0d1f9a5a29ef)
#[derive(Clone, Default)]
pub struct TrayMenuSelectionEvent {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: String,
}
impl PartialEq for TrayMenuSelectionEvent {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:50 (sha256:dd9b4ee38188837fd67cfbae2e77e2b6ac3b83b16b7f53863b80c888d142cd45)
#[derive(Clone, Default)]
pub struct TrayBalloonEvent {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub type_: String,
}
impl PartialEq for TrayBalloonEvent {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:54 (sha256:54258a9f35a5cf1a81c9d679607c6a177b69b40ffe4144b04f7a0085a1e8ba96)
pub type TrayDropEvent = crate::FlightUnion2<TrayDropEventRecord2, TrayDropEventRecord1>;

// Source: upstream/packages/types/src/Tray.ts:58 (sha256:3d85e4984989b6734248fc0e4a149c83bf1cbfa27366cfa8c94970e46ad02ab3)
pub type TrayCreateCapabilityResult = crate::FlightUnion2<
    TrayCreateCapabilityResultRecord5,
    crate::FlightUnion2<
        TrayCreateCapabilityResultRecord4,
        crate::FlightUnion2<
            TrayCreateCapabilityResultRecord3,
            crate::FlightUnion2<SharedStructuralRecord1, TrayCreateCapabilityResultRecord1>,
        >,
    >,
>;

// Source: upstream/packages/types/src/Tray.ts:65 (sha256:135599623292f26a1475e18ebb59a20a90d3e78e85fbc450cf16029dd74772f8)
#[derive(Clone)]
pub struct TrayCreateResult<Tray = TrayIcon> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub outcome: Option<String>,
    pub tray: Option<Tray>,
}
impl<Tray> Default for TrayCreateResult<Tray> {
    fn default() -> Self {
        Self {
            __flight_identity: Default::default(),
            outcome: Default::default(),
            tray: Default::default(),
        }
    }
}
impl<Tray> PartialEq for TrayCreateResult<Tray> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:69 (sha256:7593b5ae72113258dbb1178b58d2ecb5c2fc8d2fd20c69da44da879f65d372f0)
#[derive(Clone, Default)]
pub struct TrayDestroyFailure {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub error: Option<crate::FlightValue>,
    pub step: String,
}
impl PartialEq for TrayDestroyFailure {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:74 (sha256:04dc6eadf25fce1ad3ce040a6416d34bbed93c0486343ffa6f687e3b9b1757d7)
pub type TrayDestroyCapabilityResult =
    crate::FlightUnion2<TrayDestroyCapabilityResultRecord2, TrayDestroyCapabilityResultRecord1>;

// Source: upstream/packages/types/src/Tray.ts:78 (sha256:634ceb509395fb5e75d8ec77230fdfb763cf3844e9e950e5d45c4784f2ff5ec3)
pub type TrayDestroyResult =
    crate::FlightUnion2<TrayDestroyCapabilityResult, TrayDestroyResultRecord1>;

// Source: upstream/packages/types/src/Tray.ts:80 (sha256:da5708cbfd174191c76889418e71f9b16fd405e718c212759d0b4aaacd536b47)
pub type TrayImageUpdateResult = crate::FlightUnion2<
    SharedStructuralRecord2,
    crate::FlightUnion2<
        SharedStructuralRecord3,
        crate::FlightUnion2<SharedStructuralRecord1, TrayImageUpdateResultRecord1>,
    >,
>;

// Source: upstream/packages/types/src/Tray.ts:86 (sha256:e1146b39c5665683c02b1f21a0574eaae38af6bbd5b5b9e30303ab81d3e1baff)
pub type TrayTitleUpdateResult = crate::FlightUnion2<
    SharedStructuralRecord2,
    crate::FlightUnion2<SharedStructuralRecord3, TrayTitleUpdateResultRecord1>,
>;

// Source: upstream/packages/types/src/Tray.ts:91 (sha256:1cf96bf2057af455f977ff7f3790f687c7cf6cb548c263389c9c9f403b59200c)
pub type TrayTooltipUpdateResult = crate::FlightUnion2<
    SharedStructuralRecord2,
    crate::FlightUnion2<SharedStructuralRecord3, TrayTooltipUpdateResultRecord1>,
>;

// Source: upstream/packages/types/src/Tray.ts:96 (sha256:3e4d099cdabff541b39569f3a32e9022c8d0d90121830fad001323268bc29bfb)
pub type TrayTemplateImageUpdateResult = crate::FlightUnion2<
    SharedStructuralRecord2,
    crate::FlightUnion2<SharedStructuralRecord3, TrayTemplateImageUpdateResultRecord1>,
>;

// Source: upstream/packages/types/src/Tray.ts:101 (sha256:732d2504178c95c2e8a73fec8e783fd64493ab95ae12a388f49ec415c2d76cd9)
pub type TrayPressedImageUpdateResult = crate::FlightUnion2<
    SharedStructuralRecord2,
    crate::FlightUnion2<
        SharedStructuralRecord3,
        crate::FlightUnion2<SharedStructuralRecord1, TrayPressedImageUpdateResultRecord1>,
    >,
>;

// Source: upstream/packages/types/src/Tray.ts:107 (sha256:995a7d93ccfe6fd9cfdd4781b3fa844b5ef60b9c9c362407fc19e4f64633a547)
pub type TrayDoubleClickPolicyUpdateResult = crate::FlightUnion2<
    SharedStructuralRecord2,
    crate::FlightUnion2<SharedStructuralRecord3, TrayDoubleClickPolicyUpdateResultRecord1>,
>;

// Source: upstream/packages/types/src/Tray.ts:112 (sha256:f4172ac92bfac4bcda5d832cdbd1bd21bd888af93e19342ffadc2f8de0678576)
pub type TrayMenuUpdateResult = crate::FlightUnion2<
    SharedStructuralRecord2,
    crate::FlightUnion2<
        SharedStructuralRecord3,
        crate::FlightUnion2<TrayMenuUpdateResultRecord2, TrayMenuUpdateResultRecord1>,
    >,
>;

// Source: upstream/packages/types/src/Tray.ts:118 (sha256:aad3c61d3a004ab9432617dd7ebaa33a23feea0c3b9352551949bfd74fa4401a)
pub type TrayTitleReadResult = crate::FlightUnion2<
    TrayTitleReadResultRecord3,
    crate::FlightUnion2<SharedStructuralRecord3, TrayTitleReadResultRecord1>,
>;

// Source: upstream/packages/types/src/Tray.ts:123 (sha256:96624406c2914db00d7fb417e59ba955f5889d1bf2bb3b1ff9b4710d5494a2bd)
pub type TrayTooltipReadResult = crate::FlightUnion2<
    TrayTooltipReadResultRecord3,
    crate::FlightUnion2<SharedStructuralRecord3, TrayTooltipReadResultRecord1>,
>;

// Source: upstream/packages/types/src/Tray.ts:128 (sha256:c1bca810093f47271da2b22c4e52776bf240b3b5c9224d1a78201b80d850ef76)
pub type TrayBoundsResult = crate::FlightUnion2<
    TrayBoundsResultRecord3,
    crate::FlightUnion2<SharedStructuralRecord3, TrayBoundsResultRecord1>,
>;

// Source: upstream/packages/types/src/Tray.ts:133 (sha256:8c398c032137f586191ccb4796032cc47679a3ecc31d95a808db02761dd4c65b)
pub type TrayPopupMenuResult = crate::FlightUnion2<
    TrayPopupMenuResultRecord4,
    crate::FlightUnion2<
        SharedStructuralRecord3,
        crate::FlightUnion2<TrayPopupMenuResultRecord2, TrayPopupMenuResultRecord1>,
    >,
>;

// Source: upstream/packages/types/src/Tray.ts:139 (sha256:2ef458eade94e7b3485741b4bcc729676c1b2fecf86135f42180660405901944)
pub type TrayBalloonDisplayResult = crate::FlightUnion2<
    TrayBalloonDisplayResultRecord3,
    crate::FlightUnion2<SharedStructuralRecord3, TrayBalloonDisplayResultRecord1>,
>;

// Source: upstream/packages/types/src/Tray.ts:144 (sha256:3c72758bfe3687c5f29ffa3bf7d31c21004f084bfa111a6b8d15fb1eabb3a8e3)
pub type TrayBalloonRemoveResult = crate::FlightUnion2<
    TrayBalloonRemoveResultRecord4,
    crate::FlightUnion2<
        SharedStructuralRecord3,
        crate::FlightUnion2<TrayBalloonRemoveResultRecord2, TrayBalloonRemoveResultRecord1>,
    >,
>;

// Source: upstream/packages/types/src/Tray.ts:150 (sha256:0afdba974059bd85f41b7c87aa54e9fba2c1211aefb14193bcbda5a2d44db042)
pub type TrayReleaseResult = crate::FlightUnion2<
    TrayReleaseResultRecord3,
    crate::FlightUnion2<TrayReleaseResultRecord2, TrayReleaseResultRecord1>,
>;

// Source: upstream/packages/types/src/Tray.ts:155 (sha256:03edab7a618ce98f66fb30871022248f3e91803aad9197ff43d09c47e64383de)
#[derive(Clone)]
pub struct TrayEventRelease {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub release: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> crate::FlightTask<TrayReleaseResult> + Send + 'static>>,
    >,
}
impl PartialEq for TrayEventRelease {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:159 (sha256:8b319b9498f375cafc7da55b4fbd740065a359b0856196354fcf62fbb5223588)
pub type TrayEventAttachResult = crate::FlightUnion2<
    TrayEventAttachResultRecord3,
    crate::FlightUnion2<SharedStructuralRecord3, TrayEventAttachResultRecord1>,
>;

// Source: upstream/packages/types/src/Tray.ts:164 (sha256:d365d6c0f886277aa947433e9692ad9ea08ea636268cb154d8c0ee82cdc01afa)
pub type TrayAnimationStartResult = crate::FlightUnion2<
    TrayAnimationStartResultRecord2,
    crate::FlightUnion2<TrayAnimationStartResultRecord1, TrayImageUpdateResult>,
>;

// Source: upstream/packages/types/src/Tray.ts:169 (sha256:9bdc2f6126a8e428357927d5e8ece3108fc85c6fa81d9fee0e18ec2b88ac6f3c)
pub type TrayAnimationStopResult =
    crate::FlightUnion2<TrayAnimationStopResultRecord2, TrayAnimationStopResultRecord1>;

// Source: upstream/packages/types/src/Tray.ts:173 (sha256:8ddf2ff3d73c83baf7873723af71cdeda79abb129b72ff6896aad87af8670a4c)
#[derive(Clone)]
pub struct HostTrayLifecycleCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub create: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        TrayIcon,
                        TrayIconOptions,
                    ) -> crate::FlightTask<TrayCreateCapabilityResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub destroy: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(TrayIcon) -> crate::FlightTask<TrayDestroyCapabilityResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub is_destroyed:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(TrayIcon) -> bool + Send + 'static>>>,
    pub list: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Vec<TrayIcon> + Send + 'static>>>,
}
impl PartialEq for HostTrayLifecycleCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:180 (sha256:c6a8e0a2a69ba5a6a55f0893958af536f16713f8efbcc027e5015b80b8b389ea)
#[derive(Clone)]
pub struct HostTrayImageCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(TrayIcon, TrayIconSource) -> crate::FlightTask<TrayImageUpdateResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostTrayImageCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:184 (sha256:df67b3951920763c27e11519c1f93937781155417c78e64d77fb3ea092635b93)
#[derive(Clone)]
pub struct HostTrayTitleCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(TrayIcon) -> crate::FlightTask<TrayTitleReadResult> + Send + 'static>,
        >,
    >,
    pub set: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(TrayIcon, String) -> crate::FlightTask<TrayTitleUpdateResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostTrayTitleCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:189 (sha256:9ff26be6a3d4c25eb6a62fa156f6c529e562bb56acd21744572d0096cb791394)
#[derive(Clone)]
pub struct HostTrayTooltipCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(TrayIcon) -> crate::FlightTask<TrayTooltipReadResult> + Send + 'static>,
        >,
    >,
    pub set: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(TrayIcon, String) -> crate::FlightTask<TrayTooltipUpdateResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostTrayTooltipCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:194 (sha256:534b11669a56dfadbb623a1b244253e0ac6259782a1265132f2797e6770143b4)
#[derive(Clone)]
pub struct HostTrayMenuCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        TrayIcon,
                        Vec<MenuItemTemplate>,
                    ) -> crate::FlightTask<TrayMenuUpdateResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostTrayMenuCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:198 (sha256:7e51cae910e65c4aaa309fee5ba601c177508af1965dd0cdb6923dff304e5d7d)
#[derive(Clone)]
pub struct HostTrayTemplateImageCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(TrayIcon, bool) -> crate::FlightTask<TrayTemplateImageUpdateResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostTrayTemplateImageCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:202 (sha256:58fba630a7bcdbc2a88ef47e49349c0c4949fcf52f1ab2e98abf7cecb08cb013)
#[derive(Clone)]
pub struct HostTrayBoundsCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(TrayIcon) -> crate::FlightTask<TrayBoundsResult> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostTrayBoundsCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:206 (sha256:053edfee007962aef3f1184cc58b8cdcb1f1df161729feadb0ff2dc95e004f3c)
#[derive(Clone)]
pub struct HostTrayPopupMenuCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub popup: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(TrayIcon, Option<Vector2Like>) -> crate::FlightTask<TrayPopupMenuResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostTrayPopupMenuCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:210 (sha256:4043230cb0848251f6a1f59ca32374bb287e7ed3e62a6f7b8dad2a095280d753)
#[derive(Clone)]
pub struct HostTrayDoubleClickPolicyCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set_ignore: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(TrayIcon, bool) -> crate::FlightTask<TrayDoubleClickPolicyUpdateResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostTrayDoubleClickPolicyCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:214 (sha256:fed3928c7d1a6cf0aea898df6e1f6b2d86e45b72f449e61ae2380607fecf0e85)
#[derive(Clone)]
pub struct HostTrayPressedImageCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub set: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        TrayIcon,
                        TrayIconSource,
                    ) -> crate::FlightTask<TrayPressedImageUpdateResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostTrayPressedImageCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:218 (sha256:2dde3df2990472ba2cf98d0ddb219415f011334aebe6621b0b98919f40bf1b7f)
#[derive(Clone)]
pub struct HostTrayBalloonCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub display: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        TrayIcon,
                        TrayBalloonOptions,
                    ) -> crate::FlightTask<TrayBalloonDisplayResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub remove: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(TrayIcon) -> crate::FlightTask<TrayBalloonRemoveResult> + Send + 'static>,
        >,
    >,
}
impl PartialEq for HostTrayBalloonCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:223 (sha256:90f97a8739f8da1c9e5c311912c2296e48b9e8770f4a2524b3737e56d05cad79)
#[derive(Clone)]
pub struct HostTrayInteractionEventsCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_signal: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        TrayIcon,
                    ) -> Option<
                        Signal<
                            std::sync::Arc<
                                std::sync::Mutex<
                                    Box<dyn FnMut(TrayInteractionEvent) -> () + Send + 'static>,
                                >,
                            >,
                        >,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostTrayInteractionEventsCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:227 (sha256:899f0e4b1031b6426b7eaba8a5d9d63974d28989d1a9c675beae7400e1c234bb)
#[derive(Clone)]
pub struct HostTrayMenuSelectionEventsCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_signal: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        TrayIcon,
                    ) -> Option<
                        Signal<
                            std::sync::Arc<
                                std::sync::Mutex<
                                    Box<dyn FnMut(TrayMenuSelectionEvent) -> () + Send + 'static>,
                                >,
                            >,
                        >,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostTrayMenuSelectionEventsCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:231 (sha256:c1a86bda5066af984bc5c301c89126fc5076d83a0df4e3d426fa8bd2b5110463)
#[derive(Clone)]
pub struct HostTrayBalloonEventsCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_signal: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        TrayIcon,
                    ) -> Option<
                        Signal<
                            std::sync::Arc<
                                std::sync::Mutex<
                                    Box<dyn FnMut(TrayBalloonEvent) -> () + Send + 'static>,
                                >,
                            >,
                        >,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostTrayBalloonEventsCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Tray.ts:235 (sha256:1991012e5a6021062bdbddc9fa1b8a017a16315a7aaa8231b950b9e5b350578f)
#[derive(Clone)]
pub struct HostTrayDropEventsCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_signal: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        TrayIcon,
                    ) -> Option<
                        Signal<
                            std::sync::Arc<
                                std::sync::Mutex<
                                    Box<dyn FnMut(TrayDropEvent) -> () + Send + 'static>,
                                >,
                            >,
                        >,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostTrayDropEventsCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
