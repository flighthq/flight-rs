// @generated from upstream/packages/clock/src/disposeClock.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::remove_clock_child;
use flighthq_signals::clear_signal;
use flighthq_types::Clock;

// Source: upstream/packages/clock/src/disposeClock.ts:10 (sha256:d654ecb7d75778d2bc4b2fbc2fe90c930a32e598a4639a4a91b284c11985718c)
pub fn dispose_clock(clock: &mut Clock) -> () {
    if ((clock.parent).clone()).is_some() {
        remove_clock_child(clock.parent.as_mut().unwrap(), clock);
    }
    {
        let mut i = 0.0_f64;
        while (i < (clock.children.len() as f64)) {
            clock.children[i as usize].parent = None;
            {
                i += 1.0;
                i
            };
        }
    }
    clock.children.clear();
    if ((clock.on_tick).clone()).is_some() {
        clear_signal(clock.on_tick.as_mut().unwrap());
    }
}
