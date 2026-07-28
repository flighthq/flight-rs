// @generated from upstream/packages/signals/src/emitter.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Signal;

// Source: upstream/packages/signals/src/emitter.ts:5 (sha256:1f1b9cc70244b89e727742862e5eb56831b95c1aee239db6e44cd0cbdff697f5)
pub fn cancel_signal<T: crate::FlightCallback>(signal: &mut Signal<T>) -> () {
    if ((signal.data).clone()).is_some() {
        signal
            .data
            .as_ref()
            .unwrap()
            .inner
            .lock()
            .unwrap()
            .cancelled = true;
    }
}

// Source: upstream/packages/signals/src/emitter.ts:9 (sha256:261abe6510f5fb4d097cc3881c5e059005d18e406e496907085cb3c4fc5e5d94)
pub fn emit_signal<T: crate::FlightCallback>(
    signal: Signal<T>,
    args: <T as crate::FlightCallback>::Args,
) -> () {
    crate::FlightCallback::flight_call(&((signal.emit).clone()), ((args).clone()).clone());
}
