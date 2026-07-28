// @generated from upstream/packages/types/src/ResourceLoader.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/ResourceLoader.ts:4 (sha256:e3550276e310c2635b9749863ee0dcc64b1809e9ecee8dae906ecb6e6dd0fd03)
#[derive(Clone)]
pub struct ResourceLoader {
    pub on_cancel: Signal,
    pub on_complete: Signal,
    pub on_error: Signal,
    pub on_pause: Signal,
    pub on_progress: Signal,
    pub on_resume: Signal,
}
