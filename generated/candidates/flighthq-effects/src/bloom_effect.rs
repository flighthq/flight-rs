// @generated from upstream/packages/effects/src/bloomEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_gaussian_render_effect_padding, register_render_effect_padding_resolver};
use flighthq_types::{BloomEffect, RenderEffect, RenderEffectPadding, RenderState};

#[derive(Clone, Default)]
pub struct FlightOmitRecord537432717 {
    pub __flight_identity: std::sync::Arc<()>,
    pub threshold: Option<f64>,
    pub intensity: Option<f64>,
    pub radius: Option<f64>,
    pub passes: Option<f64>,
}
impl PartialEq for FlightOmitRecord537432717 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/bloomEffect.ts:9 (sha256:f34073dbd363e96e331b2a1ba428bb190daccbacf4c0dbd957021b90b65025fd)
pub fn compute_bloom_blur_radius(effect: &BloomEffect) -> f64 {
    return (0.0_f64).max((effect.radius).clone().unwrap_or(8.0_f64));
}

// Source: upstream/packages/effects/src/bloomEffect.ts:13 (sha256:4a39a4516008094d20a658965d4a461ad85821a29fc50ae8aeecc9a69bd381f7)
pub fn compute_bloom_intensity(effect: &BloomEffect) -> f64 {
    return (effect.intensity).clone().unwrap_or(1.0_f64);
}

// Source: upstream/packages/effects/src/bloomEffect.ts:17 (sha256:cd461a958a93139b3234d1695ca5bfc70a5c93c0989ea26a312389baa4929218)
pub fn compute_bloom_threshold(effect: &BloomEffect) -> f64 {
    return (effect.threshold).clone().unwrap_or(0.8_f64);
}

// Source: upstream/packages/effects/src/bloomEffect.ts:21 (sha256:617321597270316572516fdf538933545aea4fa2d8cd22b426fc4e882207e95f)
#[derive(Clone, Default)]
struct CreateBloomEffectRecord10 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateBloomEffectRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_bloom_effect(options: Option<FlightOmitRecord537432717>) -> BloomEffect {
    let options = options.unwrap_or(FlightOmitRecord537432717 {
        __flight_identity: std::sync::Arc::new(()),
        threshold: None,
        intensity: None,
        radius: None,
        passes: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        BloomEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "BloomEffect".to_owned(),
            threshold: __flight_spread_1.threshold,
            intensity: __flight_spread_1.intensity,
            radius: __flight_spread_1.radius,
            passes: __flight_spread_1.passes,
            ..Default::default()
        }
    };
}

// Source: upstream/packages/effects/src/bloomEffect.ts:25 (sha256:c9fb4dc9d651e5014aa8e911a2b6a0e80a8bf8b72712df17b40a98fce86d6b53)
pub fn get_bloom_effect_padding(effect: &BloomEffect) -> RenderEffectPadding {
    let radius = compute_bloom_blur_radius(effect);
    return get_gaussian_render_effect_padding(radius, radius);
}

// Source: upstream/packages/effects/src/bloomEffect.ts:30 (sha256:305453f734590afb0432ace0f882949597ccc7507288ed813ea898955244bb0d)
pub fn register_bloom_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "BloomEffect".to_owned(),
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: RenderEffect| -> RenderEffectPadding {
                resolve_bloom_effect_padding(&__flight_argument_0)
            },
        )
            as Box<
                dyn FnMut(RenderEffect) -> RenderEffectPadding + Send + 'static,
            >)))),
    );
}

// Source: upstream/packages/effects/src/bloomEffect.ts:34 (sha256:606e073b1a36007b715d4d7d371602364970101a663b297a791b6c12a5c13217)
fn resolve_bloom_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_bloom_effect_padding(&{
        let __flight_source = &((*effect).clone());
        BloomEffect {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            threshold: __flight_source.threshold,
            intensity: __flight_source.intensity,
            radius: __flight_source.radius,
            passes: __flight_source.passes,
            ..Default::default()
        }
    });
}
