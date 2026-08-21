// @generated from upstream/packages/types/src/SwfDocumentImport.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AdvancedBlendMode, EmbeddedImageResourceReference, Node2D, RenderEffect, Scene2DDocument,
};

// Source: upstream/packages/types/src/SwfDocumentImport.ts:12 (sha256:a5e80429b2d12890711ae06693fc29fc09675e8b56cdedee4c1da28b555d53c5)
#[derive(Clone, Default)]
pub struct SwfDocumentImport {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub appearances: Vec<SwfNodeAppearance>,
    pub document: Scene2DDocument,
    pub jpeg_alpha_payloads: Vec<SwfJpegAlphaPayload>,
}
impl PartialEq for SwfDocumentImport {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SwfDocumentImport.ts:24 (sha256:d1fe2e41c49362dce579804a7d9a43c02b30d00e6d0d3e3575eb3433fd4e3f6d)
#[derive(Clone, Default)]
pub struct SwfJpegAlphaPayload {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub character_id: f64,
    pub compressed_alpha_bytes: Vec<u8>,
    pub deblocking_parameter_raw: Option<f64>,
    pub height: f64,
    pub reference: EmbeddedImageResourceReference,
    pub width: f64,
}
impl PartialEq for SwfJpegAlphaPayload {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SwfDocumentImport.ts:46 (sha256:29d7b6847fd6dc3ce23219c04ed1eb97b80248c7c2185f4caef270d9d52afd98)
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
