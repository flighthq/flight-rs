// @generated from upstream/packages/types/src/HostPhotoCaptureDialog.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CapturePhotoDialogOptions, PhotoCaptureDialogResult};

// Source: upstream/packages/types/src/HostPhotoCaptureDialog.ts:3 (sha256:08662e7deb6ad9055ded847ddefcc64f8332657e9947cb16487f23d863db212a)
#[derive(Clone)]
pub struct HostPhotoCaptureDialogCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub capture: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Option<CapturePhotoDialogOptions>,
                    ) -> crate::FlightTask<PhotoCaptureDialogResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostPhotoCaptureDialogCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
