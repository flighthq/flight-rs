// @generated from upstream/packages/types/src/MediaSession.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/MediaSession.ts:4 (sha256:165d70f259cafc04182a0625560eaeac80af5cc5825a3f8ea7d7c8aaf2fa89fd)
#[derive(Clone)]
pub struct MediaSessionArtwork {
    pub src: String,
    pub sizes: Option<String>,
    pub type_: Option<String>,
}

// Source: upstream/packages/types/src/MediaSession.ts:12 (sha256:aee262f51e361dfe8ea521ea2ec6f94a4381baf8678d6ce00fd092f7a3fa562d)
#[derive(Clone)]
pub struct MediaSessionMetadata {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub artwork: Vec<MediaSessionArtwork>,
}

// Source: upstream/packages/types/src/MediaSession.ts:21 (sha256:b27a26dd5f1d6c5ba85e2d718a4cca89de2eb3575b06c0c04d591c0d0db8c5ed)
pub type MediaSessionAction = String;

// Source: upstream/packages/types/src/MediaSession.ts:35 (sha256:0f48bcd9db61c0a9d9c03f2b69844f40f9a14b13fb84bfeb0b893f02962fdd5d)
#[derive(Clone)]
pub struct MediaSessionActionDetails {
    pub action: MediaSessionAction,
    pub seek_time: Option<f64>,
    pub seek_offset: Option<f64>,
    pub fast_seek: Option<bool>,
}

// Source: upstream/packages/types/src/MediaSession.ts:44 (sha256:cc0c479e60d32102532af01cde025911e4d8dc14dc1b96b05e8edb295099ddef)
pub type MediaSessionPlaybackState = String;

// Source: upstream/packages/types/src/MediaSession.ts:48 (sha256:778cd16a82d742b05604332092f1827cab6eb4d6b799756b7c841c1bef9c8cae)
#[derive(Clone)]
pub struct MediaSessionPositionState {
    pub duration: f64,
    pub playback_rate: f64,
    pub position: f64,
}

// Source: upstream/packages/types/src/MediaSession.ts:58 (sha256:8b64831c37ce434f4c9511f01f4d7ac1b3711ca048574b569bb6fdb4c26cc040)
#[derive(Clone)]
pub struct MediaSessionBackend {
    pub set_metadata: crate::OpaqueHostValue,
    pub set_playback_state: crate::OpaqueHostValue,
    pub set_position_state: crate::OpaqueHostValue,
    pub set_action_handler: crate::OpaqueHostValue,
}
