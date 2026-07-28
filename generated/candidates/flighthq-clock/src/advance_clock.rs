// @generated from upstream/packages/clock/src/advanceClock.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::emit_signal;
use flighthq_types::Clock;

// Source: upstream/packages/clock/src/advanceClock.ts:10 (sha256:adc451e2f2928d39c0bfbafd55c56b8739014c142b1eee9d88e5b63ab9016c08)
pub fn advance_clock(clock: &mut Clock, delta_seconds: f64) -> () {
    let scaled_delta = if clock.paused {
        0.0_f64
    } else {
        (delta_seconds * clock.scale)
    };
    clock.delta_time = scaled_delta;
    clock.elapsed += scaled_delta;
    if ((clock.on_tick).clone()).is_some() {
        emit_signal(((clock.on_tick).clone()).unwrap(), (scaled_delta,));
    }
    {
        let mut i = 0.0_f64;
        while (i < (clock.children.len() as f64)) {
            advance_clock(&mut clock.children[i as usize], scaled_delta);
            {
                i += 1.0;
                i
            };
        }
    }
}
