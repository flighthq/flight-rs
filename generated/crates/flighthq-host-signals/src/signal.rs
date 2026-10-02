// @generated from upstream/packages/signals/src/signal.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Signal;

// Source: upstream/packages/signals/src/signal.ts:8 (sha256:234213c0985fd4de6b3d193b7f5859e4112463ca2956cae36ce8c9b019eec4a8)
pub fn create_signal<T: crate::FlightCallback>() -> Signal<T> {
    let out = allocate_entity();
    initialize_signal((out).clone());
    return finish_entity((out).clone());
}
