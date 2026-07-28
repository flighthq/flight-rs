// @generated from upstream/packages/types/src/InteractionSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/InteractionSignals.ts:6 (sha256:f69ab51c1ad10e742d8cc8e2f3720122c3c28d5f9fa84ec4b855944258e8e2df)
#[derive(Clone)]
pub struct InteractionSignals {
    pub on_click: Signal,
    pub on_context_menu: Signal,
    pub on_double_click: Signal,
    pub on_focus_in: Signal,
    pub on_focus_out: Signal,
    pub on_key_down: Signal,
    pub on_key_up: Signal,
    pub on_pointer_cancel: Signal,
    pub on_pointer_down: Signal,
    pub on_pointer_move: Signal,
    pub on_pointer_out: Signal,
    pub on_pointer_over: Signal,
    pub on_pointer_roll_out: Signal,
    pub on_pointer_roll_over: Signal,
    pub on_pointer_up: Signal,
    pub on_release_outside: Signal,
    pub on_wheel: Signal,
}
