// @generated from upstream/packages/effects/src/displacementEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::register_render_effect_padding_resolver;
use flighthq_types::{DisplacementEffect, RenderEffect, RenderEffectPadding, RenderState};

#[derive(Clone, Default)]
pub struct FlightOmitRecord1487511957 {
    pub __flight_identity: std::sync::Arc<()>,
    pub intensity: Option<f64>,
    pub frequency: Option<f64>,
    pub seed: Option<f64>,
}
impl PartialEq for FlightOmitRecord1487511957 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/displacementEffect.ts:5 (sha256:afd4c94e0ffcb042de03804d5b8d728e4525beb0996e75687a5259ff6e585183)
#[derive(Clone, Default)]
struct CreateDisplacementEffectRecord10 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateDisplacementEffectRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_displacement_effect(
    options: Option<FlightOmitRecord1487511957>,
) -> DisplacementEffect {
    let options = options.unwrap_or(FlightOmitRecord1487511957 {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        frequency: None,
        seed: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        DisplacementEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "DisplacementEffect".to_owned(),
            intensity: __flight_spread_1.intensity,
            frequency: __flight_spread_1.frequency,
            seed: __flight_spread_1.seed,
            ..Default::default()
        }
    };
}

// Source: upstream/packages/effects/src/displacementEffect.ts:9 (sha256:2d7edb8836a6c12247ca65fddd0ab3a8838823a145ca4dccc8ae3f32dabae2b4)
pub fn get_displacement_effect_padding(effect: &DisplacementEffect) -> RenderEffectPadding {
    let intensity = ((effect.intensity).unwrap_or(8.0_f64)).abs();
    let horizontal = (intensity * 1.5_f64).ceil();
    let vertical = (intensity).ceil();
    return RenderEffectPadding {
        __flight_identity: std::sync::Arc::new(()),
        bottom: vertical,
        left: horizontal,
        right: horizontal,
        top: vertical,
    };
}

// Source: upstream/packages/effects/src/displacementEffect.ts:16 (sha256:3b84c65534079002e345a21bb5f992d0306bf29ae9797c55d0f8cf54c7794461)
pub fn register_displacement_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "DisplacementEffect".to_owned(),
        &(resolve_displacement_effect_padding),
    );
}

// Source: upstream/packages/effects/src/displacementEffect.ts:20 (sha256:16da86e2886130d920473a93e08fc71a8d0c954e94c33368ecb6412980dffbfb)
fn resolve_displacement_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_displacement_effect_padding(&{
        let __flight_source = &((*effect).clone());
        DisplacementEffect {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            intensity: __flight_source.intensity,
            frequency: __flight_source.frequency,
            seed: __flight_source.seed,
            ..Default::default()
        }
    });
}
