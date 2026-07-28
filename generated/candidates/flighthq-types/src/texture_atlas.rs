// @generated from upstream/packages/types/src/TextureAtlas.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ImageResource, TextureAtlasRegion};

// Source: upstream/packages/types/src/TextureAtlas.ts:5 (sha256:f215053afab83d6cdfa985eb7c2bc5cd2fb0a3433082d3536dca6f3c923cd03c)
#[derive(Clone)]
pub struct TextureAtlas {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub image: Option<ImageResource>,
    pub regions: Vec<TextureAtlasRegion>,
}
impl PartialEq for TextureAtlas {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
