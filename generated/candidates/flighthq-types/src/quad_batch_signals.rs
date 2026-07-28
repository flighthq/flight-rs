// @generated from upstream/packages/types/src/QuadBatchSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/QuadBatchSignals.ts:3 (sha256:1d8b7c078de1b0a560d5e73436de18b1b55b1b787686405bd0c88afa884c71e9)
#[derive(Clone)]
pub struct QuadBatchSignals {
    pub on_cleared: Signal,
    pub on_instance_appended: Signal,
    pub on_instance_removed: Signal,
}
