// @generated from upstream/packages/effects/src/outlineEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::register_render_effect_padding_resolver;
use flighthq_types::{OutlineEffect, RenderEffect, RenderEffectPadding, RenderState};

#[derive(Clone, Default)]
pub struct FlightOmitRecord3030666237 {
    pub __flight_identity: std::sync::Arc<()>,
    pub threshold: Option<f64>,
    pub thickness: Option<f64>,
    pub color: Option<f64>,
}
impl PartialEq for FlightOmitRecord3030666237 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/outlineEffect.ts:5 (sha256:f7d12c4a36e281c4b105db315466a95bd7146bfe1235db79e3ac2ac54022e1b8)
#[derive(Clone, Default)]
struct CreateOutlineEffectRecord10 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateOutlineEffectRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_outline_effect(options: Option<FlightOmitRecord3030666237>) -> OutlineEffect {
    let options = options.unwrap_or(FlightOmitRecord3030666237 {
        __flight_identity: std::sync::Arc::new(()),
        threshold: None,
        thickness: None,
        color: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        OutlineEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "OutlineEffect".to_owned(),
            threshold: __flight_spread_1.threshold,
            thickness: __flight_spread_1.thickness,
            color: __flight_spread_1.color,
            ..Default::default()
        }
    };
}

// Source: upstream/packages/effects/src/outlineEffect.ts:9 (sha256:0f3a94035a4100b31452e51bdd8523363948cd316f10a033fdbb297694771bbd)
pub fn get_outline_effect_padding(effect: &OutlineEffect) -> RenderEffectPadding {
    let thickness = ((0.0_f64).max((effect.thickness).clone().unwrap_or(1.0_f64))).ceil();
    return RenderEffectPadding {
        __flight_identity: std::sync::Arc::new(()),
        bottom: thickness,
        left: thickness,
        right: thickness,
        top: thickness,
    };
}

// Source: upstream/packages/effects/src/outlineEffect.ts:14 (sha256:43d1cfd93423a8155a1b20e1c0ce1370caacda462d9f1426f449db92b00e74ce)
pub fn register_outline_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "OutlineEffect".to_owned(),
        &(Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: RenderEffect| -> RenderEffectPadding {
                resolve_outline_effect_padding(&__flight_argument_0)
            },
        )
            as Box<
                dyn FnMut(RenderEffect) -> RenderEffectPadding + Send + 'static,
            >)))),
    );
}

// Source: upstream/packages/effects/src/outlineEffect.ts:18 (sha256:c90cf33d52f42e0ccf0d6f8e89e271296ef15a6672c3c460b194575d2dcffac1)
fn resolve_outline_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_outline_effect_padding(&{
        let __flight_source = &((*effect).clone());
        OutlineEffect {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            threshold: __flight_source.threshold,
            thickness: __flight_source.thickness,
            color: __flight_source.color,
            ..Default::default()
        }
    });
}
