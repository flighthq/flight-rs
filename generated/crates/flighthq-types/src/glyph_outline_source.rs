// @generated from upstream/packages/types/src/GlyphOutlineSource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Path;

// Source: upstream/packages/types/src/GlyphOutlineSource.ts:7 (sha256:295829d765536c27703d804853c43f2324df740dd2ff35a8daf563c716fe2465)
#[derive(Clone, Default)]
pub struct GlyphOutlineMetrics {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ascent: f64,
    pub descent: f64,
    pub line_gap: f64,
    pub units_per_em: f64,
}
impl PartialEq for GlyphOutlineMetrics {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlyphOutlineSource.ts:23 (sha256:2f2aa152a7360268360f27ac0410707868b0fbde590b57869ea84d7c647714b5)
#[derive(Clone)]
pub struct GlyphOutlineSource {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_glyph_outline:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Path, f64) -> bool + Send + 'static>>>,
    pub get_glyph_outline_advance:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>>,
    pub get_glyph_outline_index_for_code_point:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>>,
    pub get_glyph_outline_metrics:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> GlyphOutlineMetrics + Send + 'static>>>,
}
impl PartialEq for GlyphOutlineSource {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
