// @generated from upstream/packages/types/src/HostAudioCodec.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/HostAudioCodec.ts:1 (sha256:01da42ca86ade588a1145e45186753e68cebd1fc3e7315b5bc83e2621e1c0273)
#[derive(Clone)]
pub struct HostAudioCodecCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub can_play_type:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
}
impl PartialEq for HostAudioCodecCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
