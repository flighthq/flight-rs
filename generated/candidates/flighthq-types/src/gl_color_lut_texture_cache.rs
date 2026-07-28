// @generated from upstream/packages/types/src/GlColorLutTextureCache.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::ColorLut;

// Source: upstream/packages/types/src/GlColorLutTextureCache.ts:8 (sha256:28022486054f5704cc934e961f951937b26459388fd4d73a20763de668fb2500)
#[derive(Clone, Default)]
pub struct GlColorLutTextureCache {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub texture: Option<crate::OpaqueHostValue>,
    pub lut: Option<ColorLut>,
}
impl PartialEq for GlColorLutTextureCache {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
