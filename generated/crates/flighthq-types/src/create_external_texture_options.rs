// @generated from upstream/packages/types/src/CreateExternalTextureOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{SamplerLike, TextureColorSpace};

// Source: upstream/packages/types/src/CreateExternalTextureOptions.ts:4 (sha256:0c6e30c09f9b1aa220dd099bab1745b568ae3029b443ad4d3c91a0af35b21d56)
#[derive(Clone, Default)]
pub struct CreateExternalTextureOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub color_space: Option<TextureColorSpace>,
    pub height: f64,
    pub sampler: Option<SamplerLike>,
    pub width: f64,
}
impl PartialEq for CreateExternalTextureOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
