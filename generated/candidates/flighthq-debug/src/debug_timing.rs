// @generated from upstream/packages/debug/src/debugTiming.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::is_debug_enabled;
use flighthq_log::{end_log_timer, log_debug, start_log_timer};
use flighthq_types::{LogData, LogDataProvider, LogTimer};

// Source: upstream/packages/debug/src/debugTiming.ts:11 (sha256:46fa671b6e7598c2290adaaaddfadaa8186eb56ad30b61072ef3897923350d47)
pub fn begin_debug_span(name: String, channel: Option<String>) -> Option<LogTimer> {
    return if is_debug_enabled() {
        Some(start_log_timer((name).clone(), ((channel).clone()).clone()))
    } else {
        None
    };
}

// Source: upstream/packages/debug/src/debugTiming.ts:18 (sha256:4cd72a4bcdf69f6c1194d6e823b36e48847e9efb457699c9a6ea7ab3d2754109)
pub fn end_debug_span(timer: &Option<LogTimer>) -> f64 {
    return if (timer).is_none() {
        (-1.0_f64)
    } else {
        end_log_timer(timer.as_ref().unwrap())
    };
}

// Source: upstream/packages/debug/src/debugTiming.ts:26 (sha256:ad3780fbc14695afa3b4e4c3ba979294048ad1584389d66835e96627767d6bb8)
pub fn mark_debug_frame(label: Option<String>, channel: Option<String>) -> () {
    if (!is_debug_enabled()) {
        return;
    }
    log_debug(
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("frame".to_owned(), {
                let __flight_portable_source = (label).unwrap_or({
                    (*_DEBUG_FRAME_NUMBER.lock().unwrap()) += 1.0;
                    (*_DEBUG_FRAME_NUMBER.lock().unwrap())
                });
                crate::FlightValue::String((&__flight_portable_source).clone())
            }));
            __flight_record
        }))),
        ((channel).clone()).clone(),
    );
}

// Source: upstream/packages/debug/src/debugTiming.ts:35 (sha256:68f087e3164fcc9a4195b86e69cc2067458dc15ee404571b30bdf8d3871a2ce8)
pub fn measure_debug_span<T: Clone>(
    name: String,
    fn_: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> T + Send + 'static>>>,
    channel: Option<String>,
) -> T {
    let timer = begin_debug_span((name).clone(), ((channel).clone()).clone());
    {
        return {
            let __flight_callback = (fn_).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
    }
    {
        end_debug_span(&(timer));
    }
}

// Source: upstream/packages/debug/src/debugTiming.ts:46 (sha256:33501a6ff34621cf8f456273017c18a42dc3ae8d74f6c73f20623e5ab795ad8a)
static _DEBUG_FRAME_NUMBER: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(0.0_f64));
