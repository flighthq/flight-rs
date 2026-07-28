// @generated from upstream/packages/clock/src/pauseClock.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Clock;

// Source: upstream/packages/clock/src/pauseClock.ts:5 (sha256:46f90f82801a01378831c3c91036b22a58c99217be9b8a49b52a3ad5a97ea7fd)
pub fn pause_clock(clock: &mut Clock) -> () {
    clock.paused = true;
}

// Source: upstream/packages/clock/src/pauseClock.ts:11 (sha256:391914411c72d6c22b6b875b5cf8bd4e95401569e9b451e825fb9c801a7a49f6)
pub fn resume_clock(clock: &mut Clock) -> () {
    clock.paused = false;
}
