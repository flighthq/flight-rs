// @generated from upstream/packages/types/src/ClipboardWatch.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/ClipboardWatch.ts:6 (sha256:6a228f60bea13bb3ad99a5c2366d343d54ec22b3841bc07b6d53c36c26087b6a)
#[derive(Clone)]
pub struct ClipboardWatch {
    pub on_change: Signal,
}
