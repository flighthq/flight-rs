// @generated from upstream/packages/types/src/ImageDecoder.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{DecodedImage, ImageDecodeOptions};

// Source: upstream/packages/types/src/ImageDecoder.ts:7 (sha256:b4e055e42ce63f2aeae225ea97b2b498d578319f13572789222cc15b6c9fbfc0)
pub type ImageDecoder = std::sync::Arc<
    dyn Fn(Vec<u8>, ImageDecodeOptions) -> crate::Promise<DecodedImage> + Send + Sync + 'static,
>;
