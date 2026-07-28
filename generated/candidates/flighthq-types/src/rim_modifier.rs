// @generated from upstream/packages/types/src/RimModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/RimModifier.ts:9 (sha256:2b4a037ea35181fae6c616f8fcdfc0bdc99d8538a1b8cf0c35814e146612ae76)
#[derive(Clone)]
pub struct RimModifier {
    pub kind: String,
    pub slot: String,
    pub color: f64,
    pub power: Option<f64>,
    pub intensity: Option<f64>,
    pub bias: Option<f64>,
}

// Source: upstream/packages/types/src/RimModifier.ts:18 (sha256:01fd7e0e8dbd5523c21cd8aaea39fd4498ced019807631774cc0e11d1f366ea7)
pub const RIM_MODIFIER_KIND: &'static str = "RimModifier";
