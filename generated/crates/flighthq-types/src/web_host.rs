// @generated from upstream/packages/types/src/WebHost.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    HostAccessibilityCapabilities, HostAppCapabilities, HostAudioCapabilities,
    HostAudioDecodeCapabilities, HostBitmapCapabilities, HostCanvasCapabilities,
    HostClipboardCapabilities, HostCompressCapabilities, HostConnectivityCapabilities,
    HostDecompressCapabilities, HostDeviceCapabilities, HostDialogCapabilities,
    HostFileSystemCapabilities, HostFontCapabilities, HostFullscreenCapabilities,
    HostGeolocationCapabilities, HostGlCapabilities, HostGlyphCapabilities,
    HostHapticsCapabilities, HostImageCapabilities, HostImageDecodeCapabilities,
    HostImageEncodeCapabilities, HostInputCapabilities, HostIpcCapabilities,
    HostLifecycleCapabilities, HostMediaSessionCapabilities, HostMenuCapabilities,
    HostMidiCapabilities, HostNetCapabilities, HostNotificationCapabilities,
    HostPermissionsCapabilities, HostPlatformCapabilities, HostPowerCapabilities,
    HostPreferencesCapabilities, HostProtocolCapabilities, HostScreenCapabilities,
    HostSensorsCapabilities, HostShareCapabilities, HostShellCapabilities,
    HostShortcutCapabilities, HostSocketCapabilities, HostSoftKeyboardCapabilities,
    HostStatusBarCapabilities, HostSurfaceCapabilities, HostTextSegmentCapabilities,
    HostTextShaperCapabilities, HostTrayCapabilities, HostUpdaterCapabilities,
    HostVideoCapabilities, HostWgpuCapabilities, HostWindowCapabilities,
};

#[derive(Clone, Default)]
pub struct FlightOmitRecord3541074404 {
    pub __flight_identity: std::sync::Arc<()>,
    pub audio_decode: HostAudioDecodeCapabilities,
    pub compress: HostCompressCapabilities,
    pub decompress: HostDecompressCapabilities,
    pub image_decode: HostImageDecodeCapabilities,
    pub image_encode: HostImageEncodeCapabilities,
    pub ipc: HostIpcCapabilities,
    pub midi: HostMidiCapabilities,
    pub shortcut: HostShortcutCapabilities,
    pub text_segment: HostTextSegmentCapabilities,
    pub text_shaper: HostTextShaperCapabilities,
    pub tray: HostTrayCapabilities,
    pub updater: HostUpdaterCapabilities,
    pub wgpu: HostWgpuCapabilities,
}
impl PartialEq for FlightOmitRecord3541074404 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WebHost.ts:43 (sha256:7b3fa8208494a5adbb281378617b0961d3389f471e0f2283706e5bee2a8212c5)
#[derive(Clone, Default)]
pub struct WebHost {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub audio_decode: HostAudioDecodeCapabilities,
    pub compress: HostCompressCapabilities,
    pub decompress: HostDecompressCapabilities,
    pub image_decode: HostImageDecodeCapabilities,
    pub image_encode: HostImageEncodeCapabilities,
    pub ipc: HostIpcCapabilities,
    pub midi: HostMidiCapabilities,
    pub shortcut: HostShortcutCapabilities,
    pub text_segment: HostTextSegmentCapabilities,
    pub text_shaper: HostTextShaperCapabilities,
    pub tray: HostTrayCapabilities,
    pub updater: HostUpdaterCapabilities,
    pub wgpu: HostWgpuCapabilities,
    pub accessibility: HostAccessibilityCapabilities,
    pub app: HostAppCapabilities,
    pub audio: HostAudioCapabilities,
    pub bitmap: HostBitmapCapabilities,
    pub canvas: HostCanvasCapabilities,
    pub clipboard: HostClipboardCapabilities,
    pub connectivity: HostConnectivityCapabilities,
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
    pub input: HostInputCapabilities,
    pub lifecycle: HostLifecycleCapabilities,
    pub media_session: HostMediaSessionCapabilities,
    pub menu: HostMenuCapabilities,
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
    pub socket: HostSocketCapabilities,
    pub soft_keyboard: HostSoftKeyboardCapabilities,
    pub status_bar: HostStatusBarCapabilities,
    pub surface: HostSurfaceCapabilities,
    pub video: HostVideoCapabilities,
    pub window: HostWindowCapabilities,
}
impl PartialEq for WebHost {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
