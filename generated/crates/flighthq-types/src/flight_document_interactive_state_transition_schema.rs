// @generated from upstream/packages/types/src/FlightDocumentInteractiveStateTransitionSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    FlightDocumentFieldSchema, FlightDocumentFields, Kind, NodeInteractiveStateTransition,
};

// Source: upstream/packages/types/src/FlightDocumentInteractiveStateTransitionSchema.ts:5 (sha256:1cef3e1b20be576c9f46020e4ac636843db1a14c3a00bffda209ea5bb1025d04)
#[derive(Clone)]
pub struct FlightDocumentInteractiveStateTransitionSchema {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub create_transition: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        FlightDocumentFields,
                    ) -> Option<
                        NodeInteractiveStateTransition<
                            crate::OpaqueHostValue,
                            crate::OpaqueHostValue,
                        >,
                    > + Send
                    + 'static,
            >,
        >,
    >,
    pub fields: Vec<FlightDocumentFieldSchema>,
    pub kind: Kind,
}
impl PartialEq for FlightDocumentInteractiveStateTransitionSchema {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
