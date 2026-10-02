// @generated from upstream/packages/types/src/HostImageOpenDialog.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ImageOpenDialogResult, OpenImageDialogOptions};

// Source: upstream/packages/types/src/HostImageOpenDialog.ts:3 (sha256:d38f571b8abc26a0478653837588ace094a0ee46e5ef5f515b84bd3c68bb33c5)
#[derive(Clone)]
pub struct HostImageOpenDialogCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub open: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Option<OpenImageDialogOptions>,
                    ) -> crate::FlightTask<ImageOpenDialogResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostImageOpenDialogCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
