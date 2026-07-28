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
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub active: bool,
    pub id: String,
    pub mime_type: String,
    pub started_at_ms: f64,
}
impl PartialEq for WebcamRecording {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
