// @generated from upstream/packages/entity/src/clone.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_entity;

// Source: upstream/packages/entity/src/clone.ts:9 (sha256:f7af5f83e0def8e4977a633745c80a0fb8ca442ec5be637335d3bbaca14d3a98)
pub fn clone_entity<Type: Clone + flighthq_types::FlightEntity>(source: Type) -> Type {
    let mut copy = flighthq_types::FlightEntity::__flight_fresh_clone(&((source).clone()));
    *flighthq_types::FlightEntity::__flight_entity_runtime(&(copy))
        .lock()
        .unwrap() = None;
    return create_entity(Some((copy).clone()));
}

// Source: upstream/packages/entity/src/clone.ts:19 (sha256:f821dde436032aea524e91fa419eba4f843d76d89558a537e9a02c7ab3c6e0c3)
pub fn strip_entity_runtime<Type: Clone + flighthq_types::FlightEntity>(source: Type) -> Type {
    let copy = flighthq_types::FlightEntity::__flight_fresh_clone(&((source).clone()));
    flighthq_types::FlightEntity::__flight_entity_runtime(&(copy))
        .lock()
        .unwrap()
        .take()
        .is_some();
    return copy;
}
