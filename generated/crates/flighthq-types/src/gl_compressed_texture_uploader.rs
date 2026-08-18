// @generated from upstream/packages/types/src/GlCompressedTextureUploader.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CompressedImage, GlCompressedTextureDecoder, TextureColorSpace};

// Source: upstream/packages/types/src/GlCompressedTextureUploader.ts:14 (sha256:9d7a1f772fd51dd2a0ce88fa6bc27bfd6808f7972eec022784099f51acd6228d)
pub type GlCompressedTextureUploader = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(
                    crate::OpaqueHostValue,
                    CompressedImage,
                    Option<GlCompressedTextureDecoder>,
                    Option<TextureColorSpace>,
                ) -> bool
                + Send
                + 'static,
        >,
    >,
>;
