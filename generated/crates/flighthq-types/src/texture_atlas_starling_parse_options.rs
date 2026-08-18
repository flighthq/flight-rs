// @generated from upstream/packages/types/src/TextureAtlasStarlingParseOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/TextureAtlasStarlingParseOptions.ts:1 (sha256:2ac90c2c6866b87895d0c19c21f0b9cf34a4de68f1985c0d5623c3ea1f3cc309)
#[derive(Clone, Default)]
pub struct TextureAtlasStarlingParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub image_width: Option<f64>,
    pub image_height: Option<f64>,
}
impl PartialEq for TextureAtlasStarlingParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
