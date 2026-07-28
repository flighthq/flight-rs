// @generated from upstream/packages/types/src/Texture.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ImageResource, Sampler, SceneResourceRef, Vector2};

// Source: upstream/packages/types/src/Texture.ts:11 (sha256:782486f0609aa06395ceefd5b4b2405fbe316447c2459f7669158fed78003f2a)
pub type TextureColorSpace = String;

// Source: upstream/packages/types/src/Texture.ts:19 (sha256:0258d751a90d1b64591856499e6b3ae95657bc6e33ee8be16ad96871bda50e91)
#[derive(Clone)]
pub struct Texture {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub uv_offset: Vector2,
    pub uv_rotation: f64,
    pub uv_scale: Vector2,
    pub color_space: TextureColorSpace,
    pub image: Option<ImageResource>,
    pub resource: Option<SceneResourceRef>,
    pub sampler: Sampler,
}
impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Texture.ts:30 (sha256:caeed47409a1f2d453e2b726cba270c79be92ac1040365ee21ee8820a9b03a50)
pub type TextureLike = Texture;
