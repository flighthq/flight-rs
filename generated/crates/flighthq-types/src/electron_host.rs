// @generated from upstream/packages/types/src/ElectronHost.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    ElectronAppCapabilitiesFor, ElectronIpcTarget, ElectronMenuCapabilities,
    ElectronPowerCapabilities, ElectronProtocolCapabilities, ElectronTrayCapabilitiesFor,
    HostAccessibilityCapabilities, HostAudioCapabilities, HostAudioDecodeCapabilities,
    HostBitmapCapabilities, HostCanvasCapabilities, HostClipboardCapabilities,
    HostCompressCapabilities, HostConnectivityCapabilities, HostDecompressCapabilities,
    HostDeviceCapabilities, HostDialogCapabilities, HostFileSystemCapabilities,
    HostFontCapabilities, HostFullscreenCapabilities, HostGeolocationCapabilities,
    HostGlCapabilities, HostGlyphCapabilities, HostHapticsCapabilities, HostImageCapabilities,
    HostImageDecodeCapabilities, HostImageEncodeCapabilities, HostInputCapabilities,
    HostIpcHandleCapability, HostIpcInvokeCapability, HostIpcMessageCapability,
    HostIpcSendCapability, HostIpcTargetedSendCapability, HostLifecycleCapabilities,
    HostMediaSessionCapabilities, HostMidiCapabilities, HostNetCapabilities,
    HostPermissionsCapabilities, HostPlatformCapabilities, HostPreferencesCapabilities,
    HostScreenCapabilities, HostSensorsCapabilities, HostShareCapabilities, HostShellCapabilities,
    HostShortcutCapabilities, HostSocketCapabilities, HostSoftKeyboardCapabilities,
    HostStatusBarCapabilities, HostSurfaceCapabilities, HostTextSegmentCapabilities,
    HostTextShaperCapabilities, HostUpdaterCapabilities, HostVideoCapabilities,
    HostWgpuCapabilities, HostWindowCapabilities,
};

#[derive(Clone, Default)]
pub struct FlightOmitRecord2344156220 {
    pub __flight_identity: std::sync::Arc<()>,
    pub accessibility: HostAccessibilityCapabilities,
    pub audio: HostAudioCapabilities,
    pub audio_decode: HostAudioDecodeCapabilities,
    pub bitmap: HostBitmapCapabilities,
    pub canvas: HostCanvasCapabilities,
    pub compress: HostCompressCapabilities,
    pub connectivity: HostConnectivityCapabilities,
    pub decompress: HostDecompressCapabilities,
    pub device: HostDeviceCapabilities,
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
    pub lifecycle: HostLifecycleCapabilities,
    pub media_session: HostMediaSessionCapabilities,
    pub midi: HostMidiCapabilities,
    pub net: HostNetCapabilities,
    pub permissions: HostPermissionsCapabilities,
    pub sensors: HostSensorsCapabilities,
    pub share: HostShareCapabilities,
    pub socket: HostSocketCapabilities,
    pub soft_keyboard: HostSoftKeyboardCapabilities,
    pub status_bar: HostStatusBarCapabilities,
    pub surface: HostSurfaceCapabilities,
    pub text_segment: HostTextSegmentCapabilities,
    pub text_shaper: HostTextShaperCapabilities,
    pub video: HostVideoCapabilities,
    pub wgpu: HostWgpuCapabilities,
}
impl PartialEq for FlightOmitRecord2344156220 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct ElectronHostRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub handle: Option<HostIpcHandleCapability>,
    pub invoke: Option<HostIpcInvokeCapability>,
    pub message: Option<HostIpcMessageCapability>,
    pub send: Option<HostIpcSendCapability>,
    pub targeted_send: HostIpcTargetedSendCapability<ElectronIpcTarget>,
}
impl PartialEq for ElectronHostRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronHost.ts:24 (sha256:561f26057d6622495af0a4dc45760f631abd81c8dfaff4c8449bf94a49145230)
pub struct ElectronNotificationCapabilitiesFor<Profile>(
    pub crate::OpaqueHostValue,
    pub core::marker::PhantomData<fn() -> (Profile,)>,
);
impl<Profile> Clone for ElectronNotificationCapabilitiesFor<Profile> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), core::marker::PhantomData)
    }
}

// Source: upstream/packages/types/src/ElectronHost.ts:28 (sha256:4977ff5ac7355974ce3c83e21ec14cfefbf8a69af27b710a41d1504e95b9401a)
pub struct ElectronShellCapabilitiesFor<Profile>(
    pub HostShellCapabilities,
    pub core::marker::PhantomData<fn() -> (Profile,)>,
);
impl<Profile> Clone for ElectronShellCapabilitiesFor<Profile> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), core::marker::PhantomData)
    }
}

// Source: upstream/packages/types/src/ElectronHost.ts:33 (sha256:a22c2f71188a9a6406cb1e131628e63795d03fee6e281ebcc2a456b21dc0e790)
#[derive(Clone)]
pub struct ElectronHost<Profile> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub accessibility: HostAccessibilityCapabilities,
    pub audio: HostAudioCapabilities,
    pub audio_decode: HostAudioDecodeCapabilities,
    pub bitmap: HostBitmapCapabilities,
    pub canvas: HostCanvasCapabilities,
    pub compress: HostCompressCapabilities,
    pub connectivity: HostConnectivityCapabilities,
    pub decompress: HostDecompressCapabilities,
    pub device: HostDeviceCapabilities,
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
    pub lifecycle: HostLifecycleCapabilities,
    pub media_session: HostMediaSessionCapabilities,
    pub midi: HostMidiCapabilities,
    pub net: HostNetCapabilities,
    pub permissions: HostPermissionsCapabilities,
    pub sensors: HostSensorsCapabilities,
    pub share: HostShareCapabilities,
    pub socket: HostSocketCapabilities,
    pub soft_keyboard: HostSoftKeyboardCapabilities,
    pub status_bar: HostStatusBarCapabilities,
    pub surface: HostSurfaceCapabilities,
    pub text_segment: HostTextSegmentCapabilities,
    pub text_shaper: HostTextShaperCapabilities,
    pub video: HostVideoCapabilities,
    pub wgpu: HostWgpuCapabilities,
    pub app: ElectronAppCapabilitiesFor<Profile>,
    pub clipboard: HostClipboardCapabilities,
    pub dialog: HostDialogCapabilities,
    pub ipc: ElectronHostRecord1,
    pub menu: ElectronMenuCapabilities,
    pub notification: ElectronNotificationCapabilitiesFor<Profile>,
    pub platform: HostPlatformCapabilities,
    pub power: ElectronPowerCapabilities,
    pub preferences: HostPreferencesCapabilities,
    pub protocol: ElectronProtocolCapabilities,
    pub screen: HostScreenCapabilities,
    pub shell: ElectronShellCapabilitiesFor<Profile>,
    pub shortcut: HostShortcutCapabilities,
    pub tray: ElectronTrayCapabilitiesFor<Profile>,
    pub updater: HostUpdaterCapabilities,
    pub window: HostWindowCapabilities,
}
impl<Profile> PartialEq for ElectronHost<Profile> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ElectronHost.ts:74 (sha256:cc7989554736490dff1b4201a60c97d4fd9c84c6a1257ba40c285472122d22a0)
pub type ElectronMacosHost = ElectronHost<String>;
