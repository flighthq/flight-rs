// @generated from upstream/packages/effects/src/tiltShiftEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::register_render_effect_padding_resolver;
use flighthq_types::{RenderEffect, RenderEffectPadding, RenderState, TiltShiftEffect};

#[derive(Clone, Default)]
pub struct FlightOmitRecord2646392041 {
    pub __flight_identity: std::sync::Arc<()>,
    pub center: Option<f64>,
    pub width: Option<f64>,
    pub blur: Option<f64>,
}
impl PartialEq for FlightOmitRecord2646392041 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/tiltShiftEffect.ts:5 (sha256:2ff62cd921c5f37d54bffaed6b7d2015ef8107f8a5f8ca2c084ded52800c023d)
#[derive(Clone, Default)]
struct CreateTiltShiftEffectRecord10 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateTiltShiftEffectRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_tilt_shift_effect(options: Option<FlightOmitRecord2646392041>) -> TiltShiftEffect {
    let options = options.unwrap_or(FlightOmitRecord2646392041 {
        __flight_identity: std::sync::Arc::new(()),
        center: None,
        width: None,
        blur: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        TiltShiftEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "TiltShiftEffect".to_owned(),
            center: __flight_spread_1.center,
            width: __flight_spread_1.width,
            blur: __flight_spread_1.blur,
            ..Default::default()
        }
    };
}

// Source: upstream/packages/effects/src/tiltShiftEffect.ts:9 (sha256:2a5b7bc34d13ede313ead5958317ed6e3422f59d7d282ef54a27b69d2f824fdb)
pub fn get_tilt_shift_effect_padding(effect: &TiltShiftEffect) -> RenderEffectPadding {
    let vertical = ((0.0_f64).max((effect.blur).clone().unwrap_or(4.0_f64)) * 3.0_f64).ceil();
    return RenderEffectPadding {
        __flight_identity: std::sync::Arc::new(()),
        bottom: vertical,
        left: 0.0_f64,
        right: 0.0_f64,
        top: vertical,
    };
}

// Source: upstream/packages/effects/src/tiltShiftEffect.ts:14 (sha256:552bbe3b57d51af21eb3e3fcb4d00062cf82d7f28f7add599504e8cccaa6dec2)
pub fn register_tilt_shift_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "TiltShiftEffect".to_owned(),
        &(resolve_tilt_shift_effect_padding),
    );
}

// Source: upstream/packages/effects/src/tiltShiftEffect.ts:18 (sha256:31c1ac3b51e158d38187dfdfcf544b136f88c14e3e561b4967a795a89786ac09)
fn resolve_tilt_shift_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_tilt_shift_effect_padding(&{
        let __flight_source = &((*effect).clone());
        TiltShiftEffect {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            center: __flight_source.center,
            width: __flight_source.width,
            blur: __flight_source.blur,
            ..Default::default()
        }
    });
}
