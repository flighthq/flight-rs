// @generated from upstream/packages/effects/src/kuwaharaEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::KuwaharaEffect;

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub radius: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/kuwaharaEffect.ts:3 (sha256:0c5c8da8cbcd8628aa61679b9b5ef5c415acf1a54c396023db9c0171f325260a)
#[derive(Clone)]
struct CreateKuwaharaEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateKuwaharaEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_kuwahara_effect(options: Option<FlightOmitRecord1>) -> KuwaharaEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        radius: None,
    });
    return {
        let __flight_spread_1 = options;
        KuwaharaEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "KuwaharaEffect".to_owned(),
            radius: __flight_spread_1.radius,
        }
    };
}
