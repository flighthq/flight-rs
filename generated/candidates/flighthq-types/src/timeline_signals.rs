// @generated from upstream/packages/types/src/TimelineSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/TimelineSignals.ts:6 (sha256:c87cb1c6c826c9d4a5ac66245abd3b19328174cc315ae7f9153271c2700ce41c)
#[derive(Clone)]
pub struct TimelineSignals {
    pub on_complete: Signal,
    pub on_enter_frame: Signal,
    pub on_exit_frame: Signal,
    pub on_frame_constructed: Signal,
    pub on_loop: Signal,
}
