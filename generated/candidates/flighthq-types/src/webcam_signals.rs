// @generated from upstream/packages/types/src/WebcamSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Signal;

// Source: upstream/packages/types/src/WebcamSignals.ts:4 (sha256:a496a3dac40a9e963083c4b8fa801622b268313628b9c6f53f049f226bf25418)
#[derive(Clone)]
pub struct WebcamSignals {
    pub on_webcam_device_change: Signal,
    pub on_webcam_permission_change: Signal,
    pub on_webcam_stream_ended: Signal,
}
