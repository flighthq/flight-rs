// @generated from upstream/packages/types/src/ToonModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ToonModifier.ts:10 (sha256:32cd129d4ac517481ab1aa7f5c0cd5b6bb8440220cfdd8973a653c570d128acf)
#[derive(Clone)]
pub struct ToonModifier {
    pub kind: String,
    pub slot: String,
    pub steps: f64,
    pub smoothness: Option<f64>,
}

// Source: upstream/packages/types/src/ToonModifier.ts:17 (sha256:76d40005572533e5b8ddff8275be511c5f4809bf81e6deca631c5477b8d4156d)
pub const TOON_MODIFIER_KIND: &'static str = "ToonModifier";
