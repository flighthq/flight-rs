// @generated from upstream/packages/types/src/GlTextureDescriptor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlTextureFilterKind, GlTextureWrapKind};

// Source: upstream/packages/types/src/GlTextureDescriptor.ts:3 (sha256:0d3187bca3d7c678f1484e6aedbf610a1a6951193c0c51e8000528cf347725f2)
#[derive(Clone)]
pub struct GlTextureDescriptor {
    pub wrap_s: Option<GlTextureWrapKind>,
    pub wrap_t: Option<GlTextureWrapKind>,
    pub min_filter: Option<GlTextureFilterKind>,
    pub mag_filter: Option<GlTextureFilterKind>,
    pub mipmaps: Option<bool>,
    pub anisotropy: Option<f64>,
    pub premultiply_alpha: Option<bool>,
    pub format: Option<GlTextureInternalFormat>,
}

// Source: upstream/packages/types/src/GlTextureDescriptor.ts:13 (sha256:361c687d58381b667145ffe38b03ff3899a72ec2a5538faf05539f40d4e473c3)
pub type GlTextureInternalFormat = String;
