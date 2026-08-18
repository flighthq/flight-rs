// @generated from upstream/packages/types/src/SwfDocumentImport.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AdvancedBlendMode, Node2D, RenderEffect, Scene2DDocument};

// Source: upstream/packages/types/src/SwfDocumentImport.ts:11 (sha256:8d0ce660add6621b14fc26492f2764fb7d8d6f7b7f4464ee00d44cf0b24878be)
#[derive(Clone, Default)]
pub struct SwfDocumentImport {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub appearances: Vec<SwfNodeAppearance>,
    pub document: Scene2DDocument,
}
impl PartialEq for SwfDocumentImport {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SwfDocumentImport.ts:29 (sha256:29d7b6847fd6dc3ce23219c04ed1eb97b80248c7c2185f4caef270d9d52afd98)
#[derive(Clone, Default)]
pub struct SwfNodeAppearance {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub advanced_blend_mode: Option<AdvancedBlendMode>,
    pub effects: Vec<RenderEffect>,
    pub frame: f64,
    pub node: Node2D,
}
impl PartialEq for SwfNodeAppearance {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
