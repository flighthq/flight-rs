// @generated from upstream/packages/tween/src/tweenProgress.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::initialize_tween;
use flighthq_signals::emit_signal;
use flighthq_types::Tween;

// Source: upstream/packages/tween/src/tweenProgress.ts:13 (sha256:1eb62d58e13c13bebb7fde02ab4c6a7740ca19ddeae30b46c6eb56103ddafcec)
pub fn get_tween_progress(tween: &Tween<crate::OpaqueHostValue>) -> f64 {
    if tween.complete {
        return 1.0_f64;
    }
    let active_elapsed = (tween.elapsed - tween.delay);
    if (active_elapsed <= 0.0_f64) {
        return 0.0_f64;
    }
    return (active_elapsed / tween.duration).min(1.0_f64);
}

// Source: upstream/packages/tween/src/tweenProgress.ts:24 (sha256:d689b54d87d8db974a447b273f862be52378ab779fb3db298e4cbcd54d83a0ca)
pub fn invalidate_tween(tween: &mut Tween<crate::OpaqueHostValue>) -> () {
    tween.initialized = false;
    tween.complete = false;
    tween.elapsed = 0.0_f64;
}

// Source: upstream/packages/tween/src/tweenProgress.ts:34 (sha256:028052517ec490c502473ad201e803f5739240da4251556a347f131cad0e65e9)
pub fn restart_tween(tween: &mut Tween<crate::OpaqueHostValue>, include_delay: Option<bool>) -> () {
    let include_delay = include_delay.unwrap_or(true);
    tween.initialized = false;
    tween.complete = false;
    tween.elapsed = if include_delay { 0.0_f64 } else { tween.delay };
}

// Source: upstream/packages/tween/src/tweenProgress.ts:49 (sha256:848feb8e6dee8ceb197ea0358bec14529717c50ca5f27753fefd1f5bb8b07d28)
#[derive(Clone, Default)]
struct SeekTweenRecord1 {
    __flight_identity: std::sync::Arc<()>,
    key: String,
    value: f64,
}
impl PartialEq for SeekTweenRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn seek_tween(tween: &mut Tween<crate::OpaqueHostValue>, time_seconds: f64) -> () {
    if (!tween.initialized) {
        initialize_tween(tween);
    }
    let max_elapsed = (tween.delay + tween.duration);
    let clamped_elapsed = (0.0_f64).max((time_seconds).min(max_elapsed));
    tween.elapsed = clamped_elapsed;
    let active_elapsed = (clamped_elapsed - tween.delay);
    if (active_elapsed <= 0.0_f64) {
        return;
    }
    let t = (active_elapsed / tween.duration).min(1.0_f64);
    let effective_t = if tween.reverse { (1.0_f64 - t) } else { t };
    let eased_t = {
        let __flight_callback = (tween.ease).clone();
        let __flight_result = __flight_callback.lock().unwrap()(effective_t);
        __flight_result
    };
    let mut writes: Vec<SeekTweenRecord1> = vec![];
    for detail in ((tween.properties).clone()).iter().cloned() {
        let mut value = (detail.start + (detail.change * eased_t));
        if tween.snapping {
            value = (value).round();
        }
        writes.push(SeekTweenRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            key: (detail.key).clone(),
            value: value,
        });
    }
    let mut target = crate::host_value::<Vec<(String, f64)>>("host.cast");
    for __iteration0 in (writes).iter().cloned() {
        let key = (__iteration0.key).clone();
        let mut value = __iteration0.value;
        target
            .iter()
            .find(|(key, _)| key == &(key).clone())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") = value;
    }
    emit_signal((tween.on_update).clone(), ());
    if (t >= 1.0_f64) && (!tween.complete) {
        tween.complete = true;
        emit_signal((tween.on_complete).clone(), ());
    }
}

// Source: upstream/packages/tween/src/tweenProgress.ts:85 (sha256:690df00d155de46b5185b31c827e12728391487c488ee6abfe5982a4ba86fbee)
pub fn set_tween_progress(tween: &mut Tween<crate::OpaqueHostValue>, progress: f64) -> () {
    let clamped = (0.0_f64).max((progress).min(1.0_f64));
    let target_elapsed = (tween.delay + (clamped * tween.duration));
    seek_tween(tween, target_elapsed);
}
