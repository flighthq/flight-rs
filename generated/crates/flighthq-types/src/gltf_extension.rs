// @generated from upstream/packages/types/src/GltfExtension.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GltfDocument, GltfTextureInfo, ImportDiagnostic, Scene3DDocument, Texture, TextureColorSpace,
    Transform3D,
};

// Source: upstream/packages/types/src/GltfExtension.ts:18 (sha256:72d72c06dea8c04621bf147d5fdcb331914d9289ae8d7a05fbb41363b6eb77ab)
#[derive(Clone)]
pub struct GltfExtensionContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub build_node_transform:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> Transform3D + Send + 'static>>>,
    pub diagnostics: Option<Vec<ImportDiagnostic>>,
    pub document: Scene3DDocument,
    pub node_indices: Vec<f64>,
    pub resolve_texture: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Option<GltfTextureInfo>, TextureColorSpace) -> Option<Texture>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub source: GltfDocument,
}
impl PartialEq for GltfExtensionContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfExtension.ts:29 (sha256:c2e8e437711ca8e9cafe6cda0b0ff7bc03862e428d0aee8fe62c33b2fe54c9b9)
#[derive(Clone)]
pub struct GltfExtensionHandler {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub apply: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(GltfExtensionContext) -> () + Send + 'static>>,
    >,
    pub kind: String,
}
impl PartialEq for GltfExtensionHandler {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfExtension.ts:37 (sha256:8d58d9bced321871b7046bb11a22e853fb7b5fe0ff38cf38366f6f19239f8092)
#[derive(Clone, Default)]
pub struct GltfImportOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub base_path: Option<String>,
    pub extension_handlers: Option<Vec<GltfExtensionHandler>>,
    pub external_buffers: Option<Vec<(String, Vec<f64>)>>,
}
impl PartialEq for GltfImportOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
