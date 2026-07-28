// @generated from upstream/packages/types/src/WgpuColorLutTextureCache.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ColorLut;

// Source: upstream/packages/types/src/WgpuColorLutTextureCache.ts:8 (sha256:c598e5b3c7afe9486b6cb5b0debd608414646860ec136cf35c318b86b51ff1b1)
#[derive(Clone)]
pub struct WgpuColorLutTextureCache {
    pub texture: Option<crate::OpaqueHostValue>,
    pub size: f64,
    pub lut: Option<ColorLut>,
}
