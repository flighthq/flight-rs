// @generated from upstream/packages/types/src/Scene2DDocument.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AudioResourceReference, ImageResourceReference, Node2D};

// Source: upstream/packages/types/src/Scene2DDocument.ts:8 (sha256:a7c86693273661b05d43e72c4f1455b69c2c1dad088297bb43dea0620c6844c2)
#[derive(Clone, Default)]
pub struct Scene2DSlotReference {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub content: Option<Node2D>,
    pub linkage: Option<String>,
    pub name: String,
    pub required: bool,
    pub target: Node2D,
}
impl PartialEq for Scene2DSlotReference {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Scene2DDocument.ts:35 (sha256:758ff282b47808c9aeba2ad4b04730d34b2d24e146ce1fa0299027f8040e4b3a)
#[derive(Clone, Default)]
pub struct Scene2DDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub audio_resources: Vec<AudioResourceReference>,
    pub background_color: Option<f64>,
    pub image_resources: Vec<ImageResourceReference>,
    pub root: Node2D,
    pub slots: Vec<Scene2DSlotReference>,
    pub source_kind: Option<String>,
}
impl PartialEq for Scene2DDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
