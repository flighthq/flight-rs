// @generated from upstream/packages/types/src/SwfDocumentImport.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AdvancedBlendMode, Effect, EmbeddedImageResourceReference, EntityRuntime, Node2D,
    Scene2DDocument,
};

// Source: upstream/packages/types/src/SwfDocumentImport.ts:13 (sha256:e4e33c9a4c3541301729a15cb430a8f6d78fc50b3e2ad58da09ad170a4583d37)
#[derive(Clone, Default)]
pub struct SwfDocumentImport {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub appearances: Vec<SwfNodeAppearance>,
    pub document: Scene2DDocument,
    pub jpeg_alpha_payloads: Vec<SwfJpegAlphaPayload>,
}
impl PartialEq for SwfDocumentImport {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for SwfDocumentImport {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/SwfDocumentImport.ts:25 (sha256:d1fe2e41c49362dce579804a7d9a43c02b30d00e6d0d3e3575eb3433fd4e3f6d)
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

// Source: upstream/packages/types/src/SwfDocumentImport.ts:47 (sha256:e1dbc46d65e6b36793ea9ea6ced9ee7e48676c0184375be01c413d70f3e5b899)
#[derive(Clone, Default)]
pub struct SwfNodeAppearance {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub advanced_blend_mode: Option<AdvancedBlendMode>,
    pub effects: Vec<Effect>,
    pub frame: f64,
    pub node: Node2D,
}
impl PartialEq for SwfNodeAppearance {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
