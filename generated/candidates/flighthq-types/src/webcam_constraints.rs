// @generated from upstream/packages/types/src/WebcamConstraints.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WebcamConstraints.ts:1 (sha256:7eb44a726269aabe8168241ccb3838d727d91d974673cb660d0d612ba2fcf4cc)
#[derive(Clone)]
pub struct WebcamConstraints {
    pub exposure_compensation: Option<f64>,
    pub exposure_mode: Option<String>,
    pub focus_distance: Option<f64>,
    pub focus_mode: Option<String>,
    pub torch: Option<bool>,
    pub white_balance_mode: Option<String>,
    pub zoom: Option<f64>,
}
