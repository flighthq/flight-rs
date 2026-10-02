// @generated from upstream/packages/types/src/Host.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    EntityRuntime, HostAccessibilityCapability, HostAppActivateCapability,
    HostAppActivationPolicyCapability, HostAppAllWindowsClosedCapability, HostAppBadgeCapability,
    HostAppDockCapability, HostAppExitCapability, HostAppFocusCapability, HostAppHideCapability,
    HostAppLocaleCapability, HostAppLoginItemCapability, HostAppLoopCapability,
    HostAppMenuCapability, HostAppNameCapability, HostAppNameWriteCapability,
    HostAppOpenFileCapability, HostAppPathCapability, HostAppQuitCapability,
    HostAppQuitRequestCapability, HostAppReadyCapability, HostAppRecentDocumentsCapability,
    HostAppRelaunchCapability, HostAppSecondInstanceCapability, HostAppShowCapability,
    HostAppSingleInstanceCapability, HostAppUserModelIdCapability, HostAppVersionCapability,
    HostAudioCodecCapability, HostAudioDecodeCapabilities, HostAudioDeviceCapability,
    HostAudioMixerCapability, HostBitmapEncodeCapability, HostBitmapReadbackCapability,
    HostCanvasCapability, HostClipboardBookmarkCapability, HostClipboardChangeCapability,
    HostClipboardFormatsCapability, HostClipboardImageCapability, HostClipboardTextCapability,
    HostCompressDeflateCapability, HostCompressLzmaCapability, HostConnectivityChangeCapability,
    HostConnectivityReachabilityCapability, HostConnectivityStatusCapability,
    HostDecompressBrotliCapability, HostDecompressDeflateCapability, HostDecompressLzmaCapability,
    HostDeviceCapability, HostDirectoryOpenDialogCapability, HostElementFullscreenCapability,
    HostFileOpenDialogCapability, HostFileSaveDialogCapability, HostFileSystemCapability,
    HostFontLoadingCapability, HostGeolocationCapability, HostGlCapability,
    HostGlyphRasterizerCapability, HostHapticsCapability, HostImageCapability,
    HostImageDecodeCapabilities, HostImageEncodeCapabilities, HostImageOpenDialogCapability,
    HostInputDropFileCapability, HostInputFocusCapability, HostInputIngressCapability,
    HostInputPointerLockCapability, HostInputTargetCapability, HostIpcHandleCapability,
    HostIpcInvokeCapability, HostIpcMessageCapability, HostIpcSendCapability,
    HostIpcTargetedSendCapability, HostLifecycleCapability, HostMediaSessionActionCapability,
    HostMediaSessionCapability, HostMenuHighlightCapability, HostMenuPopupCapability,
    HostMenuSelectCapability, HostMessageDialogCapability, HostMidiAccessCapability,
    HostMidiPermissionCapability, HostNetCapability, HostNotificationActionCapability,
    HostNotificationActiveListCapability, HostNotificationClickCapability,
    HostNotificationCloseCapability, HostNotificationDeliveryCapability,
    HostNotificationDismissCapability, HostNotificationLifecycleCapability,
    HostNotificationPermissionCapability, HostNotificationReceivedCapability,
    HostNotificationReplyCapability, HostNotificationSchedulingCapability,
    HostPermissionsCapability, HostPhotoCaptureDialogCapability, HostPlatformCapability,
    HostPowerBatteryHealthCapability, HostPowerChangeCapability, HostPowerIdleCapability,
    HostPowerKeepAwakeCapability, HostPowerSessionLockCapability, HostPowerStatusCapability,
    HostPowerSuspensionCapability, HostPowerThermalCapability, HostPreferencesCapability,
    HostPreferencesChangeCapability, HostPreferencesPersistenceQueryCapability,
    HostPreferencesPersistenceRequestCapability, HostPromptDialogCapability,
    HostProtocolDefaultCapability, HostProtocolLaunchCapability, HostProtocolOpenCapability,
    HostProtocolRegistrationCapability, HostProtocolRegistrationQueryCapability,
    HostProtocolUnregistrationCapability, HostScreenChangeCapability, HostScreenDetailsCapability,
    HostScreenPermissionChangeCapability, HostScreenQueryCapability, HostSensorsCapability,
    HostShareContentCapability, HostShareFilesCapability, HostShellBeepCapability,
    HostShellExternalCapability, HostShellPathOpenCapability, HostShellPathRevealCapability,
    HostShellProcessCapability, HostShellShortcutLinkCapability, HostShellTrashCapability,
    HostShortcutQueryCapability, HostShortcutTriggerCapability, HostSocketCapability,
    HostSoftKeyboardAccessoryBarCapability, HostSoftKeyboardChangeCapability,
    HostSoftKeyboardInfoCapability, HostSoftKeyboardResizeModeWriteCapability,
    HostSoftKeyboardScrollAssistCapability, HostSoftKeyboardStyleCapability,
    HostSoftKeyboardVisibilityCapability, HostStatusBarChangeCapability,
    HostStatusBarColorCapability, HostStatusBarInfoCapability, HostStatusBarOverlaysCapability,
    HostStatusBarStyleCapability, HostStatusBarVisibilityCapability, HostSurfaceDisplayCapability,
    HostSurfaceResizeCapability, HostTextSegmenterCapability, HostTextShaperCapability,
    HostTrayBalloonCapability, HostTrayBalloonEventsCapability, HostTrayBoundsCapability,
    HostTrayDoubleClickPolicyCapability, HostTrayDropEventsCapability, HostTrayImageCapability,
    HostTrayInteractionEventsCapability, HostTrayLifecycleCapability, HostTrayMenuCapability,
    HostTrayMenuSelectionEventsCapability, HostTrayPopupMenuCapability,
    HostTrayPressedImageCapability, HostTrayTemplateImageCapability, HostTrayTitleCapability,
    HostTrayTooltipCapability, HostUpdaterCommandCapability, HostVideoCapability,
    HostVideoCaptureDialogCapability, HostWgpuCapability, HostWindowAppearanceCapability,
    HostWindowAttachCapability, HostWindowAttentionCapability,
    HostWindowContentProtectionCapability, HostWindowFocusCapability,
    HostWindowFullscreenCapability, HostWindowGeometryCapability, HostWindowHierarchyCapability,
    HostWindowLifecycleCapability, HostWindowProgressCapability, HostWindowShadowCapability,
    HostWindowShellCapability, HostWindowSizeConstraintsCapability, HostWindowStateCapability,
    HostWindowVisibilityCapability, HostWindowZOrderCapability,
};

// Source: upstream/packages/types/src/Host.ts:218 (sha256:e62b51b1eb7bab6fd3d9f151d44438e1cd99a1dce41fce9b16562677aede43e7)
#[derive(Clone, Default)]
pub struct Host {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub accessibility: HostAccessibilityCapabilities,
    pub app: HostAppCapabilities,
    pub audio: HostAudioCapabilities,
    pub audio_decode: HostAudioDecodeCapabilities,
    pub bitmap: HostBitmapCapabilities,
    pub canvas: HostCanvasCapabilities,
    pub clipboard: HostClipboardCapabilities,
    pub compress: HostCompressCapabilities,
    pub connectivity: HostConnectivityCapabilities,
    pub decompress: HostDecompressCapabilities,
    pub device: HostDeviceCapabilities,
    pub dialog: HostDialogCapabilities,
    pub file_system: HostFileSystemCapabilities,
    pub font: HostFontCapabilities,
    pub fullscreen: HostFullscreenCapabilities,
    pub geolocation: HostGeolocationCapabilities,
    pub gl: HostGlCapabilities,
    pub glyph: HostGlyphCapabilities,
    pub haptics: HostHapticsCapabilities,
    pub image: HostImageCapabilities,
    pub image_decode: HostImageDecodeCapabilities,
    pub image_encode: HostImageEncodeCapabilities,
    pub input: HostInputCapabilities,
    pub ipc: HostIpcCapabilities,
    pub lifecycle: HostLifecycleCapabilities,
    pub media_session: HostMediaSessionCapabilities,
    pub menu: HostMenuCapabilities,
    pub midi: HostMidiCapabilities,
    pub net: HostNetCapabilities,
    pub notification: HostNotificationCapabilities,
    pub permissions: HostPermissionsCapabilities,
    pub platform: HostPlatformCapabilities,
    pub power: HostPowerCapabilities,
    pub preferences: HostPreferencesCapabilities,
    pub protocol: HostProtocolCapabilities,
    pub screen: HostScreenCapabilities,
    pub sensors: HostSensorsCapabilities,
    pub share: HostShareCapabilities,
    pub shell: HostShellCapabilities,
    pub shortcut: HostShortcutCapabilities,
    pub socket: HostSocketCapabilities,
    pub soft_keyboard: HostSoftKeyboardCapabilities,
    pub status_bar: HostStatusBarCapabilities,
    pub surface: HostSurfaceCapabilities,
    pub text_segment: HostTextSegmentCapabilities,
    pub text_shaper: HostTextShaperCapabilities,
    pub tray: HostTrayCapabilities,
    pub updater: HostUpdaterCapabilities,
    pub video: HostVideoCapabilities,
    pub wgpu: HostWgpuCapabilities,
    pub window: HostWindowCapabilities,
}
impl PartialEq for Host {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Host {
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

// Source: upstream/packages/types/src/Host.ts:272 (sha256:6fc29c074716084827d6ec65d2b7590279408d1c1603ea28bd5b49cc74a85f50)
#[derive(Clone, Default)]
pub struct HostAccessibilityCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub tree: Option<HostAccessibilityCapability>,
}
impl PartialEq for HostAccessibilityCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:276 (sha256:e741148cdafdf10960cb9fe7ba1ad87845f0f2e49a93f59b5a44d6dbccb4081f)
#[derive(Clone, Default)]
pub struct HostAppCapabilities {
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
impl PartialEq for HostAppCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:304 (sha256:2d01fb3da8ca39bb8c224f3a3328991b38fc13056317940f06b76ef18e520d15)
#[derive(Clone, Default)]
pub struct HostAudioCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub codec: Option<HostAudioCodecCapability>,
    pub device: Option<HostAudioDeviceCapability>,
    pub mixer: Option<HostAudioMixerCapability>,
}
impl PartialEq for HostAudioCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:310 (sha256:b72eac335ec0a58f925c4415f8885fcf54081d94cc07cbe796e399a657fe6b57)
#[derive(Clone, Default)]
pub struct HostBitmapCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub encode: Option<HostBitmapEncodeCapability>,
    pub readback: Option<HostBitmapReadbackCapability>,
}
impl PartialEq for HostBitmapCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:315 (sha256:408904fd472fe190eda812b7613e3d8a8d054a5810d6b882079b93573429b63d)
#[derive(Clone, Default)]
pub struct HostClipboardCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bookmark: Option<HostClipboardBookmarkCapability>,
    pub change: Option<HostClipboardChangeCapability>,
    pub formats: Option<HostClipboardFormatsCapability>,
    pub image: Option<HostClipboardImageCapability>,
    pub text: Option<HostClipboardTextCapability>,
}
impl PartialEq for HostClipboardCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:323 (sha256:0387378b9123617a43991b896f5144a3518f9212d99d30cf53e150d10ae4b4a2)
#[derive(Clone, Default)]
pub struct HostCompressCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub deflate: Option<HostCompressDeflateCapability>,
    pub lzma: Option<HostCompressLzmaCapability>,
}
impl PartialEq for HostCompressCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:328 (sha256:7c1941463612d84e81fbe85f1110fc749dba9760fc2943118478aaf95370535d)
#[derive(Clone, Default)]
pub struct HostConnectivityCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub change: Option<HostConnectivityChangeCapability>,
    pub reachability: Option<HostConnectivityReachabilityCapability>,
    pub status: Option<HostConnectivityStatusCapability>,
}
impl PartialEq for HostConnectivityCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:334 (sha256:f746f344c1f2f7186e421cdb16c7002e3de3525e76a87f1d5cda6ff0e3e177b1)
#[derive(Clone, Default)]
pub struct HostDeviceCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub info: Option<HostDeviceCapability>,
}
impl PartialEq for HostDeviceCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:338 (sha256:171b1216fa9f797debf2aed1f2639dc97be20259838c515e5f8455b6020cdcb1)
#[derive(Clone, Default)]
pub struct HostDialogCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub directory_open: Option<HostDirectoryOpenDialogCapability>,
    pub file_open: Option<HostFileOpenDialogCapability>,
    pub file_save: Option<HostFileSaveDialogCapability>,
    pub image_open: Option<HostImageOpenDialogCapability>,
    pub message: Option<HostMessageDialogCapability>,
    pub photo_capture: Option<HostPhotoCaptureDialogCapability>,
    pub prompt: Option<HostPromptDialogCapability>,
    pub video_capture: Option<HostVideoCaptureDialogCapability>,
}
impl PartialEq for HostDialogCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:349 (sha256:dfa60fb9610ff1e876cc9d26fd691fe30d1caf3010b863a146b942f96c4e4bf3)
#[derive(Clone, Default)]
pub struct HostFileSystemCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub access: Option<HostFileSystemCapability>,
}
impl PartialEq for HostFileSystemCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:353 (sha256:d703896d367116c6eae8892935a1745980078f01a4e32b4f03538476a2c9028c)
#[derive(Clone, Default)]
pub struct HostFontCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loader: Option<HostFontLoadingCapability>,
}
impl PartialEq for HostFontCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:357 (sha256:426ecbd4c53a4360b0b523beb2b2f71e0741a8d71b96b502e4875206af200029)
#[derive(Clone, Default)]
pub struct HostFullscreenCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub element: Option<HostElementFullscreenCapability>,
}
impl PartialEq for HostFullscreenCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:361 (sha256:edc5ff9dc844bf8b5cc74c5dc957392cd228906752630e6aad87959168d70be2)
#[derive(Clone, Default)]
pub struct HostGeolocationCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub position: Option<HostGeolocationCapability>,
}
impl PartialEq for HostGeolocationCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:365 (sha256:4f5e632cd038141c68e478c5b71fb0e740e1fe087d37c48e5de4a4e613d470b6)
#[derive(Clone, Default)]
pub struct HostCanvasCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub context: Option<HostCanvasCapability>,
}
impl PartialEq for HostCanvasCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:369 (sha256:18a8e53d069a15f80ac8cc6534eb074774dd716e4b08e81c47c9572d4ea8e69b)
#[derive(Clone, Default)]
pub struct HostGlCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub context: Option<HostGlCapability>,
}
impl PartialEq for HostGlCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:373 (sha256:1ae78116afadfcd245c0a9cecf02fda3626d136f6cd6018cfa762e43eac007bf)
#[derive(Clone, Default)]
pub struct HostGlyphCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub rasterizer: Option<HostGlyphRasterizerCapability>,
}
impl PartialEq for HostGlyphCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:377 (sha256:bfa6dc75dc9393906ba6bbcc8cb60e6cec07fdb2835259da0ec0882f2fb6da40)
#[derive(Clone, Default)]
pub struct HostHapticsCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub engine: Option<HostHapticsCapability>,
}
impl PartialEq for HostHapticsCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:381 (sha256:0906b4fc0d2a65147340198575d6e430ffc50fc31148e98e5a96b88f2ddbf59a)
#[derive(Clone, Default)]
pub struct HostImageCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loader: Option<HostImageCapability>,
}
impl PartialEq for HostImageCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:385 (sha256:c4f7cb8cd0d6f596d78275c336ed3717fdc0a14c240c2948e60a1e88f72b85ae)
#[derive(Clone, Default)]
pub struct HostInputCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub drop_file: Option<HostInputDropFileCapability>,
    pub focus: Option<HostInputFocusCapability>,
    pub ingress: Option<HostInputIngressCapability>,
    pub pointer_lock: Option<HostInputPointerLockCapability>,
    pub target: Option<HostInputTargetCapability>,
}
impl PartialEq for HostInputCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:393 (sha256:6c54c1c586297bbb832e8194e96b84280cc60563145ff350e308fdc7f28cdca6)
#[derive(Clone, Default)]
pub struct HostIpcCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handle: Option<HostIpcHandleCapability>,
    pub invoke: Option<HostIpcInvokeCapability>,
    pub message: Option<HostIpcMessageCapability>,
    pub send: Option<HostIpcSendCapability>,
    pub targeted_send: Option<HostIpcTargetedSendCapability<crate::OpaqueHostValue>>,
}
impl PartialEq for HostIpcCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:401 (sha256:f489ad2bd80ef95ba625ed90390aab1496005211418b048499570161034ac74b)
#[derive(Clone, Default)]
pub struct HostLifecycleCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub state: Option<HostLifecycleCapability>,
}
impl PartialEq for HostLifecycleCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:405 (sha256:be40747d6f6e4f24af7e8fd8860bd92fd53d05c1a3a339849f8cbab6b21d28d8)
#[derive(Clone, Default)]
pub struct HostMediaSessionCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub action: Option<HostMediaSessionActionCapability>,
    pub control: Option<HostMediaSessionCapability>,
}
impl PartialEq for HostMediaSessionCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:410 (sha256:fd55ac2b6eab417a6bf630c564dadf7702ca228e2514f1da8090733448bda32c)
#[derive(Clone, Default)]
pub struct HostMenuCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub app: Option<HostAppMenuCapability>,
    pub highlight: Option<HostMenuHighlightCapability>,
    pub popup: Option<HostMenuPopupCapability>,
    pub select: Option<HostMenuSelectCapability>,
}
impl PartialEq for HostMenuCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:417 (sha256:bf7f919969609b891c8317d89f4e5f4f1d80a5e1d62f7c9aa28577c9505baa12)
#[derive(Clone, Default)]
pub struct HostMidiCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub access: Option<HostMidiAccessCapability>,
    pub permission: Option<HostMidiPermissionCapability>,
}
impl PartialEq for HostMidiCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:422 (sha256:da738175230cb79f9cf02fcddc07de936d729e7ae5ca4fca2849e7c47a2bb145)
#[derive(Clone, Default)]
pub struct HostNetCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub http: Option<HostNetCapability>,
}
impl PartialEq for HostNetCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:426 (sha256:8c0221a03f780ebc94305980880edc21b54566228202b6b97e1e1b5b92a5d74e)
#[derive(Clone, Default)]
pub struct HostNotificationCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub action: Option<HostNotificationActionCapability>,
    pub active_list: Option<HostNotificationActiveListCapability>,
    pub click: Option<HostNotificationClickCapability>,
    pub close: Option<HostNotificationCloseCapability>,
    pub delivery: Option<HostNotificationDeliveryCapability>,
    pub dismiss: Option<HostNotificationDismissCapability>,
    pub lifecycle: Option<HostNotificationLifecycleCapability>,
    pub permission: Option<HostNotificationPermissionCapability>,
    pub received: Option<HostNotificationReceivedCapability>,
    pub reply: Option<HostNotificationReplyCapability>,
    pub scheduling: Option<HostNotificationSchedulingCapability>,
}
impl PartialEq for HostNotificationCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:440 (sha256:71b58a6a6387e339a53a6ad7e8ce41921bb7b88a4a7ed7ef34d9edfef0d9f4d0)
#[derive(Clone, Default)]
pub struct HostPermissionsCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub query: Option<HostPermissionsCapability>,
}
impl PartialEq for HostPermissionsCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:444 (sha256:41e5f80c4b043ec2a296d25df568ea4247a015f24f1e7c6ee4c6ad6dc0645ad2)
#[derive(Clone, Default)]
pub struct HostPlatformCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub info: Option<HostPlatformCapability>,
}
impl PartialEq for HostPlatformCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:448 (sha256:f289fbd3b15b5bca405bdc43f70969b516ed6bad108e4f7be74dc33a15449573)
#[derive(Clone, Default)]
pub struct HostPowerCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub battery_health: Option<HostPowerBatteryHealthCapability>,
    pub change: Option<HostPowerChangeCapability>,
    pub idle: Option<HostPowerIdleCapability>,
    pub keep_awake: Option<HostPowerKeepAwakeCapability>,
    pub session_lock: Option<HostPowerSessionLockCapability>,
    pub status: Option<HostPowerStatusCapability>,
    pub suspension: Option<HostPowerSuspensionCapability>,
    pub thermal: Option<HostPowerThermalCapability>,
}
impl PartialEq for HostPowerCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:459 (sha256:cd0900b0ee19e3f4368be2cf1ce1d2bad2eeb25c436412dc0ea249639ac6eaa9)
#[derive(Clone, Default)]
pub struct HostPreferencesCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub change: Option<HostPreferencesChangeCapability>,
    pub local: Option<HostPreferencesCapability>,
    pub persistence_query: Option<HostPreferencesPersistenceQueryCapability>,
    pub persistence_request: Option<HostPreferencesPersistenceRequestCapability>,
}
impl PartialEq for HostPreferencesCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:466 (sha256:dd82850217efab16b8cc1ba104d34d5f7c61a1476e3ceff9faf40b9b91cddf77)
#[derive(Clone, Default)]
pub struct HostProtocolCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub default: Option<HostProtocolDefaultCapability>,
    pub launch: Option<HostProtocolLaunchCapability>,
    pub open: Option<HostProtocolOpenCapability>,
    pub registration: Option<HostProtocolRegistrationCapability>,
    pub registration_query: Option<HostProtocolRegistrationQueryCapability>,
    pub unregistration: Option<HostProtocolUnregistrationCapability>,
}
impl PartialEq for HostProtocolCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:475 (sha256:4acc0290133799ef7f1c36d93740c8d1d8008b1b4a6d9ca10a4ed4d52208d0a7)
#[derive(Clone, Default)]
pub struct HostScreenCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub change: Option<HostScreenChangeCapability>,
    pub details: Option<HostScreenDetailsCapability>,
    pub permission_change: Option<HostScreenPermissionChangeCapability>,
    pub query: Option<HostScreenQueryCapability>,
}
impl PartialEq for HostScreenCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:482 (sha256:46e1c9a1c5cef24b197874a6f8c65560cc1619ec084fd52c49f0d3d0221864b7)
pub type WebScreenCapabilities = HostScreenCapabilities;

// Source: upstream/packages/types/src/Host.ts:484 (sha256:6be2d224116b996567d924918979f94912bc41e0d74df3096af583678f978a3a)
#[derive(Clone, Default)]
pub struct HostSensorsCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub query: Option<HostSensorsCapability>,
}
impl PartialEq for HostSensorsCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:488 (sha256:8404b75874fb9d0fea42d04ec6814c0355eefb32a7b0bb031f5283c5cce080d8)
#[derive(Clone, Default)]
pub struct HostShareCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub content: Option<HostShareContentCapability>,
    pub files: Option<HostShareFilesCapability>,
}
impl PartialEq for HostShareCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:493 (sha256:50a4f5ce6f8c92732350d28ac457016f89c1f0e37132e9c1f9cb539908920499)
#[derive(Clone, Default)]
pub struct HostShellCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub beep: Option<HostShellBeepCapability>,
    pub external: Option<HostShellExternalCapability>,
    pub path_open: Option<HostShellPathOpenCapability>,
    pub path_reveal: Option<HostShellPathRevealCapability>,
    pub process: Option<HostShellProcessCapability>,
    pub shortcut_link: Option<HostShellShortcutLinkCapability>,
    pub trash: Option<HostShellTrashCapability>,
}
impl PartialEq for HostShellCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:503 (sha256:d4e581e77a44aceae74a603b40ee43407a0ebcaa960cecc817bc144fa556022e)
#[derive(Clone, Default)]
pub struct HostShortcutCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub query: Option<HostShortcutQueryCapability>,
    pub trigger: Option<HostShortcutTriggerCapability>,
}
impl PartialEq for HostShortcutCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:508 (sha256:c309c7e4d9a62ce8f92bd57e7f2aea2639a364ea0addee45b29d04e22fe4be42)
#[derive(Clone, Default)]
pub struct HostSocketCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub connection: Option<HostSocketCapability>,
}
impl PartialEq for HostSocketCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:512 (sha256:0a3773a7358b3fe58080f7ca49b1f40299c2d19e9381e5f74da713179711b4e9)
#[derive(Clone, Default)]
pub struct HostSoftKeyboardCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub accessory_bar: Option<HostSoftKeyboardAccessoryBarCapability>,
    pub change: Option<HostSoftKeyboardChangeCapability>,
    pub info: Option<HostSoftKeyboardInfoCapability>,
    pub resize_mode_write: Option<HostSoftKeyboardResizeModeWriteCapability>,
    pub scroll_assist: Option<HostSoftKeyboardScrollAssistCapability>,
    pub style: Option<HostSoftKeyboardStyleCapability>,
    pub visibility: Option<HostSoftKeyboardVisibilityCapability>,
}
impl PartialEq for HostSoftKeyboardCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:522 (sha256:92a18da4f1771d511f6ae1e336b9f9f41081d269f3b39b2ebdeb9e0c9c62081e)
#[derive(Clone, Default)]
pub struct HostStatusBarCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub change: Option<HostStatusBarChangeCapability>,
    pub color: Option<HostStatusBarColorCapability>,
    pub info: Option<HostStatusBarInfoCapability>,
    pub overlays: Option<HostStatusBarOverlaysCapability>,
    pub style: Option<HostStatusBarStyleCapability>,
    pub visibility: Option<HostStatusBarVisibilityCapability>,
}
impl PartialEq for HostStatusBarCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:531 (sha256:2b18a6404795abdfa25d16782bb7da4aaeb3b8a66b9e87548382b6b36315f9a6)
#[derive(Clone, Default)]
pub struct HostSurfaceCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub display: Option<HostSurfaceDisplayCapability>,
    pub resize: Option<HostSurfaceResizeCapability>,
}
impl PartialEq for HostSurfaceCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:536 (sha256:72d4da61ce219a1ec3e0f26ce1e8ec0e4c53ec69c5d6a0b26518e642a1b872cc)
#[derive(Clone, Default)]
pub struct HostTextSegmentCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub segmenter: Option<HostTextSegmenterCapability>,
}
impl PartialEq for HostTextSegmentCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:544 (sha256:246a6087e16010797c17631b80d65c3c3981b2c23fd42573e983e78e37e16ba3)
#[derive(Clone, Default)]
pub struct HostDecompressCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub brotli: Option<HostDecompressBrotliCapability>,
    pub deflate: Option<HostDecompressDeflateCapability>,
    pub lzma: Option<HostDecompressLzmaCapability>,
}
impl PartialEq for HostDecompressCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:550 (sha256:f0d9448ba51f95402629ef96a886275db885cf6a44019135116ad917915e98b3)
#[derive(Clone, Default)]
pub struct HostTextShaperCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub shaper: Option<HostTextShaperCapability>,
}
impl PartialEq for HostTextShaperCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:554 (sha256:ac47fd17eede0166010746888971612ee65a322c72587fbfe6e8f31d836316ab)
#[derive(Clone, Default)]
pub struct HostTrayCapabilities {
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
impl PartialEq for HostTrayCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:572 (sha256:eb48ee1b45f35a60d066b9c488ad2ea5705969ac7c63ab5183e3d8774a5f0ef7)
#[derive(Clone, Default)]
pub struct HostUpdaterCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub command: Option<HostUpdaterCommandCapability>,
}
impl PartialEq for HostUpdaterCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:576 (sha256:d7b0165d3dc67fc73b584837559cff95c1c3601cb463355b19d6a0ce9b253e58)
#[derive(Clone, Default)]
pub struct HostVideoCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub playback: Option<HostVideoCapability>,
}
impl PartialEq for HostVideoCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:580 (sha256:15f0d0d58f606439dee299f928b1efa8e9145d16eeffaa14487ae0281ecc5af4)
#[derive(Clone, Default)]
pub struct HostWgpuCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub context: Option<HostWgpuCapability>,
}
impl PartialEq for HostWgpuCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Host.ts:584 (sha256:dc7b01368e412bd26ae5c26025d30f1e4e6cbd3988635d81371fcc24fc904603)
#[derive(Clone, Default)]
pub struct HostWindowCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub appearance: Option<HostWindowAppearanceCapability>,
    pub attach: Option<HostWindowAttachCapability>,
    pub attention: Option<HostWindowAttentionCapability>,
    pub content_protection: Option<HostWindowContentProtectionCapability>,
    pub focus: Option<HostWindowFocusCapability>,
    pub fullscreen: Option<HostWindowFullscreenCapability>,
    pub geometry: Option<HostWindowGeometryCapability>,
    pub hierarchy: Option<HostWindowHierarchyCapability>,
    pub lifecycle: Option<HostWindowLifecycleCapability>,
    pub progress: Option<HostWindowProgressCapability>,
    pub shadow: Option<HostWindowShadowCapability>,
    pub shell: Option<HostWindowShellCapability>,
    pub size_constraints: Option<HostWindowSizeConstraintsCapability>,
    pub state: Option<HostWindowStateCapability>,
    pub visibility: Option<HostWindowVisibilityCapability>,
    pub z_order: Option<HostWindowZOrderCapability>,
}
impl PartialEq for HostWindowCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
