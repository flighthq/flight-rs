// @generated from upstream/packages/effects/src/glitchEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::register_render_effect_padding_resolver;
use flighthq_types::{GlitchEffect, RenderEffect, RenderEffectPadding, RenderState};

#[derive(Clone, Default)]
pub struct FlightOmitRecord1502164903 {
    pub __flight_identity: std::sync::Arc<()>,
    pub intensity: Option<f64>,
    pub block_size: Option<f64>,
    pub color_shift: Option<f64>,
    pub seed: Option<f64>,
}
impl PartialEq for FlightOmitRecord1502164903 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/glitchEffect.ts:5 (sha256:03b2073852f91ad105439e1f2e07636700322abc0e950f3007c8e1f4ef056073)
#[derive(Clone, Default)]
struct CreateGlitchEffectRecord10 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateGlitchEffectRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_glitch_effect(options: Option<FlightOmitRecord1502164903>) -> GlitchEffect {
    let options = options.unwrap_or(FlightOmitRecord1502164903 {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        block_size: None,
        color_shift: None,
        seed: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        GlitchEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "GlitchEffect".to_owned(),
            intensity: __flight_spread_1.intensity,
            block_size: __flight_spread_1.block_size,
            color_shift: __flight_spread_1.color_shift,
            seed: __flight_spread_1.seed,
            ..Default::default()
        }
    };
}

// Source: upstream/packages/effects/src/glitchEffect.ts:9 (sha256:f344cf86d3d2f2130edaee660825fe2b797bf48467fa3f2890660e2a7b9750a9)
pub fn get_glitch_effect_padding(effect: &GlitchEffect) -> RenderEffectPadding {
    let tear = (((effect.intensity).clone().unwrap_or(0.5_f64)).abs() * 40.0_f64);
    let channel_shift = (((effect.color_shift).clone().unwrap_or(8.0_f64)).abs() * 1.4_f64);
    let horizontal = (tear + channel_shift).ceil();
    return RenderEffectPadding {
        __flight_identity: std::sync::Arc::new(()),
        bottom: 0.0_f64,
        left: horizontal,
        right: horizontal,
        top: 0.0_f64,
    };
}

// Source: upstream/packages/effects/src/glitchEffect.ts:16 (sha256:4356dc2d2b9a05416209b2a5c4663a8a1e335d2f82dd1596534d69a5d45fc5f3)
pub fn register_glitch_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "GlitchEffect".to_owned(),
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: RenderEffect| -> RenderEffectPadding {
                resolve_glitch_effect_padding(&__flight_argument_0)
            },
        )
            as Box<
                dyn FnMut(RenderEffect) -> RenderEffectPadding + Send + 'static,
            >)))),
    );
}

// Source: upstream/packages/effects/src/glitchEffect.ts:20 (sha256:37e9afaca0d485966e9320c26568e775aadeeaeed98e496fdec0e1928789b112)
fn resolve_glitch_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_glitch_effect_padding(&{
        let __flight_source = &((*effect).clone());
        GlitchEffect {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            intensity: __flight_source.intensity,
            block_size: __flight_source.block_size,
            color_shift: __flight_source.color_shift,
            seed: __flight_source.seed,
            ..Default::default()
        }
    });
}
