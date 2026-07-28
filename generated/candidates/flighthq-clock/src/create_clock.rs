// @generated from upstream/packages/clock/src/createClock.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::add_clock_child;
use flighthq_types::{Clock, ClockOptions};

// Source: upstream/packages/clock/src/createClock.ts:7 (sha256:827988cde163cc0a8c19ea99806fa81cc5522fcb1c54a74cd7c970613566273a)
pub fn create_child_clock(parent: &mut Clock, options: Option<ClockOptions>) -> Clock {
    let mut child = create_clock(Some(((options).clone().unwrap()).clone()));
    add_clock_child(parent, &mut child);
    return child;
}

// Source: upstream/packages/clock/src/createClock.ts:15 (sha256:ad5a178ea582292e1506fc5afed9cccb078120b53cbe1436fe6338d0893dfbed)
pub fn create_clock(options: Option<ClockOptions>) -> Clock {
    return Clock {
        __flight_identity: std::sync::Arc::new(()),
        scale: (options.as_ref().and_then(|value| value.scale)).unwrap_or(1.0_f64),
        paused: (options.as_ref().and_then(|value| value.paused)).unwrap_or(false),
        delta_time: 0.0_f64,
        elapsed: 0.0_f64,
        parent: None,
        children: vec![],
        on_tick: None,
    };
}
