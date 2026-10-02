// @generated from upstream/packages/types/src/FlightDocumentResource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{FlightDocumentFields, Kind};

// Source: upstream/packages/types/src/FlightDocumentResource.ts:6 (sha256:d27d3e510487bac655d990e5a64fdd8b9cb3449f4cba69fc041343a7367ca642)
#[derive(Clone, Default)]
pub struct FlightDocumentResourceDescriptor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fields: FlightDocumentFields,
    pub key: String,
    pub kind: Kind,
}
impl PartialEq for FlightDocumentResourceDescriptor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FlightDocumentResource.ts:12 (sha256:c98f01c3621ba201e917c999538bc99e6c0ac6bbe8f7e183ba33157f2b07d3bc)
pub type FlightDocumentResourceResolver = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(String, FlightDocumentResourceDescriptor) -> Option<crate::FlightValue>
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/FlightDocumentResource.ts:18 (sha256:789fdcb0eb8ce52ec4c04efb868553ee3d72d4f44f7b8ecfea9a9401668bb18a)
#[derive(Clone, Default)]
pub struct FlightDocumentResourceResolverRegistry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub resolvers: Vec<(Kind, FlightDocumentResourceResolver)>,
}
impl PartialEq for FlightDocumentResourceResolverRegistry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
