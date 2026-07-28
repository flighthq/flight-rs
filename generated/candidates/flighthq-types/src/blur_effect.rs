// @generated from upstream/packages/types/src/BlurEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BlurEffect.ts:8 (sha256:057f7b6cb433bf1bd71e9973328f74a479ab0395a4b8cf9166fc61fc32917bdf)
#[derive(Clone)]
pub struct BlurEffect {
    pub kind: String,
    pub blur_x: Option<f64>,
    pub blur_y: Option<f64>,
}
