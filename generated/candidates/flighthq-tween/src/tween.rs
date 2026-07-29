// @generated from upstream/packages/tween/src/tween.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{DEFAULT_MANAGER as default_manager_constant, initialize_tween};
use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{EasingFunction, TweenPropertyDetail};
pub use flighthq_types::{NumericProps, StopTweenOptions, Tween, TweenManager, TweenOptions};

// Source: upstream/packages/tween/src/tween.ts:19 (sha256:28c7268802f5fed54e5f79a8ac06a6f8dbe49ec6b759145b4e77b3b2ffb29d4b)
pub fn apply_tween<T: Clone>(manager: &TweenManager, target: T, property_map: NumericProps) -> () {
    stop_tweens(
        manager,
        (target).clone(),
        Some((property_map).clone()),
        None,
    );
    let mut t = target;
    let p = crate::host_value::<Vec<(String, Option<f64>)>>("host.cast");
    for key in (crate::host_value::<()>("host.keys")).iter().cloned() {
        let val = p
            .iter()
            .find(|(key, _)| key == &key)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone();
        if (val).is_some() {
            t.iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent") = *(val.as_ref().unwrap());
        }
    }
}

// Source: upstream/packages/tween/src/tween.ts:50 (sha256:4e3ce8549566ac275381fb2cf9861d663bdbf5e5d552c0193ae03d8eb2bd32fa)
pub fn create_tween<T: Clone>(
    manager_or_target: &crate::FlightUnion2<TweenManager, T>,
    target_or_duration: &crate::FlightUnion2<T, f64>,
    duration_or_props: &crate::FlightUnion2<f64, NumericProps>,
    props_or_options: Option<crate::FlightUnion2<NumericProps, TweenOptions>>,
    maybe_options: Option<TweenOptions>,
) -> Tween<T> {
    let mut manager: Option<TweenManager> = None;
    let mut target: Option<T> = None;
    let mut duration: f64;
    let mut property_map: Option<NumericProps> = None;
    let mut options: Option<TweenOptions>;
    if is_tween_manager((manager_or_target).clone()) {
        manager = Some(match (*manager_or_target).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        });
        target = Some(match (*target_or_duration).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        });
        duration = match (*duration_or_props).clone() {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        };
        property_map = Some((props_or_options).clone().unwrap());
        options = (maybe_options).clone();
    } else {
        manager = Some(default_manager_constant);
        target = Some(match (*manager_or_target).clone() {
            crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
            crate::FlightUnion2::B(value) => value,
        });
        duration = match (*target_or_duration).clone() {
            crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
            crate::FlightUnion2::B(value) => value,
        };
        property_map = Some(match (*duration_or_props).clone() {
            crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
            crate::FlightUnion2::B(value) => value,
        });
        options = props_or_options;
    }
    let tween = make_tween(
        ((target).clone().unwrap()).clone(),
        duration,
        ((property_map).clone().unwrap()).clone(),
        ((options).clone()).clone(),
        (manager.as_mut().unwrap().default_ease).clone(),
    );
    register_tween(
        manager.as_mut().unwrap(),
        &tween,
        (options.as_ref().and_then(|value| value.overwrite)).unwrap_or(true),
    );
    return tween;
}

// Source: upstream/packages/tween/src/tween.ts:82 (sha256:b50025be24ba98e84f198a008d481ed57c2aeab24888eb8114954b077053a0a4)
pub fn get_active_tween_count(manager: &TweenManager) -> f64 {
    let mut count = 0.0_f64;
    for list in (manager
        .tweens
        .iter()
        .map(|(_, value)| value.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        count += (list.len() as f64);
    }
    return count;
}

// Source: upstream/packages/tween/src/tween.ts:88 (sha256:396220455de76122d88f58a7c75268d96b93ed6c145f1efe8ec6daf5d1525e82)
pub fn get_tweens_of(
    manager: &TweenManager,
    target: crate::OpaqueHostValue,
) -> Vec<Tween<crate::OpaqueHostValue>> {
    return (manager
        .tweens
        .iter()
        .find(|(key, _)| key == &(target).clone())
        .map(|(_, value)| value.clone()))
    .unwrap_or(vec![]);
}

// Source: upstream/packages/tween/src/tween.ts:92 (sha256:4e1e0f39babc29fbda947ec0c8afd8d4a77dfcfbb4531bfa4367b575eddac184)
pub fn has_tweens_of(manager: &TweenManager, target: crate::OpaqueHostValue) -> bool {
    let list = manager
        .tweens
        .iter()
        .find(|(key, _)| key == &(target).clone())
        .map(|(_, value)| value.clone());
    return ((list).is_some()) && ((list.as_ref().unwrap().len() as f64) > 0.0_f64);
}

// Source: upstream/packages/tween/src/tween.ts:97 (sha256:58757f2cf7e188ee29bbdf94e4dd5d47a664320b942fcaea612c9928e974d207)
fn is_tween_manager(value: crate::OpaqueHostValue) -> bool {
    return ((match &(value) {
        crate::OpaqueHostValue::Undefined => "undefined",
        crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
        crate::OpaqueHostValue::Bool(_) => "boolean",
        crate::OpaqueHostValue::Number(_) => "number",
        crate::OpaqueHostValue::String(_) => "string",
    } == "object")
        && ((value).is_some()))
        && (crate::host_value::<String>("host.__brand") == "TweenManager");
}

// Source: upstream/packages/tween/src/tween.ts:101 (sha256:f9f1435bbcab8a288998c2c1123e0f4dc1c0509ea4d5d3c75a5422a3e3dff6c1)
pub fn kill_tweens_of_property(manager: &TweenManager, key: String) -> () {
    for list in (manager
        .tweens
        .iter()
        .map(|(_, value)| value.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        for tween in (list).iter().cloned() {
            let p = crate::host_value::<Vec<(String, crate::OpaqueHostValue)>>("host.cast");
            if {
                let __flight_key = (key).clone();
                p.iter().any(|(key, _)| key == &__flight_key)
            } {
                tween.complete = true;
            }
        }
    }
}

// Source: upstream/packages/tween/src/tween.ts:110 (sha256:42cbabcf216b9674ec975f7d93f329910a57275cca8d162791affd120b6ab005)
fn make_tween<T: Clone>(
    target: T,
    duration: f64,
    property_map: NumericProps,
    options: Option<TweenOptions>,
    default_ease: EasingFunction,
) -> Tween<T> {
    let keys = crate::host_value::<()>("host.keys");
    let properties: Vec<TweenPropertyDetail> = (keys.map)(std::sync::Arc::new(
        std::sync::Mutex::new(Box::new(move |key: crate::OpaqueHostValue| -> f64 {
            TweenPropertyDetail {
                __flight_identity: std::sync::Arc::new(()),
                change: 0.0_f64,
                key: key,
                start: 0.0_f64,
            }
        })
            as Box<dyn FnMut(crate::OpaqueHostValue) -> f64 + Send + 'static>),
    ));
    return Tween::<T> {
        __flight_identity: std::sync::Arc::new(()),
        complete: false,
        delay: (options.as_ref().and_then(|value| value.delay)).unwrap_or(0.0_f64),
        duration: duration,
        ease: (options.as_ref().and_then(|value| (value.ease).clone()))
            .unwrap_or((default_ease).clone()),
        elapsed: 0.0_f64,
        initialized: false,
        on_complete: create_signal(),
        on_repeat: create_signal(),
        on_update: create_signal(),
        on_yoyo: create_signal(),
        paused: false,
        properties: (properties).clone(),
        property_map: (property_map).clone(),
        reflect: (options.as_ref().and_then(|value| value.reflect)).unwrap_or(false),
        repeat: (options.as_ref().and_then(|value| value.repeat)).unwrap_or(0.0_f64),
        reverse: (options.as_ref().and_then(|value| value.reverse)).unwrap_or(false),
        smart_rotation: (options.as_ref().and_then(|value| value.smart_rotation)).unwrap_or(false),
        snapping: (options.as_ref().and_then(|value| value.snapping)).unwrap_or(false),
        target: (target).clone(),
    };
}

// Source: upstream/packages/tween/src/tween.ts:142 (sha256:d8324e6796c5a46d1b72357980a9e3b0b940c143509d07ce8ee4e9e3fd3897aa)
pub fn pause_all_tweens(manager: &TweenManager) -> () {
    for list in (manager
        .tweens
        .iter()
        .map(|(_, value)| value.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        for tween in (list).iter().cloned() {
            tween.paused = true;
        }
    }
}

// Source: upstream/packages/tween/src/tween.ts:148 (sha256:54975bcc10110dadfa0484e866c07684e4bfc3889b72366862733dc4ebd17144)
pub fn pause_tween(tween: &mut Tween<crate::OpaqueHostValue>) -> () {
    tween.paused = true;
}

// Source: upstream/packages/tween/src/tween.ts:152 (sha256:6b5c671ee8104f5731238c41930ec141db454e6c1f5af9ee28a419892d86d9d9)
pub fn pause_tweens(manager: &TweenManager, target: crate::OpaqueHostValue) -> () {
    let list = manager
        .tweens
        .iter()
        .find(|(key, _)| key == &(target).clone())
        .map(|(_, value)| value.clone());
    if (list).is_none() {
        return;
    }
    for tween in (list.as_ref().unwrap()).iter().cloned() {
        tween.paused = true;
    }
}

// Source: upstream/packages/tween/src/tween.ts:158 (sha256:d78c3b83d0a60c2ac6e901583eb731422f9a7c1729e941decc70686c7e7651dd)
fn register_tween<T: Clone>(manager: &mut TweenManager, tween: &Tween<T>, overwrite: bool) -> () {
    let mut list = manager
        .tweens
        .iter()
        .find(|(key, _)| key == &(tween.target).clone())
        .map(|(_, value)| value.clone());
    if (list).is_none() {
        list = Some(vec![]);
        {
            let __flight_key = (tween.target).clone();
            let __flight_value = (list).clone().unwrap();
            if let Some((_, value)) = manager
                .tweens
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                manager.tweens.push((__flight_key, __flight_value));
            }
        };
    }
    if overwrite {
        {
            let mut i = ((list.as_ref().unwrap().len() as f64) - 1.0_f64);
            while (i >= 0.0_f64) {
                let mut existing = list.as_mut().unwrap()[i as usize].clone();
                let existing_map =
                    crate::host_value::<Vec<(String, crate::OpaqueHostValue)>>("host.cast");
                let mut overlaps = false;
                for detail in ((tween.properties).clone()).iter().cloned() {
                    if {
                        let __flight_key = (detail.key).clone();
                        existing_map.iter().any(|(key, _)| key == &__flight_key)
                    } {
                        overlaps = true;
                        break;
                    }
                }
                if overlaps {
                    existing.complete = true;
                }
                {
                    i -= 1.0;
                    i
                };
            }
        }
    }
    list.as_mut().unwrap().push((tween).clone());
}

// Source: upstream/packages/tween/src/tween.ts:181 (sha256:952d8b95b0ee25a8c9ce7ef13092b044cffb2eed708bd55d5568dfa277213c79)
pub fn reset_all_tweens(manager: &mut TweenManager) -> () {
    manager.tweens.clear();
}

// Source: upstream/packages/tween/src/tween.ts:185 (sha256:3214b7c18319dbd8d30151c3f00dcdf69c24f6bd7c9ec3a297b401ddfca6bacb)
pub fn resume_all_tweens(manager: &TweenManager) -> () {
    for list in (manager
        .tweens
        .iter()
        .map(|(_, value)| value.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        for tween in (list).iter().cloned() {
            tween.paused = false;
        }
    }
}

// Source: upstream/packages/tween/src/tween.ts:191 (sha256:ea9a962ed9bf21b6c2845f116402ec98fbc1a7ded50c8a8926e785d9c7f6abfc)
pub fn resume_tween(tween: &mut Tween<crate::OpaqueHostValue>) -> () {
    tween.paused = false;
}

// Source: upstream/packages/tween/src/tween.ts:195 (sha256:6224c66070dae530ca62d7a3083ce8cbd7742ef68342c1ffa16120bbbdb5aca4)
pub fn resume_tweens(manager: &TweenManager, target: crate::OpaqueHostValue) -> () {
    let list = manager
        .tweens
        .iter()
        .find(|(key, _)| key == &(target).clone())
        .map(|(_, value)| value.clone());
    if (list).is_none() {
        return;
    }
    for tween in (list.as_ref().unwrap()).iter().cloned() {
        tween.paused = false;
    }
}

// Source: upstream/packages/tween/src/tween.ts:201 (sha256:276bb856d16d8012d1b3a42ea6d627762466fdfb64d3017830defe7f7387c089)
pub fn stop_all_tweens(manager: &TweenManager, options: Option<StopTweenOptions>) -> () {
    for list in (manager
        .tweens
        .iter()
        .map(|(_, value)| value.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        for tween in (list).iter().cloned() {
            stop_tween(&mut tween, Some(((options).clone().unwrap()).clone()));
        }
    }
}

// Source: upstream/packages/tween/src/tween.ts:207 (sha256:f9ff8248ba99195df55ffb5c9c9c1380f5d4695a3caa535a23461f248fe48707)
pub fn stop_tween(
    tween: &mut Tween<crate::OpaqueHostValue>,
    options: Option<StopTweenOptions>,
) -> () {
    let do_complete = (options.as_ref().and_then(|value| value.complete)).unwrap_or(false);
    let do_send_event = (options.as_ref().and_then(|value| value.send_event)).unwrap_or(true);
    if do_complete {
        if (!tween.initialized) {
            initialize_tween(tween);
        }
        let effective_t = if tween.reverse { 0.0_f64 } else { 1.0_f64 };
        let eased_t = {
            let __flight_callback = (tween.ease).clone();
            let __flight_result = __flight_callback.lock().unwrap()(effective_t);
            __flight_result
        };
        let mut t = crate::host_value::<Vec<(String, f64)>>("host.cast");
        for detail in ((tween.properties).clone()).iter().cloned() {
            let mut value = (detail.start + (detail.change * eased_t));
            if tween.snapping {
                value = (value).round();
            }
            t.iter()
                .find(|(key, _)| key == &(detail.key).clone())
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent") = value;
        }
        if do_send_event {
            emit_signal((tween.on_complete).clone(), ());
        }
    }
    tween.complete = true;
}

// Source: upstream/packages/tween/src/tween.ts:227 (sha256:a42b8be9f8d811a9d55ffea419fed534c2b1dbb3dd57b1fc25f881efaf48e658)
pub fn stop_tweens(
    manager: &TweenManager,
    target: crate::OpaqueHostValue,
    property_map: Option<NumericProps>,
    options: Option<StopTweenOptions>,
) -> () {
    let list = manager
        .tweens
        .iter()
        .find(|(key, _)| key == &(target).clone())
        .map(|(_, value)| value.clone());
    if (list).is_none() {
        return;
    }
    for tween in (list.as_ref().unwrap()).iter().cloned() {
        if (property_map).is_some() {
            let p = crate::host_value::<Vec<(String, crate::OpaqueHostValue)>>("host.cast");
            let tween_map = crate::host_value::<Vec<(String, crate::OpaqueHostValue)>>("host.cast");
            let mut overlaps = false;
            for key in (crate::host_value::<()>("host.keys")).iter().cloned() {
                if {
                    let __flight_key = key;
                    tween_map.iter().any(|(key, _)| key == &__flight_key)
                } {
                    overlaps = true;
                    break;
                }
            }
            if (!overlaps) {
                continue;
            }
        }
        stop_tween(&mut tween, Some(((options).clone().unwrap()).clone()));
    }
}
