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
#[derive(Clone, Default)]
pub struct TextSegment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub start: f64,
    pub end: f64,
    pub text: String,
    pub is_word_like: Option<bool>,
}
impl PartialEq for TextSegment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextSegment.ts:24 (sha256:6f779d73dea3b8ccd32e3072ea63d62d6703cab13841d3d52d1bdab232fb9b73)
#[derive(Clone, Default)]
pub struct TextSegmentRange {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub start: f64,
    pub end: f64,
}
impl PartialEq for TextSegmentRange {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextSegment.ts:33 (sha256:0a3e2f66fc1a39a37dc580ccaf23d8d6222e508de148314d910a79667e546c86)
#[derive(Clone)]
pub struct TextSegmenterBackend {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub segment: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String, TextSegmentGranularity, Option<String>) -> Vec<TextSegment>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for TextSegmenterBackend {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
