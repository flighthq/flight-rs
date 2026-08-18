// @generated from upstream/packages/types/src/TextureAtlasRegionTextureExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TextureAtlas;

// Source: upstream/packages/types/src/TextureAtlasRegionTextureExplanation.ts:3 (sha256:fcf653bc9f01bdfd81ed8563342265dc8924f7688f9546fc21a54a307f61ede2)
pub type TextureAtlasRegionTextureStatus = String;

// Source: upstream/packages/types/src/TextureAtlasRegionTextureExplanation.ts:5 (sha256:bdd0a70c87747fd9345b947bb95318b40437f1e3f7c4c0aff890566d7383653d)
#[derive(Clone, Default)]
pub struct TextureAtlasRegionTextureExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub status: TextureAtlasRegionTextureStatus,
}
impl PartialEq for TextureAtlasRegionTextureExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TextureAtlasRegionTextureExplanation.ts:9 (sha256:7929354cee089415a0aa0232fd70290922dc4e0c7e293d4b2eabaf33783a5f1e)
pub type TextureAtlasRegionTextureGuard = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(TextureAtlas, f64, TextureAtlasRegionTextureExplanation) -> ()
                + Send
                + 'static,
        >,
    >,
>;
