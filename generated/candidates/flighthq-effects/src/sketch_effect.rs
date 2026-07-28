// @generated from upstream/packages/effects/src/sketchEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::SketchEffect;

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub strength: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/sketchEffect.ts:3 (sha256:9f94bede0b61869b7bd3c4dd51809a07dcd6daf5562af4bae2eb181dd4e309cd)
#[derive(Clone)]
struct CreateSketchEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateSketchEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_sketch_effect(options: Option<FlightOmitRecord1>) -> SketchEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        strength: None,
    });
    return {
        let __flight_spread_1 = options;
        SketchEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "SketchEffect".to_owned(),
            strength: __flight_spread_1.strength,
        }
    };
}
