// @generated from upstream/packages/types/src/TextureUvTransform.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Vector2;

// Source: upstream/packages/types/src/TextureUvTransform.ts:8 (sha256:c7d84fc63c4e42f7111351d57c474737b772fb184ba51740fbace05a822640f6)
#[derive(Clone)]
pub struct TextureUvTransform {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub uv_offset: Vector2,
    pub uv_rotation: f64,
    pub uv_scale: Vector2,
}
impl PartialEq for TextureUvTransform {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
