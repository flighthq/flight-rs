// @generated from upstream/packages/types/src/WgpuCompressedTextureUploader.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    CompressedImage, TextureColorSpace, WgpuCompressedTextureDecoder, WgpuRenderState,
    WgpuTextureEntry,
};

// Source: upstream/packages/types/src/WgpuCompressedTextureUploader.ts:7 (sha256:30a94992373a8884b7dac6983ddc14c13ebe4e8f5f150cd113a46137e1ea86ac)
pub type WgpuCompressedTextureUploader = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(
                    WgpuRenderState,
                    CompressedImage,
                    Option<WgpuCompressedTextureDecoder>,
                    Option<TextureColorSpace>,
                ) -> Option<WgpuTextureEntry>
                + Send
                + 'static,
        >,
    >,
>;
