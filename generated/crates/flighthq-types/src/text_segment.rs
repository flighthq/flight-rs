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

// Source: upstream/packages/types/src/TextSegment.ts:33 (sha256:091bb07f4c271f58998f43f60ccc1c92cf3d34c4114438d9718b7ec5d17d892b)
#[derive(Clone)]
pub struct HostTextSegmenterCapability {
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
impl PartialEq for HostTextSegmenterCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextSegment.ts:37 (sha256:1a7468415204f7a6163109870985a6411344c0f1f75f3f003e3e63748c31eb09)
pub type TextSegmenterKind = String;

// Source: upstream/packages/types/src/TextSegment.ts:40 (sha256:1bb8eb574c31c2d2f405e015445b580f34a0fddb3985b03d10a4f9978294b587)
#[derive(Clone, Default)]
pub struct TextSegmenterExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub available: bool,
    pub backend: TextSegmenterKind,
    pub intl_segmenter_available: bool,
}
impl PartialEq for TextSegmenterExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextSegment.ts:47 (sha256:e898a6b87aba75ac8f411ab3aaa769d4602bb91b8d59d94db125480a8e24bfaf)
pub type TextSegmentGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>;
