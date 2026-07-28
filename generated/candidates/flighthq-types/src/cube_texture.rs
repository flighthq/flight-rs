// @generated from upstream/packages/types/src/CubeTexture.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ImageResource, Sampler, TextureColorSpace};

// Source: upstream/packages/types/src/CubeTexture.ts:9 (sha256:305f6d5cc9718e946c74fe51927273973a9feb0a5cd5fc2d53b823c9d295f151)
#[derive(Clone)]
pub struct CubeTexture {
    pub color_space: TextureColorSpace,
    pub faces: Vec<Option<ImageResource>>,
    pub sampler: Sampler,
}

// Source: upstream/packages/types/src/CubeTexture.ts:15 (sha256:0251690304ad56f19c5f5d38d7793b76642b7b9b547354fbd4e7650e06f6539c)
pub type CubeTextureLike = CubeTexture;

// Source: upstream/packages/types/src/CubeTexture.ts:19 (sha256:f5cab7dc993c7a722e9d8280368181df0dea775398ff282b70e61deb20fb39ff)
pub const CUBE_FACE_POSITIVE_X: f64 = 0.0_f64;

// Source: upstream/packages/types/src/CubeTexture.ts:20 (sha256:f6210211a3886e230f512f3e9484f384c0cbbccf3cd379b1528c58a4bf494e4b)
pub const CUBE_FACE_NEGATIVE_X: f64 = 1.0_f64;

// Source: upstream/packages/types/src/CubeTexture.ts:21 (sha256:dea28e704fe13da8e35756c30c5f9fb1e563913b870a0c69178bfda30995bc04)
pub const CUBE_FACE_POSITIVE_Y: f64 = 2.0_f64;

// Source: upstream/packages/types/src/CubeTexture.ts:22 (sha256:f6843c561f4d33f768c606a3744565a0d91861f527e3e59cff508c847b89aca6)
pub const CUBE_FACE_NEGATIVE_Y: f64 = 3.0_f64;

// Source: upstream/packages/types/src/CubeTexture.ts:23 (sha256:6f0b4df77c214a795198e95605a407e69a7ed90d40a2a34f757be0f7c4752762)
pub const CUBE_FACE_POSITIVE_Z: f64 = 4.0_f64;

// Source: upstream/packages/types/src/CubeTexture.ts:24 (sha256:64886f26b627f32a329fa98d699f096c9e299c7e21613330215eeeabd806f826)
pub const CUBE_FACE_NEGATIVE_Z: f64 = 5.0_f64;
