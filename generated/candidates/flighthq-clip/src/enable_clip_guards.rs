// @generated from upstream/packages/clip/src/enableClipGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::set_clip_region_release_guard;
use flighthq_log::log_once;
use flighthq_types::{ClipRegion, LogData, LogDataProvider, LogLevel};

// Source: upstream/packages/clip/src/enableClipGuards.ts:8 (sha256:75863f90a72a3e720042930769b7de3ccd462233b8eaa848164fe7cf2f6b9ff4)
pub fn disable_clip_guards() -> () {
    set_clip_region_release_guard(&(None));
}

// Source: upstream/packages/clip/src/enableClipGuards.ts:19 (sha256:4b0a4a2b5c05196bb53863dc5d8bd8cf6617e32c1bb8c8eeb9bb47eec1e7fc16)
pub fn enable_clip_guards() -> () {
    set_clip_region_release_guard(&(warn_on_double_release));
}

// Source: upstream/packages/clip/src/enableClipGuards.ts:23 (sha256:7b1e4c88ed3f25bff54d5e51cd748bb7e09999f411ff1a5b64a6f61cc47eb339)
#[derive(Clone, Default)]
struct WarnOnDoubleReleaseRecord4 {
    __flight_identity: std::sync::Arc<()>,
    message: String,
}
impl PartialEq for WarnOnDoubleReleaseRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn warn_on_double_release(_clip: &ClipRegion) -> () {
    log_once(
        "clip:double-release".to_owned(),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), { let __flight_portable_source = "releaseClipRegion: this region is already in the pool, so it is being released twice. Two later acquireClipRegion calls will hand back the same object and the clips will alias each other. Every acquireClipRegion pairs with exactly one releaseClipRegion, and the region must not be used after release.".to_owned(); crate::FlightValue::String((&__flight_portable_source).clone()) }));
            __flight_record
        }))),
        Some(("clip".to_owned()).clone()),
    );
}
