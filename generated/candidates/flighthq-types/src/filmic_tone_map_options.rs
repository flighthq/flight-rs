// @generated from upstream/packages/types/src/FilmicToneMapOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/FilmicToneMapOptions.ts:1 (sha256:90b88974fa401779beea4ea28f131baed780d42c8ed8a9e9d2bbdf626f5a2a9d)
#[derive(Clone)]
pub struct FilmicToneMapOptions {
    pub max_brightness: Option<f64>,
    pub contrast: Option<f64>,
    pub linear_start: Option<f64>,
    pub linear_length: Option<f64>,
    pub black_tighten: Option<f64>,
    pub pedestal: Option<f64>,
}
