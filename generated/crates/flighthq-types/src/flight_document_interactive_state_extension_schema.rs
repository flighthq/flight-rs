// @generated from upstream/packages/types/src/FlightDocumentInteractiveStateExtensionSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{FlightDocumentFieldSchema, Kind, NodeAny, NodeInteractiveStateExtensionRuntime};

// Source: upstream/packages/types/src/FlightDocumentInteractiveStateExtensionSchema.ts:6 (sha256:ad0e9a77b8d35a5bd5a01015423f54e3589681f1bb08fc3b01cee71aab45e840)
#[derive(Clone)]
pub struct FlightDocumentInteractiveStateExtensionSchema {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub create_extension: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(NodeAny, Vec<String>) -> Option<NodeInteractiveStateExtensionRuntime>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub fields: Vec<FlightDocumentFieldSchema>,
    pub is_supported:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(NodeAny) -> bool + Send + 'static>>>,
    pub kind: Kind,
}
impl PartialEq for FlightDocumentInteractiveStateExtensionSchema {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
