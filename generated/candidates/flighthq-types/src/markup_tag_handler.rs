// @generated from upstream/packages/types/src/MarkupTagHandler.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TextFormat;

// Source: upstream/packages/types/src/MarkupTagHandler.ts:14 (sha256:c0609467d747c51440ccdb9e1a0db9afedf742cae51c8e3f8f571cfb061e08ae)
pub type MarkupTagResult = crate::FlightUnion2<TextFormat, MarkupTagEffect>;

// Source: upstream/packages/types/src/MarkupTagHandler.ts:16 (sha256:7b8cbeecd35b9e04b3c14957a910b0b80f82dfe3f2c265b9d531aae186772ceb)
#[derive(Clone)]
pub struct MarkupTagEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub break_before: Option<bool>,
    pub format: Option<TextFormat>,
    pub text: Option<String>,
}
impl PartialEq for MarkupTagEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MarkupTagHandler.ts:32 (sha256:237d44b6317e7b18de6866656d42e8769df452608341a35e8c8d9d28355309ae)
pub type MarkupTagHandler = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Vec<(String, String)>) -> MarkupTagResult + Send + 'static>>,
>;
