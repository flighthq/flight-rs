// @generated from upstream/packages/bitmapfont/src/bitmapFontFromGlyphAtlas.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_bitmap_font;
use flighthq_texture::create_texture;
use flighthq_textureatlas::create_texture_atlas;
use flighthq_types::{
    BitmapFont, BitmapFontData, BitmapFontGlyphData, GlyphAtlas, Texture2D, TextureAtlasRegion,
    TextureFilter, TextureWrap,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub anisotropy: Option<f64>,
    pub mag_filter: Option<TextureFilter>,
    pub min_filter: Option<TextureFilter>,
    pub mipmaps: Option<bool>,
    pub wrap_u: Option<TextureWrap>,
    pub wrap_v: Option<TextureWrap>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub texture: Option<Texture2D>,
    pub regions: Option<Vec<TextureAtlasRegion>>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/bitmapfont/src/bitmapFontFromGlyphAtlas.ts:23 (sha256:ac015643681b3f50ed6411d4d231095156f1b16a4cc42f80aa1f0a5b0189757d)
pub fn create_bitmap_font_from_glyph_atlas(atlas: &GlyphAtlas) -> BitmapFont {
    let mut glyphs: Vec<BitmapFontGlyphData> = vec![];
    for __iteration0 in ((atlas.runtime.entries).clone()).iter().cloned() {
        let codepoint = __iteration0.0.clone();
        let entry = __iteration0.1.clone();
        glyphs.push(BitmapFontGlyphData {
            __flight_identity: std::sync::Arc::new(()),
            advance: entry.advance,
            bearing_x: entry.bearing_x,
            bearing_y: entry.bearing_y,
            codepoint: codepoint,
            height: entry.height,
            page: Some(0.0_f64),
            width: entry.width,
            x: entry.x,
            y: entry.y,
        });
    }
    let page = create_texture_atlas(Some(FlightPartialRecord2 {
        __flight_identity: std::sync::Arc::new(()),
        texture: Some(create_texture(Some(WgpuModifierContribution {
            __flight_identity: std::sync::Arc::new(()),
            source: (atlas.runtime.bitmap).clone(),
            declarations: None,
        }))),
        regions: None,
    }));
    return create_bitmap_font(&BitmapFontData {
        __flight_identity: std::sync::Arc::new(()),
        glyphs: (glyphs).clone(),
        metrics: ((atlas.runtime.metrics).clone()).clone(),
        pages: vec![(page).clone()],
        encoding: None,
        kerning: None,
    });
}
