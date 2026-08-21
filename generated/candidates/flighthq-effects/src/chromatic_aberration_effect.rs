// @generated from upstream/packages/effects/src/chromaticAberrationEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ChromaticAberrationEffect;

#[derive(Clone, Default)]
pub struct FlightOmitRecord1277959374 {
    pub __flight_identity: std::sync::Arc<()>,
    pub intensity: Option<f64>,
    pub radial: Option<bool>,
}
impl PartialEq for FlightOmitRecord1277959374 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/chromaticAberrationEffect.ts:3 (sha256:456450d4b5d8fd416d14fc8ec9a8ec513ca50ec885dd193183d640ea2b163629)
#[derive(Clone, Default)]
struct CreateChromaticAberrationEffectRecord2 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateChromaticAberrationEffectRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_chromatic_aberration_effect(
    options: Option<FlightOmitRecord1277959374>,
) -> ChromaticAberrationEffect {
    let options = options.unwrap_or(FlightOmitRecord1277959374 {
        __flight_identity: std::sync::Arc::new(()),
        intensity: None,
        radial: None,
    });
    return {
        let __flight_spread_1 = (options).clone();
        ChromaticAberrationEffect {
            __flight_identity: std::sync::Arc::new(()),
            kind: "ChromaticAberrationEffect".to_owned(),
            intensity: __flight_spread_1.intensity,
            radial: __flight_spread_1.radial,
            ..Default::default()
        }
    };
}
