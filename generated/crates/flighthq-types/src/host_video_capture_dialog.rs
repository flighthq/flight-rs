// @generated from upstream/packages/types/src/HostVideoCaptureDialog.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CaptureVideoDialogOptions, VideoCaptureDialogResult};

// Source: upstream/packages/types/src/HostVideoCaptureDialog.ts:3 (sha256:6d75fe1941d877b0234cc317fe0f4d29347be56219e3b5c0ef1893971eafade4)
#[derive(Clone)]
pub struct HostVideoCaptureDialogCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub capture: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Option<CaptureVideoDialogOptions>,
                    ) -> crate::FlightTask<VideoCaptureDialogResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostVideoCaptureDialogCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
