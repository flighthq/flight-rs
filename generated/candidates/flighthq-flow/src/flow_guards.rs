// @generated from upstream/packages/flow/src/flowGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_log::log_once;
use flighthq_types::{LogData, LogDataProvider, LogLevel};

// Source: upstream/packages/flow/src/flowGuards.ts:4 (sha256:098f4d0938d3bc7fa7d79fb846812dda5b71d24ee82e89c904f0386b6e4b0ce2)
pub fn disable_flow_guards() -> () {
    ENABLED.store(false, std::sync::atomic::Ordering::Relaxed);
}

// Source: upstream/packages/flow/src/flowGuards.ts:8 (sha256:942e8621a4ebc90b0cfa8cd7a27e8e4e4314475fcc53da92e1885f29b47a5b1c)
pub fn enable_flow_guards() -> () {
    ENABLED.store(true, std::sync::atomic::Ordering::Relaxed);
}

// Source: upstream/packages/flow/src/flowGuards.ts:12 (sha256:fcd58832d5fdfe2d5580906ed2529f318f289fbc63b4946cd44789c67b13242e)
fn report_flow_guard(kind: String) -> () {
    if (!ENABLED.load(std::sync::atomic::Ordering::Relaxed)) {
        return;
    }
    log_once(
        format!("flow:{}", (kind).clone()),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("kind".to_owned(), {
                let __flight_portable_source = (kind).clone();
                crate::FlightValue::String((&__flight_portable_source).clone())
            }));
            __flight_record
        }))),
        Some(("flow".to_owned()).clone()),
    );
}

// Source: upstream/packages/flow/src/flowGuards.ts:17 (sha256:72a81d401fd3e2f76fa03e7166d34ee9fecfd2dbd513444c69c4cebb7ea6c554)
static ENABLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
