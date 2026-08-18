// @generated from upstream/packages/clock/src/clockSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::create_signal;
use flighthq_types::{Clock, Signal};

// Source: upstream/packages/clock/src/clockSignals.ts:8 (sha256:5b3a33690e3d40ff3d996ce87b7388f249e452f0647781b5a76faebbda6b2fe5)
pub fn enable_clock_signals(
    clock: &mut Clock,
) -> Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>> {
    if ((clock.on_tick).clone()).is_none() {
        clock.on_tick = Some(create_signal());
    }
    return ((clock.on_tick).clone()).unwrap();
}
