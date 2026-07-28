// @generated from upstream/packages/types/src/WebcamStreamOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::WebcamFacingMode;

// Source: upstream/packages/types/src/WebcamStreamOptions.ts:2 (sha256:ce5d83bd1d8c0f3ca36c57eba31889a1e68953323ded068a867f1c1c36738707)
#[derive(Clone)]
pub struct WebcamStreamOptions {
    pub audio: Option<bool>,
    pub device_id: Option<String>,
    pub facing_mode: Option<WebcamFacingMode>,
    pub frame_rate: Option<f64>,
    pub height: Option<f64>,
    pub width: Option<f64>,
}
