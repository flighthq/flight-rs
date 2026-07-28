// @generated from upstream/packages/effects/src/contactShadowsEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ContactShadowsEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub distance: Option<f64>,
    pub opacity: Option<f64>,
    pub samples: Option<f64>,
    pub smoothness: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/contactShadowsEffect.ts:3 (sha256:4bc30aa21b51ac6d1144ef6e52410b9010dfd548130f08c88f4b071dbdaa3dd3)
#[derive(Clone, Default)]
struct CreateContactShadowsEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateContactShadowsEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_contact_shadows_effect(options: Option<FlightOmitRecord1>) -> ContactShadowsEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        distance: None,
        opacity: None,
        samples: None,
        smoothness: None,
    });
    return {
        let __flight_spread_1 = options;
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
