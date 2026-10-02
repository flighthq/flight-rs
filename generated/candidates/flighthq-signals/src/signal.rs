// @generated from upstream/packages/signals/src/signal.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{EntityConstruction, Signal};

// Source: upstream/packages/signals/src/signal.ts:8 (sha256:234213c0985fd4de6b3d193b7f5859e4112463ca2956cae36ce8c9b019eec4a8)
pub fn create_signal<T: crate::FlightCallback>() -> Signal<T> {
    let mut out = allocate_entity();
    initialize_signal((out).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/signals/src/signal.ts:14 (sha256:17e11128e7733ff14c14800343b6819bdaf1e70fc41f27cf08890f44f4628a39)
pub fn initialize_signal<T: crate::FlightCallback>(out: EntityConstruction<Signal<T>>) -> () {
    crate::host_set("host.emit", T::flight_noop());
    crate::host_set("host.data", None);
}
