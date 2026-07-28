// @generated from upstream/packages/types/src/TextFormat.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TextFormat.ts:1 (sha256:8f9fadaa849fce32cdea367226fb23201ddecc99f969884cba557b02b55e12ee)
pub type TextFormatAlign = String;

// Source: upstream/packages/types/src/TextFormat.ts:5 (sha256:b7c61369ba4b4be88cd29d2293b5073f991996a25f690b471e3a3fc37d037ac6)
pub type TextFormatListMarker = String;

// Source: upstream/packages/types/src/TextFormat.ts:7 (sha256:78b33d0aba9800116f93363692c37d18b0e439bf94df4d5120e7d11bf0280f50)
#[derive(Clone)]
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
}
impl PartialEq for TextFormat {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
