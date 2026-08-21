// @generated from upstream/packages/tray/src/enableTrayGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::set_tray_animation_guard;
use flighthq_log::log_once;
use flighthq_types::{LogData, LogDataProvider, LogLevel, TrayIcon};

// Source: upstream/packages/tray/src/enableTrayGuards.ts:8 (sha256:72112ca8e5fbf121246bfeb52102b3491dd6a4e90f510333ac6ea6e0cc594491)
pub fn disable_tray_guards() -> () {
    set_tray_animation_guard(&(None));
}

// Source: upstream/packages/tray/src/enableTrayGuards.ts:25 (sha256:0daedbf925f1b0005e871af5a04c21238b540f3ab53becbdfa0e238bc83e13ea)
pub fn enable_tray_guards() -> () {
    set_tray_animation_guard(&(warn_on_unbounded_tray_animation));
}

// Source: upstream/packages/tray/src/enableTrayGuards.ts:29 (sha256:da206eb2e5d62d94ceaf3c3c82cebe96a8a9e2366e6d330e6590e6c778c52b52)
fn warn_on_unbounded_tray_animation(_tray: &TrayIcon, _frame_count: f64, interval_ms: f64) -> () {
    if (interval_ms > 0.0_f64) {
        return;
    }
    log_once(
        "tray:non-positive-animation-interval".to_owned(),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), { let __flight_portable_source = format!("{}{}", format!("startTrayIconAnimation: intervalMs is {}, so the icon will be rewritten as fast as the ", interval_ms), "host schedules timers rather than on a frame interval. Pass a positive millisecond interval."); crate::FlightValue::String((&__flight_portable_source).clone()) }));
            __flight_record
        }))),
        Some(("tray".to_owned()).clone()),
    );
}
