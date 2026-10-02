// @generated from upstream/packages/types/src/ThreeDsChunkHandler.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    ImportDiagnostic, Scene3DDocument, ThreeDsCamera, ThreeDsDropTally, ThreeDsLight,
    ThreeDsMaterial, ThreeDsMesh,
};

// Source: upstream/packages/types/src/ThreeDsChunkHandler.ts:20 (sha256:06870e8f10b7ff32bc95a6d20edfb5dd44d7bdc37dee293e8f841aa885ae27c6)
#[derive(Clone)]
pub struct ThreeDsChunkHandler {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub chunk_ids: Vec<f64>,
    pub build: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(ThreeDsParseState) -> () + Send + 'static>>>,
    >,
    pub collect: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(ThreeDsParseState, crate::OpaqueHostValue, f64, f64, String) -> ()
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for ThreeDsChunkHandler {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ThreeDsChunkHandler.ts:34 (sha256:69ccc45776e3f562df014de6d12ba86072f78749ef58bcfde8df7f5308ceb8b9)
#[derive(Clone, Default)]
pub struct ThreeDsParseState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cameras: Vec<ThreeDsCamera>,
    pub diagnostics: Option<Vec<ImportDiagnostic>>,
    pub drops: Option<Vec<(String, ThreeDsDropTally)>>,
    pub document: Scene3DDocument,
    pub lights: Vec<ThreeDsLight>,
    pub materials: Vec<(String, ThreeDsMaterial)>,
    pub meshes: Vec<ThreeDsMesh>,
    pub pivots: Vec<(String, Vec<f64>)>,
}
impl PartialEq for ThreeDsParseState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ThreeDsChunkHandler.ts:54 (sha256:32eacd86a93b876ac59366d7ff371daf63e5b589dd5690c67b24607ee45def4a)
pub type ThreeDsChunkDispatch = Vec<(f64, ThreeDsChunkHandler)>;

// Source: upstream/packages/types/src/ThreeDsChunkHandler.ts:67 (sha256:dc269575c4cf24cefabee8d9f9b6faf6c48933c8526f9d984e44f1f5a699c712)
#[derive(Clone, Default)]
pub struct ThreeDsImportOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handlers: Option<Vec<ThreeDsChunkHandler>>,
}
impl PartialEq for ThreeDsImportOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
