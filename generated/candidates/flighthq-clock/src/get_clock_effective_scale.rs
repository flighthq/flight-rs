// @generated from upstream/packages/clock/src/getClockEffectiveScale.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Clock;

// Source: upstream/packages/clock/src/getClockEffectiveScale.ts:6 (sha256:117a788cea03f0d05b139bdaf6200843d508a8b743234552ac899617eebb73ed)
pub fn get_clock_effective_scale(clock: &mut Clock) -> f64 {
    let mut scale = clock.scale;
    let mut current: Option<Clock> = (clock.parent).clone();
    while (current).is_some() {
        scale *= current.as_mut().unwrap().scale;
        current = (current.as_mut().unwrap().parent).clone();
    }
    return scale;
}
