// @generated from upstream/packages/textureatlas-formats/src/textureAtlasAsepriteParse.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    TextureAtlasAsepriteArrayFrame, TextureAtlasAsepriteBaseFrame, TextureAtlasAsepriteDocument,
};
use flighthq_textureatlas::create_texture_atlas_region;
use flighthq_types::TextureAtlas;

#[derive(Clone)]
pub struct FlightPartialRecord1 {
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
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasAsepriteParse.ts:11 (sha256:e7f7154531cfbfdd77a10bf579fb1cb43c21adcfd53ebf3869423b485477a68b)
pub fn parse_texture_atlas_aseprite_document(
    doc: &TextureAtlasAsepriteDocument,
    atlas: &mut TextureAtlas,
) -> TextureAtlas {
    atlas.regions.clear();
    if (array.is_array)(doc.frames) {
        for entry in (doc.frames).iter().cloned() {
            apply_aseprite_frame(atlas, (entry.filename).clone(), &(entry));
        }
    } else {
        for __iteration0 in (crate::host_value::<()>("host.entries")).iter().cloned() {
            let frame_name = __iteration0[0.0_f64 as usize].clone();
            let entry = __iteration0[1.0_f64 as usize].clone();
            apply_aseprite_frame(atlas, frame_name, &(entry));
        }
    }
    return atlas.clone();
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasAsepriteParse.ts:31 (sha256:49dcbbc6245fd1e0d318f6acfad2a976025593fc803a89171c151b62f2f04fa8)
pub fn parse_texture_atlas_aseprite_json(json: String, atlas: &mut TextureAtlas) -> TextureAtlas {
    let doc = (json.parse)(json);
    return parse_texture_atlas_aseprite_document(&doc, atlas);
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasAsepriteParse.ts:36 (sha256:460aec93ddeca4df3fdd471390faa7095abfcf9431f8d999ccafc0abe05ecca6)
fn apply_aseprite_frame(
    atlas: &mut TextureAtlas,
    name: String,
    entry: &crate::FlightUnion2<TextureAtlasAsepriteArrayFrame, TextureAtlasAsepriteBaseFrame>,
) -> () {
    atlas
        .regions
        .push(create_texture_atlas_region(Some(FlightPartialRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            height: Some(entry.frame.h),
            id: Some((atlas.regions.len() as f64)),
            name: Some((name).clone()),
            original_height: Some(if entry.trimmed {
                entry.source_size.h
            } else {
                None
            }),
            original_width: Some(if entry.trimmed {
                entry.source_size.w
            } else {
                None
            }),
            pivot_x: None,
            pivot_y: None,
            rotated: Some(entry.rotated),
            source_x: Some(entry.sprite_source_size.x),
            source_y: Some(entry.sprite_source_size.y),
            trimmed: Some(entry.trimmed),
            width: Some(entry.frame.w),
            x: Some(entry.frame.x),
            y: Some(entry.frame.y),
        })));
}
