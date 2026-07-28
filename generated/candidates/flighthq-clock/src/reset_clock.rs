// @generated from upstream/packages/clock/src/resetClock.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Clock;

// Source: upstream/packages/clock/src/resetClock.ts:5 (sha256:5ec957e1ea5cf0054e328d3a11d0f50be72ce669ff436f5c45b893b299662fa9)
pub fn reset_clock(clock: &mut Clock) -> () {
    clock.elapsed = 0.0_f64;
    clock.delta_time = 0.0_f64;
}
