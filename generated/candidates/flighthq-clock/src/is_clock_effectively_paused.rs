// @generated from upstream/packages/clock/src/isClockEffectivelyPaused.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Clock;

// Source: upstream/packages/clock/src/isClockEffectivelyPaused.ts:5 (sha256:94d1d81f53c1df88a647fb9f771812bec263d9a53a8c84b6554ca25f3bf79f40)
pub fn is_clock_effectively_paused(clock: &mut Clock) -> bool {
    let mut current: Option<Clock> = clock;
    while (current).is_some() {
        if current.as_mut().unwrap().paused {
            return true;
        }
        current = (current.as_mut().unwrap().parent).clone();
    }
    return false;
}
