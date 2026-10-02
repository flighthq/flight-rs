// @generated from upstream/packages/types/src/GlCompressedTextureUploader.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CompressedImageResource, GlCompressedTextureDecoder, GlContext, TextureColorSpace};

// Source: upstream/packages/types/src/GlCompressedTextureUploader.ts:15 (sha256:dd2143e919eb7475e9f998ed6fcc85463e7fff9954fe6334617e66d4f9b601e6)
pub type GlCompressedTextureUploader = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(
                    GlContext,
                    CompressedImageResource,
                    Option<GlCompressedTextureDecoder>,
                    Option<TextureColorSpace>,
                ) -> bool
                + Send
                + 'static,
        >,
    >,
>;
