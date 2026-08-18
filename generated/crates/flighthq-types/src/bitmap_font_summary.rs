// @generated from upstream/packages/types/src/BitmapFontSummary.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BitmapFontSummary.ts:5 (sha256:09a9cebd383e555474602db4651182f506f99980444d41d6c831a9f775fb5f50)
#[derive(Clone, Default)]
pub struct BitmapFontSummary {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub glyph_count: f64,
    pub kerning_pair_count: f64,
    pub page_count: f64,
    pub byte_size: f64,
    pub min_codepoint: f64,
    pub max_codepoint: f64,
}
impl PartialEq for BitmapFontSummary {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
