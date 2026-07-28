// @generated from upstream/packages/types/src/IpcSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/IpcSignals.ts:5 (sha256:2abb12b02beb1eeefd25c469971e263caa4acf7e065a63378d94564d7a9baf93)
#[derive(Clone)]
pub struct IpcSignals {
    pub on_backend_changed: Signal,
    pub on_channel_message: Signal,
}
