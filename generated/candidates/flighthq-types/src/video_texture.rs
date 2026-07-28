// @generated from upstream/packages/types/src/VideoTexture.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Sampler, TextureColorSpace, Vector2, VideoResource};

// Source: upstream/packages/types/src/VideoTexture.ts:17 (sha256:7abb03c5c4db4ca292ed0fb8648bea6061f3639e0819d705324002234f974e73)
#[derive(Clone, Default)]
pub struct VideoTexture {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub uv_offset: Vector2,
    pub uv_rotation: f64,
    pub uv_scale: Vector2,
    pub color_space: TextureColorSpace,
    pub frame_id: f64,
    pub sampler: Sampler,
    pub source: VideoResource,
}
impl PartialEq for VideoTexture {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/VideoTexture.ts:27 (sha256:2281744b9a1e9db5ad929286a4d1a48862e74776af2e3ff4b67e2c3decc8b76e)
pub type VideoTextureLike = VideoTexture;
