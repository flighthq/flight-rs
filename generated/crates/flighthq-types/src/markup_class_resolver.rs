// @generated from upstream/packages/types/src/MarkupClassResolver.ts; do not edit.
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

// Source: upstream/packages/types/src/MarkupClassResolver.ts:9 (sha256:c052e7b035ddc0f1752c791c0cf69bcccee961e0005304d3c3b41c35f8ebe5ff)
pub type MarkupClassResolver = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(String) -> Option<FlightPartialRecord1> + Send + 'static>>,
>;
