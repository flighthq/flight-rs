// @generated from upstream/packages/effects/src/tiltShiftEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::TiltShiftEffect;

// Source: upstream/packages/effects/src/tiltShiftEffect.ts:3 (sha256:2ff62cd921c5f37d54bffaed6b7d2015ef8107f8a5f8ca2c084ded52800c023d)
#[derive(Clone)]
struct CreateTiltShiftEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateTiltShiftEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_tilt_shift_effect(options: Option<TiltShiftEffect>) -> TiltShiftEffect {
    let options = options.unwrap_or(TiltShiftEffect {
        __flight_identity: std::sync::Arc::new(()),
        center: None,
        width: None,
        blur: None,
    });
    return TiltShiftEffect {
        kind: "TiltShiftEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
