// @generated from upstream/packages/types/src/HostPromptDialog.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::PromptDialogOptions;

// Source: upstream/packages/types/src/HostPromptDialog.ts:3 (sha256:d1fe6625e2217fd877ca0f65a2d8e20209a19cc34fc1869125d01437e833de01)
#[derive(Clone)]
pub struct HostPromptDialogCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub prompt: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(PromptDialogOptions) -> crate::FlightTask<Option<String>>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostPromptDialogCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
