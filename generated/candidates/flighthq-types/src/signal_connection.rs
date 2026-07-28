// @generated from upstream/packages/types/src/SignalConnection.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/SignalConnection.ts:11 (sha256:91655ac3159a3beced78493debe1fafd9a7bd7d8955b20146b5d6596a63265ca)
#[derive(Clone)]
pub struct SignalConnection {
    pub signal: Signal,
    pub slot: crate::OpaqueHostValue,
    pub connected: bool,
    pub paused: bool,
}
