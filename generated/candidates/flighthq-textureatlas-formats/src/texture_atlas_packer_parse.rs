// @generated from upstream/packages/textureatlas-formats/src/textureAtlasPackerParse.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    TextureAtlasPackerArrayFrame, TextureAtlasPackerDocument, TextureAtlasPackerHashFrame,
};
use flighthq_textureatlas::create_texture_atlas_region;
use flighthq_types::TextureAtlas;

#[derive(Clone, Default)]
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

// Source: upstream/packages/textureatlas-formats/src/textureAtlasPackerParse.ts:10 (sha256:1be5adb06eca1d693db5886cf34c1358c86298097fc1de5452c9df16d41ba319)
#[derive(Clone, Default)]
pub struct TextureAtlasPackerParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub strip_path_prefix: Option<bool>,
}
impl PartialEq for TextureAtlasPackerParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasPackerParse.ts:16 (sha256:4df5333999f12b08cb2e8d5f1e32b88b95d9b5e4adf4ebd566e44463b4afaa99)
#[derive(Clone, Default)]
struct ParseTextureAtlasPackerDocumentRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for ParseTextureAtlasPackerDocumentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn parse_texture_atlas_packer_document(
    doc: &TextureAtlasPackerDocument,
    atlas: &mut TextureAtlas,
    options: Option<TextureAtlasPackerParseOptions>,
) -> TextureAtlas {
    apply_document(
        atlas,
        doc,
        &(options).unwrap_or(TextureAtlasPackerParseOptions {
            __flight_identity: std::sync::Arc::new(()),
            strip_path_prefix: None,
        }),
    );
    return atlas.clone();
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasPackerParse.ts:28 (sha256:d212a2bf01a3b0aee8c5a0d896d300731f4481c13c36050284ec7ea494d4b17e)
#[derive(Clone, Default)]
struct ParseTextureAtlasPackerJsonRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for ParseTextureAtlasPackerJsonRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn parse_texture_atlas_packer_json(
    json: String,
    atlas: &mut TextureAtlas,
    options: Option<TextureAtlasPackerParseOptions>,
) -> TextureAtlas {
    let doc = (json.parse)(json);
    apply_document(
        atlas,
        &doc,
        &(options).unwrap_or(TextureAtlasPackerParseOptions {
            __flight_identity: std::sync::Arc::new(()),
            strip_path_prefix: None,
        }),
    );
    return atlas.clone();
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasPackerParse.ts:40 (sha256:2a3dbdab62fa1fb4558c29da73d1dfbb0a30d3616140659c13c6ef97e59e9ee2)
fn apply_document(
    atlas: &mut TextureAtlas,
    doc: &TextureAtlasPackerDocument,
    options: &TextureAtlasPackerParseOptions,
) -> () {
    atlas.regions.clear();
    if false {
        for entry in (doc.frames).iter().cloned() {
            apply_frame(atlas, (entry.filename).clone(), &(entry), options);
        }
    } else {
        for __iteration0 in (crate::host_value::<()>("host.entries")).iter().cloned() {
            let frame_name = __iteration0[0.0_f64 as usize].clone();
            let entry = __iteration0[1.0_f64 as usize].clone();
            apply_frame(atlas, frame_name, &(entry), options);
        }
    }
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasPackerParse.ts:57 (sha256:571f6b2ce31bc9531693f6f75013e8416ac9d52c20cbf1462ef5790f1f41cc73)
fn apply_frame(
    atlas: &mut TextureAtlas,
    name: String,
    entry: &crate::FlightUnion2<TextureAtlasPackerArrayFrame, TextureAtlasPackerHashFrame>,
    options: &TextureAtlasPackerParseOptions,
) -> () {
    let normalized =
        normalize_frame_name((name).clone(), (options.strip_path_prefix).unwrap_or(false));
    let region = create_texture_atlas_region(Some(FlightPartialRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        height: Some(if entry.rotated {
            entry.frame.w
        } else {
            entry.frame.h
        }),
        id: Some((atlas.regions.len() as f64)),
        name: Some((normalized).clone()),
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
        pivot_x: Some(if (entry.pivot).is_some() {
            entry.pivot.x
        } else {
            None
        }),
        pivot_y: Some(if (entry.pivot).is_some() {
            entry.pivot.y
        } else {
            None
        }),
        rotated: Some(entry.rotated),
        source_x: Some(entry.sprite_source_size.x),
        source_y: Some(entry.sprite_source_size.y),
        trimmed: Some(entry.trimmed),
        width: Some(if entry.rotated {
            entry.frame.h
        } else {
            entry.frame.w
        }),
        x: Some(entry.frame.x),
        y: Some(entry.frame.y),
    }));
    atlas.regions.push(((region).clone()).clone());
}

// Source: upstream/packages/textureatlas-formats/src/textureAtlasPackerParse.ts:84 (sha256:0ef2ffcc9fea7d6c4cda4e7c1cdb633c489854fab3de18753ac48f8b86281bca)
fn normalize_frame_name(name: String, strip: bool) -> String {
    if (!strip) {
        return name;
    }
    let slash = ((name.last_index_of)("/")).max((name.last_index_of)("\\"));
    return if (slash >= 0.0_f64) {
        String::from_utf16_lossy(
            &(name)
                .encode_utf16()
                .skip((slash + 1.0_f64) as usize)
                .collect::<Vec<u16>>(),
        )
    } else {
        (name).clone()
    };
}
