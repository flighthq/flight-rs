// @generated from upstream/packages/types/src/BitmapFontGlyphExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BitmapFontGlyphExplanation.ts:6 (sha256:1d9865071b7ee7e1a13d9aa8b3af48d14be54dfae6c2fef24c5c466adc90e1b6)
#[derive(Clone, Default)]
pub struct BitmapFontGlyphExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub renderable: bool,
    pub reason: BitmapFontGlyphExplanationReason,
    pub page: f64,
    pub page_count: f64,
    pub glyph_width: f64,
    pub glyph_height: f64,
}
impl PartialEq for BitmapFontGlyphExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BitmapFontGlyphExplanation.ts:24 (sha256:72a50bfa077d0301e0cc35b92c97a178b7155f74d88203afc650ac09d7bd53c3)
pub type BitmapFontGlyphExplanationReason = String;
