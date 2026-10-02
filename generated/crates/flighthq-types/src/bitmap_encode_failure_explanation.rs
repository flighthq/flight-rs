// @generated from upstream/packages/types/src/BitmapEncodeFailureExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ImageFormat;

// Source: upstream/packages/types/src/BitmapEncodeFailureExplanation.ts:3 (sha256:7a4292bf7834dc82618e1722f8af92008a90259f70a8a8e22ddf86524c1fb2b3)
#[derive(Clone, Default)]
pub struct BitmapEncodeFailureExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub format: ImageFormat,
    pub reason: String,
}
impl PartialEq for BitmapEncodeFailureExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
