// @generated from upstream/packages/types/src/FlightDocumentResourceSchema.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{FlightDocumentFieldSchema, Kind};

// Source: upstream/packages/types/src/FlightDocumentResourceSchema.ts:4 (sha256:79db16411fa5bfd96918e61887eb1d36f0b8e4e4d4de137b053856ea27c9c4bb)
#[derive(Clone, Default)]
pub struct FlightDocumentResourceSchema {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fields: Vec<FlightDocumentFieldSchema>,
    pub kind: Kind,
}
impl PartialEq for FlightDocumentResourceSchema {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
