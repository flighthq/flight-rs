// @generated from upstream/packages/effects/src/medianEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::register_render_effect_padding_resolver;
use flighthq_types::{MedianEffect, RenderEffect, RenderEffectPadding, RenderState};

#[derive(Clone, Default)]
pub struct FlightOmitRecord318962587 {
    pub __flight_identity: std::sync::Arc<()>,
    pub radius: Option<f64>,
}
impl PartialEq for FlightOmitRecord318962587 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/medianEffect.ts:5 (sha256:242f61ead3733bc4a73a7602e314a77353bb6dff116c0b4df976d02486f19625)
#[derive(Clone, Default)]
struct CreateMedianEffectRecord10 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateMedianEffectRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_median_effect(options: Option<FlightOmitRecord318962587>) -> MedianEffect {
    let options = options.unwrap_or(FlightOmitRecord318962587 {
        __flight_identity: std::sync::Arc::new(()),
        radius: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        MedianEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "MedianEffect".to_owned(),
            radius: __flight_spread_1.radius,
            ..Default::default()
        }
    };
}

// Source: upstream/packages/effects/src/medianEffect.ts:9 (sha256:2c9ef796c95dd5b0549c74da8da910a37bfdaf5a13fa1ff668d02b6adf62e379)
pub fn get_median_effect_padding(effect: &MedianEffect) -> RenderEffectPadding {
    let radius = (0.0_f64).max(((effect.radius).unwrap_or(1.0_f64)).round());
    return RenderEffectPadding {
        __flight_identity: std::sync::Arc::new(()),
        bottom: radius,
        left: radius,
        right: radius,
        top: radius,
    };
}

// Source: upstream/packages/effects/src/medianEffect.ts:14 (sha256:149004403c4a80d9ae5af7623dd018e046186231d5ef2548fcd03c8deb1c20cb)
pub fn register_median_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "MedianEffect".to_owned(),
        &(resolve_median_effect_padding),
    );
}

// Source: upstream/packages/effects/src/medianEffect.ts:18 (sha256:a8f489c4c67bc9a436b12c1abb1965327f70d2c9862fbd61d182cabdfa64b356)
fn resolve_median_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_median_effect_padding(&{
        let __flight_source = &((*effect).clone());
        MedianEffect {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            radius: __flight_source.radius,
            ..Default::default()
        }
    });
}
