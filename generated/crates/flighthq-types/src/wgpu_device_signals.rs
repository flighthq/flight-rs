// @generated from upstream/packages/types/src/WgpuDeviceSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/WgpuDeviceSignals.ts:11 (sha256:c533d7bd792cae79a9f6f0f0f55f16609e6388af8ed1697d917aeaef6b4b80a3)
#[derive(Clone)]
pub struct WgpuDeviceSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_device_lost: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
        >,
    >,
}
impl PartialEq for WgpuDeviceSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
