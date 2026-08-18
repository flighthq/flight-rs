// @generated from upstream/packages/types/src/TextFormat.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::FontVariation;

// Source: upstream/packages/types/src/TextFormat.ts:3 (sha256:8f9fadaa849fce32cdea367226fb23201ddecc99f969884cba557b02b55e12ee)
pub type TextFormatAlign = String;

// Source: upstream/packages/types/src/TextFormat.ts:7 (sha256:b7c61369ba4b4be88cd29d2293b5073f991996a25f690b471e3a3fc37d037ac6)
pub type TextFormatListMarker = String;

// Source: upstream/packages/types/src/TextFormat.ts:9 (sha256:d33a2d7ed5c72aec170c5d488e8c79338f0b4d9764d020d0d649e45e8c5d0653)
#[derive(Clone, Default)]
pub struct TextFormat {
    #[doc(hidden)]
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
impl PartialEq for TextFormat {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
