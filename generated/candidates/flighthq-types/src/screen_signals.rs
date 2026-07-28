// @generated from upstream/packages/types/src/ScreenSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/ScreenSignals.ts:8 (sha256:14ccb0ac2c6140e6c6ad3b203080b39e139de7e0f6dab05c8cb102e1f2c8d163)
#[derive(Clone)]
pub struct ScreenSignals {
    pub on_screen_added: Signal,
    pub on_screen_metrics_changed: Signal,
    pub on_screen_removed: Signal,
}
