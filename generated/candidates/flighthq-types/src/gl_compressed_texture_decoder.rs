// @generated from upstream/packages/types/src/GlCompressedTextureDecoder.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TextureContainerFormat;

// Source: upstream/packages/types/src/GlCompressedTextureDecoder.ts:10 (sha256:3c6b7a0683a84c8c526c00a47a68faac566c8305a799c958e3f920bceead2784)
pub type GlCompressedTextureDecoder = std::sync::Arc<
    dyn Fn(TextureContainerFormat, f64, f64, Vec<u8>) -> Option<Vec<u8>> + Send + Sync + 'static,
>;
