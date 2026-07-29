// @generated from upstream/packages/tween/src/colorTween.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_tween;
use flighthq_signals::connect_signal;
use flighthq_types::{NumericProps, Tween, TweenManager, TweenOptions};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

// Source: upstream/packages/tween/src/colorTween.ts:6 (sha256:bc7f5f2152b7ca8c364c45401352d08ef4a55c82f02db72d671750f5a2a3dfd5)
#[derive(Clone, Default)]
struct ColorComponents {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub b: f64,
    pub g: f64,
    pub r: f64,
}
impl PartialEq for ColorComponents {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/tween/src/colorTween.ts:13 (sha256:f5ef8bf66a50098bd8c87508de0baaca9b24c36c50a3bb0f10c1813b692d2bac)
pub fn create_color_tween(
    manager: &TweenManager,
    mut target: Vec<(String, f64)>,
    property: String,
    duration: f64,
    to_color: f64,
    options: Option<TweenOptions>,
) -> Tween<ColorComponents> {
    let from_color = target
        .iter()
        .find(|(key, _)| key == &(property).clone())
        .map(|(_, value)| value)
        .expect("TypeScript Record key was absent")
        .clone();
    let components: ColorComponents = ColorComponents {
        __flight_identity: std::sync::Arc::new(()),
        b: (__flight_js_to_i32(from_color) & __flight_js_to_i32(255.0_f64)) as f64,
        g: (__flight_js_to_i32(
            (__flight_js_to_i32(from_color) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64,
        r: (__flight_js_to_i32(
            (__flight_js_to_i32(from_color) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
        ) & __flight_js_to_i32(255.0_f64)) as f64,
    };
    let mut tween = create_tween(
        &(crate::FlightUnion2::<TweenManager, T>::A((*manager).clone())),
        &(components),
        &(crate::FlightUnion2::<f64, NumericProps>::A(duration)),
        Some(ColorComponents {
            __flight_identity: std::sync::Arc::new(()),
            b: (__flight_js_to_i32(to_color) & __flight_js_to_i32(255.0_f64)) as f64,
            g: (__flight_js_to_i32(
                (__flight_js_to_i32(to_color) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
            ) & __flight_js_to_i32(255.0_f64)) as f64,
            r: (__flight_js_to_i32(
                (__flight_js_to_i32(to_color) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
            ) & __flight_js_to_i32(255.0_f64)) as f64,
        }),
        Some(((options).clone().unwrap()).clone()),
    );
    connect_signal(
        &mut tween.on_update,
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let components = components.clone();
            let mut target = target.clone();
            move || -> () {
                target
                    .iter()
                    .find(|(key, _)| key == &(property).clone())
                    .map(|(_, value)| value)
                    .expect("TypeScript Record key was absent") = (__flight_js_to_i32(
                    (__flight_js_to_i32(
                        __flight_js_to_i32(
                            (__flight_js_to_i32((components.r).round())
                                & __flight_js_to_i32(255.0_f64)) as f64,
                        )
                        .wrapping_shl((__flight_js_to_u32(16.0_f64) & 31))
                            as f64,
                    ) | __flight_js_to_i32(
                        __flight_js_to_i32(
                            (__flight_js_to_i32((components.g).round())
                                & __flight_js_to_i32(255.0_f64)) as f64,
                        )
                        .wrapping_shl((__flight_js_to_u32(8.0_f64) & 31))
                            as f64,
                    )) as f64,
                ) | __flight_js_to_i32(
                    (__flight_js_to_i32((components.b).round()) & __flight_js_to_i32(255.0_f64))
                        as f64,
                )) as f64;
            }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
        None,
    );
    return tween;
}
