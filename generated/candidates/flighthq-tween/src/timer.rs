// @generated from upstream/packages/tween/src/timer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_tween;
use flighthq_types::{NumericProps, Tween, TweenManager, TweenOptions};

// Source: upstream/packages/tween/src/timer.ts:5 (sha256:564457e1f2c13eae1faba6d86f5e319b34259a415c1b8f78c89bfa63f46de08f)
#[derive(Clone, Default)]
struct CreateTweenTimerRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateTweenTimerRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_tween_timer(
    manager: &TweenManager,
    duration: f64,
    options: Option<TweenOptions>,
) -> Tween<crate::OpaqueHostValue> {
    return create_tween(
        &(crate::FlightUnion2::<TweenManager, crate::OpaqueHostValue>::A((*manager).clone())),
        &(CreateTweenTimerRecord1 {
            __flight_identity: std::sync::Arc::new(()),
        }),
        &(crate::FlightUnion2::<f64, NumericProps>::A(duration)),
        Some(crate::FlightUnion2::<NumericProps, TweenOptions>::B(
            TweenOptions {
                __flight_identity: std::sync::Arc::new(()),
                delay: None,
                ease: None,
                overwrite: None,
                reflect: None,
                repeat: None,
                reverse: None,
                smart_rotation: None,
                snapping: None,
            },
        )),
        Some(((options).clone().unwrap()).clone()),
    );
}
