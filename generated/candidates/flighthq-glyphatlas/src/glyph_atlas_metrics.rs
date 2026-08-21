// @generated from upstream/packages/glyphatlas/src/glyphAtlasMetrics.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{GlyphAtlas, GlyphMetrics};

// Source: upstream/packages/glyphatlas/src/glyphAtlasMetrics.ts:6 (sha256:0c44680f00ca90fac37173defb200506f8da1c8da643f0fcb800ce0a1134790e)
pub fn get_glyph_atlas_kerning(_atlas: &GlyphAtlas, _left: f64, _right: f64) -> f64 {
    return 0.0_f64;
}

// Source: upstream/packages/glyphatlas/src/glyphAtlasMetrics.ts:12 (sha256:47019dff827b100285d4c2dc4646f6bdef39e3a3d3bee31177aa8efde5b3a181)
pub fn get_glyph_atlas_metrics(atlas: &GlyphAtlas) -> GlyphMetrics {
    return (atlas.runtime.metrics).clone();
}
