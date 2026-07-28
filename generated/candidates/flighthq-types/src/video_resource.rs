// @generated from upstream/packages/types/src/VideoResource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/VideoResource.ts:3 (sha256:c2942df01a5899c8fa06bb098c34069f1c2c62ae9d8d67db1c345b944e3cdaef)
pub type VideoChannelState = String;

// Source: upstream/packages/types/src/VideoResource.ts:5 (sha256:2926ca67137a821ed63a29570ef5867f19d531ce6c1e4cdf1c6dbf7b1e94f322)
#[derive(Clone)]
pub struct VideoChannel {
    pub current_time: f64,
    pub gain: f64,
    pub length: f64,
    pub loops: f64,
    pub playback_rate: f64,
    pub source: VideoResource,
    pub state: VideoChannelState,
    pub on_complete: Signal,
}

// Source: upstream/packages/types/src/VideoResource.ts:16 (sha256:c24ea8ca07cc852b9c31dff22d313919cb474b76f72883496272ba045776c236)
#[derive(Clone)]
pub struct VideoPlayOptions {
    pub current_time: Option<f64>,
    pub gain: Option<f64>,
    pub loops: Option<f64>,
    pub playback_rate: Option<f64>,
}

// Source: upstream/packages/types/src/VideoResource.ts:23 (sha256:714d02df18d5250f066b723f7273f5e53b961831609e0385ab1f94c43c7c4a2e)
#[derive(Clone)]
pub struct VideoResource {
    pub element: Option<crate::OpaqueHostValue>,
}

// Source: upstream/packages/types/src/VideoResource.ts:30 (sha256:4f40c87a867dc2c65bfcd8b5f2b66021d2d15e97b89977e5c19568929bb5da0c)
#[derive(Clone)]
pub struct VideoResourceLoadOptions {
    pub cross_origin: Option<String>,
    pub muted: Option<bool>,
    pub plays_inline: Option<bool>,
    pub preload: Option<String>,
    pub readiness: Option<String>,
}

// Source: upstream/packages/types/src/VideoResource.ts:40 (sha256:041ca805e729322fc47c91f8b88061c91f00961a9aa06f62f8d100014e6e29b3)
#[derive(Clone)]
pub struct VideoResourceUrl {
    pub url: String,
    pub type_: Option<String>,
}
