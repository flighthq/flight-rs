// @generated from upstream/packages/types/src/HostAudioDecode.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/HostAudioDecode.ts:12 (sha256:74d1fbcb85b52034799b5cd0276ba50f22cac4ce09d043ecabcf9e9f4107c654)
#[derive(Clone)]
pub struct HostAudioDecodeFormatCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub decode: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Vec<u8>,
                        crate::OpaqueHostValue,
                    ) -> crate::FlightTask<Option<crate::OpaqueHostValue>>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostAudioDecodeFormatCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostAudioDecode.ts:25 (sha256:a5e8985bce3bd273b819e2eef5895301e42862a2f7277d9cc295408bda4048f2)
#[derive(Clone, Default)]
pub struct HostAudioDecodeCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub aac: Option<HostAudioDecodeFormatCapability>,
    pub flac: Option<HostAudioDecodeFormatCapability>,
    pub mp3: Option<HostAudioDecodeFormatCapability>,
    pub mp4: Option<HostAudioDecodeFormatCapability>,
    pub ogg: Option<HostAudioDecodeFormatCapability>,
    pub wav: Option<HostAudioDecodeFormatCapability>,
    pub webm: Option<HostAudioDecodeFormatCapability>,
}
impl PartialEq for HostAudioDecodeCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
