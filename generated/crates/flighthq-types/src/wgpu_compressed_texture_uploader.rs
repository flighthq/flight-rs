// @generated from upstream/packages/types/src/WgpuCompressedTextureUploader.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    CompressedImageResource, TextureColorSpace, WgpuCompressedTextureDecoder, WgpuRenderState,
    WgpuTextureEntry,
};

// Source: upstream/packages/types/src/WgpuCompressedTextureUploader.ts:7 (sha256:4202c467f93912b113d7e9df973c5bc08c1c6c510553abdbffbb9d981289b85b)
pub type WgpuCompressedTextureUploader = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(
                    WgpuRenderState,
                    CompressedImageResource,
                    Option<WgpuCompressedTextureDecoder>,
                    Option<TextureColorSpace>,
                ) -> Option<WgpuTextureEntry>
                + Send
                + 'static,
        >,
    >,
>;
