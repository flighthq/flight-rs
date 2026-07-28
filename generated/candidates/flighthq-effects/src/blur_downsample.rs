// @generated from upstream/packages/effects/src/blurDownsample.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/effects/src/blurDownsample.ts:16 (sha256:2a088ce988c3d8cb10537834e0fe842871137875ea34a16d8047020f29e0242a)
pub fn get_blur_downsample_level(sigma: f64) -> f64 {
    if (sigma <= BLUR_DOWNSAMPLE_MAX_SIGMA) {
        return 0.0_f64;
    }
    return ((sigma / BLUR_DOWNSAMPLE_MAX_SIGMA).log2()).ceil();
}

// Source: upstream/packages/effects/src/blurDownsample.ts:27 (sha256:efb51abeddf7a55257f73c192b49475f88103c627384bdce88bd74f9c5cd8cb6)
pub fn get_blur_residual_sigma(sigma: f64, level: f64) -> f64 {
    if (sigma <= 0.0_f64) {
        return 0.0_f64;
    }
    return (sigma / (2.0_f64).powf(level));
}

// Source: upstream/packages/effects/src/blurDownsample.ts:35 (sha256:71a18dad571abea3fd66d7b2ff12557a46e24ec8aaf2ff3b2e87c61d68ec880c)
const BLUR_DOWNSAMPLE_MAX_SIGMA: f64 = 4.0_f64;
