// @generated from upstream/packages/signals/src/signal.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::null_signal_emit;
use flighthq_types::Signal;

// Source: upstream/packages/signals/src/signal.ts:7 (sha256:6e00ef651edec68c43fe8e917dc888ae5de0ae45586e74d222c40522445ee01e)
pub fn create_signal<T: crate::FlightCallback>() -> Signal<T> {
    return Signal::<T> {
        __flight_identity: std::sync::Arc::new(()),
        emit: T::flight_noop(),
        data: None,
    };
}
