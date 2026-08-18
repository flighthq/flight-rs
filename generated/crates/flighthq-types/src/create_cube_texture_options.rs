// @generated from upstream/packages/types/src/CreateCubeTextureOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{SamplerLike, TextureColorSpace, TextureSourceCubeFaces};

// Source: upstream/packages/types/src/CreateCubeTextureOptions.ts:4 (sha256:c811f99c1d699ef25b79b8cb7ec17d9151a8d319cdbdae530b1180a52a24a713)
#[derive(Clone, Default)]
pub struct CreateCubeTextureOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color_space: Option<TextureColorSpace>,
    pub sampler: Option<SamplerLike>,
    pub sources: Option<TextureSourceCubeFaces>,
}
impl PartialEq for CreateCubeTextureOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
