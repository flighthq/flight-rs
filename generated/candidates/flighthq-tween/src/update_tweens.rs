// @generated from upstream/packages/tween/src/updateTweens.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::initialize_tween;
use flighthq_signals::emit_signal;
use flighthq_types::{Tween, TweenManager};

// Source: upstream/packages/tween/src/updateTweens.ts:6 (sha256:24a758284ceab1893b26375d0b8f96301f6503cc0814f9cfd994bd0d62e4a7f2)
pub fn complete_tween<T: Clone>(tween: &mut Tween<T>) -> () {
    if tween.complete {
        return;
    }
    if (!tween.initialized) {
        initialize_tween(tween);
    }
    let effective_t = if tween.reverse { 0.0_f64 } else { 1.0_f64 };
    let eased_t = {
        let __flight_callback = (tween.ease).clone();
        let __flight_result = __flight_callback.lock().unwrap()(effective_t);
        __flight_result
    };
    let mut target = (tween.target).clone();
    for detail in ((tween.properties).clone()).iter().cloned() {
        let mut value = (detail.start + (detail.change * eased_t));
        if tween.snapping {
            value = (value).round();
        }
        target
            .iter()
            .find(|(key, _)| key == &(detail.key).clone())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") = value;
    }
    tween.complete = true;
    emit_signal((tween.on_complete).clone(), ());
}

// Source: upstream/packages/tween/src/updateTweens.ts:21 (sha256:565e5a509c9151432ab676921fae27bc17b9e6a182d93c7267ecef448505f4f4)
fn update_tween<T: Clone>(tween: &mut Tween<T>, delta_time: f64) -> () {
    if (tween.paused) || (tween.complete) {
        return;
    }
    tween.elapsed += delta_time;
    let active_elapsed = (tween.elapsed - tween.delay);
    if (active_elapsed <= 0.0_f64) {
        return;
    }
    if (!tween.initialized) {
        initialize_tween(tween);
    }
    let t = (active_elapsed / tween.duration).min(1.0_f64);
    let effective_t = if tween.reverse { (1.0_f64 - t) } else { t };
    let eased_t = {
        let __flight_callback = (tween.ease).clone();
        let __flight_result = __flight_callback.lock().unwrap()(effective_t);
        __flight_result
    };
    let mut target = (tween.target).clone();
    for detail in ((tween.properties).clone()).iter().cloned() {
        let mut value = (detail.start + (detail.change * eased_t));
        if tween.snapping {
            value = (value).round();
        }
        target
            .iter()
            .find(|(key, _)| key == &(detail.key).clone())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") = value;
    }
    emit_signal((tween.on_update).clone(), ());
    if (t >= 1.0_f64) {
        if (tween.repeat == 0.0_f64) {
            tween.complete = true;
            emit_signal((tween.on_complete).clone(), ());
        } else {
            if tween.reflect {
                tween.reverse = (!tween.reverse);
                emit_signal((tween.on_yoyo).clone(), ());
            }
            tween.elapsed = tween.delay;
            if (tween.repeat > 0.0_f64) {
                {
                    tween.repeat -= 1.0;
                    tween.repeat
                };
            }
            emit_signal((tween.on_repeat).clone(), ());
        }
    }
}

// Source: upstream/packages/tween/src/updateTweens.ts:65 (sha256:5d40b010423f9ec2bb366ca29023031873a86dad49284f3f7c2f4f0305b6ad54)
pub fn update_tweens(manager: &mut TweenManager, delta_time: f64) -> () {
    for __iteration0 in ((manager.tweens).clone()).iter().cloned() {
        let target = __iteration0[0.0_f64 as usize].clone();
        let mut list = __iteration0[1.0_f64 as usize].clone();
        let mut i = (list.length - 1.0_f64);
        while (i >= 0.0_f64) {
            if list[i as usize].complete {
                (list.splice)(i, 1.0_f64);
            } else {
                update_tween(&mut list[i as usize], delta_time);
            }
            {
                i -= 1.0;
                i
            };
        }
        if (list.length == 0.0_f64) {
            {
                let __flight_key = target;
                if let Some(__flight_index) = manager
                    .tweens
                    .iter()
                    .position(|(key, _)| key == &__flight_key)
                {
                    manager.tweens.remove(__flight_index);
                    true
                } else {
                    false
                }
            };
        }
    }
}
