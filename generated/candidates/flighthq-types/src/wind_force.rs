// @generated from upstream/packages/types/src/WindForce.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WindForce.ts:1 (sha256:27ca0b806294b5d3ba41620b3e85e381c09e51e239b7dc6e2b247981fa00a48a)
#[derive(Clone)]
pub struct WindForce {
    pub kind: String,
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

// Source: upstream/packages/types/src/WindForce.ts:8 (sha256:186630e6e3592fd1c07f73a43e905b7c9503e7389acf44e4c4483d53384512dc)
pub const WIND_FORCE_KIND: &'static str = "WindForce";
