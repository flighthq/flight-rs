// @generated from upstream/packages/types/src/BitmapTextMissingGlyphs.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BitmapTextMissingGlyphs.ts:1 (sha256:34552fa7397cbd48f0df42d26c99d2b87846006c0c9434a12fd7c7380d4d48c6)
#[derive(Clone, Default)]
pub struct BitmapTextMissingGlyphs {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub missing_codepoints: Vec<f64>,
    pub total_codepoints: f64,
}
impl PartialEq for BitmapTextMissingGlyphs {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
