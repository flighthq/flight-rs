// @generated from upstream/packages/types/src/CameraMotionBlurEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/CameraMotionBlurEffect.ts:3 (sha256:14fb90d626988346800d7e25bf8c35acbc18a26771202aba8b1af87a98b3a431)
#[derive(Clone)]
pub struct CameraMotionBlurEffect {
    pub kind: String,
    pub intensity: Option<f64>,
    pub samples: Option<f64>,
}
