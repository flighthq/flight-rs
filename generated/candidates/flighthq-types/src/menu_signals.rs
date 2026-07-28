// @generated from upstream/packages/types/src/MenuSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/MenuSignals.ts:7 (sha256:59672e85023a8fa3e3bf9fe945d6666e92b4dc898f3eedbdc34aa957dd48c507)
#[derive(Clone)]
pub struct MenuSignals {
    pub on_context_menu_open: Signal,
    pub on_context_menu_close: Signal,
    pub on_menu_item_highlight: Signal,
    pub on_menu_item_select: Signal,
}
