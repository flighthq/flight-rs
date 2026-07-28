// @generated from upstream/packages/textureatlas/src/textureAtlas.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_image::get_image_resource_byte_size;
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

// Source: upstream/packages/textureatlas/src/textureAtlas.ts:5 (sha256:43a064b3237a5e6192cb3831e8e1cc289f21f72f4f42c59f6b91232ccaf1aca6)
pub fn create_texture_atlas(obj: Option<FlightPartialRecord1>) -> TextureAtlas {
    return create_entity(Some(TextureAtlas {
        __flight_identity: std::sync::Arc::new(()),
        image: obj.as_ref().and_then(|value| (value.image).clone()),
        regions: (obj.as_ref().and_then(|value| (value.regions).clone())).unwrap_or(vec![]),
    }));
}

// Source: upstream/packages/textureatlas/src/textureAtlas.ts:14 (sha256:b90a1343b858142ac7dc18717a32a93337d11cbe854f7beca6aaa7ecb1e8d693)
pub fn get_texture_atlas_byte_size(atlas: &TextureAtlas) -> f64 {
    return if ((atlas.image).clone()).is_some() {
        get_image_resource_byte_size(atlas.image.as_ref().unwrap())
    } else {
        0.0_f64
    };
}
