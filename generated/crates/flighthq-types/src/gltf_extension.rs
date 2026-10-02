// @generated from upstream/packages/types/src/GltfExtension.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    GltfDocument, GltfTextureInfo, ImportDiagnostic, ImportDiagnosticSeverity, Scene3DDocument,
    Texture, TextureColorSpace, Transform3D,
};

// Source: upstream/packages/types/src/GltfExtension.ts:10 (sha256:0684b09047f181369bf955bc48d99f1c8ada3d0f8ed4b253a2518969279b328f)
#[derive(Clone, Default)]
pub struct GltfAccessorData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub count: f64,
    pub data: Vec<f64>,
    pub fault: Option<GltfAccessorFault>,
}
impl PartialEq for GltfAccessorData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfExtension.ts:16 (sha256:150914b7e1bff322d00b42934a5c26eb08cf55c57396b48b03886e058d8d0819)
#[derive(Clone, Default)]
pub struct GltfAccessorFault {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub detail: Vec<(String, f64)>,
    pub kind: String,
}
impl PartialEq for GltfAccessorFault {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfExtension.ts:25 (sha256:bedb41b322c5a730846fa18ffed6739f58303d40a02287c231c1678ac9e8370a)
#[derive(Clone)]
pub struct GltfCoreFeatureContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub build_node_transform:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> Transform3D + Send + 'static>>>,
    pub document: Scene3DDocument,
    pub mesh_indices: Vec<Vec<f64>>,
    pub node_indices: Vec<f64>,
    pub primitive_node_indices: Vec<Vec<f64>>,
    pub read_accessor: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(f64, Option<String>) -> GltfAccessorData + Send + 'static>>,
    >,
    pub report_accessor_fault: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(ImportDiagnosticSeverity, GltfAccessorFault) -> () + Send + 'static>,
        >,
    >,
    pub report_diagnostic: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        ImportDiagnosticSeverity,
                        String,
                        Option<
                            Vec<(
                                String,
                                crate::FlightUnion2<bool, crate::FlightUnion2<f64, String>>,
                            )>,
                        >,
                        Option<String>,
                    ) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub source: GltfDocument,
}
impl PartialEq for GltfCoreFeatureContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfExtension.ts:45 (sha256:f06debacd309cab035feeb13aa3ee987639519b6b8aee0191e40893e49a33108)
#[derive(Clone)]
pub struct GltfCoreFeatureHandler {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub apply: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(GltfCoreFeatureContext) -> () + Send + 'static>>,
    >,
    pub kind: String,
}
impl PartialEq for GltfCoreFeatureHandler {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfExtension.ts:61 (sha256:72d72c06dea8c04621bf147d5fdcb331914d9289ae8d7a05fbb41363b6eb77ab)
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

// Source: upstream/packages/types/src/GltfExtension.ts:72 (sha256:c2e8e437711ca8e9cafe6cda0b0ff7bc03862e428d0aee8fe62c33b2fe54c9b9)
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

// Source: upstream/packages/types/src/GltfExtension.ts:80 (sha256:8d58d9bced321871b7046bb11a22e853fb7b5fe0ff38cf38366f6f19239f8092)
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
