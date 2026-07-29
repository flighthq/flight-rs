// @generated from upstream/packages/tween/src/tweenStagger.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_tween;
use flighthq_types::{EasingFunction, NumericProps, Tween, TweenManager, TweenOptions};

// Source: upstream/packages/tween/src/tweenStagger.ts:5 (sha256:ba960751ea24df55f05f1205d33acaf88f17c7d0c860e0cc564d667cd2f97448)
#[derive(Clone, Default)]
pub struct TweenStaggerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub each: Option<f64>,
    pub from: Option<crate::FlightUnion2<String, f64>>,
    pub stagger_ease: Option<EasingFunction>,
}
impl PartialEq for TweenStaggerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/tween/src/tweenStagger.ts:30 (sha256:5c65bbeea71ecdfe4104501281840d0038b37920c4c55d02c4dd472074052451)
pub fn create_tween_stagger<T: Clone>(
    manager: &TweenManager,
    targets: &Vec<T>,
    duration: f64,
    property_map: NumericProps,
    stagger: Option<TweenStaggerOptions>,
    options: Option<TweenOptions>,
) -> Vec<Tween<T>> {
    if ((targets.len() as f64) == 0.0_f64) {
        return vec![];
    }
    let each = (stagger.as_ref().and_then(|value| value.each)).unwrap_or(0.1_f64);
    let from = (stagger.as_ref().and_then(|value| (value.from).clone()))
        .unwrap_or(crate::FlightUnion2::<String, f64>::A("start".to_owned()));
    let stagger_ease = stagger
        .as_ref()
        .and_then(|value| (value.stagger_ease).clone());
    let base_delay = (options.as_ref().and_then(|value| value.delay)).unwrap_or(0.0_f64);
    let count = (targets.len() as f64);
    let mut tweens: Vec<Tween<T>> = vec![];
    {
        let mut i = 0.0_f64;
        while (i < count) {
            let stagger_offset = compute_stagger_delay(
                i,
                count,
                each,
                &((from).clone()),
                Some(((stagger_ease).clone().unwrap()).clone()),
            );
            let tween = create_tween(
                &(crate::FlightUnion2::<TweenManager, T>::A((*manager).clone())),
                &(crate::FlightUnion2::<T, f64>::A(targets[i as usize].clone())),
                &(crate::FlightUnion2::<f64, NumericProps>::A(duration)),
                Some(
                    (crate::FlightUnion2::<NumericProps, TweenOptions>::A((property_map).clone()))
                        .clone(),
                ),
                Some(TweenOptions {
                    delay: Some((base_delay + stagger_offset)),
                    ..((options).clone().unwrap()).clone()
                }),
            );
            tweens.push(((tween).clone()).clone());
            {
                i += 1.0;
                i
            };
        }
    }
    return tweens;
}

// Source: upstream/packages/tween/src/tweenStagger.ts:56 (sha256:cd824d6aa67acb78269706a8ee8f64f670e53695f650a76fe4eae26a36e20f31)
fn compute_stagger_delay(
    index: f64,
    count: f64,
    each: f64,
    from: &crate::FlightUnion2<String, f64>,
    stagger_ease: Option<EasingFunction>,
) -> f64 {
    if (count <= 1.0_f64) {
        return 0.0_f64;
    }
    let mut normalized_position: f64;
    if (from == "start") {
        normalized_position = (index / (count - 1.0_f64));
    } else {
        if (from == "end") {
            normalized_position = (((count - 1.0_f64) - index) / (count - 1.0_f64));
        } else {
            if (from == "center") {
                let center = ((count - 1.0_f64) / 2.0_f64);
                normalized_position = ((index - center).abs() / center);
            } else {
                let origin = (0.0_f64).max((from).min((count - 1.0_f64)));
                let max_distance = (origin).max(((count - 1.0_f64) - origin));
                normalized_position = if (max_distance > 0.0_f64) {
                    ((index - origin).abs() / max_distance)
                } else {
                    0.0_f64
                };
            }
        }
    }
    let eased = if (stagger_ease).is_some() {
        {
            let __flight_callback = (stagger_ease.as_ref().unwrap()).clone();
            let __flight_result = __flight_callback.lock().unwrap()(normalized_position);
            __flight_result
        }
    } else {
        normalized_position
    };
    return ((eased * each) * (count - 1.0_f64));
}
