// @generated from upstream/packages/effects/src/glitchEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::GlitchEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub intensity: Option<f64>,
    pub block_size: Option<f64>,
    pub color_shift: Option<f64>,
    pub seed: Option<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/glitchEffect.ts:3 (sha256:03b2073852f91ad105439e1f2e07636700322abc0e950f3007c8e1f4ef056073)
#[derive(Clone, Default)]
struct CreateGlitchEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateGlitchEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_glitch_effect(options: Option<FlightOmitRecord1>) -> GlitchEffect {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        block_size: None,
        color_shift: None,
        seed: None,
    });
    return {
        let __flight_spread_1 = options;
        GlitchEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "GlitchEffect".to_owned(),
            intensity: __flight_spread_1.intensity,
            block_size: __flight_spread_1.block_size,
            color_shift: __flight_spread_1.color_shift,
            seed: __flight_spread_1.seed,
            ..Default::default()
        }
    };
}
