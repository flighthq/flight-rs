// @generated from upstream/packages/types/src/Md2SectionHandler.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ImportDiagnostic, MeshMorph, Scene3DDocument};

// Source: upstream/packages/types/src/Md2SectionHandler.ts:13 (sha256:093e1728b043e64f8815e6054a020cc432e0e256078fd358c0d37ede1d98d104)
#[derive(Clone, Default)]
pub struct Md2Frame {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub name: String,
    pub normals: Vec<f32>,
    pub positions: Vec<f32>,
}
impl PartialEq for Md2Frame {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Md2SectionHandler.ts:27 (sha256:b7d7bda5e9e582e745c56691c0f05f516792d877ff23ec66e6852c75e6cb6f4c)
#[derive(Clone, Default)]
pub struct Md2ParseContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bytes: Vec<u8>,
    pub diagnostics: Option<Vec<ImportDiagnostic>>,
    pub document: Scene3DDocument,
    pub frames: Vec<Md2Frame>,
    pub mesh_materials: Vec<f64>,
    pub morph: Option<MeshMorph>,
    pub num_skins: f64,
    pub off_skins: f64,
}
impl PartialEq for Md2ParseContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Md2SectionHandler.ts:62 (sha256:4da8858235fe121d932252ed8a33b2989e32f4ab3eeeb8b0212ba634ed6948e7)
#[derive(Clone)]
pub struct Md2SectionHandler {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub feature: String,
    pub collect:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Md2ParseContext) -> () + Send + 'static>>>,
}
impl PartialEq for Md2SectionHandler {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Md2SectionHandler.ts:75 (sha256:4da56778b334cff53589508bbd2576b824ea360df632f8da3db959bb9020219c)
#[derive(Clone, Default)]
pub struct Md2ImportOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub section_handlers: Option<Vec<Md2SectionHandler>>,
}
impl PartialEq for Md2ImportOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
