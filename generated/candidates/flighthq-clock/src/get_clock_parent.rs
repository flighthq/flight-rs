// @generated from upstream/packages/clock/src/getClockParent.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Clock;

// Source: upstream/packages/clock/src/getClockParent.ts:5 (sha256:63fab6c43184be1e74ebf0e90578139752db73173af703fd4feb5554cc79242e)
pub fn get_clock_parent(clock: &Clock) -> Option<Clock> {
    return (clock.parent).clone();
}
