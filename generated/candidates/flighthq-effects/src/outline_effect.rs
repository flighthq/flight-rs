// @generated from upstream/packages/effects/src/outlineEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::OutlineEffect;

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub threshold: Option<f64>,
    pub thickness: Option<f64>,
    pub color: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/outlineEffect.ts:3 (sha256:f7d12c4a36e281c4b105db315466a95bd7146bfe1235db79e3ac2ac54022e1b8)
#[derive(Clone)]
struct CreateOutlineEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateOutlineEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_outline_effect(options: Option<FlightOmitRecord1>) -> OutlineEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        threshold: None,
        thickness: None,
        color: None,
    });
    return {
        let __flight_spread_1 = options;
        OutlineEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "OutlineEffect".to_owned(),
            threshold: __flight_spread_1.threshold,
            thickness: __flight_spread_1.thickness,
            color: __flight_spread_1.color,
        }
    };
}
