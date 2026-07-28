// @generated from upstream/packages/types/src/StageSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/StageSignals.ts:3 (sha256:a6c4a9630a187dfae81bfed350ffca90d7ec9c4f1805a0ed819f67cf0e6ac791)
#[derive(Clone)]
pub struct StageSignals {
    pub on_fullscreen_changed: Signal,
    pub on_orientation_changed: Signal,
    pub on_resize: Signal,
}
