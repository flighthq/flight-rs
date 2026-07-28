// @generated from upstream/packages/effects/src/lensFlareEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::LensFlareEffect;

// Source: upstream/packages/effects/src/lensFlareEffect.ts:3 (sha256:59979a471e4b460a4183b4739fd11756d4610ff8c7ddeae72ca27034a6283cc6)
#[derive(Clone)]
struct CreateLensFlareEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateLensFlareEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_lens_flare_effect(options: Option<LensFlareEffect>) -> LensFlareEffect {
    let options = options.unwrap_or(LensFlareEffect {
        __flight_identity: std::sync::Arc::new(()),
        threshold: None,
        intensity: None,
        ghosts: None,
        halo: None,
    });
    return LensFlareEffect {
        kind: "LensFlareEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
