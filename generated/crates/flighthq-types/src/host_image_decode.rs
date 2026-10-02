// @generated from upstream/packages/types/src/HostImageDecode.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{DecodedImage, ImageDecodeOptions};

// Source: upstream/packages/types/src/HostImageDecode.ts:4 (sha256:5f2e3f5853038006341ce668ed3dfa214eadbfb2d5578be92c4e77033601977a)
#[derive(Clone)]
pub struct HostImageDecodeFormatCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub decode: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Vec<u8>, Option<ImageDecodeOptions>) -> crate::FlightTask<DecodedImage>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostImageDecodeFormatCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostImageDecode.ts:8 (sha256:552cc23ab475820ca98732c2dc0e8581de085bbc2c02105ce74149f7ce21e28a)
#[derive(Clone, Default)]
pub struct HostImageDecodeCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub avif: Option<HostImageDecodeFormatCapability>,
    pub bmp: Option<HostImageDecodeFormatCapability>,
    pub gif: Option<HostImageDecodeFormatCapability>,
    pub jpeg: Option<HostImageDecodeFormatCapability>,
    pub png: Option<HostImageDecodeFormatCapability>,
    pub webp: Option<HostImageDecodeFormatCapability>,
}
impl PartialEq for HostImageDecodeCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
