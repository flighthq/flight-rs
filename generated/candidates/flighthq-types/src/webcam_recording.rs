// @generated from upstream/packages/types/src/WebcamRecording.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WebcamRecording.ts:2 (sha256:ebc991d9c9e88ecc39034abee14a1612490800439ac6fd345ed0a3d9656f89f5)
#[derive(Clone)]
pub struct WebcamRecording {
    pub active: bool,
    pub id: String,
    pub mime_type: String,
    pub started_at_ms: f64,
}
