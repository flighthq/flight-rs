// @generated from upstream/packages/types/src/HostImageEncode.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{DecodedImage, ImageEncodeOptions};

// Source: upstream/packages/types/src/HostImageEncode.ts:4 (sha256:619ce8a184bfd468c98cf60ba27c1c420dccb4c6a409418d2457138c33a6964b)
#[derive(Clone)]
pub struct HostImageEncodeFormatCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub encode: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(DecodedImage, Option<ImageEncodeOptions>) -> crate::FlightTask<Vec<u8>>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostImageEncodeFormatCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostImageEncode.ts:8 (sha256:ccfcc5a6ea6bc3563db9d596b85dab091669b528cb9ca9437d0879e9fb1129f0)
#[derive(Clone, Default)]
pub struct HostImageEncodeCapabilities {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub jpeg: Option<HostImageEncodeFormatCapability>,
    pub png: Option<HostImageEncodeFormatCapability>,
    pub webp: Option<HostImageEncodeFormatCapability>,
}
impl PartialEq for HostImageEncodeCapabilities {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
