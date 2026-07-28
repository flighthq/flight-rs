// @generated from upstream/packages/types/src/WebcamStream.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::WebcamFacingMode;

// Source: upstream/packages/types/src/WebcamStream.ts:3 (sha256:4dd420a3de66629fd9d20d6edebb45002d12629e66fe948f85923169759058ea)
#[derive(Clone)]
pub struct WebcamStream {
    pub active: bool,
    pub device_id: String,
    pub facing_mode: Option<WebcamFacingMode>,
    pub frame_rate: f64,
    pub height: f64,
    pub id: String,
    pub width: f64,
}
