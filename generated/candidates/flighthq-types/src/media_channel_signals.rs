// @generated from upstream/packages/types/src/MediaChannelSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/MediaChannelSignals.ts:2 (sha256:9b25c6ec95899a6da6bb480240484680f8a765976df53dc60c2bebda4c039847)
pub type MediaReadyState = String;

// Source: upstream/packages/types/src/MediaChannelSignals.ts:3 (sha256:2dd1022f7a41d0f5bc443fa7736e7be869b06faae79ef1dc39976e5cb321df5c)
#[derive(Clone)]
pub struct MediaChannelSignals {
    pub on_buffering: Signal,
    pub on_error: Signal,
    pub on_ready: Signal,
    pub on_seeked: Signal,
}
