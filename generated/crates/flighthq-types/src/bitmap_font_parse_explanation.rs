// @generated from upstream/packages/types/src/BitmapFontParseExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BitmapFontParseExplanation.ts:1 (sha256:deabc29005c19e79c0ce7bc693339b02c86c116db6adec93ec36a1ab588077ba)
pub type BitmapFontParseExplanationFormat = String;

// Source: upstream/packages/types/src/BitmapFontParseExplanation.ts:3 (sha256:cb0902dfda5e792d5fe293b0ccfa61db429a55aa98a14d5555945fd26016f582)
pub type BitmapFontParseExplanationReason = String;

// Source: upstream/packages/types/src/BitmapFontParseExplanation.ts:10 (sha256:a9513d24c7dddf0c807555ec1ae0c194710f6890f854082703b8013552360cf6)
#[derive(Clone, Default)]
pub struct BitmapFontParseExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub char_count: f64,
    pub detected_format: BitmapFontParseExplanationFormat,
    pub kerning_count: f64,
    pub page_count: f64,
    pub reason: BitmapFontParseExplanationReason,
    pub success: bool,
    pub unresolved_pages: Vec<f64>,
}
impl PartialEq for BitmapFontParseExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
