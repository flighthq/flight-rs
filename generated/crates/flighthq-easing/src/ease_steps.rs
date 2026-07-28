// @generated from upstream/packages/easing/src/easeSteps.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use crate::{EasingFunction, StepPosition};

// Source: upstream/packages/easing/src/easeSteps.ts:15 (sha256:3cf16fc37f154567c667dbbdd828c0f6c6b17504043d5e1e6c8d0ea80de978ad)
pub fn ease_steps(count: f64, position: Option<StepPosition>) -> EasingFunction {
    let position = position.unwrap_or("jumpEnd");
    let jumps = if (position == "jumpNone") {
        (count - 1.0_f64)
    } else {
        if (position == "jumpBoth") {
            (count + 1.0_f64)
        } else {
            count
        }
    };
    let start_offset = if ((position == "jumpStart") || (position == "jumpBoth")) {
        1.0_f64
    } else {
        0.0_f64
    };
    return std::sync::Arc::new(move |t: f64| -> f64 {
        let mut step = ((t * count).floor() + start_offset);
        if ((t >= 0.0_f64) && (step < 0.0_f64)) {
            {
                step = 0.0_f64;
                step
            };
        }
        if ((t <= 1.0_f64) && (step > jumps)) {
            {
                step = jumps;
                step
            };
        }
        return (step / jumps);
    });
}
