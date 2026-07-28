// @generated from upstream/packages/effects/src/screenSpaceFogEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ScreenSpaceFogEffect;

// Source: upstream/packages/effects/src/screenSpaceFogEffect.ts:3 (sha256:6a0bbf280ed41f5d3c962e3862f956fe874b6e6e3fec9362c09c85d2e424dd5e)
#[derive(Clone)]
struct CreateScreenSpaceFogEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateScreenSpaceFogEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_screen_space_fog_effect(
    options: Option<ScreenSpaceFogEffect>,
) -> ScreenSpaceFogEffect {
    let options = options.unwrap_or(ScreenSpaceFogEffect {
        __flight_identity: std::sync::Arc::new(()),
        color: None,
        near: None,
        far: None,
        density: None,
    });
    return ScreenSpaceFogEffect {
        kind: "ScreenSpaceFogEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
