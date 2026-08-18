// @generated from upstream/packages/types/src/GlyphAtlasEntryExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GlyphAtlasEntryExplanation.ts:4 (sha256:99d2b25a22836208d2e981a1a6bb61c22d4ef381d2ecf6975aeab7601f2c43ef)
#[derive(Clone, Default)]
pub struct GlyphAtlasEntryExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub renderable: bool,
    pub reason: GlyphAtlasEntryBlockReason,
    pub glyph_width: f64,
    pub glyph_height: f64,
    pub usable_width: f64,
    pub usable_height: f64,
}
impl PartialEq for GlyphAtlasEntryExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlyphAtlasEntryExplanation.ts:23 (sha256:c804d7e50e154fbc1150157b5eeeaeb08ed9a88bb68e723c9c4d00df8c1d5dda)
pub type GlyphAtlasEntryBlockReason = String;
