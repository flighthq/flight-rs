// @generated from upstream/packages/types/src/MarkupTagHandler.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{FontVariation, TextFormatAlign, TextFormatListMarker};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub align: Option<TextFormatAlign>,
    pub block_indent: Option<f64>,
    pub bold: Option<bool>,
    pub bullet: Option<bool>,
    pub color: Option<f64>,
    pub font: Option<String>,
    pub indent: Option<f64>,
    pub italic: Option<bool>,
    pub kerning: Option<bool>,
    pub leading: Option<f64>,
    pub left_margin: Option<f64>,
    pub letter_spacing: Option<f64>,
    pub list_marker: Option<TextFormatListMarker>,
    pub right_margin: Option<f64>,
    pub size: Option<f64>,
    pub strikethrough: Option<bool>,
    pub tab_stops: Option<Vec<f64>>,
    pub target: Option<String>,
    pub underline: Option<bool>,
    pub url: Option<String>,
    pub variations: Option<Vec<FontVariation>>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MarkupTagHandler.ts:14 (sha256:c0609467d747c51440ccdb9e1a0db9afedf742cae51c8e3f8f571cfb061e08ae)
pub type MarkupTagResult = crate::FlightUnion2<FlightPartialRecord1, MarkupTagEffect>;

// Source: upstream/packages/types/src/MarkupTagHandler.ts:16 (sha256:7b8cbeecd35b9e04b3c14957a910b0b80f82dfe3f2c265b9d531aae186772ceb)
#[derive(Clone, Default)]
pub struct MarkupTagEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub break_before: Option<bool>,
    pub format: Option<FlightPartialRecord1>,
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
