// @generated from upstream/packages/types/src/TauriHost.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    EntityRuntime, HostAccessibilityCapabilities, HostAppCapabilities, HostAudioCapabilities,
    HostAudioDecodeCapabilities, HostBitmapCapabilities, HostCanvasCapabilities,
    HostClipboardCapabilities, HostCompressCapabilities, HostConnectivityCapabilities,
    HostDecompressCapabilities, HostDeviceCapabilities, HostDialogCapabilities,
    HostFileSystemCapabilities, HostFontCapabilities, HostFullscreenCapabilities,
    HostGeolocationCapabilities, HostGlCapabilities, HostGlyphCapabilities,
    HostHapticsCapabilities, HostImageCapabilities, HostImageDecodeCapabilities,
    HostImageEncodeCapabilities, HostInputCapabilities, HostIpcCapabilities,
    HostLifecycleCapabilities, HostMediaSessionCapabilities, HostMenuCapabilities,
    HostMidiCapabilities, HostNetCapabilities, HostPermissionsCapabilities,
    HostPlatformCapabilities, HostPowerCapabilities, HostPreferencesCapabilities,
    HostProtocolCapabilities, HostScreenCapabilities, HostSensorsCapabilities,
    HostShareCapabilities, HostShellCapabilities, HostShortcutCapabilities, HostSocketCapabilities,
    HostSoftKeyboardCapabilities, HostStatusBarCapabilities, HostSurfaceCapabilities,
    HostTextSegmentCapabilities, HostTextShaperCapabilities, HostUpdaterCapabilities,
    HostVideoCapabilities, HostWgpuCapabilities, HostWindowCapabilities,
    TauriNotificationCapabilities, TauriTrayCapabilitiesFor,
};

// Source: upstream/packages/types/src/TauriHost.ts:16 (sha256:f39c9361e0045a64d11be44b538222af0496da5b2a17c61c1ec54d04d611da34)
#[derive(Clone)]
pub struct TauriHost<Profile> {
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
    pub notification: TauriNotificationCapabilities,
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
    pub tray: TauriTrayCapabilitiesFor<Profile>,
    pub updater: HostUpdaterCapabilities,
    pub video: HostVideoCapabilities,
    pub wgpu: HostWgpuCapabilities,
    pub window: HostWindowCapabilities,
}
impl<Profile> PartialEq for TauriHost<Profile> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl<Profile: Clone + Send + Sync + 'static> crate::FlightEntity for TauriHost<Profile> {
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
