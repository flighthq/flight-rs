// @generated from upstream/packages/types/src/FlightDocumentSchemaRegistry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    FlightDocumentInteractiveStateExtensionSchema, FlightDocumentInteractiveStateTransitionSchema,
    FlightDocumentNodeSchema, FlightDocumentResourceSchema, Kind, ShapeCommandSchema,
};

// Source: upstream/packages/types/src/FlightDocumentSchemaRegistry.ts:10 (sha256:7a0fe0691ce082c402fade69285ec9b4784111efd0ff40acb1d0599a6a49c7a1)
#[derive(Clone, Default)]
pub struct FlightDocumentSchemaRegistry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub interactive_state_extension_schemas:
        Vec<(Kind, FlightDocumentInteractiveStateExtensionSchema)>,
    pub interactive_state_transition_schemas:
        Vec<(Kind, FlightDocumentInteractiveStateTransitionSchema)>,
    pub node_schemas: Vec<(Kind, FlightDocumentNodeSchema)>,
    pub resource_schemas: Vec<(Kind, FlightDocumentResourceSchema)>,
    pub shape_command_schemas: Vec<(Kind, ShapeCommandSchema<crate::OpaqueHostValue>)>,
}
impl PartialEq for FlightDocumentSchemaRegistry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
