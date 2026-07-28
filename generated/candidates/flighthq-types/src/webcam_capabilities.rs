// @generated from upstream/packages/types/src/WebcamCapabilities.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{WebcamCapabilityRange, WebcamFacingMode};

// Source: upstream/packages/types/src/WebcamCapabilities.ts:3 (sha256:87fe517940a95013951f6aa994e4805d07583a2bd3485d8dc037c4ff60dfabdd)
#[derive(Clone)]
pub struct WebcamCapabilities {
    pub exposure_compensation: Option<WebcamCapabilityRange>,
    pub exposure_modes: Vec<String>,
    pub focus_distance: Option<WebcamCapabilityRange>,
    pub focus_modes: Vec<String>,
    pub frame_height: Option<WebcamCapabilityRange>,
    pub frame_rate: Option<WebcamCapabilityRange>,
    pub frame_width: Option<WebcamCapabilityRange>,
    pub supported_facing_modes: Vec<WebcamFacingMode>,
    pub torch: bool,
    pub white_balance_modes: Vec<String>,
    pub zoom: Option<WebcamCapabilityRange>,
}
