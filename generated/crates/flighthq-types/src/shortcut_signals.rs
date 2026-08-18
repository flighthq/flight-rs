// @generated from upstream/packages/types/src/ShortcutSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ShortcutEvent, Signal};

// Source: upstream/packages/types/src/ShortcutSignals.ts:6 (sha256:5130b8c1f92a27b3cbd08c9df044203693ef4199cfc6b4d774d6106c1471d378)
#[derive(Clone)]
pub struct ShortcutSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_trigger: Signal<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ShortcutEvent) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for ShortcutSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
