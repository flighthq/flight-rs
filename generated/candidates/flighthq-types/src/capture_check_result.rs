// @generated from upstream/packages/types/src/CaptureCheckResult.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/CaptureCheckResult.ts:7 (sha256:0f33d23a7c65f0166bba1a41ba7dc198ccaa34b6d6d012761be31a66384a27f4)
#[derive(Clone)]
pub struct CaptureCheckResult {
    pub pass: bool,
    pub difference: f64,
    pub tolerance: f64,
}
