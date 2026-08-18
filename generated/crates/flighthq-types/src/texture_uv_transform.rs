// @generated from upstream/packages/types/src/TextureUvTransform.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Vector2;

// Source: upstream/packages/types/src/TextureUvTransform.ts:13 (sha256:e4f1a08e4ac64e7cadca1d54e3ee9dca2556c520ef326531ed1fff222705f2d0)
#[derive(Clone, Default)]
pub struct TextureUvTransform {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub flip_x: bool,
    pub flip_y: bool,
    pub uv_offset: Vector2,
    pub uv_rotation: f64,
    pub uv_scale: Vector2,
}
impl PartialEq for TextureUvTransform {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
