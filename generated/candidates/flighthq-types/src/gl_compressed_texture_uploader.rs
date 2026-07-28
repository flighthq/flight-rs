// @generated from upstream/packages/types/src/GlCompressedTextureUploader.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlCompressedTextureDecoder, ImageResource};

// Source: upstream/packages/types/src/GlCompressedTextureUploader.ts:13 (sha256:050c2cbf39a926bb954d62f68f47f7fd326904b2c2ee683e8bdb5048c732e3d6)
pub type GlCompressedTextureUploader = std::sync::Arc<
    dyn Fn(crate::OpaqueHostValue, ImageResource, Option<GlCompressedTextureDecoder>) -> bool
        + Send
        + Sync
        + 'static,
>;
