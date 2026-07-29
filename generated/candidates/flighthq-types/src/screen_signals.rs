// @generated from upstream/packages/types/src/ScreenSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ScreenInfo, Signal};

// Source: upstream/packages/types/src/ScreenSignals.ts:8 (sha256:14ccb0ac2c6140e6c6ad3b203080b39e139de7e0f6dab05c8cb102e1f2c8d163)
#[derive(Clone)]
pub struct ScreenSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_screen_added:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ScreenInfo) -> () + Send + 'static>>>>,
    pub on_screen_metrics_changed: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> () + Send + 'static>>,
        >,
    >,
    pub on_screen_removed:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ScreenInfo) -> () + Send + 'static>>>>,
}
impl PartialEq for ScreenSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
