// @generated from upstream/packages/types/src/WebcamDevice.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::WebcamFacingMode;

// Source: upstream/packages/types/src/WebcamDevice.ts:2 (sha256:db3309a70b1f1ba4f37c304d478dba321eaeaaa7171564a8d22535cf9465714b)
#[derive(Clone)]
pub struct WebcamDevice {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub device_id: String,
    pub facing_mode: Option<WebcamFacingMode>,
    pub kind: String,
    pub label: String,
}
impl PartialEq for WebcamDevice {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
