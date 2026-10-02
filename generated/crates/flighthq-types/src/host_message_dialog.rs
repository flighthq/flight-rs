// @generated from upstream/packages/types/src/HostMessageDialog.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{MessageDialogOptions, MessageDialogResult};

// Source: upstream/packages/types/src/HostMessageDialog.ts:4 (sha256:8f09855eebd8aa24558781befa07c65a19a8e965b04323d2c004f40217b26802)
#[derive(Clone)]
pub struct HostMessageDialogCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub confirm: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(MessageDialogOptions) -> crate::FlightTask<bool> + Send + 'static>,
        >,
    >,
    pub message: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(MessageDialogOptions) -> crate::FlightTask<MessageDialogResult>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostMessageDialogCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
