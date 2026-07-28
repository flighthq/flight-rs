// @generated from upstream/packages/clock/src/setClockScale.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Clock;

// Source: upstream/packages/clock/src/setClockScale.ts:5 (sha256:009ae0c20f8b455ead83020a0e6b90a84bfbd2f4e380869f204bc7973fcea8ab)
pub fn set_clock_scale(clock: &mut Clock, scale: f64) -> () {
    clock.scale = scale;
}
