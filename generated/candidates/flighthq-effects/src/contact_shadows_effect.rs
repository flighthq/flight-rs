// @generated from upstream/packages/effects/src/contactShadowsEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::register_render_effect_padding_resolver;
use flighthq_types::{ContactShadowsEffect, RenderEffect, RenderEffectPadding, RenderState};

#[derive(Clone, Default)]
pub struct FlightOmitRecord3390735950 {
    pub __flight_identity: std::sync::Arc<()>,
    pub distance: Option<f64>,
    pub opacity: Option<f64>,
    pub samples: Option<f64>,
    pub smoothness: Option<f64>,
}
impl PartialEq for FlightOmitRecord3390735950 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/contactShadowsEffect.ts:5 (sha256:4bc30aa21b51ac6d1144ef6e52410b9010dfd548130f08c88f4b071dbdaa3dd3)
#[derive(Clone, Default)]
struct CreateContactShadowsEffectRecord10 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateContactShadowsEffectRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_contact_shadows_effect(
    options: Option<FlightOmitRecord3390735950>,
) -> ContactShadowsEffect {
    let options = options.unwrap_or(FlightOmitRecord3390735950 {
        __flight_identity: std::sync::Arc::new(()),
        distance: None,
        opacity: None,
        samples: None,
        smoothness: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        ContactShadowsEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "ContactShadowsEffect".to_owned(),
            distance: __flight_spread_1.distance,
            opacity: __flight_spread_1.opacity,
            samples: __flight_spread_1.samples,
            smoothness: __flight_spread_1.smoothness,
            ..Default::default()
        }
    };
}

// Source: upstream/packages/effects/src/contactShadowsEffect.ts:12 (sha256:ba29f48595bb0a5adf3447f4e643460f9bb3259ec40ccff504004f3031955dc7)
#[derive(Clone, Default)]
struct GetContactShadowsEffectPaddingRecord10 {
    __flight_identity: std::sync::Arc<()>,
    bottom: f64,
    left: f64,
    right: f64,
    top: f64,
}
impl PartialEq for GetContactShadowsEffectPaddingRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn get_contact_shadows_effect_padding(_effect: &ContactShadowsEffect) -> RenderEffectPadding {
    return RenderEffectPadding {
        __flight_identity: std::sync::Arc::new(()),
        bottom: 0.0_f64,
        left: 0.0_f64,
        right: 0.0_f64,
        top: 0.0_f64,
    };
}

// Source: upstream/packages/effects/src/contactShadowsEffect.ts:16 (sha256:8da31227e4448d2dea0905421d910eabbc7a5595d257a03868182566caf3a4bb)
pub fn register_contact_shadows_effect_padding_resolver(state: &RenderState) -> () {
    register_render_effect_padding_resolver(
        state,
        "ContactShadowsEffect".to_owned(),
        &(resolve_contact_shadows_effect_padding),
    );
}

// Source: upstream/packages/effects/src/contactShadowsEffect.ts:20 (sha256:835df9ef0639539de6a73f48e3553eeedb0d8b8c366436c14982e223bc4c7a03)
fn resolve_contact_shadows_effect_padding(effect: &RenderEffect) -> RenderEffectPadding {
    return get_contact_shadows_effect_padding(&{
        let __flight_source = &((*effect).clone());
        ContactShadowsEffect {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            distance: __flight_source.distance,
            opacity: __flight_source.opacity,
            samples: __flight_source.samples,
            smoothness: __flight_source.smoothness,
            ..Default::default()
        }
    });
}
