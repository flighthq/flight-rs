// @generated from upstream/packages/types/src/ShapeJson.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Texture;

// Source: upstream/packages/types/src/ShapeJson.ts:8 (sha256:7bb7e99a82cb1bfb18350c67c8f1f23fdd4f3a957ab458781a28588924ab0ada)
#[derive(Clone, Default)]
pub struct ShapeTextureReference {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub index: f64,
}
impl PartialEq for ShapeTextureReference {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ShapeJson.ts:12 (sha256:8835de7663cbaa767b9bedaafb2dd5bc62e57f8140aefdf2f2f384c10b5b7f1a)
#[derive(Clone, Default)]
pub struct ShapeJsonFormatOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub space: Option<crate::FlightUnion2<f64, String>>,
}
impl PartialEq for ShapeJsonFormatOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ShapeJson.ts:17 (sha256:f5c0a5884f65a87413919eda74ad5555b49fcdd4a77566aa290a5daf29dfda2b)
#[derive(Clone, Default)]
pub struct ShapeJsonParseOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub resolve_texture: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(ShapeTextureReference) -> Option<Texture> + Send + 'static>,
            >,
        >,
    >,
}
impl PartialEq for ShapeJsonParseOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
