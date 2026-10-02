// @generated from upstream/packages/types/src/HostBitmapEncode.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Bitmap, ImageFormat};

// Source: upstream/packages/types/src/HostBitmapEncode.ts:6 (sha256:3564b28b93ae56afefa2d92314a59eac324ab1327122b02b08433270119f4130)
#[derive(Clone)]
pub struct HostBitmapEncodeCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub encode_bitmap: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Bitmap, ImageFormat, f64) -> Vec<u8> + Send + 'static>>,
    >,
    pub supported_formats: Vec<ImageFormat>,
}
impl PartialEq for HostBitmapEncodeCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/HostBitmapEncode.ts:11 (sha256:9414bc5b2aa82167694e8239fafd299460d229cdf4d41c7959b80c568f5c5805)
pub type BitmapEncodeOperation = HostBitmapEncodeCapability;
