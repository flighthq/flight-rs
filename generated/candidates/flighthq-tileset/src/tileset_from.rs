// @generated from upstream/packages/tileset/src/tilesetFrom.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{build_tileset_regions, create_tileset};
use flighthq_textureatlas::create_texture_atlas;
use flighthq_types::{ImageResource, TextureAtlas, TextureAtlasRegion, Tileset};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub image: Option<ImageResource>,
    pub regions: Option<Vec<TextureAtlasRegion>>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub height: Option<f64>,
    pub id: Option<f64>,
    pub name: Option<String>,
    pub original_height: Option<f64>,
    pub original_width: Option<f64>,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotated: Option<bool>,
    pub source_x: Option<f64>,
    pub source_y: Option<f64>,
    pub trimmed: Option<bool>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub atlas: Option<TextureAtlas>,
    pub columns: Option<f64>,
    pub margin: Option<f64>,
    pub rows: Option<f64>,
    pub spacing: Option<f64>,
    pub tile_height: Option<f64>,
    pub tile_width: Option<f64>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/tileset/src/tilesetFrom.ts:15 (sha256:43c5bd631788374771be58268993c22b525be98fee7cdbbaf3bcfa586ac11233)
pub fn create_tileset_from_atlas(
    atlas: &TextureAtlas,
    tile_width: f64,
    tile_height: f64,
    margin: Option<f64>,
    spacing: Option<f64>,
) -> Tileset {
    let margin = margin.unwrap_or(0.0_f64);
    let spacing = spacing.unwrap_or(0.0_f64);
    let image = (atlas.image).clone();
    let columns = if ((image).is_some()) && (tile_width > 0.0_f64) {
        (((image.as_ref().unwrap().width - (margin * 2.0_f64)) + spacing) / (tile_width + spacing))
            .floor()
    } else {
        0.0_f64
    };
    let rows = if ((image).is_some()) && (tile_height > 0.0_f64) {
        (((image.as_ref().unwrap().height - (margin * 2.0_f64)) + spacing)
            / (tile_height + spacing))
            .floor()
    } else {
        0.0_f64
    };
    let mut tileset = create_tileset(Some(FlightPartialRecord3 {
        __flight_identity: std::sync::Arc::new(()),
        atlas: Some((*atlas).clone()),
        columns: Some(columns),
        margin: Some(margin),
        rows: Some(rows),
        spacing: Some(spacing),
        tile_height: Some(tile_height),
        tile_width: Some(tile_width),
    }));
    build_tileset_regions(&mut tileset);
    return tileset;
}

// Source: upstream/packages/tileset/src/tilesetFrom.ts:32 (sha256:b95d60389c4f7fffd881a2d166a34b925fddd8f814072067ffd6edc298954926)
pub fn create_tileset_from_image_resource(
    resource: &ImageResource,
    tile_width: f64,
    tile_height: f64,
    margin: Option<f64>,
    spacing: Option<f64>,
) -> Tileset {
    let margin = margin.unwrap_or(0.0_f64);
    let spacing = spacing.unwrap_or(0.0_f64);
    return create_tileset_from_atlas(
        &create_texture_atlas(Some(FlightPartialRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            image: Some((*resource).clone()),
            regions: None,
        })),
        tile_width,
        tile_height,
        Some(margin),
        Some(spacing),
    );
}

// Source: upstream/packages/tileset/src/tilesetFrom.ts:42 (sha256:ace18fa5f17b2f42f62ad5efc04ce0728152e40de50ca09ad6d2f4e5fe2e982b)
pub fn load_tileset_from_base64(
    base64: String,
    mime_type: String,
    tile_width: f64,
    tile_height: f64,
    margin: Option<f64>,
    spacing: Option<f64>,
    signal: Option<crate::OpaqueHostValue>,
) -> crate::Promise<Tileset> {
    Default::default()
}

// Source: upstream/packages/tileset/src/tilesetFrom.ts:60 (sha256:36a767bf7e046eb72890abb39757391b694908b4556fb161934e8c759b4b8fa7)
pub fn load_tileset_from_blob(
    blob: crate::OpaqueHostValue,
    tile_width: f64,
    tile_height: f64,
    margin: Option<f64>,
    spacing: Option<f64>,
    signal: Option<crate::OpaqueHostValue>,
) -> crate::Promise<Tileset> {
    Default::default()
}

// Source: upstream/packages/tileset/src/tilesetFrom.ts:77 (sha256:3977b4718275eb6dd4585d28eebd7a18346623d8f300e108fbc55429e97473b1)
pub fn load_tileset_from_bytes(
    bytes: &Vec<u8>,
    tile_width: f64,
    tile_height: f64,
    margin: Option<f64>,
    spacing: Option<f64>,
    mime_type: Option<String>,
    signal: Option<crate::OpaqueHostValue>,
) -> crate::Promise<Tileset> {
    Default::default()
}

// Source: upstream/packages/tileset/src/tilesetFrom.ts:95 (sha256:a92858f561134da3392997d41a495f068693bf0f134207e5a37005b85e48d375)
pub fn load_tileset_from_url(
    url: String,
    tile_width: f64,
    tile_height: f64,
    margin: Option<f64>,
    spacing: Option<f64>,
    cross_origin: Option<String>,
    signal: Option<crate::OpaqueHostValue>,
) -> crate::Promise<Tileset> {
    Default::default()
}
