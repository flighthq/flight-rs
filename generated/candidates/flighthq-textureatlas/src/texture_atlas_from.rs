// @generated from upstream/packages/textureatlas/src/textureAtlasFrom.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_texture_atlas;
use flighthq_image::{
    create_image_resource_from_canvas, create_image_resource_from_image_bitmap,
    create_image_resource_from_image_element,
};
use flighthq_types::{ImageResource, TextureAtlas, TextureAtlasRegion};

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

// Source: upstream/packages/textureatlas/src/textureAtlasFrom.ts:14 (sha256:2c190730bb4fba1d10492c4cbcb3aaeb16d4f0e890b61fe7cd3c6e80ae8cfc0b)
pub fn create_texture_atlas_from_canvas(canvas: crate::OpaqueHostValue) -> TextureAtlas {
    return create_texture_atlas(Some(FlightPartialRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        image: Some(create_image_resource_from_canvas((canvas).clone())),
        regions: None,
    }));
}

// Source: upstream/packages/textureatlas/src/textureAtlasFrom.ts:18 (sha256:3b42769f553dcfcfeef983e524505cf5aa8affef4847543f2e61eb09bce12831)
pub fn create_texture_atlas_from_image_bitmap(bitmap: crate::OpaqueHostValue) -> TextureAtlas {
    return create_texture_atlas(Some(FlightPartialRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        image: Some(create_image_resource_from_image_bitmap((bitmap).clone())),
        regions: None,
    }));
}

// Source: upstream/packages/textureatlas/src/textureAtlasFrom.ts:22 (sha256:928ae507b545db49854a065125fbc20113fd27e4292240ab8dc98aea0b83b5c1)
pub fn create_texture_atlas_from_image_element(img: crate::OpaqueHostValue) -> TextureAtlas {
    return create_texture_atlas(Some(FlightPartialRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        image: Some(create_image_resource_from_image_element((img).clone())),
        regions: None,
    }));
}

// Source: upstream/packages/textureatlas/src/textureAtlasFrom.ts:26 (sha256:9823f8f0ae125e78729eeb0017c6a8fa72e96360c6b58627d03925286b222372)
pub fn create_texture_atlas_from_image_resource(resource: &ImageResource) -> TextureAtlas {
    return create_texture_atlas(Some(FlightPartialRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        image: Some((*resource).clone()),
        regions: None,
    }));
}

// Source: upstream/packages/textureatlas/src/textureAtlasFrom.ts:30 (sha256:c3b8831bbb577f82b4ecd77ec700c8eb9c03989c66a46b55c9a8dc00a57228c4)
pub fn load_texture_atlas_from_base64(
    base64: String,
    mime_type: String,
    signal: Option<crate::OpaqueHostValue>,
) -> crate::Promise<TextureAtlas> {
    Default::default()
}

// Source: upstream/packages/textureatlas/src/textureAtlasFrom.ts:38 (sha256:7f84b1d162a9f648364d02ce2871669e570676045f31e732203694a2ca574321)
pub fn load_texture_atlas_from_blob(
    blob: crate::OpaqueHostValue,
    signal: Option<crate::OpaqueHostValue>,
) -> crate::Promise<TextureAtlas> {
    Default::default()
}

// Source: upstream/packages/textureatlas/src/textureAtlasFrom.ts:42 (sha256:6ac6dc7033bfcd7d150920334b8cc158aa2fd830661a0084495ba72fdc98574b)
pub fn load_texture_atlas_from_bytes(
    bytes: &Vec<u8>,
    mime_type: Option<String>,
    signal: Option<crate::OpaqueHostValue>,
) -> crate::Promise<TextureAtlas> {
    Default::default()
}

// Source: upstream/packages/textureatlas/src/textureAtlasFrom.ts:50 (sha256:e7a6e6d985a77feeffa76d7f124f2033cf98759af424baec6ff02000912842ad)
pub fn load_texture_atlas_from_url(
    url: String,
    cross_origin: Option<String>,
    signal: Option<crate::OpaqueHostValue>,
) -> crate::Promise<TextureAtlas> {
    Default::default()
}
