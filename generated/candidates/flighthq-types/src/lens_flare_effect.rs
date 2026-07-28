// @generated from upstream/packages/types/src/LensFlareEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/LensFlareEffect.ts:3 (sha256:fd7a053e9ad1fb2da43a056833df82c8be9d0574e7f6abf1be97c220b5f28e87)
#[derive(Clone)]
pub struct LensFlareEffect {
    pub kind: String,
    pub threshold: Option<f64>,
    pub intensity: Option<f64>,
    pub ghosts: Option<f64>,
    pub halo: Option<f64>,
}
