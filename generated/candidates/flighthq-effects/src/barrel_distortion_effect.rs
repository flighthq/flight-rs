// @generated from upstream/packages/effects/src/barrelDistortionEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::BarrelDistortionEffect;

// Source: upstream/packages/effects/src/barrelDistortionEffect.ts:3 (sha256:ff0c477d6ebfcc411a27e6bd6821f14499dd408424a1de8104a3d5b67093846e)
#[derive(Clone)]
struct CreateBarrelDistortionEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateBarrelDistortionEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_barrel_distortion_effect(
    options: Option<BarrelDistortionEffect>,
) -> BarrelDistortionEffect {
    let options = options.unwrap_or(BarrelDistortionEffect {
        __flight_identity: std::sync::Arc::new(()),
        amount: None,
        scale: None,
    });
    return BarrelDistortionEffect {
        kind: "BarrelDistortionEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
