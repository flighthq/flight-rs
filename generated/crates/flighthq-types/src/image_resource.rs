// @generated from upstream/packages/types/src/ImageResource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use crate::{AlphaType, ImageResourceCompressed, PixelFormat};

// Source: upstream/packages/types/src/ImageResource.ts:18 (sha256:e28dc5618fbdb55d16b93f846822b1c8f71235c796f967cfb1ae7ef45b99657c)
#[derive(Clone)]
pub struct ImageResource {
    pub alpha_type: AlphaType,
    pub compressed: Option<ImageResourceCompressed>,
    pub data: Option<Vec<u8>>,
    pub format: PixelFormat,
    pub height: f64,
    pub source: Option<crate::OpaqueHostValue>,
    pub version: f64,
    pub width: f64,
}
