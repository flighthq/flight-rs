// @generated from upstream/packages/effects/src/posterizeEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::PosterizeEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub levels: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/posterizeEffect.ts:3 (sha256:b10227aa045230cc1a29b322f74d83f21cf6ba06a76918beed134fd9eb4f9e45)
#[derive(Clone, Default)]
struct CreatePosterizeEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreatePosterizeEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_posterize_effect(options: Option<FlightOmitRecord1>) -> PosterizeEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        levels: None,
    });
    return {
        let __flight_spread_1 = options;
        PosterizeEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "PosterizeEffect".to_owned(),
            levels: __flight_spread_1.levels,
            ..Default::default()
        }
    };
}
