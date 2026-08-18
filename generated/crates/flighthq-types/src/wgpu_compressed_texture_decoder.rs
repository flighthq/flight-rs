// @generated from upstream/packages/types/src/WgpuCompressedTextureDecoder.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::TextureContainerFormat;

// Source: upstream/packages/types/src/WgpuCompressedTextureDecoder.ts:6 (sha256:1b1cdb6989bf20ff578374090571f8131f4e4ddd309808e6dce55110ac5c0da0)
pub type WgpuCompressedTextureDecoder = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(TextureContainerFormat, f64, f64, Vec<u8>) -> Option<Vec<u8>>
                + Send
                + 'static,
        >,
    >,
>;
