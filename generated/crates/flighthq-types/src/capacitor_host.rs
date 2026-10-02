// @generated from upstream/packages/types/src/CapacitorHost.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    CapacitorAppCapabilitiesFor, CapacitorNotificationCapabilities, EntityRuntime,
    HostAccessibilityCapabilities, HostAudioCapabilities, HostAudioDecodeCapabilities,
    HostBitmapCapabilities, HostCanvasCapabilities, HostCapacitorShareContentCapability,
    HostClipboardBookmarkCapability, HostClipboardChangeCapability, HostClipboardFormatsCapability,
    HostClipboardImageCapability, HostClipboardTextCapability, HostCompressCapabilities,
    HostConnectivityChangeCapability, HostConnectivityReachabilityCapability,
    HostConnectivityStatusCapability, HostDecompressCapabilities, HostDeviceCapabilities,
    HostDirectoryOpenDialogCapability, HostFileOpenDialogCapability, HostFileSaveDialogCapability,
    HostFileSystemCapabilities, HostFontCapabilities, HostFullscreenCapabilities,
    HostGeolocationCapabilities, HostGlCapabilities, HostGlyphCapabilities,
    HostHapticsCapabilities, HostImageCapabilities, HostImageDecodeCapabilities,
    HostImageEncodeCapabilities, HostImageOpenDialogCapability, HostInputCapabilities,
    HostIpcCapabilities, HostLifecycleCapabilities, HostMediaSessionCapabilities,
    HostMenuCapabilities, HostMessageDialogCapability, HostMidiCapabilities, HostNetCapabilities,
    HostPermissionsCapabilities, HostPhotoCaptureDialogCapability, HostPlatformCapabilities,
    HostPowerCapabilities, HostPreferencesCapabilities, HostPromptDialogCapability,
    HostProtocolCapabilities, HostScreenCapabilities, HostSensorsCapabilities,
    HostShareFilesCapability, HostShellCapabilities, HostShortcutCapabilities,
    HostSocketCapabilities, HostSoftKeyboardCapabilities, HostStatusBarCapabilities,
    HostSurfaceCapabilities, HostTextSegmentCapabilities, HostTextShaperCapabilities,
    HostTrayCapabilities, HostUpdaterCapabilities, HostVideoCapabilities,
    HostVideoCaptureDialogCapability, HostWgpuCapabilities, HostWindowCapabilities,
};

#[derive(Clone)]
pub struct CapacitorHostRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub content: HostCapacitorShareContentCapability,
    pub files: Option<HostShareFilesCapability>,
}
impl PartialEq for CapacitorHostRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CapacitorHostRecord2 {
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
impl PartialEq for CapacitorHostRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CapacitorHostRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub change: Option<HostConnectivityChangeCapability>,
    pub reachability: Option<HostConnectivityReachabilityCapability>,
    pub status: Option<HostConnectivityStatusCapability>,
}
impl PartialEq for CapacitorHostRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CapacitorHostRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub bookmark: Option<HostClipboardBookmarkCapability>,
    pub change: Option<HostClipboardChangeCapability>,
    pub formats: Option<HostClipboardFormatsCapability>,
    pub image: Option<HostClipboardImageCapability>,
    pub text: Option<HostClipboardTextCapability>,
}
impl PartialEq for CapacitorHostRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/CapacitorHost.ts:20 (sha256:b7ba7624a46fc39a3d2eab0be5f61d6a5c1a2c8cee03072a006b533a45b060cb)
#[derive(Clone)]
pub struct CapacitorHost<Profile> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub accessibility: HostAccessibilityCapabilities,
    pub app: CapacitorAppCapabilitiesFor<Profile>,
    pub audio: HostAudioCapabilities,
    pub audio_decode: HostAudioDecodeCapabilities,
    pub bitmap: HostBitmapCapabilities,
    pub canvas: HostCanvasCapabilities,
    pub clipboard: CapacitorHostRecord4,
    pub compress: HostCompressCapabilities,
    pub connectivity: CapacitorHostRecord3,
    pub decompress: HostDecompressCapabilities,
    pub device: HostDeviceCapabilities,
    pub dialog: CapacitorHostRecord2,
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
    pub notification: CapacitorNotificationCapabilities,
    pub permissions: HostPermissionsCapabilities,
    pub platform: HostPlatformCapabilities,
    pub power: HostPowerCapabilities,
    pub preferences: HostPreferencesCapabilities,
    pub protocol: HostProtocolCapabilities,
    pub screen: HostScreenCapabilities,
    pub sensors: HostSensorsCapabilities,
    pub share: CapacitorHostRecord1,
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
impl<Profile> PartialEq for CapacitorHost<Profile> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl<Profile: Clone + Send + Sync + 'static> crate::FlightEntity for CapacitorHost<Profile> {
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
