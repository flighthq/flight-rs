// @generated from upstream/packages/types/src/TextSegment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TextSegment.ts:10 (sha256:ca7ff40e5f56caab89f6c08d7448d50148d4736c5c0175819d7b299f10d47ab2)
pub type TextSegmentGranularity = String;

// Source: upstream/packages/types/src/TextSegment.ts:15 (sha256:9b243f874c904eab543ac8b4bf5d2d573d276fd43d61844e87ac58bb55bfba3c)
#[derive(Clone)]
pub struct TextSegment {
    pub start: f64,
    pub end: f64,
    pub text: String,
    pub is_word_like: Option<bool>,
}

// Source: upstream/packages/types/src/TextSegment.ts:24 (sha256:6f779d73dea3b8ccd32e3072ea63d62d6703cab13841d3d52d1bdab232fb9b73)
#[derive(Clone)]
pub struct TextSegmentRange {
    pub start: f64,
    pub end: f64,
}

// Source: upstream/packages/types/src/TextSegment.ts:33 (sha256:0a3e2f66fc1a39a37dc580ccaf23d8d6222e508de148314d910a79667e546c86)
#[derive(Clone)]
pub struct TextSegmenterBackend {
    pub segment: crate::OpaqueHostValue,
}
