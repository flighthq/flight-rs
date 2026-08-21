// @generated from upstream/packages/glyphatlas/src/explainGlyphAtlasEntry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_glyph_rasterizer_backend;
use flighthq_types::{GlyphAtlas, GlyphAtlasEntryExplanation};

// Source: upstream/packages/glyphatlas/src/explainGlyphAtlasEntry.ts:15 (sha256:c5eb390db2d98245c2155765e109914f02b36808aa320777e03f280d04e8d2dd)
pub fn explain_glyph_atlas_entry(atlas: &GlyphAtlas, codepoint: f64) -> GlyphAtlasEntryExplanation {
    let padding = atlas.runtime.padding;
    let usable_width = (atlas.runtime.bitmap.width - (2.0_f64 * padding));
    let usable_height = (atlas.runtime.bitmap.height - (2.0_f64 * padding));
    if atlas
        .runtime
        .entries
        .iter()
        .any(|(entry_key, _)| entry_key == &codepoint)
    {
        let entry = atlas
            .runtime
            .entries
            .iter()
            .find(|(entry_key, _)| entry_key == &codepoint)
            .map(|(_, value)| value.clone());
        return GlyphAtlasEntryExplanation {
            __flight_identity: std::sync::Arc::new(()),
            renderable: true,
            reason: "ok".to_owned(),
            glyph_width: entry.as_ref().unwrap().width,
            glyph_height: entry.as_ref().unwrap().height,
            usable_width: usable_width,
            usable_height: usable_height,
        };
    }
    let bitmap = {
        let __flight_callback = (get_glyph_rasterizer_backend().rasterize).clone();
        let __flight_result =
            __flight_callback.lock().unwrap()(codepoint, (atlas.runtime.rasterize_options).clone());
        __flight_result
    };
    if (bitmap).is_none() {
        return GlyphAtlasEntryExplanation {
            __flight_identity: std::sync::Arc::new(()),
            renderable: false,
            reason: "rasterizer-returned-null".to_owned(),
            glyph_width: 0.0_f64,
            glyph_height: 0.0_f64,
            usable_width: usable_width,
            usable_height: usable_height,
        };
    }
    let fits = (bitmap.as_ref().unwrap().width <= usable_width)
        && (bitmap.as_ref().unwrap().height <= usable_height);
    return GlyphAtlasEntryExplanation {
        __flight_identity: std::sync::Arc::new(()),
        renderable: fits,
        reason: if fits {
            "ok".to_owned()
        } else {
            "glyph-larger-than-atlas".to_owned()
        },
        glyph_width: bitmap.as_ref().unwrap().width,
        glyph_height: bitmap.as_ref().unwrap().height,
        usable_width: usable_width,
        usable_height: usable_height,
    };
}
