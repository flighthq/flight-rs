// @generated from upstream/packages/easing/src/easeSteps.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{EasingFunction, EasingStepsGuard, StepPosition};

// Source: upstream/packages/easing/src/easeSteps.ts:16 (sha256:c3162b18bf5d93fcdd7b7dee613223e1e9503f981a4355658553affaf3876435)
pub fn ease_steps(count: f64, position: Option<StepPosition>) -> EasingFunction {
    let position = position.unwrap_or("jumpEnd".to_owned());
    if ((position).clone() == "jumpNone") && (count < 2.0_f64) {
        {
            let __flight_callback = (*_STEPS_GUARD.lock().unwrap()).clone();
            __flight_callback
                .as_ref()
                .map(|callback| callback.lock().unwrap()(count, (position).clone()))
        };
    }
    let jumps = if (position == "jumpNone") {
        (count - 1.0_f64)
    } else {
        if (position == "jumpBoth") {
            (count + 1.0_f64)
        } else {
            count
        }
    };
    let start_offset = if (position == "jumpStart") || (position == "jumpBoth") {
        1.0_f64
    } else {
        0.0_f64
    };
    return std::sync::Arc::new(std::sync::Mutex::new(Box::new(move |t: f64| -> f64 {
        let mut step = ((t * count).floor() + start_offset);
        if (t >= 0.0_f64) && (step < 0.0_f64) {
            step = 0.0_f64;
        }
        if (t <= 1.0_f64) && (step > jumps) {
            step = jumps;
        }
        return (step / jumps);
    })
        as Box<dyn FnMut(f64) -> f64 + Send + 'static>));
}

// Source: upstream/packages/easing/src/easeSteps.ts:37 (sha256:f32fb71d7f2446645b24b853cb7ddbc191f1cea28d08fdaff1514a3108e86e07)
pub fn set_easing_steps_guard(guard: Option<EasingStepsGuard>) -> () {
    (*_STEPS_GUARD.lock().unwrap()) = (guard).clone();
}

// Source: upstream/packages/easing/src/easeSteps.ts:41 (sha256:9a521db5d67417fd09ba38338690f82ac7792afc4fa7690d8fbc25aa709f2aff)
static _STEPS_GUARD: std::sync::LazyLock<std::sync::Mutex<Option<EasingStepsGuard>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
