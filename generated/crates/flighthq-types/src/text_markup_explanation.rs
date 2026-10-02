// @generated from upstream/packages/types/src/TextMarkupExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TextMarkupExplanation.ts:1 (sha256:00d23ddfcc9b915d9b800a1bc7d0161f8c5e7846c05e93bb0db49596d7163c37)
pub type TextMarkupIssueKind = String;

// Source: upstream/packages/types/src/TextMarkupExplanation.ts:5 (sha256:d399db6bc3304c07667fb0d95b3065f47afc2c6771356f15c67c08bcf38bd338)
#[derive(Clone, Default)]
pub struct TextMarkupIssue {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: TextMarkupIssueKind,
    pub offset: f64,
    pub tag: String,
    pub value: Option<String>,
}
impl PartialEq for TextMarkupIssue {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextMarkupExplanation.ts:14 (sha256:7f7e3df724664f816e7ddc29f69941a02ad3f3f5d23a99780755e739d7eb884c)
#[derive(Clone, Default)]
pub struct TextMarkupExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub issues: Vec<TextMarkupIssue>,
}
impl PartialEq for TextMarkupExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextMarkupExplanation.ts:19 (sha256:ed9d954ffe627842e2e23aac25af32c4dcfbb362a480a2d988f96c2adaeb76ff)
pub type TextMarkupGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(TextMarkupIssue) -> () + Send + 'static>>>;
