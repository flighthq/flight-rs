// @generated from upstream/packages/effects/src/fxaaEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::FxaaEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub edge_threshold: Option<f64>,
    pub subpixel: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/fxaaEffect.ts:3 (sha256:238a02304c004187854c6b923e219342a070d34b4315aa3251a8725eb037fd13)
#[derive(Clone, Default)]
struct CreateFxaaEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateFxaaEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_fxaa_effect(options: Option<FlightOmitRecord1>) -> FxaaEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        edge_threshold: None,
        subpixel: None,
    });
    return {
        let __flight_spread_1 = options;
        FxaaEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "FxaaEffect".to_owned(),
            edge_threshold: __flight_spread_1.edge_threshold,
            subpixel: __flight_spread_1.subpixel,
            ..Default::default()
        }
    };
}
