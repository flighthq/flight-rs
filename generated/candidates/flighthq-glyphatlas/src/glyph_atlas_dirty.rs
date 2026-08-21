// @generated from upstream/packages/glyphatlas/src/glyphAtlasDirty.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::create_rectangle;
use flighthq_types::{GlyphAtlas, Rectangle};

// Source: upstream/packages/glyphatlas/src/glyphAtlasDirty.ts:6 (sha256:113fca69dd9c76733c12038178d80dcd4cdea1bf45ff4e1277d24c423e194786)
pub fn clear_glyph_atlas_dirty(atlas: &mut GlyphAtlas) -> () {
    atlas.runtime.dirty = false;
}

// Source: upstream/packages/glyphatlas/src/glyphAtlasDirty.ts:13 (sha256:5367cb3679d42a3df40c88796a9f3944e5e5e07dc22e1ea95ab9ee36edb1de1f)
pub fn get_glyph_atlas_dirty_region(atlas: &GlyphAtlas) -> Option<Rectangle> {
    if (!atlas.runtime.dirty) {
        return None;
    }
    return Some(create_rectangle(
        Some(atlas.runtime.dirty_min_x),
        Some(atlas.runtime.dirty_min_y),
        Some((atlas.runtime.dirty_max_x - atlas.runtime.dirty_min_x)),
        Some((atlas.runtime.dirty_max_y - atlas.runtime.dirty_min_y)),
    ));
}
