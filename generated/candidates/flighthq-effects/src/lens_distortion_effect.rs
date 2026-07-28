// @generated from upstream/packages/effects/src/lensDistortionEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::LensDistortionEffect;

// Source: upstream/packages/effects/src/lensDistortionEffect.ts:3 (sha256:3c272223b98303b3de96ea3f91824da1e932424f3a5eda24672deda3d98e0062)
#[derive(Clone)]
struct CreateLensDistortionEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateLensDistortionEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_lens_distortion_effect(
    options: Option<LensDistortionEffect>,
) -> LensDistortionEffect {
    let options = options.unwrap_or(LensDistortionEffect {
        __flight_identity: std::sync::Arc::new(()),
        amount: None,
        scale: None,
    });
    return LensDistortionEffect {
        kind: "LensDistortionEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
