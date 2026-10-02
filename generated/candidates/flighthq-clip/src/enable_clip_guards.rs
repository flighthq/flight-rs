// @generated from upstream/packages/clip/src/enableClipGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    set_clip_region_contours_guard, set_clip_region_release_guard, set_clip_region_use_guard,
};
use flighthq_log::log_once;
use flighthq_types::{
    ClipRegion, ClipRegionContoursExplanation, LogData, LogDataProvider, LogLevel,
};

// Source: upstream/packages/clip/src/enableClipGuards.ts:8 (sha256:1bef3b234f15a99407946bd816e3ea75a8fb92b36ec19e9a0821e459f0e2be9c)
pub fn disable_clip_guards() -> () {
    set_clip_region_contours_guard(&(None));
    set_clip_region_release_guard(&(None));
    set_clip_region_use_guard(&(None));
}

// Source: upstream/packages/clip/src/enableClipGuards.ts:21 (sha256:07ae6cd1107b087e2201f200c1fdf7bd3fe60d7c10f99848b08e4e5b67b9bb93)
pub fn enable_clip_guards() -> () {
    set_clip_region_contours_guard(&(warn_on_invalid_contours));
    set_clip_region_release_guard(&(warn_on_double_release));
    set_clip_region_use_guard(&(warn_on_use_after_release));
}

// Source: upstream/packages/clip/src/enableClipGuards.ts:27 (sha256:16aec442fad1be33774623a4c62af051a792293dfeec69eebac9eee6a360f76e)
#[derive(Clone, Default)]
struct WarnOnInvalidContoursRecord1 {
    __flight_identity: std::sync::Arc<()>,
    message: String,
}
impl PartialEq for WarnOnInvalidContoursRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn warn_on_invalid_contours(explanation: &ClipRegionContoursExplanation) -> () {
    log_once(
        format!("clip:invalid-contour:{}", (explanation.reason).clone()),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), { let __flight_portable_source = format!("createClipRegionFromContours: contour {} has {} coordinates ({}). Contours require at least three complete x/y pairs.", explanation.contour_index, explanation.coordinate_count, (explanation.reason).clone()); crate::FlightValue::String((&__flight_portable_source).clone()) }));
            __flight_record
        }))),
        Some(("clip".to_owned()).clone()),
    );
}

// Source: upstream/packages/clip/src/enableClipGuards.ts:38 (sha256:7b1e4c88ed3f25bff54d5e51cd748bb7e09999f411ff1a5b64a6f61cc47eb339)
#[derive(Clone, Default)]
struct WarnOnDoubleReleaseRecord1 {
    __flight_identity: std::sync::Arc<()>,
    message: String,
}
impl PartialEq for WarnOnDoubleReleaseRecord1 {
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

// Source: upstream/packages/clip/src/enableClipGuards.ts:50 (sha256:50d9962f738be24560ba8d8d7e4ce9daeb4b4fe5beff259004bdf2b07d185af2)
#[derive(Clone, Default)]
struct WarnOnUseAfterReleaseRecord1 {
    __flight_identity: std::sync::Arc<()>,
    message: String,
}
impl PartialEq for WarnOnUseAfterReleaseRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn warn_on_use_after_release(_clip: &ClipRegion) -> () {
    log_once(
        "clip:use-after-release".to_owned(),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), { let __flight_portable_source = "A released ClipRegion was used while it was still in the pool. Do not retain or access a region after releaseClipRegion; acquire a new region before use.".to_owned(); crate::FlightValue::String((&__flight_portable_source).clone()) }));
            __flight_record
        }))),
        Some(("clip".to_owned()).clone()),
    );
}
