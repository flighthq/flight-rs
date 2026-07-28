// @generated from upstream/packages/effects/src/autoExposureEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::AutoExposureEffect;

// Source: upstream/packages/effects/src/autoExposureEffect.ts:3 (sha256:747c0be704448251bac3207e5d4e1c414cd5a0fffcf06e7482a98db6f499be88)
#[derive(Clone)]
struct CreateAutoExposureEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateAutoExposureEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_auto_exposure_effect(options: Option<AutoExposureEffect>) -> AutoExposureEffect {
    let options = options.unwrap_or(AutoExposureEffect {
        __flight_identity: std::sync::Arc::new(()),
        adaptation_speed: None,
        exposure_compensation: None,
        max_exposure: None,
        min_exposure: None,
    });
    return AutoExposureEffect {
        kind: "AutoExposureEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
